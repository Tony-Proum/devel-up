use std::str::FromStr;

pub enum Operation {
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