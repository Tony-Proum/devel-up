extern crate clap;
extern crate services;
use clap::{Arg, App};

fn main() {
    let matches = App::new("devel-up")
        .about("Helps devel'upers to work with their favorite ecosystem")
        .author("Tony Proum <t.proum@gmail.com>")
        .version("0.0.1")
        .arg(Arg::with_name("OPERATION")
            .required(true)
            .takes_value(true)).get_matches();
    let operation = matches.value_of("OPERATION").unwrap();
    services::test();
    println!("my operation is {}", operation);
}
