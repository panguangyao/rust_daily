//same directory
mod same_dir;

//diffrent directory
pub mod dir1;
use crate::dir1::dir1_file1;

pub mod dir2;
use crate::dir2::dir2_hashmap;

pub mod mem_man;
use crate::mem_man::mm_macro;

pub mod trait_practise;
use crate::trait_practise::tp;

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

    //hashmap
    dir2_hashmap::iterator_method();

    //memory manage
    println!("----------memory manage start--------");
    mm_macro::method();

    mm_macro::method2();
    println!("----------memory manage end----------");

    println!("----------trait practise start--------");
    tp::method();
    println!("----------trait practise end--------");

}

fn main_method() {
    println!("this is main method!")
}

fn main_method1(i: i8) -> i8 {
    return i;
}
