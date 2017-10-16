
use utils::get_rgb_representation;


#[derive(Serialize, Deserialize)]
pub struct Style {
    color: (u8, u8, u8),
    width: u32,
    //More to come ... 
}


impl Style {

    pub fn new(width: u32, color: &str) -> Style {
        Style {
            color: get_rgb_representation(color).unwrap(),
            width: width,
        }
    }

}
