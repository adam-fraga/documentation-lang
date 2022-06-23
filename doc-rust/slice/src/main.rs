fn main() {
    //Slice in rust are part of string and are immutables.

    let s = String::from("Hello world");

    first_word(&s);
    first_word_right_way(&s);
    slice_are_awesomes();

    let s = String::from("127.0.0.1:8080");
    let s_slice = &s[10..]; //Syntax to display part of slice
    /*
    (Warning number in this syntaxe refere to byte wich is not a good way to manage slices)
    some character as emoji or foreigners alphabet are encoded into more than one byte if you
    try to retrieve that info with that method rust compiler will throw an error
    */
    let s_borrow: &str = &s;
    let s_litteral = "1234"; //Litteral in rust are slices

    dbg!(&s);
    dbg!(s_slice);
    dbg!(s_borrow);
    dbg!(s_litteral);
}

// Function that return the index of the word to finaly get the first word (constraint)
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //as_byte convert string to a table of bytes (octets)

    for (i, &element) in bytes.iter().enumerate() {
        //Enumerate return a tuple of (index, item) -> destructurate here
        if element == b' ' {
            //Byte litreral syntax
            return i;
        }
    }
    s.len()
}

fn first_word_right_way(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; //Return a sclite which is a reference to s
                             //from begin to index which equels to "space"
        }
    }
    &s[..] //if no space found return reference to the entire string
}

//Prefere to use Slice as python Rust provide slices it work on string

fn slice_are_awesomes() {
    let s = String::from("hello world");

    let size = s.len();

    let hello = &s[0..5];
    let world = &s[6..11];
    let slice = &s[..2];
    let slice = &s[3..size];
    let slice = &s[..];

    println!("{}", hello);
    println!("{}", world);
    println!("{}", slice);
}
