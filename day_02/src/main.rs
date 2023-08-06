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

    fn choose_strategy_base_on_letter(other: &Choice, letter: &str) -> Result<Self, String> {
        match letter {
            "X" =>  Ok(other.inferior_choice()),
            "Y" => Ok(other.clone()),
            "Z" => Ok(other.superior_choice()),
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

    fn superior_choice(&self) -> Choice {
        match self {
            Choice::Rock=> Choice::Paper,
            Choice::Paper=> Choice::Scissors,
            Choice::Scissors=> Choice::Rock,
        }
    }

    fn inferior_choice(&self) -> Choice {
        match self {
            Choice::Paper=> Choice::Rock,
            Choice::Scissors=> Choice::Paper,
            Choice::Rock=> Choice::Scissors,
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
    fn from_string_v1(s: &str) -> Self{
        let round= s.split_whitespace().collect::<Vec<&str>>();
        let first = Choice::convert_letter_to_object(round[0]).unwrap();
        let second = Choice::convert_letter_to_object(round[1]).unwrap();
        Round{enemy:first, me:second}
    }

    fn from_string_v2(s: &str) -> Self{
        let round= s.split_whitespace().collect::<Vec<&str>>();
        let first = Choice::convert_letter_to_object(round[0]).unwrap();
        let second = Choice::choose_strategy_base_on_letter(&first,round[1]).unwrap();
        Round{enemy:first, me:second}
    }
}

fn load_data(file_name: &str) -> Vec<String>{
    let mut data = Vec::new();
    let file = File::open(file_name).expect("file not found");
    for line in BufReader::new(file).lines() {
        data.push(line.expect("Could not parser line"));
    }
    data
}


fn get_points(round: &Round) -> u32 {
    let outcome = Outcome::from_round(round);
    outcome.get_points() + round.me.get_points()
}


fn main() {
    let data = load_data("src/input.txt");
    let rounds_v1 = data.iter().map(|line| Round::from_string_v1(&line)).collect::<Vec<Round>>();
    let rounds_v2 = data.iter().map(|line| Round::from_string_v2(&line)).collect::<Vec<Round>>();

    let total_v1 = rounds_v1.iter().map(|round| get_points(round)).sum::<u32>();
    let total_v2 = rounds_v2.iter().map(|round| get_points(round)).sum::<u32>();

    dbg!(total_v1);
    // 15523
    dbg!(total_v2);
    // 15702
}
