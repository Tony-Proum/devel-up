extern crate clap;

use std::str::FromStr;

use clap::{App, Arg};

mod operations;
use operations::{exec, Operations};

fn main() {
    let mut app = App::new("devel-up")
        .about("Helps devel'upers to work with their favorite ecosystem")
        .author("Tony Proum <t.proum@gmail.com>")
        .version("0.0.1")
        .arg(Arg::with_name("OPERATION").required(true).takes_value(true));
    let operation = Operations::from_str(app.clone().get_matches().value_of("OPERATION").unwrap());
    match operation {
        Ok(_) => {
            exec(operation.unwrap());
        }
        Err(_) => {
            app.print_help().expect("help should be print");
        }
    }
}
