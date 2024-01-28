pub struct Tolerance {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub min_brightness: u8,
    pub max_brightness: u8,
}

pub const tolerance: Tolerance = Tolerance {
    red: 16,
    green: 16,
    blue: 16,
    min_brightness: 16,
    max_brightness: 240,
};
