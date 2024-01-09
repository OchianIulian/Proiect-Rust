use std::env;
// Importăm serărirea și deserializarea pentru JSON
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self}; //folosit pentru a traversa folderele
use std::io::Write;
use std::path::{Path};


#[derive(Serialize, Deserialize)]
struct FileInfo{
    name: String,
    is_file:bool,
    size: Option<u64>,
    children: Option<Vec<FileInfo>>
}

// Definim o structură de date pe care dorim să o serializăm în JSON
#[derive(Serialize, Deserialize)]
struct FolderInfo {
    file_count: u32,
    folder_count: u32,
    extension_counts: HashMap<String, u32>,
    file_info: FileInfo
}

fn get_file_info(path: &Path) -> Option<FileInfo>{
    if let Ok(metadata) = fs::metadata(path){
        let name = path.file_name()?.to_string_lossy().to_string();
        let is_file = metadata.is_file();
        let mut size = if is_file { Some(metadata.len())} else {None};
        let mut children = None;
    
        if metadata.is_dir(){
            if let Ok(entries) = fs::read_dir(path) {
                let mut child_info = Vec::new();
                let mut folder_size = 0;

                for entry in entries.flatten() {
                    if let Some(child) = get_file_info(&entry.path()){
                        if let Some(child_size) = child.size {
                            folder_size += child_size;
                        }
                        child_info.push(child);
                    }
                }
                children = Some(child_info);
                size = Some(folder_size);
            }
        }

        return Some(FileInfo{
            name, 
            is_file,
            size,
            children
        });
    }
    None
}

fn count_files_and_folders(
    path: &Path,
    file_count: &mut u32,
    folder_count: &mut u32,
    extension_count: &mut HashMap<String, u32>,
) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let sub_path = entry.path();
            if sub_path.is_file() {
                //incrementam numarul de fisiere
                *file_count += 1;

                //colectam informatii despre extensii
                if let Some(ext) = sub_path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    let count = extension_count.entry(ext_str).or_insert(0);
                    *count += 1;
                }

            } else if sub_path.is_dir() {
                //incrementam numarul de foldere
                *folder_count += 1;

                //apelam recursiv functia pe path ul subfolderelor
                count_files_and_folders(
                    &sub_path,
                    file_count,
                    folder_count,
                    extension_count,
                );
            }
        }
    }
}

fn main() {
    /*take the folder path as argument from cmd */

    //Retrieve command line arguments
    let args: Vec<String> = env::args().collect();

    //Check if at least one argument is passed (the first argument is program's name)
    if args.len() < 2 {
        eprintln!("Trebuie sa introduci path ul");
        return;
    }

    //Access the second argument (index 1) as folder path
    let folder_path = &args[1];

    
    let path = Path::new(folder_path);
    if path.exists() {
        println!("Folder path: {}", folder_path);
    } else {
        eprintln!("Path-ul {:?} nu există sau nu este valid.", path);
        return;
    }

    let mut file_count = 0;
    let mut folder_count = 0;
    let mut extension_counts: HashMap<String, u32> = HashMap::new();
    

    count_files_and_folders(
        path,
        &mut file_count,
        &mut folder_count,
        &mut extension_counts,
    );

    let file_info = get_file_info(path);

    //cream structura de date care va fi serializata in JSON
    let folder_info = FolderInfo {
        file_count,
        folder_count,
        extension_counts,
        file_info: file_info.unwrap_or_else(|| FileInfo{
            name: String::new(),
            is_file: false,
            size: None,
            children: None,
        })
    };

    //serializam structura in format JSON
    let json_data = serde_json::to_string_pretty(&folder_info).unwrap();

    //scriem JSON ul intr un fisier
    let mut file = fs::File::create("folder_info.json").expect("Nu s-a putut crea fisierul");
    file.write_all(json_data.as_bytes())
        .expect("Nu s-a putut scrie in fisier");

    println!("Informațiile colectate au fost scrise în fișierul folder_info.json.");
}
