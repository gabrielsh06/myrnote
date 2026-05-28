use etcetera::{BaseStrategy, choose_base_strategy};
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let strategy = choose_base_strategy().unwrap();
    let directory = strategy.data_dir().join("myrnote");
    fs::create_dir_all(&directory).unwrap();

    let path = directory.join("myrnote.txt");

    let mut args = env::args();
    args.next();

    let default_message = r#"
        use: myrnote *your notes*
             myrnote --list
             myrnote --clear
        "#;

    if let Some(argument) = args.next() {
        match argument.as_str() {
            "--list" => read_file(&path),
            "--clear" => clear_file(&path),
            _ => println!("{}", default_message),
        }
    } else {
        write_file(&path);
    }
}

fn clear_file(path: &PathBuf) {
    fs::write(path, "").unwrap();
}

fn read_file(path: &PathBuf) {
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
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
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

    let mut content = String::new();
    io::stdin().read_line(&mut content).expect("msg");

    writeln!(file, "{}", content.trim()).unwrap();
}
