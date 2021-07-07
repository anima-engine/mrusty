// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate cc;
extern crate tar;
extern crate walkdir;

use std::fs::File;

use tar::Archive;
use walkdir::{DirEntry, WalkDir};

fn is_c(entry: &DirEntry) -> bool {
    match entry.path().extension() {
        Some(ext) => "c" == ext,
        None => false,
    }
}

fn main() {
    let mut archive = Archive::new(File::open("src/mruby/mruby-out.tar").unwrap());
    archive.unpack("target").unwrap();

    let mut config = cc::Build::new();

    for entry in WalkDir::new("target/mruby-out/src")
        .into_iter()
        .filter_entry(|e| e.file_type().is_dir() || is_c(e))
    {
        let entry = entry.unwrap();

        if is_c(&entry) {
            config.file(entry.path());
        }
    }

    config
        .include("target/mruby-out/include")
        .include("target/mruby-out/src/mrbgems/mruby-time/include")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-variable")
        .flag("-Wno-missing-field-initializers")
        .define("MRB_NO_BOXING", "1")
        .compile("libmruby.a");

    let mut config = cc::Build::new();

    config
        .file("src/mrb_ext.c")
        .include("target/mruby-out/include")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-variable")
        .flag("-Wno-missing-field-initializers")
        .define("MRB_NO_BOXING", "1")
        .compile("libmrbe.a");
}
