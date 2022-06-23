use std::fs::File;
use std::io::ErrorKind;
use std::io;

fn main() {
    // Some function in rust return a Result type, Result is an enum like Option it will make
    // The error management more easy to deal with.
    // To fast determine the return type of a function you can set an other type and the compiler
    // will tell you in the error's message the type founded here Result<File, std::io::Error>

    //UNCOMMENT THE LINE BELLOW TO SEE tHE ERROR

    //let f: u32 = File::open("hello.txt");

    let f = File::open("hello.txt");

    //As the function File::open return a Result type you have to describe the behavior
    let f = match f {
        Ok(file) => file,
        //kind method return the type of the error then we can chain match
        Err(error) => match error.kind() {
            //ErrorKind is imported and allow us in this case to test if the error is "NotFound"
            ErrorKind::NotFound => match File::create("hello.txt") {
                //If not found create the file and return it
                Ok(fc) => fc,
                //If error throw and display the error
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            // in other case display the error whatever the kind of it
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    //Same management but without match (using closure)
    let f2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //NOTE unwrap(), expect() and "?" can only be called on Result or Option Type

    //Automaticaly call panic if an error is throw and the concerned error or return the result.
    let f3 = File::open("hello.txt").unwrap();
    //Automaticaly call panic if an error is throw, Personalize the msg or return the result.
    let f4 = File::open("hello.txt").expect("Personalize panic msg error");

    //The ? is an operator which make more easy to deal with errors propagation, it's similar to
    //unwrap or expect but more powefull inside of function if we want to return the error encounter
    //At a point and leave the function we can use "?" right after a function that return a Result
    // type.
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();
        //Return created file or leave the function and return the errors.
        //Return the correct result or leave the function and return the error
        let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
}
