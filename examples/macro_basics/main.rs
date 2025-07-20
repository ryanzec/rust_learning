// this just replaces the expressions passed in as variables in the function, this is not a useful macro (a function
// would be better) but it should illustrate how macros work as a very basic level
macro_rules! greatest_common_denominator {
    ($a: expr, $b: expr) => {{
        let mut m = $b;
        let mut n = $a;

        while m != 0 {
            if m < n {
                let t = m;
                m = n;
                n = t;
            }

            m = m % n;
        }

        n
    }};
}

fn main() -> () {
    println!("{}", greatest_common_denominator!(12, 15));
}
