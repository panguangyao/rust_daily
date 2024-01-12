//same directory
mod same_dir;

//diffrent directory
pub mod dir1;
use crate::dir1::dir1_file1;

fn main() {
    println!("this is main!");

    //same rs
    main_method();
    let a = main_method1(8);
    println!("{}", a);

    //same directory
    same_dir::method();

    //diffrent directory
    dir1_file1::method();

}

fn main_method() {
    println!("this is main method!")
}

fn main_method1(i: i8) -> i8 {
    return i;
}