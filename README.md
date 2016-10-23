Giter8 Rust Project Template
===============================

This is [Giter8](https://github.com/foundweekends/giter8) template for minimal Rust project
using separated library and bin.

Usage
------

    $ g8 anvie/rust-project

    name [My Cool Rust App]:
    desc [My Cool Rust App]:
    app_name [myapp]:
    version [0.0.1]:
    author [robin <r@nosql.asia>]:

    Template applied in ./my-cool-rust-app

    $ cd my-cool-rust-app/
    $ make
    Generating build.rs ...

    $ cargo run --bin myapp_main
    Compiling ...

    Finished debug [unoptimized + debuginfo] target(s) in 15.71 secs
    Running `target/debug/myapp_main`
    My Cool Rust App CLI 0.0.1

     Invalid arguments.

    Usage:
        myapp_main hello <name>
        myapp_main (-h | --help)
        myapp_main --version
