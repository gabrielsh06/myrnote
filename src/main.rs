use etcetera::{BaseStrategy, choose_base_strategy};
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let strategy = choose_base_strategy().unwrap();
    let directory = strategy.data_dir().join("myrnote");
    fs::create_dir_all(&directory).unwrap();
    println!("Directory created successfully in: {}", directory.display());

    let path = directory.join("myrnote.txt");

    read_file(&path);
    write_file(&path);
}

fn read_file(path: &PathBuf) {
    let file = fs::OpenOptions::new()
        .read(true)
        .truncate(false)
        .open(path)
        .unwrap();

    match io::read_to_string(file) {
        Ok(message) => println!("{}", message),
        Err(error) => panic!("Error: {}", error),
    };
}

fn write_file(path: &PathBuf) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(false)
        .open(path)
        .unwrap();
    let saludo = "Hola amigo";
    writeln!(file, "{}", saludo).unwrap();
}
