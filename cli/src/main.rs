extern crate clap;

use clap::{App, Arg};

mod project;

fn main() {
    let matches = App::new("devel-up")
        .about("Helps devel'upers to work with their favorite ecosystem")
        .author("Tony Proum <t.proum@gmail.com>")
        .version("0.0.1")
        .arg(Arg::with_name("OPERATION")
            .required(true)
            .takes_value(true)).get_matches();
    let operation = matches.value_of("OPERATION").unwrap();
    match operation {
        "create" => project::create(),
        _ => ()
    }
}
