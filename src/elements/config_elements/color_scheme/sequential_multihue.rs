use super::colors::Colors;
use serde::{Serialize, Serializer};

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SpecialMultihues {
    Viridis,
    Inferno,
    Magma,
    Plasma,
    Bluegreen,
}

pub enum SequentialMultihue {
    Special(SpecialMultihues),
    Normal((Vec<Colors>, i32)),
}

impl Serialize for SequentialMultihue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SequentialMultihue::Special(multihue) => multihue.serialize(serializer),
            SequentialMultihue::Normal((colors, number)) => {
                let joined_base: Vec<String> =
                    colors.into_iter().map(|col| col.to_string()).collect();
                let joined_base = joined_base.join("");
                serializer.serialize_str(&(joined_base + "-" + &number.to_string()))
            }
        }
    }
}
