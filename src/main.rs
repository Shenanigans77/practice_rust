use crate::kitchen_garden::vegetables::Vegetable;
//pub use crate::garden::good_snails;

mod kitchen_garden;
mod crisper;

fn main() {
    println!("Hello, world!");

    let mut drawer = crisper::Crisper{
        volume: 160,
        occupied_volume: 0,
        contents: Vec::new()
    };

    println!("{}", drawer.crisper_space());

    

    let squash = Vegetable{
        name: String::from("Squash"),
        color: String::from("Yellow"),
        family: String::from("Gourd"),
    };

    let spinach = Vegetable{
        name: String::from("Spinach"),
        color: String::from("Green"),
        family: String::from("Leafy"),
    };

    drawer.add_to_crisper(&squash.name);
    drawer.add_to_crisper(&spinach.name);

    println!("{}: a {} colored vegetable of the {} family.", squash.name, squash.color, squash.family);

    println!("{}", drawer.contents[1]);

    for i in &drawer.contents {
        println!("{i}")
    }
}
 