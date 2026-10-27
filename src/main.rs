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
/// path: path to the folder or the path of all folders.
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
/// folders: all the folders to be created
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
fn core(entries: Vec<String>)->Result<()>{

    if entries.len() == 2{

        let result = create_directory(&entries[1], false);
        match result {
            Ok(()) => Ok(()),
            Err(error) =>  if error.kind() == AlreadyExists {
                eprintln!("folder already exist");
                Err(error)
            }else if error.kind() == PermissionDenied {
                eprintln!("permission denied.");
                Err(error)
            }else{
                Err(error)
            }
        }
    }else if entries.len() == 3 && entries[1] == "-p" {
        let result = create_directory(&entries[2],true);
        match result {
            Ok(()) => Ok(()),
            Err(error) => if error.kind() == NotFound{
                eprintln!("one of the parent folders not found");
                Err(error)
            }else if error.kind() == AlreadyExists{
                eprintln!("folder ready !");
                Err(error)
            }else{
                Err(error)
            }
        }
    }else if entries.len() > 3 && entries[1] == "-p" {
        let result = recursive_folders(&entries[2..]);
        match result {
            Ok(()) => Ok(()),
            Err(error) => Err(error),
        }
    }else {
        eprintln!("Error usage");
        eprintln!("Usage: makedir <folder>");
        eprintln!("Usage: makedir -p <path>");
        eprintln!("Usage: makedire -p <folder_1> <folder_2> ...");

        Ok(())
    }
}

fn main(){

    match core(entry()){
        Ok(()) => (),
        Err(error) => eprintln!("{}",error)
    }
    
}
