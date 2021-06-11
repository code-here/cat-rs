use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    match env::args().nth(1) {
        Some(_) => {
            let filenames = env::args().skip(1).collect::<Vec<String>>();
            for filename in filenames {
                let content = get_file_content(&filename)?;
                print!("{}", content);
            }
        }
        None => {
            let reader = BufReader::new(io::stdin());
            for line in reader.lines() {
                let line = line?;
                println!("{}", line);
            }
        }
    }
    Ok(())
}

fn get_file_content( filename: &str ) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(&Path::new(&filename))?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
