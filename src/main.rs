use std::fs::File; //importing file structure
use std::io::ErrorKind; //importing ErrorKind Structure



fn main() { //declare main method

    let new_file: Result<File, Error> = File::open(path: "randomFile.txt");

    let new_file: File = match new_file {
        Ok(file: File) => file,
        Err(error: Error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path: "randomFile.txt") {
                Ok(file_created: File) = file_created,
                Err(error: Error) => panic!("There was an error making file: {:?}", error),    
            },
            other_error: ErrorKind => {
                panic!("There was an error opening file: {:?}", other_error)
            }
        }
    }




}
