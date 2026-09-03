use std::fs;
use std::io::Write;
// use std::path::Path;

pub fn read_text_file(path: &str) -> Result<String, String> {
    // let current: String;
    // match fs::read_to_string(path) {
    //     Ok(n) => current = n,
    //     Err(e) => {
    //         let err_msg: String = format!("Cannot read file {} : {}", path, e);
    //         return Err(err_msg);
    //     }
    // }
    // return Ok(current);

    let current =
        fs::read_to_string(path).map_err(|e| format!("Cannot read file: {}\n err: {}", path, e))?;
    Ok(current)
}

pub fn write_text_file(path: &str, contents: &str) -> Result<(), String> {
    fs::write(path, contents).map_err(|e| format!("Cannot write to {}\n err: {}", path, e))?;
    Ok(())
}

pub fn append_text_file(path: &str, contents: &str) -> Result<(), String> {
    let mut content = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("Cannot append file: {}\n err: {}", path, e))?;

    content
        .write_all(contents.as_bytes())
        .map_err(|e| format!("Cannot append file: {}\n err: {}", path, e))?;

    Ok(())
}
