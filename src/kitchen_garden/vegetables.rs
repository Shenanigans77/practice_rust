
#[derive(Debug)]
pub struct Vegetable{
    pub name: String,
    pub color: String,
    pub family: String,    
}

impl Vegetable {
    pub fn veg_color(&self) -> &String {
        &self.color
    }

    pub fn veg_family(&self) -> &String {
        &self.family
    }
} 