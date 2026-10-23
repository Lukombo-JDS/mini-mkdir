use std::{fs};

fn create_directory(path: &str){

    let result = fs::create_dir_all(path);

    match result {
            Ok(()) => println!("Folder created"),
            Err(error) => if error.kind() == std::io::ErrorKind::AlreadyExists {
                println!("Folder already exist");
            }else{
                println!("Error: {}", error);
            }
        }
}

fn main() {
    create_directory("parent_inexistant/demo")
}
