/* Author: Samuel Resendez
 * Contains most of the needed elements for
 * area_charts
 */

use backend::elements::general::*;


use serde::ser::Serialize;


pub struct AreaChartSignal {
    name: String,
    value: String,
    bind: SignalBinding,
}
#[derive(Serialize)]
pub struct SignalBinding {
    input: String,
    options: Vec<String>,
}
pub struct AreaChartData {
    name: String,
    values: Vec<AreaChartDataEntry>,
}
#[derive(Serialize)]
pub struct AreaChartDataEntry {
    u: i32,
    v: i32,
}


#[derive(Serialize)]
pub struct AreaChartAxis {
    orient: Orientation,
    scale: String,
}
pub struct AreaChartMark {
    mark_type: String,
    from: String,
    encode: AreaChartEncoding,

}

pub struct AreaChartEncoding {
     enter: AreaChartEnter,
     update: AreaChartUpdate,
     hover: AreaChartHover, 
}

pub struct AreaChartEnter {
    x: JSONDict, 
    y: JSONDict, 
    y2: JSONDict, 
    fill: JSONDict, 
} 
pub struct AreaChartUpdate {
   interpolate: KeyVal,
   fillOpacity: QualKeyVal, 
}
    

pub struct AreaChartScale {
    name: String,
    scale_type: String,
    range: String,
    zero: bool,
    domain: JSONDict,
}


