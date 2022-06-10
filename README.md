rustydebug
==========

This crate adds a `debug!()` macro useful for print-style debugging in Rust.

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

    % main.rs:148 queens(): recursing
    % main.rs:152 queens(): pop; x == 5  y == 6
    % main.rs:152 queens(): pop; x == 4  y == 4
    % main.rs:148 queens(): recursing
    % main.rs:148 queens(): recursing
    % main.rs:148 queens(): recursing
    % main.rs:148 queens(): recursing
    % main.rs:42 Board::show(): printing board

The `debug` macro uses a `func` macro that produces the name of
the current function. The `func` macro is public, so you may also use it.

    func!()


Copyright & License
-------------------
Copyright 2022 by Walter de Jong <walter@heiho.net>
This software is available as Open Source under terms described in
the MIT license.

