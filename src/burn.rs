use jwalk::WalkDir;
use rand::Rng;
use std::fs::metadata;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, os::unix::prelude::FileExt};

pub fn burn_system(root: PathBuf, n_iterations: &usize) -> std::io::Result<()> {
    if root.is_file() {
        burn_file(
            root.to_str()
                .unwrap_or("File does not exist at path provided."),
            n_iterations,
        )?;
    } else {
        //for every file on the computer, burn_file
        for file_path in WalkDir::new(&root) {
            let file_path = file_path.unwrap().path().display().to_string();
            let md = metadata(&file_path).unwrap();

            //if the file_path is a file
            if md.is_file() {
                println!("Burning {}", &file_path);
                //burn the file
                burn_file(file_path.as_str(), n_iterations)?;
            }
        }

        //delete the directories that are in root
        fs::remove_dir_all(&root)?;
    }

    Ok(())
}

fn burn_file(path: &str, n_iterations: &usize) -> std::io::Result<()> {
    //get the file size
    let file_size = metadata(path)?.len();
    //create a random number generator
    let mut rng = rand::thread_rng();
    //open the file and give write permissions
    let mut f = std::fs::OpenOptions::new().write(true).open(path)?;

    for _ in 0..*n_iterations {
        //for each byte in the file, overwrite it with a random byte
        for i in 0..file_size {
            let rand_byte = rng.gen::<u8>();
            f.write_at(&[rand_byte], i)?;
        }

        //ensure that every byte could be overwritten, if not return an error
        f.flush()?;
    }

    //delete the file
    fs::remove_file(path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{burn_file, burn_system};
    use rand::Rng;
    use std::fs::{create_dir_all, File};
    use std::io::Write;
    use std::path::PathBuf;

    #[test]
    fn test_burn_system() {
        let dir_path = PathBuf::from("./temp_test_dir");
        create_dir_all(&dir_path).expect("could not create temp test dir");

        // Create some files with random data
        for i in 0..10 {
            let file_path = dir_path.join(format!("file_{}.txt", i));
            let mut file = File::create(file_path).expect("could not create new file");

            let random_data: Vec<u8> = (0..1024).map(|_| rand::random::<u8>()).collect();
            file.write_all(&random_data)
                .expect("could not write new file");
        }

        let n_iterations = 25;
        burn_system(PathBuf::from(dir_path.clone()), &n_iterations)
            .expect("could not burn temp test dir");
    }
}
