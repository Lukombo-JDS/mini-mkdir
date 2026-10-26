use std::{
    env, fs, io::{
        ErrorKind::{
            AlreadyExists, 
            NotFound,
            PermissionDenied
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
fn recursive_folders(folders: &[String])->Result<()>{

    for folder in folders{
        let result = create_directory(folder,true);
        match result {
            Ok(()) => continue,
            Err(error) => if error.kind() == AlreadyExists{
                
                eprintln!("folder {} aldready exist.", folder);
                // Err(error);
            }else if error.kind() == PermissionDenied{
                
                eprintln!("permission denied");
                return Err(error);
            }else{
                
                eprintln!("Error: {}", error);
                return Err(error);
            }
        }
    }
    Ok(())
}

/// core logic of the makedir command
///
///entries user entries for the usage of makedir
///
fn logic(entries: Vec<String>)->Result<()>{

    let mut recursive:bool = false;

    if entries.len() == 3 && entries[1] == "-p" {
        
        recursive = true;
        let result = create_directory(&entries[2], recursive);
        match result {
            Ok(()) => {
                println!("folder ready.");
                Ok(())
            },
            Err(error) => if error.kind() == AlreadyExists {
                eprintln!("folders exists");
                Err(error)
            }else if error.kind() == NotFound {
                eprintln!("parents folder not found.");
                Err(error)
            }else{
                eprintln!("Error: {}", error);
                Err(error)
            }
        }
    }else if entries.len() == 2 {
        
      let result =  create_directory(&entries[1], recursive);
      match result {
          Ok(()) => {
              println!("folder created");
              Ok(())
          },
          Err(error) => if error.kind() == NotFound {
              eprintln!("parents folder not found.");
              Err(error)
          }else if error.kind() == AlreadyExists {
              eprintln!("folder aldready exist.");
              Ok(())
          }else{
              eprintln!("Error: {}", error);
              Err(error)
          }
      }
    }else if entries.len() > 2 && entries[1] == "-p" {
        
        recursive_folders(&entries[2..])
        
    }else {
        eprintln!("Error usage");
        eprintln!("Usage: makedir <folder_path>");
        eprintln!("Usage: makedir [-option] <folders_path>");
        Ok(())
    }
}

fn main(){

    match logic(entry()){
        Ok(()) => (),
        Err(error) => eprintln!("{}",error)
    }
    
}
