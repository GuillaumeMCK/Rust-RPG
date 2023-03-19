use std::env;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::Path;

fn main() -> io::Result<()> {
    let res_dir_source = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("resources");
    let res_dir_target = Path::new(&env::var("OUT_DIR").unwrap()).join("../../../resources");

    if let Err(io_error) = add_resources(&res_dir_source, &res_dir_target) {
        println!("Error: {}", io_error);
    }

    Ok(())
}

fn add_resources(source_path: &Path, target_path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(source_path)? {
        let entry = entry?;
        let source_path = entry.path();
        let target_path = target_path.join(entry.file_name());

        if entry.file_type()?.is_dir() {
            fs::create_dir_all(&target_path)?;
            add_resources(&source_path, &target_path)?;
        } else {
            match fs::copy(&source_path, &target_path) {
                Ok(_) => (),
                Err(error) if error.kind() == ErrorKind::AlreadyExists => (),
                Err(error) => return Err(error),
            }
        }
    }

    Ok(())
}
