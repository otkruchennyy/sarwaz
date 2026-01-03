use serde_json::json;
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn create_json() -> std::io::Result<()> {
    let data = json!({});
    let mut file_path = get_project_root();

    file_path.push("data.json");
    let json_string = serde_json::to_string_pretty(&data)?;

    // create or rewrite JSON
    fs::write(&file_path, json_string)?;

    println!("Файл создан: {:?}", file_path);
    Ok(())
}

pub fn add_propet_json() {}

pub fn remove_propet_json() {}

pub fn get_project_root() -> PathBuf {
    let mut current_path = env::current_exe()
        .expect("Не удалось получить путь к исполняемому файлу")
        .parent()
        .expect("Исполняемый файл не имеет родительской директории")
        .to_path_buf();

    // 1. find "sarwaz"
    let mut sarwaz_folder: Option<PathBuf> = None;
    let mut temp_path = current_path.clone();

    loop {
        if let Some(folder_name) = temp_path.file_name() {
            let folder_name_str = folder_name.to_string_lossy().to_lowercase();
            if folder_name_str.contains("sarwaz") {
                sarwaz_folder = Some(temp_path.clone());
                break;
            }
        }

        match temp_path.parent() {
            Some(parent) => temp_path = parent.to_path_buf(),
            None => break,
        }
    }

    // 2. if resources not in "sarwaz" create them
    if let Some(sarwaz_path) = sarwaz_folder {
        let resources_dir = sarwaz_path.join("resources");
        if !resources_dir.exists() {
            fs::create_dir_all(&resources_dir).expect("Не удалось создать папку resources");
        }
        return sarwaz_path;
    }

    // 3. find "resources"
    loop {
        let resources_path = current_path.join("resources");

        if resources_path.exists() && resources_path.is_dir() {
            return current_path;
        }

        match current_path.parent() {
            Some(parent) => current_path = parent.to_path_buf(),
            None => break,
        }
    }

    // 4. Fallback: current directory
    let current_dir = env::current_dir().unwrap();
    let resources_dir = current_dir.join("resources");

    if !resources_dir.exists() {
        fs::create_dir_all(&resources_dir).expect("Не удалось создать папку resources");
    }

    current_dir
}

pub fn file_exists(path: &PathBuf) -> bool {
    path.exists() && path.is_file()
}
