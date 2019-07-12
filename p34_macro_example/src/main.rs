use p34_macro_example_lib::HelloMacro;
use p34_macro_example_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Testname;

fn main() {
    Pancakes::hello_macro();
    Testname::hello_macro();
}
