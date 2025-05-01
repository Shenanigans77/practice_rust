
#[derive(Debug)]
pub struct Vegetable{
    pub name: String,
    pub color: String,
    pub family: VegetableFamily,    
}

impl Vegetable {
    pub fn veg_color(&self) -> String {
        String::from(&self.color)
    }

    pub fn veg_spoil_rate(&self) -> i32 {
        match &self.family {
            VegetableFamily::Gourd => 10,
            VegetableFamily::Leafy => 25,
            VegetableFamily::Cruciferous => 23,
            VegetableFamily::Nightshade => 8,
            VegetableFamily::Tuber => 9,
        }
    }

    //pub fn veg_family_to_string(&self) -> &String {
    //    match &self.family {
    //        VegetableFamily::Gourd => $String::from
    //    }
    //}
}  

// playing with Enums - Todo - should this be a pub enum?
#[derive(Debug)]
pub enum VegetableFamily {
    Gourd,
    Leafy,
    Cruciferous,
    Nightshade,
    Tuber
}

// Match a string to an Enum
fn match_name_to_family(family_name: String) -> VegetableFamily {
    if family_name == String::from("Gourd") {
        VegetableFamily::Gourd
    } else if family_name == String::from("Leafy") {
        VegetableFamily::Leafy
    } else if family_name == String::from("Cruciferous") {
        VegetableFamily::Cruciferous
    } else if family_name == String::from("Nightshade") {
        VegetableFamily::Nightshade  
    } else if family_name == String::from("Tuber") {
        VegetableFamily::Tuber   
    } else {
        // This should not return something else. ToDo - more elegant way to handle this?
        panic!()
    }
}

// A function to build vegetables
pub fn new_vegetable(name: String, color: String, family_name: String) -> Vegetable {
    Vegetable { 
        name: name, 
        color: color, 
        family: match_name_to_family(family_name),
    }
}