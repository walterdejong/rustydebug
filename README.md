rustydebug
==========

This crate adds a `debug!()` macro useful for print-style debugging in Rust.
The macro works much the same as `println!()`:

    use rustydebug::debug;

    debug!("plain string");
    debug!("the value of x == {}", x);      // println!() style
    debug!("here we have s == {:?}", &s);
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

The debug output is written to stdout. This is on purpose so that the
debug output is 'synchronized' with the regular output of the program.
Sometimes it is convenient to have the debug output appear on stderr instead.
For that you can use `edebug!()`:

    use rustydebug::edebug;

    edebug!("message to stderr");


Other functionality in rustydebug
---------------------------------

* `typename(x)` returns the type of x as a static &str
* `func!()` gives the current function name as a static &str


Copyright & License
-------------------
Copyright 2022 by Walter de Jong <walter@heiho.net>
This software is available as Open Source under terms described in
the MIT license.
