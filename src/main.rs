use etcetera::{choose_base_strategy, BaseStrategy};
fn main() {
    let strategy = choose_base_strategy().unwrap();
    let data_dir = strategy.data_dir();
    println!("{}", data_dir.display());
}