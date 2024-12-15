use std::fs;
use std::path::PathBuf;


pub fn master() {
   println!("=== Begin master analysis ===");
   analyze_files();
}

fn analyze_files() {
   // Open current directory
   let current_dir = crate::utilities::get_dir();
   println!("Analyzing files in {}", current_dir.display());

   // Read files
   let paths = current_dir.read_dir().unwrap();

   // Iterate over files
   for entry in paths {
      match entry {
         Ok (entry) => {
            traverse(&entry.path());
         }
         Err(error) => {
            println!("Could not read entry: {}", error)
         }
      }
   }
}

fn traverse(path: &PathBuf) { // Accept path so we can recurse
   let filename = path.file_name().unwrap_or_default(); //Get that filename.
   
   let metadata = fs::metadata(path); // get the metadata
   match metadata {
      Ok(metadata) => { // handle metadata results.
        let filesize = metadata.len();
        if metadata.is_dir() {
          println!("{}/, Filesize: {} bytes", filename.to_string_lossy(), filesize);
            let paths = fs::read_dir(path).unwrap(); // Recursively dive into directories.
             for entry in paths {
                match entry {
                   Ok(entry) => { // handle the entry.
                        traverse(&entry.path());
                    }
                    Err(error) => {
                        println!("Could not read directory entry: {}", error)
                    }
                 }
             }
        }
        else if metadata.is_file() {
          println!("{}, Filesize: {} bytes", filename.to_string_lossy(), filesize);
        }
      },
      Err(error) => {
        println!("Could not get file data for: {:?} Error: {}", filename, error)
      }
    }
}