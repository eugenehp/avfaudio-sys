// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn sdk_path() -> Result<String, std::io::Error> {
    use std::process::Command;

    let target = env::var("TARGET").unwrap();
    let sdk = if target.ends_with("apple-ios") {
        "iphoneos"
    } else if target.ends_with("apple-ios-sim") {
        "iphonesimulator"
    } else if target.ends_with("apple-visionos") {
        "xros"
    } else if target.ends_with("apple-visionos-sim") {
        "xrsimulator"
    } else if target.ends_with("apple-darwin") {
        "macosx"
    } else if target.ends_with("apple-watchos") {
        "watchos"
    } else if target.ends_with("apple-watchos-sim") {
        "watchsimulator"
    } else if target.ends_with("apple-tvos") {
        "appletvos"
    } else {
        panic!("unknown target: {}", target);
    };

    let output = Command::new("xcrun")
        .args(["--sdk", sdk])
        .args(["--show-sdk-path"])
        .output()?
        .stdout;
    let prefix_str = std::str::from_utf8(&output).expect("invalid output from `xcrun`");
    Ok(prefix_str.trim_end().to_string())
}

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=framework=AVFAudio");

    // Tell cargo to invalidate the built crate whenever the wrapper changes

    // See https://github.com/rust-lang/rust-bindgen/issues/1211
    // Technically according to the llvm mailing list, the argument to clang here should be
    // -arch arm64 but it looks cleaner to just change the target.
    let target = env::var("TARGET").unwrap();

    let is_visionos = target.contains("apple-visionos");

    let clang_target = if target.starts_with("aarch64-apple-ios") {
        "arm64-apple-ios"
    } else if target.starts_with("aarch64-apple-visionos") {
        "arm64-apple-xros"
    } else if target.starts_with("aarch64-apple-darwin") {
        "arm64-apple-darwin"
    } else {
        &target
    };
    let target_arg = format!("--target={}", clang_target);
    let sdk = sdk_path().ok();
    let sdk = sdk.as_ref().map(String::as_ref);
    let mut clang_args = vec![
        "-x",
        "objective-c",
        "-fblocks",
        "-fretain-comments-from-system-headers",
        &target_arg,
    ];
    if let Some(sdk) = sdk {
        clang_args.extend(["-isysroot", sdk]);
    }

    let mut headers = vec![""];

    if is_visionos {
        // headers.push("#ifndef TARGET_OS_IPHONE");
        headers.push("#define TARGET_OS_IPHONE 1");
        // headers.push("#endif");

        headers.push("typedef long           NSInteger;");
        headers.push("typedef unsigned long  NSUInteger;");
        headers.push("#define NSIntegerMin   INT_MIN");
        headers.push("#define NSIntegerMax   INT_MAX");
        headers.push("#define NSUIntegerMax  UINT_MAX");
    }

    headers.push("AVFAudio/AVFAudio.h");

    let meta_headers:Vec<_> = headers
    .iter()
    .map(|h| {
        if h.ends_with(".h") {
            format!("#include<{}>\n", h)
        }else{
            format!("{}\n", h)
        }
    })
    .collect();

    let contents = meta_headers.concat();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .clang_args(&clang_args)
        .objc_extern_crate(true)
        .block_extern_crate(true)
        .generate_block(true)
        .generate_comments(true)
        .formatter(bindgen::Formatter::Rustfmt)
        .blocklist_item("objc_object")
        .blocklist_item("id")
        .blocklist_item("timezone")
        .blocklist_function("settimeofday")
        .opaque_type("FndrOpaqueInfo")
        .opaque_type("HFSPlusCatalogFile")
        .opaque_type("HFSCatalogFile")
        .opaque_type("HFSPlusCatalogFolder")
        .opaque_type("HFSCatalogFolder")
        .no_copy("AudioUnitRenderContext")
        .no_debug("AudioUnitRenderContext")
        .derive_default(false)
        .header_contents("AVFAudio.h", &contents)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
