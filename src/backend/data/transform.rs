
/// Represents a transformation on the data in the data object
/// Is in part responsible for functionality such as the stacked bar chart
/// 

#[derive(Serialize)]
pub struct Transform {
    transform_type: String, 
    group_by: Vec<String>,
    sort: Field,
    field: String, 
}

#[derive(Serialize)]
struct Field {
    field: String,
}