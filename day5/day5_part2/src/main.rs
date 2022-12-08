use std::{fs::File, io::{BufReader, BufRead}};

fn parse_row(row: String) -> Vec<char> {
    row.as_bytes().chunks(4).map(|c| c[1] as char).collect()
}

fn rows_to_cols(rows: &[Vec<char>]) -> Vec<Vec<char>> {
    let len = rows[0].len();

    let mut cols = vec![Vec::new(); len];

    for row in rows.iter().rev() {
        for (i, c) in row.iter().enumerate() {
            if *c != ' ' {
                cols[i].push(*c);
            }
        }
    }

    cols
}

fn parse_move(mv: &str) -> (usize, usize, usize) {
    let tokens: Vec<&str> = mv.split(" ").collect();

    let count = tokens[1].parse().unwrap();
    let from: usize = tokens[3].parse().unwrap();
    let to: usize = tokens[5].parse().unwrap();

    (count, from - 1, to - 1)
}

fn do_move(count: usize, from: usize, to: usize, stacks: &mut Vec<Vec<char>>) {
    let len = stacks[from].len();
    let cs = stacks[from][len - count..].to_owned();
    
    stacks[to].extend(cs);
    stacks[from].truncate(len - count);
}

fn main() {
    let f = File::open("moves.txt").unwrap();
    let buf = BufReader::new(f);

    let mut rows = Vec::new();
    let mut moves = Vec::new();
    let mut parse_stacks = true;
    for line in buf.lines() {
        if parse_stacks {
            let row = parse_row(line.unwrap());
    
            if row.len() == 0 {
                rows.pop();
                parse_stacks = false;
            } else {
                rows.push(row);
            }
        } else {
            let mv = parse_move(line.unwrap().as_str());
            moves.push(mv);
        }
    }

    let mut stacks = rows_to_cols(&rows[..]);

    for (count, from, to) in moves {
        do_move(count, from, to, &mut stacks);
    }

    println!("{:?}", stacks);

    for col in stacks {
        print!("{}", col.last().unwrap());
    }
}
