#[macro_use]
extern crate nom;

use nom::{alpha, digit, space};
use std::str::{self, FromStr};

#[derive(Debug)]
enum Unit {
    Cups,
    Grams,
    Milligrams,
    Teaspoons,
    Tablespoons,
}

impl FromStr for Unit {
    type Err = ();
    fn from_str(s: &str) -> Result<Unit, ()> {
        match s {
            _ => Err(()),
        }
    }
}

named!(int<&[u8], f32>,
       map_res!(
           map_res!(digit, str::from_utf8),
           str::parse::<f32>));

named!(unit<&[u8], Unit>,
       map_res!(
           map_res!(alpha, str::from_utf8),
           Unit::from_str));

// We need a parser that can accept a sequence of:
// * an amount (either a decimal number or floating point number)
// * a unit string (eg. cups, mg, tablespoons)
// * "of"
// * an ingredient string

// Useful parsers and combinators:
// * alt!(parser_a | parser_b)
// * many1!(parser)
// * take_until_and_consume!(..)
// * space (parses a whitespace value)

named!(ingredients<&[u8], Vec<(f32, Unit, &str)>>,
       /* finish me */
       do_parse!(space >> (vec![])));

static INPUT: &[u8] = br#"3.5 cups of flour
3 cups of coconut milk
0.5 mg of chili powder
10 tablespoons of salt
"#;

fn main() {
    println!("{:?}", ingredients(INPUT));
}
