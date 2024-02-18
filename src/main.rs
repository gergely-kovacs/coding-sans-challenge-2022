mod answer_writer;
mod challenges;
mod types;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use challenges::{challenge_one, challenge_two};
use types::BakeryData;

fn read_data_from_json() -> serde_json::Result<BakeryData> {
    let path = Path::new("src/data/bakery.json");
    let mut file = File::open(path).expect("Failed to open file.");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Failed to read file.");
    let bakery_data: BakeryData = serde_json::from_str(&data)?;
    Ok(bakery_data)
}

fn main() {
    let input_data = read_data_from_json().unwrap();
    challenge_one::solve_challenge_one(&input_data);
    challenge_two::solve_challenge_two(&input_data);
}
