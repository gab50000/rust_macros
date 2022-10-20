#![feature(trace_macros)]
#![feature(log_syntax)]

macro_rules! times_5 {
    ($e:expr) => {
        5 * $e
    };
}

macro_rules! mul {
    (
        $($element: expr) , *
    ) => {
         {
            let mut prod = 1;
            $( prod *= $element;)*
            prod
    }
    };
}

macro_rules! make_struct {
    ($id:ident, $type:ty) => {
        #[derive(Debug)]
        struct $id($type);
    };
}

trace_macros!(true);
make_struct!(Blablub, i32);
make_struct!(Blablib, u32);
trace_macros!(false);

fn main() {
    println!("{}", times_5!(123));
    println!("{}", mul![1, 2, 3, 4, 5]);
    println!("{}", mul![]);
    println!("{:?}", Blablub(-123));
    println!("{:?}", Blablib(123));
}
