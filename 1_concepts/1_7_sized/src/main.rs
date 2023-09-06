#![feature(trait_alias)]

use std::borrow::Cow;

trait Command {}

trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &mut Self::Context) -> Self::Result;
}
#[derive(Clone)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}
trait UserStorage = Storage<String, User>;

#[derive(Debug)]
struct UserError {
    cause: String,
}

trait UserRepository: Storage<String, User> {
    fn replace_storage(&mut self, storage: Box<dyn UserStorage>) -> Box<dyn UserStorage>;
}

struct CreateUser {
    key: String,
}

impl Command for CreateUser {}

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = Result<(), UserError>;

    fn handle_command(
        &self,
        CreateUser { key }: &CreateUser,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        if ctx.get(key).is_some() {
            return Err(UserError {
                cause: "User exists".to_owned(),
            });
        }
        ctx.set(key.clone(), self.clone());
        Ok(())
    }
}

mod testing {
    use std::borrow::Cow;

    use crate::{CommandHandler, CreateUser, Storage, User, UserRepository};

    const JOHN_GOLT: &User = &User {
        id: 52,
        email: Cow::Borrowed(""),
        activated: true,
    };
    struct BadStorage;

    impl Storage<String, User> for BadStorage {
        fn set(&mut self, _key: String, _val: User) {}

        fn get(&self, _key: &String) -> Option<&User> {
            if rand::random() {
                Some(JOHN_GOLT)
            } else {
                None
            }
        }

        fn remove(&mut self, _key: &String) -> Option<User> {
            None
        }
    }

    impl UserRepository for BadStorage {
        fn replace_storage(
            &mut self,
            storage: Box<dyn crate::UserStorage>,
        ) -> Box<dyn crate::UserStorage> {
            storage
        }
    }

    #[test]
    pub fn test_with_fails() {
        let emails = [
            "Carl@gmail.com",
            "Meh@gmail.com",
            "Moh@gmail.com",
            "Simple@gmail.com",
        ];
        let act = [true, false];

        let keys = ["Carl", "Meh", "Moh", "Simple"];

        emails
            .iter()
            .enumerate()
            .zip(act.into_iter().cycle())
            .map(|((id, email), activated)| User {
                id: id as u64,
                email: Cow::Borrowed(email),
                activated,
            })
            .zip(keys.iter())
            .fold(BadStorage, |mut storage, (usr, &key)| {
                if let Err(err) = usr.handle_command(
                    &CreateUser {
                        key: key.to_owned(),
                    },
                    &mut storage,
                ) {
                    println!("Error: {:?}", err);
                }

                storage
            });
    }
}

fn main() {
    println!("Implement me!");
}
