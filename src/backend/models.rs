



/*
 * Data is the atomic unit of structure for all of grust
 * Each Data element represents a single point or element on the chart
 * This could be a point in line chart, or a bar in a bar chart
 */
#[derive(Deserialize)]
pub struct DataPoint {
    pub series_id: u32,
    pub x: Option<i32>,
    pub y: i32,
}

/*
 * GraphType represents the different types of graphs which may, or may not be supported by
 * various grust frontends
 */
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
pub struct Graph {
    pub title: String,
    pub graph_id: u32,
    pub graph_type: GraphType,
    
}