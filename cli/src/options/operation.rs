use std::str::FromStr;

pub enum Operation {
    Init,
    Update,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "init" => Ok(Operation::Init),
            "update" => Ok(Operation::Update),
            _ => Err(()),
        }
    }
}
