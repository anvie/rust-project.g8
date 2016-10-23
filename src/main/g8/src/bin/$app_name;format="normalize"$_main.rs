
extern crate $app_name;format="snake"$;
extern crate docopt;
extern crate rustc_serialize;

#[macro_use]
extern crate log;
extern crate env_logger;

use docopt::Docopt;

const USAGE:&'static str = "
$name$ CLI

Usage:
    $app_name;format="normalize"$_main hello <name>
    $app_name;format="normalize"$_main (-h | --help)
    $app_name;format="normalize"$_main --version

Options:
    -h --help       Show this screen.
    --version       Show version.
    name            Your name.

";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_version:bool,
    cmd_hello:bool,
    arg_name:String,
}

fn main() {

    env_logger::init().unwrap();

    println!("$name$ CLI {}\n", $app_name;format="snake"$::build::VERSION);

    let args:Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());

    //println!("{:?}", args);

    if args.flag_version {
        println!("{}", $app_name;format="snake"$::build::VERSION);
    }

    if args.cmd_hello {
        say_hello(&args.arg_name);
    }
}


fn say_hello(name:&str){
    println!("hello {}!", name);
}
