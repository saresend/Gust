


#[derive(Serialize)]
pub struct Fill {
    pub fill: Val,
}


#[derive(Serialize)]
pub struct Val {
    pub value: String,
}


impl Val {
    pub fn new(color: &str) -> Val {
        Val {
            value: color.to_string(),
        }
    }
}

impl Fill {
    pub fn new(color: &str) -> Fill {
        Fill {
            fill: Val::new(color),
        }
    }
}