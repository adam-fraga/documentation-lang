fn main() {
    let sentence = String::from("Here is the sentences");

    //chars method return an iterator of character
    let mut iter_char = sentence.chars();

    //bytes method return an iterator of character
    let mut iter_byte = sentence.bytes();

    loop {
        //Next give us the next element in the iterator
        let item = iter_char.next();
        match item {
            Some(c) => {
                println!("Char: {}", c)
            }
            None => break,
        }
    }
    /*
    Enumerate method provide a way to obteain a key value pair in a
    tuple it generates a new iterator. (Here we use destructuration)
    */
    for (i, c) in sentence.chars().enumerate() {
        println!("Index: {} Value: {}", i, c);
        /*
        &str[..i] ->Fetch  All character between index 0 to i
        &str[i + 1..] -> Fetch All character after index i + 1 to skip the space
        i + 1 = +1 byte which is normaly unsafe but here we arre sure " " is ecoding in one byte
        So this code is safe.
        */
        if c == ' ' {
            Some((&sentence[..i], &sentence[i + 1..]));
        }
    }
}
