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

fn get_shared_type(a: &str, b: &str, c: &str) -> Option<char> {
    let mut types = [0; 58];

    for ch in a.chars() {
        let t = &mut types[get_id(ch)];
        *t = *t | 1;
    }

    for ch in b.chars() {
        let t = &mut types[get_id(ch)];
        *t = *t | 2;
    }

    for ch in c.chars() {
        if types[get_id(ch)] == 3 {
            return Some(ch);
        }
    }

    None
}

fn main() {
    let groups: Vec<(String, String, String)> = {
        let f = File::open("sacks.txt").unwrap();
        let buf = BufReader::new(f);

        let t: Vec<String> = buf.lines().map(|line| line.unwrap()).collect();

        t.chunks(3).map(|c| (c[0].to_owned(), c[1].to_owned(), c[2].to_owned())).collect()
    };

    let mut total = 0;
    for (a, b, c) in groups {
        let t = get_shared_type(&a, &b, &c).expect(format!("{}\n{}\n{}", a, b, c).as_str());

        let score = get_score(t);
        total += score;
    }

    println!("total: {}", total);
}
