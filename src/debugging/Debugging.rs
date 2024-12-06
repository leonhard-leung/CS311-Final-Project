use std::error::Error;
use std::fs::{metadata, File};
use std::io::BufReader;
use std::path::Path;
use csv::ReaderBuilder;

pub fn debugging() -> Result<(), Box<dyn Error>> {
    // Example logic (commented out) for testing/debugging
    let a = 2;
    let b = 5;

    let a = a + 2;

    let sample = 10;

    let result = a * b;

    if result > sample {
        println!("{} is greater than {}", result, sample);
    } else {
        println!("{} is less than {}", result, sample);
    }

    let list = vec![12, 34, 56, 78, 91];

    for i in &list {
        println!("{i}");
    }

    // Uncommenting the line below would cause a panic due to out-of-bounds access:
    println!("Accessing out-of-bounds index: {}", list[5]);

    // Read CSV data
    match read_numbers_from_csv("src/debugging/Numbers.csv") {
        Ok(csv_data) => {
            println!("Numbers from CSV:");
            for num in csv_data {
                println!("{}", num);
            }
        }
        Err(e) => {
            eprintln!("Failed to read CSV: {}", e);
        }
    }

    Ok(())
}

fn read_numbers_from_csv(filename: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    if !Path::new(filename).exists() {
        return Err(From::from(format!(
            "Error: File not found at path '{}'. Please provide a valid file path.",
            filename
        )));
    }

    if metadata(filename)?.is_dir() {
        return Err(From::from(format!(
            "Error: The path '{}' is a directory, not a file. Please provide a valid file.",
            filename
        )));
    }

    let file = File::open(filename).map_err(|e| {
        format!(
            "Error: Failed to open file '{}'. Please check permissions and try again. Details: {}",
            filename, e
        )
    })?;
    let mut rdr = ReaderBuilder::new().from_reader(BufReader::new(file));
    let mut numbers = Vec::new();

    for result in rdr.records() {
        let record = result?;
        if let Some(value) = record.get(0) {
            match value.trim().parse::<i32>() {
                Ok(num) => numbers.push(num),
                Err(_) => eprintln!("Warning: Could not parse '{}' as a number.", value),
            }
        }
    }

    Ok(numbers)
}
