#[macro_use]
extern crate nom;

use std::str::FromStr;

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

named!(image<&[u8], Vec<Vec<Pixel>>>,
       /* finish me */);

fn main() {
    println!("{:?}", image(INPUT.as_bytes()));
}
