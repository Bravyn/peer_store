use peer_store::{file::File};
use peer_store::os_walker::Dirbrute ;


fn main() {
    /*let path  = std::path::PathBuf::from(r"/root/projects/Rust/peer_store/Cargo.toml");
    let file = File::new(path);
    file.split_file();*/

    Dirbrute::walk();
}
