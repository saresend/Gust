use std::fs::File;

pub fn get_rgb_representation(hex: &str) -> Result<(u8, u8, u8), &'static str> {
    if hex.len() < 7 {
        return Err("Malformatted");
    }
    if let (Ok(r1), Ok(r2), Ok(r3)) =
        (
            u8::from_str_radix(&hex[1..3], 16),
            u8::from_str_radix(&hex[3..5], 16),
            u8::from_str_radix(&hex[5..7], 16),
        )
    {
        Ok((r1, r2, r3))
    } else {
        Err("Malformatted")
    }
}



pub fn create_file(filename: String) -> Option<File> {
    match File::create(&filename) {
        Err(why) => why,
        Ok(file) => Ok(file),
    };
}
