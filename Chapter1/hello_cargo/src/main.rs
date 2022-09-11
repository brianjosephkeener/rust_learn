// cargo is rust's build system and package manager
// used cargo new hello_cargo to create this
// use cargo new --vcs flag to not make a new git repository on creation

// a file called Cargo.toml was also created. 
// toml file extension stands for Tom's Obvious, Minimal Language format

/*

Let’s recap what we’ve learned so far about Cargo:

---> We can create a project using 'cargo new'.
---> We can build a project using 'cargo build'.
---> We can build and run a project in one step using 'cargo run'.
---> We can build a project without producing a binary to check for errors
    using 'cargo check'.
---> We can build a release version with 'cargo build --release'

Instead of saving the result of the build in the same directory as our code,
Cargo stores it in the target/debug directory.

*/


fn main() {
    println!("Hello, world!");
}
