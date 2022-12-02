#[derive(Clone, Copy)]
enum Result {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}

trait Game {
    fn value(&self) -> u32;
    fn result(&self, vs: &Shape) -> Result;
    fn vs(&self, result: &Result) -> Shape;
}

impl Game for Shape {
    fn value(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn result(&self, vs: &Shape) -> Result {
        match (self, vs) {
            // lose
            (Shape::Rock, Shape::Paper)
            | (Shape::Paper, Shape::Scissors)
            | (Shape::Scissors, Shape::Rock) => Result::Lose,

            // draw
            (Shape::Rock, Shape::Rock)
            | (Shape::Paper, Shape::Paper)
            | (Shape::Scissors, Shape::Scissors) => Result::Draw,

            // win
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => Result::Win,
        }
    }

    fn vs(&self, result: &Result) -> Shape {
        match (self, result) {
            // rock
            (Shape::Rock, Result::Draw)
            | (Shape::Paper, Result::Lose)
            | (Shape::Scissors, Result::Win) => Shape::Rock,

            // paper
            (Shape::Rock, Result::Win)
            | (Shape::Paper, Result::Draw)
            | (Shape::Scissors, Result::Lose) => Shape::Paper,

            // scissors
            (Shape::Rock, Result::Lose)
            | (Shape::Paper, Result::Win)
            | (Shape::Scissors, Result::Draw) => Shape::Scissors,
        }
    }
}

fn shape_from(c: &str) -> Option<Shape> {
    match c {
        "A" | "X" => Some(Shape::Rock),
        "B" | "Y" => Some(Shape::Paper),
        "C" | "Z" => Some(Shape::Scissors),
        _ => None,
    }
}

fn result_from(c: &str) -> Option<Result> {
    match c {
        "X" => Some(Result::Lose),
        "Y" => Some(Result::Draw),
        "Z" => Some(Result::Win),
        _ => None,
    }
}

fn main() {
    let txt = include_bytes!("input.txt");
    let input = String::from_utf8(txt.to_vec()).unwrap();

    let first = input.split("\n").filter_map(|matchup| {
        let hands = &matchup
            .split(" ")
            .filter_map(|shape| shape_from(shape))
            .collect::<Vec<Shape>>()[..];
        if let [opp, you] = hands {
            Some(you.value() + you.result(opp) as u32)
        } else {
            None
        }
    });

    println!("Suspected score: {}", first.sum::<u32>());

    let second = input.split("\n").filter_map(|matchup| {
        if let [opp, result] = matchup.split(" ").collect::<Vec<&str>>()[..] {
            let opp = shape_from(opp).unwrap();
            let result = result_from(result).unwrap();
            Some(result as u32 + opp.vs(&result).value())
        } else {
            None
        }
    });

    println!("Actual score: {}", second.sum::<u32>());
}
