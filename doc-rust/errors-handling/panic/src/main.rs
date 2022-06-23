fn main() {
    /*
    ** Rust did not clear the stack by default when he is panicking, if you need to set:
    ** [profile.release]
    ** panic = 'abort';
    ** In the Cargo.toml file
    ** Here the code is panicking beacause index is out of vec range you can display a Backtrace
    ** By taping in cli "RUST_BACKTRACE=1 cargo run"
    */

    let v = vec![1, 2, 3];

    v[99];
}
