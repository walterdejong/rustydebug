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

use isatty::stdout_isatty;

#[allow(unused_macros)]
#[macro_export]
macro_rules! func {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name
    }}
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! debug {
    () => {
        #[cfg(debug_assertions)]
        rustydebug::debug_print(file!(), line!(), rustydebug::func!(), &String::new())
    };

    ($msg: expr) => {
        #[cfg(debug_assertions)]
        rustydebug::debug_print(file!(), line!(), rustydebug::func!(), &$msg.to_string())
    };

    ($fmt: expr, $($args: tt)*) => {
        #[cfg(debug_assertions)]
        rustydebug::debug_print(file!(), line!(), rustydebug::func!(), &format!($fmt, $($args)*))
    };
}

#[allow(dead_code)]
pub fn debug_print(long_filename: &str, lineno: u32, funcname: &str, msg: &String) {
    // print debug message

    let mut filename = long_filename;
    if filename.starts_with("src/") || filename.starts_with("src\\") {
        filename = &filename[4..];
    }

    let mut start_pos = funcname.find(':').unwrap_or(0);
    if start_pos > 0 {
        start_pos += 2;
    }
    let mut end_pos = funcname.rfind(':').unwrap_or(funcname.len());
    if end_pos > 0 && end_pos < funcname.len() {
        end_pos -= 1;
    }
    let func = &funcname[start_pos..end_pos];

    if stdout_isatty() {
        // output color codes
        println!(concat!("\x1b[32;1m", "% {}:{} {}():", "\x1b[0m", " {}"), filename, lineno, func, msg);
    } else {
        println!("% {}:{} {}(): {}", filename, lineno, func, msg);
    }
}

// EOB
