use std::fs;
use std::vec::Vec;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct GameSet {
    blue: i32,
    red: i32,
    green: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: i32,
    sets: Vec<GameSet>
}

#[derive(Debug, PartialEq, Eq)]
enum Colour {
    BLUE,
    RED,
    GREEN
}

#[derive(Debug, PartialEq, Eq)]
struct Cubes {
    colour: Colour,
    number: i32
}

fn parse_cube(cube: &str) -> Cubes {
    let xs: Vec<&str> = cube.split(" ").collect();
    return Cubes {
        colour: match xs[1] {
            "blue"  => Colour::BLUE,
            "red"   => Colour::RED,
            "green" => Colour::GREEN,
            _ => {
                dbg!(cube);
                panic!("Unrecognizable colour")
            }
        },
        number: xs[0].parse::<i32>().unwrap()
    }

}

fn parse_cubes(cubes: &str) -> Vec<Cubes> {
    return cubes.split(", ").map(parse_cube).collect();
}

fn parse_gameset(sets: &str) -> Vec<GameSet> {
    let mut ret: Vec<GameSet> = Vec::new();

    for set in sets.split("; ") {
        let mut gameset = GameSet { blue: 0, red: 0, green: 0 };

        for cube in parse_cubes(set) {
            match cube.colour {
                Colour::BLUE  => gameset.blue  += cube.number,
                Colour::GREEN => gameset.green += cube.number,
                Colour::RED   => gameset.red   += cube.number,
            }
        }

        ret.push(gameset);
    }

    return ret;
}

fn process_line(line: &str) -> Game {
    let re = Regex::new(r"Game (\d+): (.*)").unwrap();
    let groups = re.captures(line).unwrap();

    return Game{
        id: groups.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        sets: parse_gameset(groups.get(2).unwrap().as_str())
    };
}

fn possible_set(set: &GameSet) -> bool {
    return set.blue <= 14
        && set.green <= 13
        && set.red <= 12;
}

pub fn run() {
    let data = fs::read_to_string("res/day2.txt").unwrap();

    let sum: i32 = data.lines()
        .into_iter()
        .map(|line| process_line(line))
        .filter(|game| {
            return game.sets
                .iter()
                .filter(|set| !possible_set(set))
                .count() == 0;
        })
        .map(|game| game.id)
        .sum();

    println!("Day 2 - {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cubes_test() {
        let input = "3 blue, 4 red";
        assert_eq!(
            parse_cubes(input),
            vec![
                Cubes { colour: Colour::BLUE, number: 3 },
                Cubes { colour: Colour::RED , number: 4 },
            ]
        );
    }

    #[test]
    fn process_line_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game{
            id: 1,
            sets: vec![
                GameSet{ blue: 3, red: 4, green: 0 },
                GameSet{ blue: 6, red: 1, green: 2 },
                GameSet{ blue: 0, red: 0, green: 2 }
            ]

        };
        assert_eq!(process_line(input), game);
    }
}
