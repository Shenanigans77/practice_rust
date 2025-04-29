use crate::kitchen_garden::vegetables::Vegetable;
//pub use crate::garden::good_snails;

mod kitchen_garden;
mod crisper;

fn main() {
    println!("Hello, world!");

    let drawer = crisper::Crisper{
        volume: 160,
        occupied_volume: 0,
    };

    println!("{}", drawer.crisper_space());

    let squash = Vegetable{
        name: String::from("Squash"),
        color: String::from("Yellow"),
        family: String::from("Gourd"),
    };

    println!("{}: a {} colored vegetable of the {} family.", squash.name, squash.color, squash.family)
}
 