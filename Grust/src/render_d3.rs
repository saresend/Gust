use std::fs::File;
use std::io::Write;

pub fn startFile(filename: String) -> Result<File, &'static str> {

    let file = File::create(filename);
    
    let mut file = match file {
        Ok(file) => {
            file
          },
        Err(_) => return Err("File could not be opened"),
    };

    let payload = "<!DOCTYPE html> \n <meta charset=\"utf-8\"> \n <div class=\"chart\"></div>";
    let fWrite = file.write_all(payload.as_bytes());
}
