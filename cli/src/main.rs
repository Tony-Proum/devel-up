extern crate clap;

use std::str::FromStr;

use clap::{App, Arg};

use options::operation::Operation;
use project::create;

mod project;
mod options;


fn main() {
    let matches = App::new("devel-up")
        .about("Helps devel'upers to work with their favorite ecosystem")
        .author("Tony Proum <t.proum@gmail.com>")
        .version("0.0.1")
        .arg(Arg::with_name("OPERATION")
            .required(true)
            .takes_value(true)).get_matches();
    let operation: Operation = Operation::from_str(matches.value_of("OPERATION").unwrap()).unwrap();
    match operation {
        Operation::Create => create(),
        Operation::Update => println!("Update all")
    }
}
