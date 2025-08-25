use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("somefile.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("somefile.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating file : {:?}", e),
            },
            _ => {
                panic!("problem opening file : {:?}", error);
            }
        },
    };
}
