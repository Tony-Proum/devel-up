extern crate clap;

use std::str::FromStr;

use clap::{App, Arg};


use project::create;

mod project;

enum Operation {
    Create,
    Update,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "create" => Ok(Operation::Create),
            "update" => Ok(Operation::Update),
            _ => Err(())
        }
    }
}

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
