// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Jareer Abdullah and Concat contributors

//! Generates the gRPC service from `proto/vibecut.proto` when the `grpc`
//! feature is on. protox compiles the proto in Rust, so nothing needs
//! installing.

fn main() {
    #[cfg(feature = "grpc")]
    {
        let proto = "proto/vibecut.proto";
        println!("cargo:rerun-if-changed={proto}");
        let descriptors = protox::compile([proto], ["proto"]).expect("vibecut.proto compiles");
        tonic_prost_build::configure()
            .compile_fds(descriptors)
            .expect("the gRPC service generates");
    }
}
