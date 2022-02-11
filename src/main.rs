use std::fs;
use std::fs::metadata;
extern crate glob;
use jwalk::WalkDir;
use std::time::{Instant};
use rand::{thread_rng, Rng};

fn main() -> std::io::Result<()> {
    // burn_operation()?;     
     Ok(())


}

    
fn burn_operation() -> std::io::Result<()> {
    //TODO: remove after debugging
    let now = Instant::now();

    for file_path in WalkDir::new("/") {
        println!("{}", file_path.unwrap().path().display());
        // burn_file(file_path.unwrap().path().display().to_string().as_str())?;
      }

    //TODO: remove after debugging

      println!("Time elapsed: {} seconds", now.elapsed().as_secs());

   Ok(())
}


fn burn_file(path: &str) -> std::io::Result<()> {
    //TODO: Logic to wipe file with rand bye write logic
    let file_size = metadata(path)?.len();
    //create byte array with file size
    let mut arr = [0i8; file_size];
    //write rand bytes to the array
    thread_rng().fill(&mut arr[..]);
    //write the rand byte array to the file
    fs::write("./test.txt", arr);
    //delete the file
    // fs::remove_file(path)?;
    
    Ok(())
}


