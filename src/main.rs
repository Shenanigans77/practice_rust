use kitchen_garden::vegetables::new_vegetable;

use crate::kitchen_garden::vegetables::Vegetable;
//pub use crate::garden::good_snails;

mod kitchen_garden;
mod crisper;

fn main() {
    // New crisper drawer
    let mut drawer = crisper::Crisper::new_crisper(160);

    println!("{}", drawer.crisper_space());

    // New vegetables
    let squash = new_vegetable(String::from("Squash"), String::from("Yellow"), String::from("Gourd"));
  
    let spinach = new_vegetable(String::from("Spinach"), String::from("Green"), String::from("Leafy"));
        

    drawer.add_to_crisper(&squash.name);
    drawer.add_to_crisper(&spinach.name);


    // Print all vegetables in the drawer.
    for i in &drawer.contents_names {
        println!("{i}")
    }
}
 