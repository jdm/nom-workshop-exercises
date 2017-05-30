#[macro_use]
extern crate nom;

use std::str::FromStr;

static INPUT: &[u8] = br#"P1
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

// We need a parser that recognizes the image header,
// ignores any comments that it contains (lines starting with #)
// then determines the width and height.
//
// Finally, we can parse an appropriate number of 1/0 pixels based
// on these width and height values.

// Useful parsers/combinators:
// * count!(parser, N) (apply a parser N times)
// * alt!(parser_a | parser_b) (apply one of parser_a or parser_b)
// * many0!(parser) (apply parser zero or more times)
// * take_until_and_consume!("string") (consume the input until a string is matched)

named!(image<&[u8], Vec<Vec<Pixel>>>,
       /* finish me */
       do_parse!(tag!("P1") >> (vec![])));

fn main() {
    println!("{:?}", image(INPUT));
}
