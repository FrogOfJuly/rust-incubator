mod global_stack {
    use std::{cell::RefCell, rc::Rc};

    #[derive(Clone, Debug)]
    pub struct GlobalStack<T> {
        instance: Rc<RefCell<Stack<T>>>,
    }

    impl<T> Default for GlobalStack<T> {
        fn default() -> Self {
            Self {
                instance: Default::default(),
            }
        }
    }

    impl<T> GlobalStack<T> {
        pub fn new() -> GlobalStack<T> {
            GlobalStack::default()
        }

        pub fn pop(&self) -> Option<T> {
            self.instance.as_ref().borrow_mut().pop()
        }

        pub fn push(&self, v: T) {
            self.instance.as_ref().borrow_mut().push(v)
        }
    }

    #[derive(Debug)]
    struct Stack<T> {
        storage: Vec<T>,
    }

    impl<T> Default for Stack<T> {
        fn default() -> Stack<T> {
            Self { storage: vec![] }
        }
    }

    impl<T> Stack<T> {
        fn pop(&mut self) -> Option<T> {
            self.storage.pop()
        }

        fn push(&mut self, v: T) {
            self.storage.push(v)
        }
    }
}
fn main() {
    let stack = global_stack::GlobalStack::<u32>::new();

    let stack_clone = stack.clone();

    [1, 2, 3].into_iter().for_each(|x| stack.push(x));

    println!("{:?}", stack_clone);

    [1, 2, 3].into_iter().for_each(|_| {
        stack.pop();
    });

    println!("{:?}", stack_clone);
}
