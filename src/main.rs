
//use crate::garden::good_snails;
mod crisper;
use crisper::Crisper;

fn main() {
    println!("Hello, world!");

    let drawer = Crisper{
        volume: 160,
        occupied_volume: 0,
    };

    println!("{}", drawer.crisper_space())
}
 