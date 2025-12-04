mod parse {
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::u32, multi::separated_list1,
        IResult, Parser,
    };

    use super::*;

    pub fn input(input: &str) -> IResult<&str, Input> {
        separated_list1(tag("\r\n"), rotation)
            .map(|rotations| Input { rotations })
            .parse(input)
    }

    fn rotation(input: &str) -> IResult<&str, Rotation> {
        (direction, u32)
            .map(|(direction, distance)| Rotation {
                direction,
                distance,
            })
            .parse(input)
    }

    fn direction(input: &str) -> IResult<&str, Direction> {
        alt((
            tag("L").map(|_| Direction::Left),
            tag("R").map(|_| Direction::Right),
        ))
        .parse(input)
    }
}

#[derive(Debug)]
struct Input {
    rotations: Vec<Rotation>,
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    distance: u32,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Lock {
    current_number: u32,
    max_number: u32,
}

impl Input {
    pub fn puzzle1(&self) -> u32 {
        let lock = Lock {
            current_number: 50,
            max_number: 100,
        };

        self.rotations
            .iter()
            .scan(lock, |acc, rotation| Some(acc.just_rotate(rotation)))
            .filter(|lock| lock.current_number == 0)
            .count() as u32
    }

    pub fn puzzle2(&self) -> u32 {
        let lock = Lock {
            current_number: 50,
            max_number: 100,
        };

        self.rotations
            .iter()
            .scan(lock, |acc, rotation| Some(acc.rotate_with_count(rotation)))
            .sum()
    }
}

impl Lock {
    pub fn rotate_with_count(&mut self, rotation: &Rotation) -> u32 {
        let mut zero_counter = 0;
        let movement = rotation.direction.movement();
        let mut total_movement = movement * (rotation.distance as i32);

        while total_movement.abs() >= self.max_number as i32 {
            total_movement += -total_movement.signum() * self.max_number as i32;
            zero_counter += 1;
        }

        let mut new_number = self.current_number as i32 + total_movement;

        if new_number < 0 {
            new_number += self.max_number as i32;
            if self.current_number != 0 {
                zero_counter += 1;
            }
        } else if new_number >= self.max_number as i32 {
            new_number -= self.max_number as i32;
            zero_counter += 1;
        } else if new_number == 0 {
            zero_counter += 1;
        }

        *self = Lock {
            current_number: new_number as u32,
            max_number: self.max_number,
        };

        zero_counter
    }

    pub fn just_rotate(&mut self, rotation: &Rotation) -> Lock {
        self.rotate_with_count(rotation);
        self.clone()
    }
}

impl Direction {
    pub fn movement(&self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}

fn main() {
    let str = include_str!("../assets/input.txt");
    let (_, input) = parse::input(str).unwrap();
    println!("{:#?}", input.puzzle2());
}
