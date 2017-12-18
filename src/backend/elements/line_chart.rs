

#[derive(Serialize)]
pub struct LineChartSignal {
    pub name: String,
    pub value: String,
    pub bind: LineChartInterpolation,
}
impl LineChartSignal {
    pub fn new() -> LineChartSignal {
        LineChartSignal {
            name: String::from("interpolate"),
            value: String::from("linear"),
            bind: LineChartInterpolation::new(),
        }
    }
}

#[derive(Serialize)]
pub struct LineChartInterpolation {
    pub input: String,
    pub options: Vec<String>,
}
impl LineChartInterpolation {
    pub fn new() -> LineChartInterpolation {
        LineChartInterpolation {
            input: String::from("select"),
            options: vec![
                String::from("basis"),
                String::from("cardinal"),
                String::from("catmull-rom"),
                String::from("linear"),
                String::from("monotone"),
                String::from("natural"),
                String::from("step"),
                String::from("step-after"),
                String::from("step-before"),
            ],
        }
    }
}

pub struct LineChartData {}

pub struct LineChartScale {}

pub struct LineChartAxis {}

pub struct LineChartMark {}
