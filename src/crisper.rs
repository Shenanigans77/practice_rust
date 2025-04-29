//mod crisper;

#[derive(Debug)]
pub struct Crisper {
    pub volume: i32,
    pub occupied_volume: i32,
    pub contents: Vec<String>
}

impl Crisper {
    pub fn crisper_space(&self) -> i32 {
        let open_space = self.volume - self.occupied_volume;
        open_space
    }
    // Add contents to the crisper.
    pub fn add_to_crisper(&mut self, vegetable: &String) {
        self.contents.push(vegetable.to_string());
    }
}