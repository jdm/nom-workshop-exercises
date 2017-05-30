#[macro_use]
extern crate nom;

use nom::{alpha, digit, float, space};
use std::str::{self, FromStr};

named!(int<&[u8], f32>,
       map_res!(
           map_res!(digit, str::from_utf8),
           str::parse::<f32>));

named!(amount<&[u8], f32>,
       alt_complete!(float | int));

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
            "cups" => Ok(Unit::Cups),
            "g" => Ok(Unit::Grams),
            "mg" => Ok(Unit::Milligrams),
            "teaspoons" => Ok(Unit::Teaspoons),
            "tablespoons" => Ok(Unit::Tablespoons),
            _ => Err(()),
        }
    }
}

named!(unit<&[u8], Unit>,
       map_res!(
           map_res!(alpha, str::from_utf8),
           Unit::from_str));

named!(unit_amount<&[u8], (f32, Unit)>,
       do_parse!(
           amount: amount >>
           space >>
           unit: unit >>
           (amount, unit)));

named!(ingredient_name<&[u8], &str>,
       map_res!(take_until_and_consume!("\n"), str::from_utf8));

named!(ingredient<&[u8], ((f32, Unit), &str)>,
       do_parse!(
           unit_amount: unit_amount >>
           space >>
           tag!("of") >>
           space >>
           ingredient_name: ingredient_name >>
           (unit_amount, ingredient_name)));

named!(ingredients<&[u8], Vec<((f32, Unit), &str)>>,
       many1!(ingredient));

fn main() {
    println!("{:?}", ingredients(b"3.5 cups of flour\n3 cups of coconut milk\n"));
}
