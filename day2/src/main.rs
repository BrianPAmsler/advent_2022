use std::{fs::File, io::{BufReader, BufRead}};

fn update_top(top: &mut [i32], new_val: i32, index: usize) {
    if index >= top.len() {
        return;
    }

    if new_val > top[index] {
        let temp = top[index];
        top[index] = new_val;

        update_top(top, temp, index + 1);
    } else {
        update_top(top, new_val, index + 1);
    }
}

fn main() {
    let f = File::open("calories.txt").unwrap();
    let reader = BufReader::new(f);

    let mut current = 0;
    let mut top = [0; 3];
    for line in reader.lines() {
        let line = line.unwrap();
        match &line[..] {
            "" => {
                update_top(&mut top, current, 0);

                current = 0;
            },
            _ => {
                let amount: i32 = line.parse().unwrap();
                current += amount;
            }
        }
    }

    println!("max: {}", top[0] + top[1] + top[2]);
}
