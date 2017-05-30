#[macro_use]
extern crate nom;

use nom::{digit, newline, space};
use std::str::{self, FromStr};

static INPUT: &str = r#"P1
# This is an example bitmap of the letter "J"
6 10
0 0 0 0 1 0
0 0 0 0 1 0
0 0 0 0 1 0
0 0 0 0 1 0
0 0 0 0 1 0
0 0 0 0 1 0
1 0 0 0 1 0
0 1 1 1 0 0
0 0 0 0 0 0
0 0 0 0 0 0
"#;

named!(comment<&[u8], ()>,
       do_parse!(
           char!('#') >>
           take_until_and_consume!("\n") >>
           ()));

named!(comments<&[u8], Vec<()>>,
       many0!(comment));

named!(header<&[u8], ()>,
       do_parse!(
           tag!("P1") >>
           newline >>
           comments >>
               (())));

#[derive(Debug)]
enum Pixel {
    On,
    Off,
}

impl FromStr for Pixel {
    type Err = ();
    fn from_str(s: &str) -> Result<Pixel, ()> {
        match s {
            "1" => Ok(Pixel::On),
            "0" => Ok(Pixel::Off),
            _ => Err(()),
        }
    }
}

named!(pixel<&[u8], Pixel>,
       map_res!(
           map_res!(
               alt!(tag!("1") | tag!("0")),
               str::from_utf8),
           Pixel::from_str));

named!(size<&[u8], usize>,
       map_res!(
           map_res!(digit, str::from_utf8),
           str::parse));

named!(pixels<&[u8], Vec<Vec<Pixel>>>,
       do_parse!(
           width: size >>
           space >>
           height: size >>
           newline >>
           rows: count!(
               count!(ws!(pixel), width),
               height) >>
               (rows)));

named!(image<&[u8], Vec<Vec<Pixel>>>,
       do_parse!(header >> pixels: pixels >> (pixels)));

fn main() {
    println!("{:?}", image(INPUT.as_bytes()));
}
