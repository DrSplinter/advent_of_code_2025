mod parse {
    use nom::{
        bytes::complete::tag, character::complete::usize, multi::separated_list1,
        sequence::separated_pair, IResult, Parser,
    };

    use super::*;

    fn range(input: &str) -> IResult<&str, Range> {
        separated_pair(usize, tag("-"), usize)
            .map(|(start, end)| Range { start, end })
            .parse(input)
    }

    pub fn input(input: &str) -> IResult<&str, Input> {
        separated_list1(tag(","), range).map(Input).parse(input)
    }
}

#[derive(Debug)]
struct Input(Vec<Range>);

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Input {
    fn puzzle1(&self) -> usize {
        self.0
            .iter()
            .flat_map(|range| range.invalid_ids(invalid_id))
            .sum()
    }

    fn puzzle2(&self) -> usize {
        self.0
            .iter()
            .flat_map(|range| range.invalid_ids(is_even_more_invalid))
            .sum()
    }
}

impl Range {
    fn invalid_ids<'a>(
        &'a self,
        invalidness: impl Fn(usize) -> bool + 'a,
    ) -> impl Iterator<Item = usize> + 'a {
        (self.start..=self.end).filter(move |id| invalidness(*id))
    }
}

fn invalid_id(id: usize) -> bool {
    let str = id.to_string();
    let is_even_len = str.len() % 2 == 0;
    let first_half: String = str.chars().take(str.len() / 2).collect();
    let second_half: String = str.chars().skip(str.len() / 2).collect();

    is_even_len && first_half == second_half
}

fn is_even_more_invalid(id: usize) -> bool {
    let str = id.to_string();
    let chars = str.chars().collect::<Vec<_>>();

    (1..=str.len() / 2)
        .filter(|x| str.len() % x == 0)
        .any(|chunk_size| {
            chars
                .chunks_exact(chunk_size)
                .collect::<Vec<_>>()
                .windows(2)
                .all(|w| w[0] == w[1])
        })
}

fn main() {
    let str = include_str!("../assets/input.txt");
    let (_, input) = parse::input(str).unwrap();
    println!("{:#?}", input.puzzle2());
}
