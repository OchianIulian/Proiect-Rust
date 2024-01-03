use std::env;
// Importăm serărirea și deserializarea pentru JSON
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fs;//folosit pentru a traversa folderele
use std::path::Path;

// Definim o structură de date pe care dorim să o serializăm în JSON
#[derive(Serialize, Deserialize)]
struct FolderInfo{
    file_count: u32,
    folder_count: u32,
    extension_counts: HashMap<String, u32>,
}

fn count_files_and_folders(path: &Path, file_count: &mut u32, folder_count: &mut u32, extension_count: &mut HashMap<String, u32>){
    if let Ok(entries) = fs::read_dir(path){
        for entry in entries {
            if let Ok(entry) = entry {
                let sub_path = entry.path();
                if sub_path.is_file(){
                    //incrementam numarul de fisiere
                    *file_count+=1;

                    //colectam informatii despre extensii
                    if let Some(ext) = sub_path.extension() {
                        let ext_str = ext.to_string_lossy().to_lowercase();
                        let count = extension_count.entry(ext_str.into()).or_insert(0);
                        *count += 1;
                    }
                } else if sub_path.is_dir() {
                    //incrementam numarul de foldere
                    *folder_count += 1;

                    //apelam recursiv functia pe path ul subfolderelor
                    count_files_and_folders(&sub_path, file_count, folder_count, extension_count);
                }
            }
        }
    }   
}

fn main() {
    /*take the folder path as argument from cmd */

    //Retrieve command line arguments
    let args: Vec<String> = env::args().collect();

    //Check if at least one argument is passed (the first argument is program's name)
    if args.len()<2 {
        eprintln!("Trebuie sa introduci path ul");
        return;
    }

    //Access the second argument (index 1) as folder path
    let folder_path = &args[1];
    println!("Folder path: {}", folder_path);

    let mut file_count = 0;
    let mut folder_count = 0;
    let mut extension_counts: HashMap<String, u32> = HashMap::new();

    count_files_and_folders(Path::new(&folder_path), &mut file_count, &mut folder_count, &mut extension_counts);

    //cream structura de date care va fi serializata in JSON
    let folder_info = FolderInfo {
        file_count,
        folder_count,
        extension_counts
    };

    //serializam structura in format JSON
    let json_data = serde_json::to_string_pretty(&folder_info).unwrap();

    //scriem JSON ul intr un fisier
    let mut file = fs::File::create("folder_info.json").expect("Nu s-a putut crea fisierul");
    file.write_all(json_data.as_bytes()).expect("Nu s-a putut scrie in fisier");

    println!("Informațiile colectate au fost scrise în fișierul folder_info.json.");
}  
