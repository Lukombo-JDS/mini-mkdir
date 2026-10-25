use std::{
    env, fs, io::{
        ErrorKind::{
            AlreadyExists, 
            NotFound
        }, 
        Result
    }
};


/// Calling function to the two main core std directory creation
///
/// Arguments: 
///      path: path to the folder or the path of all folders.
///      recursive: option of creating multiple folders
/// 
fn create_directory(path: &str, recursive: bool)->Result<()>{

    if recursive {
        fs::create_dir_all(path)
    }else{
       fs::create_dir(path)
    }
}

fn entry()->Vec<String>{
    env::args().collect()
}

///
/// Create multiple folders in the same parent folder
/// 
/// Arguments:
///         folders: all the folders to be created
///
fn recursive_folders(folders: &[String]){

    for i in 1..folders.len(){
        let result = create_directory(&folders[i],false);
        match result {
            Ok(()) => (),
            Err(error) => if error.kind() == AlreadyExists{
                eprintln!("folder {} aldready exist.", folders[i]);
            }
        }
    }
}

///
/// Create a single folder from parent folder path
/// 



/// core logic of the makedir command
///
///entries user entries for the usage of makedir 
///
fn logic(entries: Vec<String>){

    let mut recursive:bool = false;

    if entries[1] == "-p" && entries.len() == 3{
        
        recursive = true;
        let result = create_directory(&entries[2], recursive);
        match result {
            Ok(()) => println!("folder ready."),
            Err(error) => if error.kind() == AlreadyExists {
                eprintln!("folders exists")
            }
        }
    }else if entries.len() == 2 && recursive==true {
        
      let result =  create_directory(&entries[1], recursive);
      match result {
          Ok(()) => println!("folder created"),
          Err(error) => if error.kind() == NotFound {
              eprintln!("parents folder not found.")
          }else if error.kind() == AlreadyExists {
              eprintln!("folder aldready exist.")
          }else{
              eprintln!("Error: {}", error)
          }
      }
    }else if entries[1]== "-p" && entries.len() > 2 {
        
        // let folders: Vec<String> = Vec::from(&entries[1..]);
        recursive_folders(&entries[1..])
        
    }else {
        eprintln!("Error usage");
        eprintln!("Usage: makedir <folder_path>");
        eprintln!("Usage: makedir [-option] <folders_path>")
        
    }
    
    
}

fn main() {

    logic(entry())
    
}
