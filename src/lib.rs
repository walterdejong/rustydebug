//
//  rustydebug  WJ122
//  lib.rs
//
/*
The MIT License (MIT)
=====================

Copyright (c) 2022 Walter de Jong <walter@heiho.net>

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use isatty::{stdout_isatty, stderr_isatty};
use std::sync::{Mutex, Once};

static mut MUTEX: Option<Mutex<i32>> = None;
static INIT_MUTEX: Once = Once::new();


pub fn typename<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

#[doc(hidden)]
pub fn make_funcname__(funcname: &'static str) -> &'static str {
    // funcname will end with "::f__" because that's our sentinel function
    assert!(funcname.len() > 5);
    &funcname[..funcname.len() - 5]
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! func {
    () => {{
        fn f__() {}
        rustydebug::make_funcname__(rustydebug::typename(f__))
    }}
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! debug {
    () => {
        #[cfg(debug_assertions)]
        rustydebug::debug_printfd(1, file!(), line!(), rustydebug::func!(), "")
    };

    ($msg: expr) => {
        #[cfg(debug_assertions)]
        rustydebug::debug_printfd(1, file!(), line!(), rustydebug::func!(), $msg)
    };

    ($fmt: expr, $($args: tt)*) => {
        #[cfg(debug_assertions)]
        rustydebug::debug_printfd(1, file!(), line!(), rustydebug::func!(), &format!($fmt, $($args)*))
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! edebug {
    () => {
        #[cfg(debug_assertions)]
        rustydebug::debug_printfd(2, file!(), line!(), rustydebug::func!(), "")
    };

    ($msg: expr) => {
        #[cfg(debug_assertions)]
        rustydebug::debug_printfd(2, file!(), line!(), rustydebug::func!(), $msg)
    };

    ($fmt: expr, $($args: tt)*) => {
        #[cfg(debug_assertions)]
        rustydebug::debug_printfd(2, file!(), line!(), rustydebug::func!(), &format!($fmt, $($args)*))
    };
}

#[doc(hidden)]
pub fn debug_printfd(fd: i32, long_filename: &str, lineno: u32, funcname: &str, msg: &str) {
    // print debug message

    let mut filename = long_filename;
    if filename.starts_with("src/") || filename.starts_with("src\\") {
        filename = &filename[4..];
    }

    let mut start_pos = funcname.find(':').unwrap_or(0);
    if start_pos > 0 {
        start_pos += 2;
    }
    let func = &funcname[start_pos..];

    /*
        Note: nix::unistd::isatty(fd) is a UNIX thing
        while stdout_isatty() / stderr_isatty() is more portable
    */
    let isatty = (fd == 2 && stderr_isatty()) || stdout_isatty();
    let s: String;
    if isatty {
        s = format!(concat!("\x1b[32;1m", "% {}:{} {}():", "\x1b[0m", " {}"), filename, lineno, func, msg);
    } else {
        s = format!("% {}:{} {}(): {}", filename, lineno, func, msg);
    }

    {
        // synchronize multithreaded output
        let _guard = *synchronized().lock().unwrap();

        if fd == 2 {
            eprintln!("{}", s);
        } else {
            println!("{}", s);
        }
    }
}

#[doc(hidden)]
fn synchronized<'a>() -> &'a Mutex<i32> {
    // this function is an accessor for a global mutex
    // having a global mutex in Rust is somewhat difficult to pull of ...

    INIT_MUTEX.call_once(|| {
        unsafe {
            MUTEX = Some(Mutex::new(0));
        }
    });
    // return "static" ref to global mutex
    unsafe {
        MUTEX.as_ref().unwrap()
    }
}

// EOB
