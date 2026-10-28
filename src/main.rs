use std::{ env, fs, io::{ Error, ErrorKind::{
            AlreadyExists, PermissionDenied, Other
        }, Result
    }
};

/// Creates a file with the path
fn create_file(path: &str)->Result<()>{
   fs::File::create_new(path)?;
   Ok(())
}

/// Creates all files the path of each file
fn create_all_files(files_path: &[String])->Result<()>{

    for path in files_path{
        fs::File::create_new(path)?;
    }
    Ok(())
}

/// Select function to switch between creating multiple files or one file
fn touch_command(recursive: bool, paths: &[String], path: &str)->Result<()>{

    if recursive {
        create_all_files(paths)?;
        Ok(())
    }else{
        create_file(path)?;
        Ok(())
    }
}



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

    let mut recursive: bool = false;
    let mut touch_recusive: bool = false;

    if entries.len() == 2{

        create_directory(&entries[1], recursive)?;
        Ok(())
    }else if entries.len() == 3  {
        match entries[1].as_str() {
            "-p" => {
                recursive = true;
                create_directory(&entries[2],recursive)?;
                Ok(())
            },
            "-t" => {
                let path = entries.get(2).ok_or_else(
                    || {
                        Error::new(Other, "index out of range")
                    }
                )?.as_str();
                touch_command(touch_recusive,&vec![],path)?;
                Ok(())
            },
            
            _ => Err(Error::new(Other, "invalid flag usage"))
        }
    }else if entries.len() > 3 && entries[1] == "-p" {
        recursive_folders(&entries[2..])?;
        Ok(())
    }else if entries.len()>3 {
        match entries[1].as_str() {
            "-p" => {
                recursive_folders(&entries[2..])?;
                Ok(())
            },
            "-tp" |"-pt" => {
                touch_recusive = true;
                touch_command(touch_recusive,&entries[2..],&"")?;
                Ok(())
            },
            _ => Err(Error::new(Other, "invalid: flag do not exist"))
        }
    }else {
        eprintln!("Error usage");
        eprintln!();
        eprintln!("Usage: makedir <folder>");
        eprintln!("Usage: makedir -p <path>");
        eprintln!("Usage: makedir -p <folder_1> <folder_2> ...");
        eprintln!();
        eprintln!("Usage: makedir -t <path_file>");
        eprintln!("Usage: makedir -tp <path_file_1> <path_file_2> ...");

        Ok(())
    }
}

fn main(){

    match core(entry()){
        Ok(()) => (),
        Err(error) => eprintln!("{}",error)
    }
    
}
