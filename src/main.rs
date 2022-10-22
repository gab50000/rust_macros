#![feature(trace_macros)]
#![feature(log_syntax)]

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1};
use nom::combinator::map;
use nom::combinator::map_res;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use std::{io, str::FromStr};

macro_rules! times_5 {
    ($e:expr) => {
        5 * $e
    };
}

macro_rules! mul {
    (
        $($element: expr) , *
    ) => {
         {
            let mut prod = 1;
            $( prod *= $element;)*
            prod
    }
    };
}

trait Parse {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

macro_rules! make_struct {
    ($id:ident, &str, $tag:literal) => {
        #[derive(Debug)]
        struct $id<'a>(&'a str);
        impl<'a> $id<'a> {
            fn parse(input: &'a str) -> IResult<&'a str, Self> {
                let prefix = tuple((tag($tag), char(':')));
                let parser = preceded(prefix, alpha1);
                let mut p = map(parser, Self);
                p(input)
            }
        }
    };
    ($id:ident,$type:ty, $tag:literal) => {
        #[derive(Debug)]
        struct $id($type);
        impl $id {
            fn parse(input: &str) -> IResult<&str, Self> {
                let prefix = tuple((tag($tag), char(':')));
                let parse_numbers = map_res(digit1, <$type>::from_str);
                let parser = preceded(prefix, parse_numbers);
                let mut p = map(parser, Self);
                p(input)
            }
        }
    };
}

trace_macros!(true);
make_struct!(BirthYear, u32, "byr");
make_struct!(EyeColor, &str, "ecl");
trace_macros!(false);

fn main() {
    println!("{}", times_5!(123));
    println!("{}", mul![1, 2, 3, 4, 5]);
    println!("{}", mul![]);
    println!("{:?}", BirthYear::parse("byr:499"));
    println!("{:?}", EyeColor::parse("ecl:blablub"));
}
