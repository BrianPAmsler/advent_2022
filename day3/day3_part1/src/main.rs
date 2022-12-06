use std::{fs::File, io::{BufReader, BufRead}};

fn get_score(t: char) -> i32 {
    if t as i32 > 'a' as i32 {
        t as i32 - 'a' as i32 + 1
    } else {
        t as i32 - 'A' as i32 + 27
    }
}

fn get_id(t: char) -> usize {
    t as usize - 'A' as usize
}

fn get_shared_type(a: &str, b: &str) -> Option<char> {
    let mut types = [false; 58];

    for c in a.chars() {
        types[get_id(c)] = true;
    }

    for c in b.chars() {
        if types[get_id(c)] {
            return Some(c);
        }
    }

    None
}

fn main() {
    println!("{}, {}", get_score('p'), get_score('L'));

    let sacks: Vec<String> = {
        let f = File::open("sacks.txt").unwrap();
        let buf = BufReader::new(f);

        buf.lines().map(|line| line.unwrap()).collect()
    };

    let mut total = 0;
    for sack in sacks {
        let l = sack.len() / 2;
        let a = &sack[..l];
        let b = &sack[l..];

        let t = get_shared_type(a, b).unwrap();

        let score = get_score(t);
        total += score;
    }

    println!("total: {}", total);
}
