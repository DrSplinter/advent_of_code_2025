use std::iter::once;

mod parse {
    use nom::{
        IResult, Parser,
        branch::alt,
        bytes::complete::tag,
        character::complete::{one_of, space0},
        multi::{many1, separated_list1},
        sequence::separated_pair,
    };

    use super::*;

    pub fn input(input: &str) -> IResult<&str, Input> {
        separated_pair(grid_of_digits, tag("\r\n"), operations)
            .map(|(rows_of_digits, operations)| Input {
                grid_of_digits: rows_of_digits,
                operations,
            })
            .parse(input)
    }

    fn operations(input: &str) -> IResult<&str, Vec<Operation>> {
        let operation = alt((
            tag("+").map(|_| Operation::Add),
            tag("*").map(|_| Operation::Multiply),
        ));

        separated_list1(space0, operation).parse(input)
    }

    fn grid_of_digits(input: &str) -> IResult<&str, Vec<Vec<char>>> {
        separated_list1(tag("\r\n"), many1(one_of("0123456789 "))).parse(input)
    }
}

#[derive(Debug)]
struct Input {
    grid_of_digits: Vec<Vec<char>>,
    operations: Vec<Operation>,
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl Input {
    fn puzzle1(&self) -> u128 {
        self.number_groups()
            .iter()
            .zip(&self.operations)
            .map(|(group, op)| op.apply(group.interpret_human()))
            .sum()
    }

    fn puzzle2(&self) -> u128 {
        self.number_groups()
            .iter()
            .zip(&self.operations)
            .map(|(group, op)| op.apply(group.interpret_cephalopod()))
            .sum()
    }

    fn number_groups<'a>(&'a self) -> Vec<NumberGroup<'a>> {
        let row_len = self.grid_of_digits[0].len();
        let empty_cols: Vec<_> = (0..row_len)
            .filter(|&col_idx| self.grid_of_digits.iter().all(|row| row[col_idx] == ' '))
            .collect();
        let starts = once(0).chain(empty_cols.iter().cloned().map(|i| i + 1));
        let ends = empty_cols.iter().cloned().chain(once(row_len));

        starts
            .zip(ends)
            .map(|(s, e)| NumberGroup(self.grid_of_digits.iter().map(|row| &row[s..e]).collect()))
            .collect()
    }
}

#[derive(Debug)]
struct NumberGroup<'a>(Vec<&'a [char]>);

impl<'a> NumberGroup<'a> {
    fn interpret_human(&self) -> impl Iterator<Item = u128> {
        self.0
            .iter()
            .map(|digits| chars_to_number(digits.iter().cloned()))
    }

    fn interpret_cephalopod(&self) -> impl Iterator<Item = u128> {
        (0..self.0[0].len()).map(|idx| chars_to_number(self.0.iter().map(|row| row[idx])))
    }
}

impl Operation {
    fn apply<I: IntoIterator<Item = u128>>(&self, numbers: I) -> u128 {
        match self {
            Operation::Add => numbers.into_iter().sum(),
            Operation::Multiply => numbers.into_iter().product(),
        }
    }
}

fn chars_to_number(digits: impl IntoIterator<Item = char>) -> u128 {
    digits
        .into_iter()
        .collect::<String>()
        .trim()
        .parse::<u128>()
        .unwrap()
}

fn main() {
    let str = include_str!("../assets/input.txt");
    let (_, input) = parse::input(str).unwrap();
    println!("{:#?}", input.puzzle2());
}
