use std::process::Command;

pub struct Dirbrute {
    pub path : std::path::PathBuf,
}

impl Dirbrute{
    pub fn new(path: std::path::PathBuf) -> Dirbrute {
        Dirbrute{path}
    }

    pub fn walk(/*start : std::path::PathBuf*/)/* -> Vec<std::path::PathBuf> */{
        
        let mut output = Command::new("ls")
                              .output()
                              .expect("failed to run");

        println!("{:?}", String::from_utf8_lossy(&output.stdout));

        output.current_dir("/");
    }
}