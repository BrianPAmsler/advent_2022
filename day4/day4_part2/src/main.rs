use std::{fs::File, io::{BufReader, BufRead}};

fn test_ranges(a: (i32, i32), b: (i32, i32)) -> bool {
    let (a, b) = if a.0 < b.0 {
        (a, b)
    } else {
        (b, a)
    };


    a.1 >= b.0
}

fn parse_range(range: &str) -> (i32, i32) {
    let (a, b) = range.split_once('-').unwrap();

    (a.parse().unwrap(), b.parse().unwrap())
}

fn main() {
    let pairs: Vec<((i32, i32), (i32, i32))> = {
        let f = File::open("pairs.txt").unwrap();
        let buf = BufReader::new(f);

        buf.lines().map(|r| {
            let line = r.unwrap();
            let (a, b) = line.split_once(',').unwrap();

            (parse_range(a), parse_range(b))
        }).collect()
    };

    let mut count = 0;

    for (a, b) in pairs {
        if test_ranges(a, b) {
            count += 1;
        }
    }

    println!("count: {}", count);
}
