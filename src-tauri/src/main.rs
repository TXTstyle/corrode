#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{path::{PathBuf, Path}, fs, io, sync::{Arc, Mutex}};

#[derive(Default)]
pub struct ProgramState(Arc<Mutex<Program>>);

#[derive(Default)]
pub struct Program {
    _proj_path: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileItem {
    name: String,
    is_dir: bool,
}

impl FileItem {
    #[allow(non_snake_case)]
    fn from_pathBuf(buf: &PathBuf) -> FileItem {
        FileItem {
            is_dir: buf.is_dir(),
            name: buf.display().to_string()
        }
    }
}

#[tauri::command]
fn get_proj_dir(path: String) -> Result<Vec<FileItem>, String> {
    let mut files = vec![];
    for path_buf in read_proj_dir(path).map_err(|x| x.to_string())?.iter() {
        files.push(FileItem::from_pathBuf(path_buf));
    }
    files.sort_unstable_by_key(|item| (!item.is_dir, item.name.to_owned()));
    Ok(files)
}

#[tauri::command]
fn get_file(file: String) -> Result<String, String> {
    read_file(Path::new(&file)).map_err(|err| err.to_string())
}

fn read_proj_dir<P>(path: P) -> Result<Vec<PathBuf>, io::Error> 
where P: AsRef<Path> {
    fs::read_dir(path).unwrap()
        .into_iter()
        .map(|x| x.map(|entry| entry.path()))
        .collect()
}

fn read_file(path: &Path) -> Result<String, io::Error>{
    fs::read_to_string(path)
}


fn main() {
    tauri::Builder::default()
        .manage(ProgramState(Default::default()))
        .invoke_handler(tauri::generate_handler![get_file, get_proj_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
