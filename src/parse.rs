use std::fs;
use std::io::{Error, Read};
use std::path::{Path, PathBuf};
use crate::http;


pub fn parser() -> Result<String, Error> {
    let folder_path: &str = "./app";
    let dir_path: &Path = Path::new(folder_path);
    let entries: fs::ReadDir = fs::read_dir(folder_path)?;
    let mut new_content = String::new();
    for entry in entries {

        // File Informations
        let entry: fs::DirEntry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Check if file name ends with ".html"
        if file_name_str.ends_with(".html") {

            // Check if Folder
            if entry.file_type()?.is_dir() {
                println!("Directory: {}", file_name_str);
            } else {

                // Reading the file
                let file_path: PathBuf = dir_path.join(file_name_str.as_ref());
                let mut read_file: fs::File = fs::File::open(&file_path)?;

                // The Content of File
                let mut content = String::new();
                read_file.read_to_string(&mut content)?;

                // The lines of the File
                for line in content.lines() {

                    // the modified line
                    let mut modified_line = line.to_string();
                    let line = line.trim();
                    if line.starts_with("{") && line.ends_with("}") {
                        let content = line.trim_matches('{').trim_matches('}');
                        let content_items: Vec<&str> = content.split_whitespace().collect();
                        if content_items.get(0) == Some(&"print") {
                            modified_line =
                                content_items[1..].join(" ").trim_matches('"').to_string();
                        }
                    }
                    new_content.push_str(&modified_line);
                    new_content.push('\n');
                }
            }
        }
    }

    Ok(new_content)
}
