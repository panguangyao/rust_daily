use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::mem::size_of;
use std::net::TcpStream;
enum E {
    A(f64),
    B(HashMap<String, String>),
    C(Result<Vec<u8>, String>),
}

// 这是一个声明宏，它会打印各种数据结构本身的大小，在 Option 中的大小，以及在 Result 中的大小
macro_rules! show_size {
    (header) => {
        println!(
            "{:<24} {:>4}   {}  {}",
            "Type", "T", "Option<T>", "Result<T, io::Error>"
        );
        // println!("{}", "-".repeat(64));
    };
    ($t:ty) => {
        println!(
            "{:<24} {:4} {:8} {:12}",
            stringify!($t),
            size_of::<$t>(),
            size_of::<Option<$t>>(),
            size_of::<Result<$t, std::io::Error>>(),
        )
    };
}

pub fn method() {
    show_size!(header);
    show_size!(u8);
    show_size!(f64);
    show_size!(&u8);
    show_size!(Box<u8>);
    show_size!(&[u8]);

    show_size!(String);
    show_size!(Vec<u8>);
    show_size!(HashMap<String, String>);
    show_size!(E);
}

// enum Option<T>{
//     Some(T),
//     None,
// }
//
// pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned{
//     Borrowed(&'a B),
//     Owned(<B as ToOwned>::Owned),
// }

#[derive(Debug)]
struct MyWriter<W> {
    writer: W,
}

impl MyWriter<BufWriter<TcpStream>> {
    pub fn new(addr: &str) -> Self {
        let stream = TcpStream::connect(addr).unwrap();
        Self{
            writer:BufWriter::new(stream),
        }
    }

    pub fn write(&mut self, buf :&str) -> std::io::Result<()>{
        self.writer.write_all(buf.as_bytes())
    }
}

pub fn method2(){
    let mut writer= MyWriter::new("127.0.0.1:8080");
    writer.write("hello world!").expect("i don't know this");
}

// 思考题代码报错的主要原因是，实现 new 方法时，对泛型的约束要求要满足 W: Write，而 new 的声明返回值是 Self，也就是说 self.wirter 必须是 W: Write 类型(泛型)，但实际返回值是一个确定的类型 BufWriter<TcpStream>，这不满足要求。
//
// 修改方法有这么几个思路
//
// 1. 修改 new 方法的返回值
//
// ```rust
// impl<W: Write> MyWriter<W> {
//     pub fn new(addr: &str) -> MyWriter<BufWriter<TcpStream>> {
//         let stream = TcpStream::connect(addr).unwrap();
//         MyWriter {
//             writer: BufWriter::new(stream),
//         }
//     }
// }
//
// fn main() {
//     let mut writer = MyWriter::<BufWriter<TcpStream>>::new("127.0.0.1:8080");
//     writer.write("hello world!");
// }
// ```
//
// 2. 对确定的类型 MyWriter<BufWriter<TcpStream>>实现 new 方法：
//
// ```rust
// impl MyWriter<BufWriter<TcpStream>> {
//     pub fn new(addr: &str) -> Self {
//         let stream = TcpStream::connect(addr).unwrap();
//         Self {
//             writer: BufWriter::new(stream),
//         }
//     }
// }
//
// fn main() {
//     let mut writer = MyWriter::new("127.0.0.1:8080");
//     writer.write("hello world!");
// }
// ```
//
// 3. 修改 new 方法的实现，使用依赖注入
//
// ```rust
// impl<W: Write> MyWriter<W> {
//     pub fn new(writer: W) -> Self {
//         Self {
//             writer,
//         }
//     }
// }
//
// fn main() {
//     let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
//     let mut writer = MyWriter::new(BufWriter::new(stream));
//     writer.write("hello world!");
// }
// ```
//
// PS：第2种解法还可以对不同具体类型实现多个new方法：
//
// ```rust
// impl MyWriter<BufWriter<TcpStream>> {
//     pub fn new(addr: &str) -> Self {
//         let stream = TcpStream::connect(addr).unwrap();
//         Self {
//             writer: BufWriter::new(stream),
//         }
//     }
// }
//
// impl MyWriter<File> {
//     pub fn new(addr: &str) -> Self {
//         let file = File::open(addr).unwrap();
//         Self { writer: file }
//     }
// }
//
// fn main() {
//     let mut writer = MyWriter::<BufWriter<TcpStream>>::new("127.0.0.1:8080");
//     writer.write("hello world!");
//
//     let mut writer = MyWriter::<File>::new("/etc/hosts");
//     writer.write("127.0.0.1 localhost");
// }
// ```