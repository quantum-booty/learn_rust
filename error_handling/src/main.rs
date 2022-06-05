use std::{
    fs::{File, self},
    io::{self, ErrorKind, Read}, error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    handling_error_with_match();
    handling_error_with_closure();
    unwrap();
    let f = read_user_name_from_file();
    println!("yaya {:?}", f);
    let f = read_user_name_from_file_short();
    println!("yaya {:?}", f);
    let f = read_user_name_from_file_shorter();
    println!("yaya {:?}", f);
    let f = read_user_name_from_file_even_shorter();
    println!("yaya {:?}", f);

    let c = last_char_of_first_line("hiesatiea\nisetasitea");
    println!("{:?}", c);

    // main can return Result too
    // Box<dyn Error> mean "any kind of error"
    // the executable will exit with a value 0 if main returns Ok(())
    // or exit with nonzero value if main returns an Err value
    let _f = File::open("yayayaa.txt")?;
    Ok(())
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    // ? also works on Option type
    text.lines().next()?.chars().last()
}

fn read_user_name_from_file_short() -> Result<String, io::Error> {
    // the ? opterator convert the Err to io::Error in the function signature
    // and returns the error
    // if Ok it unwraps the value and assign it to f
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_user_name_from_file_shorter() -> Result<String, io::Error> {
    // can chain methods after ?
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_user_name_from_file_even_shorter() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn read_user_name_from_file() -> Result<String, io::Error> {
    // demonstrates propagation of error via Result
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn handling_error_with_match() {
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opining the file: {:?}", other_error)
            }
        },
    };
}

fn handling_error_with_closure() {
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn unwrap() {
    let _f = File::open("hello.txt").unwrap(); // would panic if Err
    let _f = File::open("hello.txt").expect("would panic with this message if Err");
}
