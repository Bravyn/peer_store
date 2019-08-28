use std::path;
//use std::io::prelude::*;
use std::fs;

pub struct File {
    pub path : path::PathBuf,
    
}

impl File {
    pub fn new(path : path::PathBuf) -> File {
        File {
            path,
        }
    }

    pub fn get_file_extension(&self) -> String {
        let file = self.path.to_string_lossy();
        let col : Vec<&str> = file.split(".").collect();
        col[1].to_owned()
        
        
    }

    pub fn split_file(self) {
        //let contents = String::new();
        let file = fs::read_to_string(self.path).unwrap();

        for line in file.lines() {
           
            println!("{:?}",line);
        }
    }

}

#[cfg(test)]

#[test]
fn test_get_file_extension(){
    let path  = std::path::PathBuf::from(r"/root/projects/Rust/peer_store/Cargo.toml");
    let file = File::new(path);
    assert_eq!(String::from("toml"),file.get_file_extension());
}
