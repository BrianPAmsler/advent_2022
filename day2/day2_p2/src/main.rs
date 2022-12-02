use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Clone, Copy, Debug)]
enum Choice {
    Rock,
    Paper,
    Scisors
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Win,
    Loss,
    Tie
}

fn mod_(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

fn get_outcome(round: (Choice, Choice)) -> Outcome {
    match mod_(round.1 as i32 - round.0 as i32, 3) {
        1 => Outcome::Win,
        0 => Outcome::Tie,
        _ => Outcome::Loss
    }
}

fn calc_score(round: (Choice, Choice)) -> i32 {
    let outcome = get_outcome(round);

    let outcome_score = match outcome {
        Outcome::Win => 6,
        Outcome::Tie => 3,
        Outcome::Loss => 0,
    };

    let choice_score = match round.1 {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scisors => 3,
    };

    outcome_score + choice_score
}

fn load_rounds() -> Vec<(Choice, Choice)> {
    let mut rounds = Vec::new();

    let f = File::open("strategy.txt").unwrap();
    let reader = BufReader::new(f);

    for r in reader.lines() {
        let line = r.unwrap();
        let (opp, me) = line.split_once(" ").unwrap();

        let opp = match opp {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scisors,
            _ => panic!("bad file!")
        };

        let me = match me {
            "X" => int_to_choice((opp as i32+ 2) % 3),
            "Y" => opp,
            "Z" => int_to_choice((opp as i32 + 1) % 3),
            _ => panic!("bad file!")
        };

        rounds.push((opp, me));
    }

    rounds
}

fn int_to_choice(int: i32) -> Choice {
    match int {
        0 => Choice::Rock,
        1 => Choice::Paper,
        2 => Choice::Scisors,
        _ => panic!("bad")
    }
}

fn main() {
    let rounds = load_rounds();

    let mut total = 0;
    for round in rounds {
        let score = calc_score(round);

        total += score;
    }

    println!("total: {}", total);
}
