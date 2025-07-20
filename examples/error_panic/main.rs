fn panic() -> () {
    back();
}

fn back() -> () {
    trace();
}

fn trace() -> () {
    // when your application gets into a state that is can't recover from it should use the panic!
    // marco,
    //
    // it should only be used as a last resort where the application has no ability to
    // recover gracefully
    //
    // it almost never be used in libraries
    panic!("can't recover, help!!!");
}

fn main() -> () {
    panic();
}
