mod init;

pub fn init() {
    let project_name = init::init();
    println!("{} was initialized successfully", project_name)
}
