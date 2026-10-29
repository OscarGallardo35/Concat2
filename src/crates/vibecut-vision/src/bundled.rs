// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Jareer Abdullah and Concat contributors

//! Los modelos que viajan dentro del binario.
//!
//! Dos de los cinco de [`crate::models::MODELS`] no se bajan: van embebidos
//! acá, con el mismo `include_bytes!` con el que `vibecut-speech` lleva su
//! Silero VAD. Son los dos chicos, y los dos que hacen que una instalación
//! recién hecha sirva de entrada: quitar el fondo de una persona (15 MB) y
//! mejorar una foto (4,9 MB). Los otros tres —objetos, y los dos del pincel—
//! son de 16 a 178 MB y seguirían engordando el instalador sin necesidad.
//!
//! Nada de esto cambia de dónde se leen los modelos: si el archivo está en
//! `<app data>/cutout-models/`, se lee de ahí y punto; si no está,
//! [`crate::runtime`] cae a estos bytes y la función anda igual. Así el
//! usuario que ya los bajó sigue usando su copia —y su espejo, si lo cambió—
//! y el que nunca los bajó no ve un cartel pidiéndolos.
//!
//! Los archivos son copias verbatim de los que publica cada proyecto, con el
//! SHA-256 que está en la tabla: `rvm_mobilenetv3_fp32.onnx` de
//! PeterL1n/RobustVideoMatting (GPL-3.0) y `realesr-general-x4v3.onnx` del
//! export de CoderViking (BSD-3-Clause). Ver [`crate::models::MODELS`].

/// Los modelos embebidos, por el nombre de archivo que usa la tabla.
pub const BUNDLED: &[(&str, &[u8])] = &[
    (
        "rvm-mobilenetv3.onnx",
        include_bytes!("../assets/rvm-mobilenetv3.onnx"),
    ),
    (
        "realesr-general-x4v3.onnx",
        include_bytes!("../assets/realesr-general-x4v3.onnx"),
    ),
];

/// Los bytes del modelo embebido que se llama así, si es uno de ellos.
pub fn get(file: &str) -> Option<&'static [u8]> {
    BUNDLED
        .iter()
        .find(|(name, _)| *name == file)
        .map(|(_, bytes)| *bytes)
}

/// Si ese archivo viaja dentro del binario.
pub fn is_bundled(file: &str) -> bool {
    get(file).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::MODELS;

    /// Lo que se embebe es lo que la tabla dice: mismo nombre, mismo tamaño.
    /// El hash no se comprueba acá porque no hay sha256 en esta crate; el
    /// `scripts/models.py --check` y el CI sí lo hacen contra el manifiesto.
    #[test]
    fn every_bundled_model_is_in_the_table_and_the_same_size() {
        for (file, bytes) in BUNDLED {
            let spec = MODELS
                .iter()
                .find(|spec| spec.file == *file)
                .unwrap_or_else(|| panic!("{file} no está en la tabla"));
            assert!(
                !bytes.is_empty(),
                "{file} se embebió vacío (¿falta el asset?)"
            );
            assert_eq!(
                bytes.len() as u64,
                spec.bytes,
                "{file}: el archivo embebido no mide lo que dice la tabla"
            );
        }
        assert!(is_bundled("rvm-mobilenetv3.onnx"));
        assert!(!is_bundled("isnet-general-use.onnx"));
        assert!(get("isnet-general-use.onnx").is_none());
    }
}
