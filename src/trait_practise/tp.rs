// use std::fmt;
// use std::fmt::{Formatter, write};
// use std::io::Write;
//
// struct BufBuilder{
//     buf: Vec<u8>,
// }
//
// impl BufBuilder {
//     pub fn new() -> Self{
//         Self {
//             buf: Vec::with_capacity(1024),
//         }
//     }
// }
//
// impl fmt::Debug for BufBuilder{
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f,"{}",String::from_utf8_lossy(&self.buf))
//     }
// }
//
// impl Write for BufBuilder{
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         //把buf添加到bufbuilder的尾部
//         self.buf.extend_from_slice(buf);
//         Ok(buf.len())
//     }
//
//     fn flush(&mut self) -> std::io::Result<()> {
//         //由于是在内存中操作，不需要flush
//         Ok(())
//     }
// }
//
// fn main() {
//     let mut buf = BufBuilder::new();
//     buf.write_all(b"hello world~").unwrap();
//     println!("{:?}",buf);
// }




// use regex::Regex;
//
// pub trait Parse{
//     fn parse(s:&str) -> Self;
// }
//
// impl Parse for u8{
//     fn parse(s: &str) -> Self {
//         let re: Regex = Regex::new(r"^\d+").unwrap();
//         if let Some(captures) = re.captures(s){
//             captures.get(0).map_or(0,|s| s.as_str().parse().unwrap_or(0))
//         }else {
//             0
//         }
//     }
// }
//
// #[test]
// fn parse_should_work(){
//     assert_eq!(u8::parse("123abcd"), 123);
//     assert_eq!(u8::parse("1234abcd"), 0);
//     assert_eq!(u8::parse("abcd"), 0);
// }
//
// fn main() {
//     println!("result: {}", u8::parse("255 hello world"));
// }




// use std::str::FromStr;
// use regex::Regex;
//
// pub trait Parse{
//     fn parse(s:&str) -> Self;
// }
//
// impl<T> Parse for T
// where T: FromStr + Default,{
//     fn parse(s: &str) -> Self {
//         let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
//         let d = || Default::default();
//         if let Some(captures) = re.captures(s){
//             captures.get(0).map_or(d(),|s|s.as_str().parse().unwrap_or(d()))
//         }else{
//             d()
//         }
//     }
// }
//
// fn parse_should_work() {
//     assert_eq!(u32::parse("123abcd"), 123);
//     assert_eq!(u32::parse("123.45abcd"), 0);
//     assert_eq!(f64::parse("123.45abcd"), 123.45);
//     assert_eq!(f64::parse("abcd"), 0);
// }
//
// fn main() {
//     println!("result:{}", u8::parse("255 hello world"));
// }


// struct Cat;
// struct Dog;
//
// trait Animal{
//     fn name(&self) -> &'static str;
// }
//
// impl Animal for Cat {
//     fn name(&self) -> &'static str {
//         "cat"
//     }
// }
//
// impl Animal for Dog{
//     fn name(&self) -> &'static str {
//         "dog"
//     }
// }
//
// fn name(animal: impl Animal) -> &'static str{
//     animal.name()
// }
//
// fn main() {
//     let cat = Cat;
//     println!("cat : {}", name(cat));
// }



use std::{fmt, slice};

// 注意这里，我们实现了 Copy，这是因为 *mut u8/usize 都支持 Copy
#[derive(Clone, Copy)]
struct RawBuffer {
    // 裸指针用 *const / *mut 来表述，这和引用的 & 不同
    ptr: *mut u8,
    len: usize,
}

impl From<Vec<u8>> for RawBuffer {
    fn from(vec: Vec<u8>) -> Self {
        let slice = vec.into_boxed_slice();
        Self {
            len: slice.len(),
            // into_raw 之后，Box 就不管这块内存的释放了，RawBuffer 需要处理释放
            ptr: Box::into_raw(slice) as *mut u8,
        }
    }
}

