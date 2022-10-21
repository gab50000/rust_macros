#![feature(trace_macros)]
#![feature(log_syntax)]

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
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

macro_rules! make_struct {
    ($id:ident) => {
        #[derive(Debug)]
        struct $id<T: FromStr>(T);
        impl<T> $id<T>
        where
            T: FromStr,
        {
            fn parse(input: &str) -> IResult<&str, Self> {
                let prefix = tuple((tag("byr"), char(':')));
                let parse_numbers = map_res(digit1, T::from_str);
                let parser = preceded(prefix, parse_numbers);
                let mut p = map(parser, Self);
                p(input)
            }
        }
    };
}

trace_macros!(true);
make_struct!(Blablub);
make_struct!(Blablib);
make_struct!(ABC);
trace_macros!(false);

fn main() {
    println!("{}", times_5!(123));
    println!("{}", mul![1, 2, 3, 4, 5]);
    println!("{}", mul![]);
    println!("{:?}", Blablub(-123));
    println!("{:?}", Blablib::<i32>::parse("byr:499"));
}
