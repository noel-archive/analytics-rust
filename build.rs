// 🧭📦 protobufs-rust: Rust crate for the generated protobufs for Noelware Analytics
// Copyright (c) 2022 Noelware
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::ops::Deref;
use std::{ffi::OsStr, process::Command};
use std::path::Path;

/// execute returns the standard output of the command specified.
fn execute<T: Into<String> + AsRef<OsStr>>(
    command: T,
    args: &[&str],
) -> Result<String, Box<dyn std::error::Error + 'static>> {
    let res = Command::new(command).args(args).output().unwrap();
    Ok(String::from_utf8(res.stdout)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the commit hash of the protobufs submodule
    let result_uno = execute("git", &["submodule"]).unwrap();
    let result_trimmed = result_uno.trim().split(' ');
    let result_dos = result_trimmed.collect::<Vec<_>>();

    if let Some(commit) = result_dos.first() {
        let sha = commit.deref();
        println!("cargo:rustc-env=PROTOBUF_COMMIT_SHA={}", sha);
    } else {
        println!("cargo:rustc-env=PROTOBUF_COMMIT_SHA=none");
    }

    let proto_path: &Path = "./protos/connection.proto".as_ref();
    let proto_dir = proto_path.parent().expect("proto file should be in a directory.");

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir("./src/stubs")
        .compile(&[proto_path], &[proto_dir])?;

    Ok(())
}
