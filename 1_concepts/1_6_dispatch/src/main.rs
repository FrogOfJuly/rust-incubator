#![feature(trait_alias)]

use std::borrow::Cow;

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

trait UserStorage = Storage<String, User>;

mod static_dispatch {
    use crate::{UserStorage, User, Storage};

    //??
}

mod dynamic_dispatch {
    use std::mem::replace;

    use crate::{UserStorage, User, Storage};

    struct UserRepository {
        storage: Box<dyn UserStorage>,
    }

    impl UserRepository {
        pub fn new(storage: Box<dyn UserStorage>) -> Self {
            Self { storage }
        }

        pub fn replace_storage(&mut self, storage: Box<dyn UserStorage>) -> Box<dyn UserStorage> {
            replace(&mut self.storage, storage)
        }
    }

    impl Storage<String, User> for UserRepository{
        fn set(&mut self, key: String, val: User) {
            self.storage.set(key, val)
        }

        fn get(&self, key: &String) -> Option<&User> {
            self.storage.get(key)
        }

        fn remove(&mut self, key: &String) -> Option<User> {
            self.storage.remove(key)
        }
    }
}

fn main() {
    println!("Implement me!");
}
