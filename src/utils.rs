




pub fn get_rgb_representation(hex_string: String) -> Result<(u8, u8, u8), &'static str> {

    let r1 = match u8::from_str_radix(&hex_string[1..3], 16) {
        Ok(val) => val,
        Err(_) => return Err("Malformatted"),
    };
    let r2 = match u8::from_str_radix(&hex_string[3..5], 16) {
        Ok(val) => val,
        Err(_) => return Err("Malformatted"),
    };
    let  r3 = match u8::from_str_radix(&hex_string[5..7], 16) {
        Ok(val) => val,
        Err(_) => return Err("Malformatted"),
    };
   
    Ok((r1, r2, r3))
    
}
