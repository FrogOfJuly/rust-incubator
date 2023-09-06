#![feature(trait_alias)]

use std::borrow::Cow;

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}
trait UserStorage = Storage<String, User>;

struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}



mod static_dispatch {
    use crate::{UserStorage, User, Storage};

    //??
}

mod dynamic_dispatch {
    use std::mem::replace;

    use crate::{UserStorage, User, Storage};

}

fn main() {
    println!("Implement me!");
}
