use std::collections::HashMap;


use kitchen_garden::vegetables::new_vegetable;

//use crate::kitchen_garden::vegetables::Vegetable;
//pub use crate::garden::good_snails;

mod kitchen_garden;
mod crisper;

fn main() {
    // Basic information to manipulate later.
    let num_ordered_list: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let num_random_list= [100,250,375,500];

    let mut unsorted_nums: Vec<i32> = Vec::new();

    let mut vegetable_colors: Vec<String> = Vec::new();

    let mut drawer_contents = HashMap::new();
    
    // New crisper drawer
    let mut drawer = crisper::Crisper::new_crisper(160);

    println!("{}", drawer.crisper_space());

    // New vegetables
    let squash = new_vegetable(String::from("Squash"), String::from("Yellow"), String::from("Gourd"));
  
    let spinach = new_vegetable(String::from("Spinach"), String::from("Green"), String::from("Leafy"));

    let cauliflower = new_vegetable(String::from("Cauliflower"), String::from("White"), String::from("Cruciferous"));

    drawer.add_to_crisper(&squash.name);
    drawer_contents.insert (&squash.name, squash.veg_color());
    print!("Spoil rate for {} is {}.\n", squash.name, squash.veg_spoil_rate());    
    drawer.add_to_crisper(&spinach.name);
    drawer_contents.insert(&spinach.name, spinach.veg_color());
    print!("Spoil rate for {} is {}.\n", spinach.name, spinach.veg_spoil_rate());
    drawer.add_to_crisper(&cauliflower.name);
    drawer_contents.insert(&cauliflower.name, cauliflower.veg_color());
    print!("Spoil rate for {} is {}.\n", cauliflower.name, cauliflower.veg_spoil_rate());

    vegetable_colors.push(squash.color);
    vegetable_colors.push(spinach.color);
    vegetable_colors.push(cauliflower.veg_color());

    

    // Print all vegetables in the drawer.
    for i in &drawer.contents_names {
        println!("{i}")
    }

    // Show the spoil rate for all vegetables in the drawer
    //for i in &drawer {
    //    print!("Spoil rate for {} is {}.", i.name, i.veg_spoil_rate)
    //}

    // Use any unused variables.
    let mut i: usize = 0;
    let mut number = 0;
    while i < 10 {
        print!("{}", i);
        for j in num_random_list {
            number = num_ordered_list[i] + j;
            print!("{}\n", number);
        } 
        unsorted_nums.push(number);
        i+= 1;
    }
}
 