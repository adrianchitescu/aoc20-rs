pub mod utils {
    use std::{env, fs};
    use std::io::{Error, ErrorKind, Result};
    pub fn get_file_input(arg_position : usize) -> Result<String> {
        if let Some(file_name) = env::args().nth(arg_position) {
            fs::read_to_string(file_name)
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Invalid file_name"))
        }
    }
}