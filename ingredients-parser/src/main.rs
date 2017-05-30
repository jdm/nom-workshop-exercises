#[macro_use]
extern crate nom;

use std::str::FromStr;

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

named!(ingredients<&[u8], Vec<(f32, Unit, &str)>>,
       /* finish me */);

static INPUT: &[u8] = r#"3.5 cups of flour
3 cups of coconut milk
0.5 mg of chili powder
10 tablespoons of salt
"#;

fn main() {
    println!("{:?}", ingredients(INPUT.as_bytes()));
}
