
/// Represents a transformation on the data in the data object
/// Is in part responsible for functionality such as the stacked bar chart
/// 
/// 
/// 

use backend::constants::*;

use serde::ser::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

pub struct Transform {
    pub transform_type: TransformType, 
    pub group_by: Vec<String>,
    pub sort: Field,
    pub field: String, 
}

#[derive(Serialize)]
pub struct Field {
    field: String,
}


#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TransformType {
    Linkpath,
    Pie,
    Stack,
    Force,
    Voronoi,
    Wordcloud,
}


impl Transform {


    pub fn new(transform_type: TransformType) -> Transform {
        Transform {
            transform_type: transform_type,
            group_by: vec![XCOORD.to_string()],
            sort: Field {field: ZCOORD.to_string()},
            field: YCOORD.to_string(),
        }
    }
}

impl Serialize for Transform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("transform", 4)?;
        let _ = s.serialize_field("type", &self.transform_type);
        let _ = s.serialize_field("groupby", &self.group_by);
        let _ = s.serialize_field("sort", &self.sort);
        let _ = s.serialize_field("field", &self.field);
        s.end()
    }

}