mod parse {
    use nom::{
        IResult, Parser, bytes::complete::tag, character::complete::u128, multi::separated_list1,
        sequence::separated_pair,
    };

    use super::*;

    pub fn input(input: &str) -> IResult<&str, Input> {
        separated_pair(intervals, tag("\r\n\r\n"), ingredients)
            .map(|(intervals, ingredients)| Input {
                intervals,
                ingredients,
            })
            .parse(input)
    }

    fn intervals(input: &str) -> IResult<&str, Vec<Interval>> {
        let interval =
            separated_pair(u128, tag("-"), u128).map(|(start, end)| Interval { start, end });

        separated_list1(tag("\r\n"), interval).parse(input)
    }

    fn ingredients(input: &str) -> IResult<&str, Vec<u128>> {
        separated_list1(tag("\r\n"), u128)
            .map(|v| v.into_iter().collect())
            .parse(input)
    }
}

#[derive(Debug)]
struct Input {
    intervals: Vec<Interval>,
    ingredients: Vec<u128>,
}

impl Input {
    fn puzzle1(&self) -> u128 {
        self.ingredients
            .iter()
            .filter(|i| self.intervals.iter().any(|r| r.contains(i)))
            .count() as u128
    }

    fn puzzle2(&self) -> u128 {
        self.intervals
            .iter()
            .fold(FiniteNumberSet::new(), |set, interval| {
                set.with_interval(*interval)
            })
            .cardinality()
    }
}

#[derive(Debug)]
struct FiniteNumberSet {
    intervals: Vec<Interval>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Interval {
    start: u128,
    end: u128,
}

impl std::fmt::Debug for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl FiniteNumberSet {
    fn new() -> Self {
        FiniteNumberSet { intervals: vec![] }
    }

    fn cardinality(&self) -> u128 {
        self.intervals
            .iter()
            .map(|interval| interval.cardinality())
            .sum()
    }

    fn with_interval(self, interval: Interval) -> Self {
        let (merges, mut intervals): (Vec<_>, Vec<_>) = self
            .intervals
            .into_iter()
            .partition(|current| current.merges_with(&interval));
        intervals.push(merges.into_iter().fold(interval, Interval::merge));

        FiniteNumberSet { intervals }
    }
}

impl Interval {
    fn contains(&self, n: &u128) -> bool {
        &self.start <= n && n <= &self.end
    }

    fn merges_with(&self, other: &Interval) -> bool {
        !(self.end < other.start || self.start > other.end)
    }

    fn merge(self, other: Interval) -> Interval {
        Interval {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    fn cardinality(&self) -> u128 {
        self.end - self.start + 1
    }
}

fn main() {
    let str = include_str!("../assets/input.txt");
    let (_, input) = parse::input(str).unwrap();
    println!("{:#?}", input.puzzle2());
}
