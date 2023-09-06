#![feature(trait_alias)]

use std::{borrow::Cow, mem::replace};

trait Command {}

trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &Self::Context) -> Self::Result;
}

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

struct UserError {
    cause: String,
}

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

impl Storage<String, User> for UserRepository {
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

struct CreateUser;

impl Command for CreateUser {}

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = Result<(), UserError>;

    fn handle_command(&self, cmd: &CreateUser, ctx: &Self::Context) -> Self::Result {
        // Here we operate with UserRepository
        // via its trait object: &dyn UserRepository
        todo!()
    }
}

fn main() {
    println!("Implement me!");
}
