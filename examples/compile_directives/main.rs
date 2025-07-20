// this allows dead code for the entire file
#![allow(dead_code)]

#[allow(unused_variables)] // this would allow unused variables for the entire function
fn compiler_directive() {
    #[allow(unused_variables)] // for a specific variable
    let count = 10;
}

fn main() {}