// 如果 RawBuffer 实现了 Drop trait，就可以在所有者退出时释放堆内存
// 然后，Drop trait 会跟 Copy trait 冲突，要么不实现 Copy，要么不实现 Drop
// 如果不实现 Drop，那么就会导致内存泄漏，但它不会对正确性有任何破坏
// 比如不会出现 use after free 这样的问题。
// 你可以试着把下面注释去掉，看看会出什么问题
// impl Drop for RawBuffer {
//     #[inline]
//     fn drop(&mut self) {
//         let data = unsafe { Box::from_raw(slice::from_raw_parts_mut(self.ptr, self.len)) };
//         drop(data)
//     }
// }

// impl fmt::Debug for RawBuffer {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let data = self.as_ref();
//         write!(f, "{:p}: {:?}", self.ptr, data)
//     }
// }
//
// impl AsRef<[u8]> for RawBuffer {
//     fn as_ref(&self) -> &[u8] {
//         unsafe { slice::from_raw_parts(self.ptr, self.len) }
//     }
// }
//
// fn main() {
//     let data = vec![1, 2, 3, 4];
//
//     let buf: RawBuffer = data.into();
//
//     // 因为 buf 允许 Copy，所以这里 Copy 了一份
//     use_buffer(buf);
//
//     // buf 还能用
//     println!("buf: {:?}", buf);
// }
//
// fn use_buffer(buf: RawBuffer) {
//     println!("buf to die: {:?}", buf);
//
//     // 这里不用特意 drop，写出来只是为了说明 Copy 出来的 buf 被 Drop 了
//     drop(buf)
// }


// use std::borrow::Cow;
//
// use url::Url;
// fn main() {
//     let url = Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();
//     let mut pairs = url.query_pairs();
//
//     assert_eq!(pairs.count(), 3);
//
//     let (mut k, v) = pairs.next().unwrap();
//     // 因为 k, v 都是 Cow<str> 他们用起来感觉和 &str 或者 String 一样
//     // 此刻，他们都是 Borrowed
//     println!("key: {}, v: {}", k, v);
//     // 当修改发生时，k 变成 Owned
//     k.to_mut().push_str("_lala");
//
//     print_pairs((k, v));
//
//     print_pairs(pairs.next().unwrap());
//     // 在处理 extra=hello%20world 时，value 被处理成 "hello world"
//     // 所以这里 value 是 Owned
//     print_pairs(pairs.next().unwrap());
// }
//
// fn print_pairs(pair: (Cow<str>, Cow<str>)) {
//     println!("key: {}, value: {}", show_cow(pair.0), show_cow(pair.1));
// }
//
// fn show_cow(cow: Cow<str>) -> String {
//     match cow {
//         Cow::Borrowed(v) => format!("Borrowed {}", v),
//         Cow::Owned(v) => format!("Owned {}", v),
//     }
// }



use lazy_static::lazy_static;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// lazy_static 宏可以生成复杂的 static 对象
lazy_static! {
    // 一般情况下 Mutex 和 Arc 一起在多线程环境下提供对共享内存的使用
    // 如果你把 Mutex 声明成 static，其生命周期是静态的，不需要 Arc
    static ref METRICS: Mutex<HashMap<Cow<'static, str>, usize>> =
        Mutex::new(HashMap::new());
}

pub fn method() {
    // 用 Arc 来提供并发环境下的共享所有权（使用引用计数）
    let metrics: Arc<Mutex<HashMap<Cow<'static, str>, usize>>> =
        Arc::new(Mutex::new(HashMap::new()));
    for _ in 0..32 {
        let m = metrics.clone();
        thread::spawn(move || {
            let mut g = m.lock().unwrap();
            // 此时只有拿到 MutexGuard 的线程可以访问 HashMap
            let data = &mut *g;
            // Cow 实现了很多数据结构的 From trait，
            // 所以我们可以用 "hello".into() 生成 Cow
            let entry = data.entry("hello".into()).or_insert(0);
            *entry += 1;
            // MutexGuard 被 Drop，锁被释放
        });
    }

    thread::sleep(Duration::from_millis(100));

    println!("metrics: {:?}", metrics.lock().unwrap());
}