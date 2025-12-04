mod parse {
    use nom::{
        bytes::complete::tag,
        character::complete::one_of,
        multi::{many1, separated_list1},
        IResult, Parser,
    };

    use super::*;

    pub fn input(input: &str) -> IResult<&str, Input> {
        separated_list1(tag("\r\n"), bank).map(Input).parse(input)
    }

    fn bank(input: &str) -> IResult<&str, Bank> {
        many1(battery).map(Bank).parse(input)
    }

    fn battery(input: &str) -> IResult<&str, Battery> {
        one_of("0123456789")
            .map(|c| Battery(c.to_digit(10).unwrap()))
            .parse(input)
    }
}

#[derive(Debug)]
struct Input(Vec<Bank>);

#[derive(Debug)]
struct Bank(Vec<Battery>);

#[derive(Debug, Clone, Copy)]
struct Battery(u32);

impl Input {
    fn puzzle1(&self) -> u128 {
        self.0.iter().map(|bank| bank.max_jolts(2)).sum()
    }

    pub fn puzzle2(&self) -> u128 {
        self.0.iter().map(|bank| bank.max_jolts(12)).sum()
    }
}

impl Bank {
    fn max_jolts(&self, digits: u32) -> u128 {
        let capacities = self.0.iter().map(|b| b.0 as u128).collect::<Vec<u128>>();
        let mut jolts = 0;
        let mut start = 0;

        for n in (0..digits).rev() {
            let (max_index, max_digit) = capacities[start..capacities.len() - n as usize]
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|&(_, &digit)| digit)
                .unwrap();

            jolts += max_digit * 10u128.pow(n);
            start += max_index + 1;
        }

        jolts
    }
}

fn main() {
    let str = include_str!("../assets/input.txt");
    let (_, input) = parse::input(str).unwrap();
    println!("{:#?}", input.puzzle2());
}
