#![feature(trait_alias)]

use std::borrow::Cow;

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Clone)]
pub struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

pub struct UserRepositoryError {
    cause: String,
}

mod dynamic_dispatch {
    use crate::{User, UserRepositoryError};

    #[allow(clippy::ptr_arg)]
    pub trait UserRepository {
        fn get(&self, key: &String) -> Option<User>;

        fn add(&self, key: String, user: User) -> Result<(), UserRepositoryError>;

        fn update(&self, key: &String, upd: User) -> Result<(), UserRepositoryError>;

        fn remove(&self, key: &String) -> Option<User>;
    }
}

trait Command {}

trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &Self::Context) -> Self::Result;
}

struct CreateUser {
    key: String,
}

impl Command for CreateUser {}

impl CommandHandler<CreateUser> for User {
    type Context = dyn dynamic_dispatch::UserRepository;
    type Result = Result<(), UserRepositoryError>;

    fn handle_command(&self, CreateUser { key }: &CreateUser, ctx: &Self::Context) -> Self::Result {
        ctx.add(key.clone(), self.clone())
    }
}

mod testing {
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    use crate::{
        dynamic_dispatch::UserRepository, CommandHandler, Storage, User, UserRepositoryError,
    };

    trait UserStorage = Storage<String, User>;

    impl UserStorageRepository {
        pub fn new(storage: Box<dyn UserStorage>) -> Self {
            Self {
                storage: Rc::new(RefCell::new(storage)),
            }
        }
    }

    struct UserStorageRepository {
        storage: Rc<RefCell<Box<dyn UserStorage>>>,
    }

    impl UserRepository for UserStorageRepository {
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
            if self.get(key).is_none() {
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

    #[derive(Default)]
    struct InternalStorage {
        container: HashMap<String, User>,
    }

    impl Storage<String, User> for InternalStorage {
        fn set(&mut self, key: String, val: User) {
            self.container.insert(key, val);
        }

        fn get(&self, key: &String) -> Option<&User> {
            self.container.get(key)
        }

        fn remove(&mut self, key: &String) -> Option<User> {
            self.container.remove(key)
        }
    }

    #[test]
    fn test() {
        let users: Vec<_> = ["Carl", "Klara", "Dmirtriy", "Meh"]
            .into_iter()
            .map(|name| name.to_owned())
            .map(|name| (name.clone(), name + "@gmail.com"))
            .enumerate()
            .map(|(id, (key, email))| {
                (
                    key,
                    User {
                        id: id as u64,
                        email: std::borrow::Cow::Owned(email),
                        activated: false,
                    },
                )
            })
            .collect();

        let ctx = UserStorageRepository::new(Box::new(InternalStorage::default()));
        users.iter().for_each(|(key, usr)| {
            let cmd_res = usr.handle_command(&crate::CreateUser { key: key.clone() }, &ctx);
            assert!(cmd_res.is_ok());
        });

        users.iter().for_each(|(key, usr)| {
            let cmd_res = usr.handle_command(&crate::CreateUser { key: key.clone() }, &ctx);
            assert!(cmd_res.is_err());
        });
    }
}

fn main() {
    println!("Implement me!");
}
