use std::fs;
use std::path::Path;

fn main() {
    let style_path = Path::new("target/styles/style.css");

    if !style_path.exists() {
        println!("cargo:warning=Style file not found, generating empty placeholder");
        fs::create_dir_all("target/styles").ok();
        fs::write(style_path, "/* empty */").expect("Failed to write placeholder");
    }
}
