use std::env;
// Importăm serărirea și deserializarea pentru JSON
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::prelude::*;

// Definim o structură de date pe care dorim să o serializăm în JSON
#[derive(Serialize, Deserialize)]
struct Persoana{
    nume: String,
    varsta: u32,
    activ: bool
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

    let persoana  = Persoana{
        nume:String::from("Alex"),
        varsta:24,
        activ: false
    };

    //Serializam structura in format JSON
    let json_generat = serde_json::to_string(&persoana).unwrap();

    //adaugam datele in fisier
    let mut file = File::create("persoana.json").expect("Nu s-a putut crea fisierul");
    file.write_all(json_generat.as_bytes()).expect("Nu s-a putut scrie in fisier");

    println!("JSON generat și scris în fișierul persoana.json.");
    // Afisăm JSON-ul generat
    println!("JSON generat: {}", json_generat);

}  
