use std::fs::File;
use std::io::Write;

pub fn start_file(filename: String) -> Result<File, &'static str> {

    let file = File::create(filename);
    
    let mut file = match file {
        Ok(file) => {
            file
          },
        Err(_) => return Err("File could not be opened"),
    };

    let payload = "<!DOCTYPE html> \n <meta charset=\"utf-8\"> \n <div class=\"chart\"></div>";
    let f_write = file.write_all(payload.as_bytes());
    match f_write {
        Ok(_) => Ok(file),
        Err(_) => Err("Couldn't write to the file"),
    }
}
