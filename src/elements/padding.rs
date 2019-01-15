// We slightly dodge the issue of have an ADT for padding
// where each type follows different serialization semantics
// by simply removing the subtype (i32)

#[derive(Serialize, Deserialize)]
pub struct Padding {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}
