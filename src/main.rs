use std::{fs::File, io::{BufReader, BufRead}};



fn main() {
    let f = File::open("calories.txt").unwrap();
    let reader = BufReader::new(f);

    let mut current = 0;
    let mut max = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        match line.as_str() {
            "" => {
                if current > max {
                    max = current;
                }
                
                current = 0;
            },
            _ => {
                let amount: i32 = line.parse().unwrap();
                current += amount;
            }
        }
    }

    println!("max: {}", max);
}
