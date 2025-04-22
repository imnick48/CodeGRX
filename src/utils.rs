use serde_json::{self, Value};
use std::fs::{create_dir, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

pub fn create_files(path: &str) {
    let _ = File::create(path);
}
pub fn create_files_from_arr(paths: &Vec<String>) {
    for path in paths {
        create_files(path);
    }
}
pub fn create_directory(path: &str) {
    let _ = create_dir(path);
}
pub fn create_directory_from_arr(paths: &Vec<String>) {
    for path in paths {
        create_directory(path);
    }
}

pub fn read_file(path: &str) -> String {
    let file = File::open(path).expect("Unable to open file");
    let reader=BufReader::new(file);
    let mut output: String = String::new();
    for line in reader.lines() {
        match line {
            Ok(line) => output.push_str(&line),
            Err(e) => println!("Error reading line: {}", e),
        }
    }
    output
}

pub fn extract_json(json_string: &str) -> Option<&str> {
    if let Some(start) = json_string.find("```json") {
        if let Some(end) = json_string.rfind("```") {
            let block = &json_string[start..end].trim();
            if let Ok(_) = serde_json::from_str::<Value>(block) {
                return Some(block);
            }
        }
    }
    let start = json_string.find('{')?;
    let end = json_string.rfind('}')?;
    if end <= start {
        return None;
    }
    let j_final = &json_string[start..=end];
    serde_json::from_str::<Value>(j_final).ok()?;
    Some(j_final)
}

pub fn modify_line_in_file<P: AsRef<Path>>(path: P, line_number: u64, new_line: &str, change_type: &str) -> io::Result<()> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let line_num = match usize::try_from(line_number) {
        Ok(num) => {
            num
        },
        Err(_) => {
            let err = format!("Line number {} is too large", line_number);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, err));
        },
    };
    let adjusted_line_num = if line_num > 0 { line_num - 1 } else { line_num };

    match change_type {
        "replace" => {
            if adjusted_line_num < lines.len() {
                lines[adjusted_line_num] = new_line.to_string();
            } else {
                let err = format!("Line number {} out of range (file has {} lines)",
                    line_number, lines.len());
                println!("{}", err);
                return Err(io::Error::new(io::ErrorKind::InvalidInput, err));
            }
        }
        "insert" => {
            if adjusted_line_num > lines.len() {
                let err = format!("Cannot insert at line {} - file only has {} lines",
                    line_number, lines.len());
                println!("{}", err);
                return Err(io::Error::new(io::ErrorKind::InvalidInput, err));
            }
            lines.insert(adjusted_line_num, new_line.to_string());
        }
        _ => {
            let err = format!("Unknown change type: '{}'. Use 'replace' or 'insert'.", change_type);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, err));
        }
    }
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)?;
    
    for line in &lines {
        writeln!(file, "{}", line)?;
    }    
    Ok(())
}