macro_rules! times_5 {
    ($e:expr) => {
        5 * $e
    };
}

fn main() {
    println!("{}", times_5!(123));
}
