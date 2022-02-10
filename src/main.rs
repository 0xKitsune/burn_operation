use std::fs;
extern crate glob;
use jwalk::WalkDir;
use std::time::{Instant};


fn main() -> std::io::Result<()> {
    burn_operation()?;
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
    fs::remove_file(path)?;
    Ok(())
}


