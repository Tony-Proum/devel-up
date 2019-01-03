extern crate clients;
use clients::github;

pub fn test(){
    println!("test");
    github::get()
}