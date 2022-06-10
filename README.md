rustydebug
==========

This crate adds macros useful for print-style debugging.

    use rustydebug::debug;

    debug!("plain string");
    debug!("the value of x == {}", x);      // println!() style
    debug!();                               // empty debug line

Unlike `dbg!`, the `debug` macro produces _no_ code in release builds.
You can keep the debug statements in your source code, but they will
not show in shipped products.

Debug prints are colorized in green, start with a percent sign, and
show the source file, line number, and function.

Example debug output of my eight queens solver program:

    % src/main.rs:148 queens(): recursing
    % src/main.rs:152 queens(): pop; x == 5  y == 6
    % src/main.rs:152 queens(): pop; x == 4  y == 4
    % src/main.rs:148 queens(): recursing
    % src/main.rs:148 queens(): recursing
    % src/main.rs:148 queens(): recursing
    % src/main.rs:148 queens(): recursing
    % src/main.rs:42 Board::show(): printing board

The `debug` macro uses a `func` macro that produces the name of
the current function. The `func` macro is public, so you may also use it.

    func!()
