use std::error::Error;
use utils::{map_from_file, parse_map_values};

fn main() {
    pub fn result() -> Result<(), Box<dyn Error>> {
        let mut map_file = map_from_file("./day1/input.txt")?;
        let map = parse_map_values(&mut map_file)?;

        let (key_max, sum_max) = map
            .iter()
            .map(|(k, v)| (k, v.iter().sum::<i32>()))
            .max_by_key(|&(_, sum)| sum)
            .unwrap();

        if sum_max == i32::MIN {
            println!("The map is empty.");
        } else {
            println!(
                "The key with the maximum sum is {}, with a sum of {} calories.",
                key_max, sum_max
            );
        }

        Result::Ok(())
    }

    match result() {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
