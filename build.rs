// 🐻‍❄️🐾 analytics-rs: Rust client and server implementation of the Noelware Analytics Protocol
// Copyright (c) 2022-2023 Noelware, LLC. <team@noelware.org>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
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

use std::{error::Error, ffi::OsStr, process::Command};

fn execute<T>(command: T, args: &[&str]) -> Result<String, Box<dyn Error + 'static>>
where
    T: Into<String> + AsRef<OsStr>,
{
    let res = Command::new(command).args(args).output()?;
    Ok(String::from_utf8(res.stdout)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    let binding = execute("git", &["submodule"])?;
    let binding = binding.split(' ').collect::<Vec<_>>();
    let protobufs_commit = binding.get(1).unwrap();

    println!("cargo:rustc-env=PROTOBUFS_COMMIT_HASH={protobufs_commit}");

    tonic_build::compile_protos("./vendor/protobufs/connection.proto")?;
    Ok(())
}
