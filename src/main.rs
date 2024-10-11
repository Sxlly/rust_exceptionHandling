use std::fs::File;



fn main() { //declare main method

    let outcome = File::open("text.txt"); //attempt to open text File -> "text.txt"

    let new_file = match outcome { //declare new variable and assigne value -> match case -> if OK-> open file, if Err -> assign error message to variable as its value
        Ok(file) => file, //new_file = file -> if OK
        Err(error) => panic!("error {:?}", error), //new_file = "error {error message}" -> if Err
    };

    println!("{:?}",new_file); //print to temrinal contents of new_file variable -> either text in .txt file or error message


}
