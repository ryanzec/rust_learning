fn main() {
    test::debug_query!("test");

    // this will show the error at the call of the macro instead of where the marco is defined
    // test::debug_query!("test", "message");
}

mod test {
    macro_rules! debug_query {
        ($msg:literal) => {
            #[cfg(debug_assertions)]
            {
                println!("{}", $msg);
            }
        };

        // this should catch improper usage of the macro and show the error where it is called instead of where
        // the macro is defined
        ($($args:tt)*) => {
            #[cfg(debug_assertions)]
            {
                compile_error!(
                    "debug_query! expects exactly 1 argument: debug_query!(\"message\")"
                );
            }
        };
    }

    pub(crate) use debug_query;
}
