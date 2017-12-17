


#[derive(Serialize)]
pub struct BarChartData {
    pub name: String,
    pub values: Vec<BarChartValue>,
}


#[derive(Serialize)]
pub struct BarChartValue {
    category: String,
    amount: i32,
}


pub struct BarChartScale {
    pub name: String, 
    pub scale_type: String, 
    pub domain: BarChart4Tuple,
    pub range: String, 
    pub padding: f64,
}

struct BarChart4Tuple {
    data: String, 
    field: String,
}

impl BarChartScale {
    pub fn create_xscale() -> BarChartScale {
        BarChartScale {
            name: String::from("xscale"),
            scale_type: String::from("band"),
            domain: BarChart4Tuple {
                data: String::from("table"),
                field: String::from("category"),
            },
            range: String::from("width"),
            padding: 0.05,
        }
    }

    pub fn create_yscale() -> BarChartScale {
        BarChartScale {
            name: String::from("yscale"),
            scale_type: String::from("linear"),
            domain: BarChart4Tuple {
                data: String::from("table"),
                field: String::from("amount"),
            },
            range: String::from("height"),
            padding: 0.05,
        }
    }
}

pub struct BarChartAxis {

    pub fn create_xaxis() -> BarChartAxis {}

    pub fn create_yaxis() -> BarChartAxis {}


}

pub struct BarChartMark {}
