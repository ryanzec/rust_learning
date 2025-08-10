fn main() {
    test::debug_query!("test");
}

mod test {
    macro_rules! debug_query {
        ($msg:literal) => {
            #[cfg(debug_assertions)]
            {
                println!("{}", $msg);
            }
        };
    }

    // this is needed to keep the macro in the test module
    pub(crate) use debug_query;

    // if you have a mod.rs file that you want to export this file, you will need to do the following
    // which is slightly different from other export patterns
    // mod debug_query;

    // pub(crate) use debug_query::debug_query;
}
