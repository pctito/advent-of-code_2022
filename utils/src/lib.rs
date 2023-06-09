use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::Path;
use uuid::Uuid;

pub fn map_from_file<P>(file: P) -> io::Result<HashMap<String, Vec<String>>>
where
    P: AsRef<Path>,
{
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_key: Option<String> = None;
    let mut current_values: Vec<String> = Vec::new();

    let buffer = read_lines_into_buffer(file)?;
    for line in buffer {
        let line = line?;
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            if let Some(key) = current_key.take() {
                map.insert(key, current_values);
                current_values = Vec::new();
            }
        } else {
            if current_key.is_none() {
                current_key = Some(format!("Elf-{}", gen_key(4)));
            }
            current_values.push(trimmed_line.to_string());
        }
    }

    if let Some(key) = current_key {
        map.insert(key, current_values);
    }

    Ok(map)
}

pub fn read_lines_into_buffer<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn gen_key(length: usize) -> String {
    let key = Uuid::new_v4().to_string();
    key.chars().take(length).collect()
}

pub fn parse_map_values(
    map: &mut HashMap<String, Vec<String>>,
) -> Result<HashMap<String, Vec<i32>>, ParseIntError> {
    let updated_map = map
        .drain()
        .map(|(k, v)| {
            let parsed_vec = v.into_iter().map(|s| s.parse::<i32>()).collect();
            match parsed_vec {
                Ok(v) => Ok((k, v)),
                Err(e) => Err(e),
            }
        })
        .collect::<Result<HashMap<String, Vec<i32>>, ParseIntError>>()?;

    Ok(updated_map)
}
