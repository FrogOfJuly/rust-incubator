#![feature(trait_alias)]

use std::borrow::Cow;

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Clone)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

trait UserStorage = Storage<String, User>;

struct UserRepositoryError {
    cause: String,
}

mod static_dispatch {
    use crate::{Storage, User, UserStorage, UserRepositoryError};
    use std::{cell::RefCell, rc::Rc};

    struct UserRepository<T: UserStorage> {
        storage: Rc<RefCell<T>>,
    }

    impl<T: UserStorage> UserRepository<T> {
        pub fn new(storage: T) -> Self {
            Self {
                storage: Rc::new(RefCell::new(storage)),
            }
        }
    }

    impl<T: UserStorage> UserRepository<T> {
        fn get(&self, key: &String) -> Option<User> {
            self.storage.borrow().get(key).cloned()
        }

        fn add(&self, key: String, user: User) -> Result<(), UserRepositoryError> {
            if self.get(&key).is_some() {
                return Err(UserRepositoryError {
                    cause: "This user already exists".to_owned(),
                });
            }

            self.storage.borrow_mut().set(key, user);

            Ok(())
        }

        fn update(&self, key: &String, upd: User) -> Result<(), UserRepositoryError> {
            if self.get(&key).is_none() {
                return Err(UserRepositoryError {
                    cause: "This user does not exists".to_owned(),
                });
            }

            self.storage.borrow_mut().set(key.to_owned(), upd);

            Ok(())
        }

        fn remove(&self, key: &String) -> Option<User> {
            self.storage.borrow_mut().remove(key)
        }
    }
}

mod dynamic_dispatch {
    use std::{cell::RefCell, rc::Rc};

    use crate::{Storage, User, UserRepositoryError, UserStorage};

    struct UserRepository {
        storage: Rc<RefCell<Box<dyn UserStorage>>>,
    }

    impl UserRepository {
        pub fn new(storage: Box<dyn UserStorage>) -> Self {
            Self {
                storage: Rc::new(RefCell::new(storage)),
            }
        }
    }

    impl UserRepository {
        fn get(&self, key: &String) -> Option<User> {
            self.storage.borrow().get(key).cloned()
        }

        fn add(&self, key: String, user: User) -> Result<(), UserRepositoryError> {
            if self.get(&key).is_some() {
                return Err(UserRepositoryError {
                    cause: "This user already exists".to_owned(),
                });
            }

            self.storage.borrow_mut().set(key, user);

            Ok(())
        }

        fn update(&self, key: &String, upd: User) -> Result<(), UserRepositoryError> {
            if self.get(&key).is_none() {
                return Err(UserRepositoryError {
                    cause: "This user does not exists".to_owned(),
                });
            }

            self.storage.borrow_mut().set(key.to_owned(), upd);

            Ok(())
        }

        fn remove(&self, key: &String) -> Option<User> {
            self.storage.borrow_mut().remove(key)
        }
    }
}

fn main() {
    println!("Implement me!");
}
