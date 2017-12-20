/* File Stub for the Future */
/* Author: Samuel Resendez */



use std::path::Path;
use backend::traits::Graphable;
use std::fs::File;
use std::io::Write;

const FPART: &'static str = "<head>
  <script src=\"https://cdnjs.cloudflare.com/ajax/libs/vega/3.0.8/vega.min.js\"></script>
  <script type=\"text/javascript\">
    var view;
    vega.loader()
      .load('";
const SPART: &'static str = "')
      .then(function(data) { render(JSON.parse(data)); });

    function render(spec) {
      view = new vega.View(vega.parse(spec))
        .renderer('canvas')  // set renderer (canvas or svg)
        .initialize('#view') // initialize view within parent DOM container
        .hover()             // enable hover encode set processing
        .run();
    }
  </script>
</head>
<body>
  <div id=\"view\"></div>
</body>";

pub fn render_graph(&graph: Graphable, file: File) {
  let html_text = format!("{}{}{}", FPART, json_path.to_str().unwrap(), SPART);
  println!("{}", html_text);
  let _ = file.write_all(html_text.as_bytes());
}
