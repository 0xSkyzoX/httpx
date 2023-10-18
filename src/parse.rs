use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

struct MemoryVar {
    name: String,
    content: String,
}

pub fn parser() -> Result<String, io::Error> {
    let folder_path: PathBuf = Path::new("./app").to_path_buf();
    let mut new_content = String::new();
    let mut memory: Vec<MemoryVar> = Vec::new();

    for entry in fs::read_dir(&folder_path)? {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            if file_name_str.ends_with(".html") && !entry.file_type()?.is_dir() {
                let file_path = folder_path.join(&*file_name_str);
                let file = fs::File::open(&file_path)?;
                let reader = io::BufReader::new(file);

                for line in reader.lines() {
                    let line = line?;
                    let mut modified_line = line.clone();

                    if let Some(trimmed) = line.trim().strip_prefix('{').and_then(|l| l.strip_suffix('}')) {
                        let content_items: Vec<&str> = trimmed.split_whitespace().collect();

                        match content_items.get(0) {
                            Some(&"print") => {
                                modified_line = content_items[1..].join(" ").trim_matches('"').to_string();
                            }
                            Some(&"var") => {
                                if let Some(name) = content_items.get(1) {
                                    memory.push(MemoryVar {
                                        name: name.to_string(),
                                        content: "haha".to_string(),
                                    });
                                }
                            }
                            _ => {}
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
