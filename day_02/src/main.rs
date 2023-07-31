use std::fs::File;
use std::io::{BufReader, BufRead};


#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

impl Choice {
    fn get_points(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn convert_letter_to_object(letter: &str) -> Result<Self, String> {
        match letter {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => Err("Invalid input".to_string()),
        }
    }

    fn beats(&self, other: &Choice) -> bool {
        match (self, other) {
            (Choice::Rock, Choice::Scissors) => true,
            (Choice::Paper, Choice::Rock) => true,
            (Choice::Scissors, Choice::Paper) => true,
            _ => false,
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn get_points(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }

    fn from_round(round: &Round) -> Self {
        if round.me.beats(&round.enemy) {
            Outcome::Win
        } else if round.enemy.beats(&round.me) {
            Outcome::Lose
        } else {
            Outcome::Draw
        }
    }
}

struct Round {
    enemy: Choice,
    me: Choice,
}

impl Round {
    fn from_string(s: &str) -> Self{
        let round= s.split_whitespace().collect::<Vec<&str>>();
        let first = Choice::convert_letter_to_object(round[0]).unwrap();
        let second = Choice::convert_letter_to_object(round[1]).unwrap();
        Round{enemy:first, me:second}
    }
}

fn load_data(file_name: &str) -> Vec<Round>{
    let mut data = Vec::new();
    let file = File::open(file_name).expect("file not found");
    for line in BufReader::new(file).lines() {
        data.push(Round::from_string(&line.unwrap().to_string()));
    }
    data
}


fn get_points(round: &Round) -> u32 {
    let outcome = Outcome::from_round(round);
    outcome.get_points() + round.me.get_points()
}


fn main() {
    let data = load_data("src/input.txt");
    let total = data.iter().map(|round| get_points(round)).sum::<u32>();
    dbg!(total);
    // 15702
}
