



/*
 * Data is the atomic unit of structure for all of grust
 * Each Data element represents a single point or element on the chart
 * This could be a point in line chart, or a bar in a bar chart
 */
#[derive(Serialize)]
pub struct Data {
    pub name: String,
    pub values: Vec<DataPoint>,
}

/* 
 * DataPoint represent one entry in a data object 
 */
#[derive(Serialize)]
pub struct DataPoint {
    pub category: String,
    pub amount: i32,
}

/*
 * GraphType represents the different types of graphs which may, or may not be supported by
 * various grust frontends
 */
#[derive(Serialize)]
pub enum GraphType {
    Bar,
    Line,
    Pie,
    Scatter,
}

/*
 * Graph represents an actual visualized graph, that is to be
 * serialized and rendered by the selected grust frontend
 */
#[derive(Serialize)]
pub struct Graph {
   pub width: i32,
   pub height: i32,
   pub padding: i32,
   pub autosize: String,
}

/*
 * 
 */

#[derive(Serialize)]
pub struct Scale {
    pub name: String,
    pub scale_type: String, 
    
}
