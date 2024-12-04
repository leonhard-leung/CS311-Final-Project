use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader};
use csv::ReaderBuilder;

pub fn debugging() -> Result<(), Box<dyn Error>> {
    let a = 2;
    let b = 5;

    let a = a+2;

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

    println!("Accessing out-of-bounds index: {}", list[5]);

    // Read CSV data
    let csv_data = read_numbers_from_csv("src/debugging/Numbers.csv")?;
    println!("Numbers from CSV:");
    for num in csv_data {
        println!("{}", num);
    }

    Ok(())
}

fn read_numbers_from_csv(filename: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = ReaderBuilder::new().from_reader(BufReader::new(file));
    let mut numbers = Vec::new();

    for result in rdr.records() {
        let record = result?;
        // Assuming the first column contains the numbers
        if let Some(value) = record.get(0) {
            match value.trim().parse::<i32>() {
                Ok(num) => numbers.push(num),
                Err(_) => eprintln!("Could not parse number: {}", value),
            }
        }
    }

    Ok(numbers)
}
