use std::{collections::BTreeSet, iter::successors};

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        multi::{many1, separated_list1},
        IResult, Parser,
    };

    use super::*;

    pub fn input(input: &str) -> IResult<&str, Input> {
        separated_list1(tag("\r\n"), many1(place))
            .map(|grid| Input { grid })
            .parse(input)
    }

    fn place(input: &str) -> IResult<&str, Place> {
        alt((
            tag(".").map(|_| Place::Empty),
            tag("@").map(|_| Place::PaperRoll),
        ))
        .parse(input)
    }
}

#[derive(Debug)]
struct Input {
    grid: Vec<Vec<Place>>,
}

#[derive(Debug)]
enum Place {
    Empty,
    PaperRoll,
}

impl Input {
    pub fn puzzle1(&self) -> u32 {
        places_to_remove(&roll_places(self)).count() as u32
    }

    pub fn puzzle2(&self) -> u32 {
        let places = roll_places(self);

        successors(Some((places, 0)), |(places, removed)| {
            let to_remove = places_to_remove(places).collect::<BTreeSet<_>>();

            (!to_remove.is_empty()).then(|| {
                let new_places = places
                    .difference(&to_remove)
                    .map(|(x, y)| (*x, *y))
                    .collect::<BTreeSet<_>>();
                (new_places, removed + to_remove.len() as u32)
            })
        })
        .last()
        .map(|(_, removed)| removed)
        .unwrap_or(0)
    }
}

fn roll_places(input: &Input) -> BTreeSet<(i32, i32)> {
    input
        .grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, place)| {
                matches!(place, Place::PaperRoll).then_some((x as i32, y as i32))
            })
        })
        .collect()
}

fn neighbors(x: i32, y: i32, places: &BTreeSet<(i32, i32)>) -> impl Iterator<Item = (i32, i32)> {
    [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ]
    .into_iter()
    .filter_map(move |(dx, dy)| places.get(&(x + dx, y + dy)).map(|x| *x))
}

fn places_to_remove(places: &BTreeSet<(i32, i32)>) -> impl Iterator<Item = (i32, i32)> + '_ {
    places
        .iter()
        .copied()
        .filter(move |(x, y)| neighbors(*x, *y, &places).count() < 4)
}

fn main() {
    let str = include_str!("../assets/input.txt");
    let (_, input) = parse::input(str).unwrap();
    println!("{:#?}", input.puzzle2());
}
