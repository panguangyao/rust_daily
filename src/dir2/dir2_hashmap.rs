// use std::collections::HashMap;
//
// fn main() {
//     let mut map = HashMap::new();
//     explain("empty", &map);
//
//     map.insert('a', 1);
//     explain("added 1", &map);
//
//     map.insert('b', 2);
//     map.insert('c', 3);
//     explain("added 3", &map);
//
//     map.insert('d', 4);
//     explain("added 4", &map);
//
//     // get 时需要使用引用，并且也返回引用
//     assert_eq!(map.get(&'a'), Some(&1));
//     assert_eq!(map.get_key_value(&'b'), Some((&'b', &2)));
//
//     map.remove(&'a');
//     // 删除后就找不到了
//     assert_eq!(map.contains_key(&'a'), false);
//     assert_eq!(map.get(&'a'), None);
//     explain("removed", &map);
//     // shrink 后哈希表变小
//     map.shrink_to_fit();
//     explain("shrinked", &map);
// }
//
// fn explain<K, V>(name: &str, map: &HashMap<K, V>) {
//     println!("{}: len: {}, cap: {}", name, map.len(), map.capacity());
// }


// use std::collections::HashMap;
//
// fn main() {
//     let map = HashMap::new();
//     let mut map = explain("empty", map);
//
//     map.insert('a', 1);
//     let mut map = explain("added 1", map);
//     map.insert('b', 2);
//     map.insert('c', 3);
//
//     let mut map = explain("added 3", map);
//
//     map.insert('d', 4);
//
//     let mut map = explain("added 4", map);
//
//     map.remove(&'a');
//
//     explain("final", map);
// }
//
// // HashMap 结构有两个 u64 的 RandomState，然后是四个 usize，
// // 分别是 bucket_mask, ctrl, growth_left 和 items
// // 我们 transmute 打印之后，再 transmute 回去
// fn explain<K, V>(name: &str, map: HashMap<K, V>) -> HashMap<K, V> {
//     let arr: [usize; 6] = unsafe { std::mem::transmute(map) };
//     println!(
//         "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {}, items: {}",
//         name, arr[2], arr[3], arr[4], arr[5]
//     );
//     unsafe { std::mem::transmute(arr) }
// }


use std::collections::BTreeMap;

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Name {
    pub name: String,
    pub flags: u32,
}

impl Name {
    pub fn new(name: impl AsRef<str>, flags: u32) -> Self {
        Self {
            name: name.as_ref().to_string(),
            flags,
        }
    }
}

pub fn iterator_method() {
    let mut map = BTreeMap::new();
    map.insert(Name::new("/etc/password", 0x1), 12);
    map.insert(Name::new("/etc/hosts", 0x1), 4);
    map.insert(Name::new("/home/tchen", 0x0), 28);

    for item in map.iter() {
        println!("{:?}", item);
    }
}