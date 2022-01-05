use std::str::FromStr;

mod init;

pub enum Operations {
    Init,
}

impl FromStr for Operations {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "init" => Ok(Operations::Init),
            _ => Err(()),
        }
    }
}

pub fn exec(operation: Operations) -> String {
    match operation {
        Operations::Init => init::init(),
    }
}

