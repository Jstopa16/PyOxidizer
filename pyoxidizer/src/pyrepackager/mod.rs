// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod bytecode;
pub mod config;
pub mod dist;
pub mod fsscan;
pub mod repackage;

#[allow(unused)]
const STDLIB_NONTEST_IGNORE_DIRS: &[&str] = &[
    // The config directory describes how Python was built. It isn't relevant.
    "config",
    // ensurepip is useful for Python installs, which we're not. Ignore it.
    "ensurepip",
    // We don't care about the IDLE IDE.
    "idlelib",
    // lib2to3 is used for python Python 2 to Python 3. While there may be some
    // useful generic functions in there for rewriting Python source, it is
    // quite large. So let's not include it.
    "lib2to3",
    // site-packages is where additional packages go. We don't use it.
    "site-packages",
];

#[allow(unused)]
const STDLIB_IGNORE_FILES: &[&str] = &[
    // These scripts are used for building macholib. They don't need to be in
    // the standard library.
    "ctypes/macholib/fetch_macholib",
    "ctypes/macholib/etch_macholib.bat",
    "ctypes/macholib/README.ctypes",
    "distutils/README",
    "wsgiref.egg-info",
];
