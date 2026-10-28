// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Jareer Abdullah and Concat contributors

//! The provider keys, sealed with the Windows account.
//!
//! A key is not a project file: it must not travel with the footage, must
//! not sit in a folder anyone can open, and must not be readable by another
//! account on the same machine. DPAPI answers all three - the ciphertext is
//! bound to the Windows user, so a copied file is useless elsewhere - and
//! it is already part of Windows, so nothing is added to the build for it.
//!
//! One file per provider under `keys/`, named after the provider's id. The
//! plaintext is never written anywhere: [`save`] protects before writing,
//! [`load`] unprotects after reading, and a key that fails to unprotect -
//! another account's, another machine's - is treated as absent, because a
//! chat that cannot authenticate has to ask for the key again rather than
//! fail at the first message.
//!
//! Only Windows is wired. Elsewhere [`save`] refuses and [`load`] answers
//! nothing, which the settings pane shows as "not configured": the app is
//! built for four platforms and a chat that cannot keep a key must not
//! pretend it has one.

use std::path::{Path, PathBuf};

/// The file `provider`'s key is kept in, under `dir`.
pub fn path(dir: &Path, provider: &str) -> PathBuf {
    // A provider id becomes a file name here, so anything that is not one
    // is refused rather than escaped: ids are ours, not the file system's.
    let safe: String = provider
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect();
    dir.join("keys").join(format!("{safe}.bin"))
}

/// Whether a key is stored for `provider`. Says nothing about whether it
/// still unprotects: that is [`load`]'s to answer.
pub fn stored(dir: &Path, provider: &str) -> bool {
    path(dir, provider).is_file()
}

/// Stores `key` for `provider`, replacing whatever was there.
pub fn save(dir: &Path, provider: &str, key: &str) -> Result<(), String> {
    if key.trim().is_empty() {
        return forget(dir, provider);
    }
    let file = path(dir, provider);
    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("could not make {}: {error}", parent.display()))?;
    }
    let sealed = seal(key.trim().as_bytes())?;
    std::fs::write(&file, sealed).map_err(|error| format!("could not write {}: {error}", file.display()))
}

/// The key stored for `provider`, if there is one that still opens.
pub fn load(dir: &Path, provider: &str) -> Option<String> {
    let bytes = std::fs::read(path(dir, provider)).ok()?;
    let opened = unseal(&bytes).ok()?;
    String::from_utf8(opened).ok().filter(|key| !key.is_empty())
}

/// Removes the key stored for `provider`. A key that was never there is
/// not an error: the pane calls this whenever the field is emptied.
pub fn forget(dir: &Path, provider: &str) -> Result<(), String> {
    let file = path(dir, provider);
    match std::fs::remove_file(&file) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("could not remove {}: {error}", file.display())),
    }
}

/// The key as it goes into a header: the stored one, or nothing.
pub fn header_key(dir: &Path, provider: &str) -> Option<String> {
    load(dir, provider)
}

#[cfg(windows)]
mod dpapi {
    //! The two calls DPAPI offers, and the one that frees what it hands
    //! back. Declared here rather than through a binding crate: two
    //! functions and a struct do not need one.

    use std::ffi::c_void;

    #[repr(C)]
    struct Blob {
        size: u32,
        data: *mut u8,
    }

    // SAFETY: both take a blob pointing at `size` readable bytes and hand
    // back a blob the caller frees with `LocalFree`, which is what these
    // wrappers do.
    unsafe extern "system" {
        fn CryptProtectData(
            data_in: *const Blob,
            description: *const u16,
            entropy: *const Blob,
            reserved: *const c_void,
            prompt: *const c_void,
            flags: u32,
            data_out: *mut Blob,
        ) -> i32;
        fn CryptUnprotectData(
            data_in: *const Blob,
            description: *mut *mut u16,
            entropy: *const Blob,
            reserved: *const c_void,
            prompt: *const c_void,
            flags: u32,
            data_out: *mut Blob,
        ) -> i32;
        fn LocalFree(memory: *mut c_void) -> *mut c_void;
    }

    /// Any prompt would be a dialog from a process with no window to show
    /// it in, so DPAPI is told not to ask.
    const UI_FORBIDDEN: u32 = 0x1;

