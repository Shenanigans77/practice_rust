//mod crisper;

#[derive(Debug)]
pub struct Crisper {
    volume: i32,
    occupied_volume: i32,
}

impl Crisper {
    pub fn crisper_space(&self) -> i32 {
        let open_space = self.volume - self.occupied_volume;
        open_space
    }
}