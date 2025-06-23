#![feature(rustc_private)]

extern crate rustc_driver;

fn main() {
    struct MyCallbacks;
    impl rustc_driver::Callbacks for MyCallbacks{}

    let _result = rustc_driver::RunCompiler::new(
        &[
            "".to_string(), // arg[0] needs to be null
            "-C".to_string(), "panic=abort".to_string(), // Removes the built in panic
            "-C".to_string(), "link-arg=-nostdlib".to_string(), // Does not include stdlib
            "-C".to_string(), "link-arg=-static".to_string(), //Statically links the binary
            "-C".to_string(), "link-arg=nodefaultlibs".to_string(), // Includes no default libs
            "-C".to_string(), "opt-level=z".to_string(), // Optimizes for binary size
            "--emit=obj".to_string(), // Emits an object file
            "./shellcode.rs".to_string(), // Defines the input file  
            "--target".to_string(), "x86_64-unknown-linux-gnu".to_string(), // Sets the target as a
            // linux target (This would work just the same for windows)
            "-o".to_string(), "./shellcode.o".to_string() //Defines the output directory
        ],
        &mut MyCallbacks)
        .run();
}
