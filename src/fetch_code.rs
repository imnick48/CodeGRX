use std::fs;
use std::path::Path;

//For files
pub fn get_all_files<P: AsRef<Path>>(path: P) -> Vec<String> {
    let mut files = Vec::new();
    let allowed_extensions = [
        // Source Code
        "rs", "c", "cpp", "h", "hpp", "py", "java", "go", "js", "ts", "rb", "php", "cs", "swift",
        "kt", "kts", // Scripts
        "sh", "bat", "cmd", "ps1", "pl", "lua", // Config / Data
        "ini", "cfg", "conf", "toml", "yaml", "yml", "json", "xml", "env",
        // Data / Storage
        "sqlite", "db", "mdb", "accdb", "csv", "tsv", "parquet", "orc", "avro",
        // Documentation / Markup
        "md", "txt", "rst", "html", "htm", "xhtml", "tex", // Binary / Executables
        "exe", "dll", "so", "out", "bin", "class", "jar", "wasm",
    ];

    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.flatten() {
            let path_buf = entry.path();
            if let Some(name) = path_buf.file_name().and_then(|n| n.to_str()) {
                if name.starts_with('.') {
                    continue;
                }
            }
            if path_buf.is_dir() {
                files.extend(get_all_files(&path_buf));
            } else if path_buf.is_file() {
                if let Some(ext) = path_buf.extension().and_then(|e| e.to_str()) {
                    if allowed_extensions.contains(&ext) {
                        if fs::read_to_string(&path_buf).is_ok() {
                            files.push(path_buf.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }

    files
}
