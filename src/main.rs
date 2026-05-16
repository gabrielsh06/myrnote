use std::{fs};
use etcetera::{choose_base_strategy, BaseStrategy};

fn main() {
    let strategy = choose_base_strategy().unwrap();
    let directory = strategy.data_dir().join("myrnote");
    fs::create_dir_all(&directory).unwrap();

    println!("Directory successfully created in: {}", directory.display());
}