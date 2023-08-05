use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

pub struct Record {
    pub sensor: Point,
    pub beacon: Point,
}

impl Record {
    pub fn must_parse(input: &str) -> Self {
        all_consuming(Self::parse)(input).finish().unwrap().1
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                preceded(tag("Sensor at "), Point::parse),
                tag(": closest beacon is at "),
                Point::parse,
            ),
            |(sensor, beacon)| Record { sensor, beacon },
        )(input)
    }
}

impl Point {
    fn parse(input: &str) -> IResult<&str, Point> {
        map(
            separated_pair(
                preceded(tag("x="), nom::character::complete::i64),
                tag(", "),
                preceded(tag("y="), nom::character::complete::i64),
            ),
            |(x, y)| Point { x, y },
        )(input)
    }

    pub fn manhattan_dist(self, other: Self) -> i64 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i64
    }
}
