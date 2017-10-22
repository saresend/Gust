

#[derive(Serialize, Deserialize)]
pub struct Bar {
    pub height: i64,
    pub width: u32,
    pub label: String,
    pub style: String,
}


impl Bar {

    pub fn render(&self, x: i32) -> String {
        format!("<rect class='bar' x={} width={} y={}></rect>", x, self.width, self.height)
    }

    


}


