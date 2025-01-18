use std::fs;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    let folders: Vec<&str> = Vec::from(["observables"]);

    for folder in folders {
        let dir = Path::new("src").join(folder);
        process_path(&dir);
    }
}

fn process_path(dir: &Path) {
    let mut mods = String::new();
    let mod_file_path = dir.join("mod.rs");
    
    let mut mod_file_content = String::new();
    {
        if let Ok(mut mod_file) = fs::File::open(&mod_file_path) {
            mod_file.read_to_string(&mut mod_file_content).expect("Could not read mod.rs file");
        } else if let Ok(mut mod_file) = fs::File::create(&mod_file_path) {
            mod_file_content = String::from("//start mods\n//end mods");
            mod_file.write_all(mod_file_content.as_bytes()).expect("Could not write to mod.rs file");
        } else {
            panic!("Could not open or create mod.rs file");
        }
    }

    for entry in fs::read_dir(dir).expect("Could not read directory") {
        let entry = entry.expect("Could not read directory entry");
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("rs") && path.file_name().and_then(|name| name.to_str()) != Some("mod.rs") {
            if let Some(file_stem) = path.file_stem().and_then(|stem| stem.to_str()) {
                mods = format!("{}pub mod {};\n", mods, file_stem);
            }
        } else if path.is_dir() {
            process_path(&path);
            mods = format!("{}pub mod {};\n", mods, path.file_name().and_then(|name| name.to_str()).expect("Could not get directory name"));
        }
    }

    let start_mods_position = mod_file_content.find("//start mods").expect("Could not find start mods comment in mod.rs file");
    let end_mods_position = mod_file_content.find("//end mods").expect("Could not find end mods comment in mod.rs file");

    let mut new_mod_file_content = mod_file_content[..start_mods_position + "//start mods".len()].to_string() + "\n";
    new_mod_file_content.push_str(&mods);
    new_mod_file_content.push_str(&mod_file_content[end_mods_position..]);

    {
        let mut mod_file = fs::File::create(&mod_file_path).expect("Could not create mod.rs file");
        mod_file.write_all(new_mod_file_content.as_bytes()).expect("Could not write to mod.rs file");
    }

}