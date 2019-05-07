pub fn init(name: &str) -> &str {
    println!("create a github project");
    name
}

#[cfg(test)]
mod tests {
    use super::init;

    #[test]
    fn create_should_return_the_created_github_repository() {
        // WHEN
        let repository = init("my-repo");
        // THEN
        assert_eq!("my-repo", repository)
    }
}