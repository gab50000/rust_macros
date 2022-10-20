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
    ($id:ident) => {
        #[derive(Debug)]
        struct $id(i32);
    };
}

trace_macros!(true);
make_struct!(Blablub);
make_struct!(Blablib);
trace_macros!(false);

fn main() {
    println!("{}", times_5!(123));
    println!("{}", mul![1, 2, 3, 4, 5]);
    println!("{}", mul![]);
    println!("{:?}", Blablib(123));
}