    pub fn protect(plain: &[u8]) -> Result<Vec<u8>, String> {
        let input = Blob {
            size: plain.len() as u32,
            data: plain.as_ptr().cast_mut(),
        };
        let mut output = Blob {
            size: 0,
            data: std::ptr::null_mut(),
        };
        // SAFETY: `input` outlives the call and describes `plain` exactly.
        let done = unsafe {
            CryptProtectData(
                &input,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                UI_FORBIDDEN,
                &mut output,
            )
        };
        if done == 0 {
            return Err(format!(
                "Windows refused to encrypt the key: {}",
                std::io::Error::last_os_error()
            ));
        }
        Ok(take(output))
    }

    pub fn unprotect(sealed: &[u8]) -> Result<Vec<u8>, String> {
        let input = Blob {
            size: sealed.len() as u32,
            data: sealed.as_ptr().cast_mut(),
        };
        let mut output = Blob {
            size: 0,
            data: std::ptr::null_mut(),
        };
        // SAFETY: as above; no description is asked for, so nothing else
        // is allocated for the caller to free.
        let done = unsafe {
            CryptUnprotectData(
                &input,
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                UI_FORBIDDEN,
                &mut output,
            )
        };
        if done == 0 {
            return Err(format!(
                "this key cannot be opened by this Windows account: {}",
                std::io::Error::last_os_error()
            ));
        }
        Ok(take(output))
    }

    /// The bytes of a blob DPAPI filled, and the blob freed.
    fn take(output: Blob) -> Vec<u8> {
        // SAFETY: DPAPI answered success, so `output` holds `size` bytes
        // it allocated; `LocalFree` gives them back once copied out.
        let bytes = unsafe {
            std::slice::from_raw_parts(output.data, output.size as usize).to_vec()
        };
        unsafe { LocalFree(output.data.cast()) };
        bytes
    }
}

#[cfg(windows)]
fn seal(plain: &[u8]) -> Result<Vec<u8>, String> {
    dpapi::protect(plain)
}

#[cfg(windows)]
fn unseal(sealed: &[u8]) -> Result<Vec<u8>, String> {
    dpapi::unprotect(sealed)
}

#[cfg(not(windows))]
fn seal(_plain: &[u8]) -> Result<Vec<u8>, String> {
    Err("keys are kept on Windows only, for now".to_owned())
}

#[cfg(not(windows))]
fn unseal(_sealed: &[u8]) -> Result<Vec<u8>, String> {
    Err("keys are kept on Windows only, for now".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A directory of its own per test, so two runs cannot see each other.
    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("vibecut-keys-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        dir
    }

    #[cfg(windows)]
    #[test]
    fn a_key_survives_a_round_trip_and_is_not_stored_in_the_clear() {
        let dir = scratch("round-trip");
        save(&dir, "openai", "sk-test-123").expect("saved");
        assert_eq!(load(&dir, "openai").as_deref(), Some("sk-test-123"));
        assert!(stored(&dir, "openai"));

        // What is on disk must not contain the key.
        let raw = std::fs::read(path(&dir, "openai")).expect("the file");
        let needle = b"sk-test-123";
        assert!(
            !raw.windows(needle.len()).any(|window| window == needle),
            "the key is readable in the file"
        );

        forget(&dir, "openai").expect("forgotten");
        assert_eq!(load(&dir, "openai"), None);
        assert!(!stored(&dir, "openai"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[cfg(windows)]
    #[test]
    fn each_provider_keeps_its_own() {
        let dir = scratch("apart");
        save(&dir, "openai", "one").expect("saved");
        save(&dir, "anthropic", "two").expect("saved");
        assert_eq!(load(&dir, "openai").as_deref(), Some("one"));
        assert_eq!(load(&dir, "anthropic").as_deref(), Some("two"));
        assert_eq!(load(&dir, "deepseek"), None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn an_absent_key_is_not_an_error_to_forget() {
        let dir = scratch("absent");
        forget(&dir, "openrouter").expect("forgotten");
        assert_eq!(load(&dir, "openrouter"), None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn an_empty_key_clears_the_stored_one() {
        let dir = scratch("empty");
        #[cfg(windows)]
        save(&dir, "openai", "sk-x").expect("saved");
        save(&dir, "openai", "   ").expect("cleared");
        assert_eq!(load(&dir, "openai"), None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_provider_id_cannot_escape_the_keys_folder() {
        let dir = scratch("escape");
        let escaped = path(&dir, "../../etc/passwd");
        assert!(escaped.starts_with(dir.join("keys")));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
