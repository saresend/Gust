use std::fmt;
use std::fmt::Display;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Colors {
    Blue,
    Green,
    Grey,
    Purple,
    Red,
    Orange,
    Yellow,
    Brown,
}

impl Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Colors::Blue => write!(f, "blue"),
            Colors::Green => write!(f, "green"),
            Colors::Grey => write!(f, "grey"),
            Colors::Purple => write!(f, "purple"),
            Colors::Red => write!(f, "red"),
            Colors::Orange => write!(f, "orange"),
            Colors::Yellow => write!(f, "yellow"),
            Colors::Brown => write!(f, "brown"),
        }
    }
}
