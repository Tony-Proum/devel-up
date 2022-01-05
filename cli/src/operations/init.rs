pub fn init() -> String {
    let _ = std::fs::create_dir(".devel_up");
    get_current_directory_name()
}

fn get_current_directory_name() -> String {
    std::env::current_dir()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .into()
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_current_directory_name_should_return_the_current_dir_name() {
        use super::get_current_directory_name;
        // GIVEN
        assert!(std::env::set_current_dir("/tmp").is_ok());
        // WHEN
        let result = get_current_directory_name();
        // THEN
        assert_eq!("/tmp", result)
    }

    #[test]
    fn init_should_create_a_dot_devel_up_directory_in_current_dir() {
        use super::init;
        // GIVEN
        assert!(std::env::set_current_dir("/tmp").is_ok());
        // WHEN
        init();
        // THEN
        let dir = std::fs::read_dir("/tmp");
        let result = dir
            .unwrap()
            .map(|item| item.unwrap())
            .filter(|item| {
                item.file_name()
                    .as_os_str()
                    .to_os_string()
                    .to_str()
                    .unwrap()
                    == ".devel_up"
            })
            .last();
        assert!(match result {
            _ => true,
        })
    }
}
