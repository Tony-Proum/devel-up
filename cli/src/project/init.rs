use std::fmt::Error;

pub fn init() -> Result<String, Error> {
    let current_path = std::env::current_dir().unwrap().display().to_string();
    let current_project = current_path.split_terminator("/").last().unwrap();
    Ok(current_project.to_string())
}

#[cfg(test)]
mod tests {

    #[test]
    fn create_should_return_the_created_github_repository() {
        use super::init;
        // GIVEN
        assert!(std::env::set_current_dir("/tmp").is_ok());
        // WHEN
        let repository = init();
        // THEN
        assert_eq!("tmp", repository.unwrap())
    }
}