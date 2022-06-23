//    Length	Signed	Unsignd
//    8-bit	    i8	    u8
//    16-bit	i16	    u16
//    32-bit	i32  	u32
//    64-bit	i64	    u64
//    128-bit	i128	u128
//    arch	    isize   usize     32 or 64 depend of the architecture


//Rust is a strong static typed langage it must know each type of variable befor the compilation

fn main() {
    let _guess: u32 = "42".parse().expect("Not a number!");
    let _x: f64 = 2.0; // f64
    let _f: bool = false; // with explicit type annotation
    let _c: char = 'z'; //char type

    //Compound type

    //Tuple are used when you want to a function return multiple values
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let tuple = (500, 6.4, 1);
    let (x, y, z) = tuple;

    fn return_multiple_values(tuple :(i32, f64, u8)) -> (i32, f64, u8) {
       tuple
    }

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    let unity = () //Unity type expression return unity type when there are empty

    //Array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; //Same as let a = [3, 3, 3, 3, 3];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
}
