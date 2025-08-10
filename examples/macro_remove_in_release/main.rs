fn main() {
    test::debug_query!("test");
}

mod test {
    macro_rules! debug_query {
        ($msg:literal) => {
            // this will cause this marco and all calls to it ve completely removed from release builds
            #[cfg(debug_assertions)]
            {
                println!("{}", $msg);
            }
        };

        ($($args:tt)*) => {
            // this will cause this marco and all calls to it ve completely removed from release builds
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
