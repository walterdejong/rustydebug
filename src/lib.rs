//
//  rustydebug  WJ122
//  lib.rs
//

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
pub fn debug_print(filename: &str, lineno: u32, funcname: &str, msg: &String) {
    // print debug message

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
