Step 0: Become familiar with Rust basics
========================================

__Estimated time__: 3 days

Read through [Rust Book], [Rust FAQ], and become familiar with basic [Rust] concepts, syntax, memory model, type and module systems.

Polish your familiarity by completing [Rust By Example] and [rustlings].

Read through [Cargo Book] and become familiar with [Cargo] and its workspaces.

After completing these steps, you should be able to answer (and understand why) the following questions:
- What memory model [Rust] has? Is it single-threaded or multiple-threaded? Is it synchronous or asynchronous?
    * The memory model in Rust is directly inherited from C++.
    * Multithreaded
    * Both

- What runtime [Rust] has? Does it use a GC (garbage collector)?
    * Panic handling, global allocator
    * No
- What statically typing means? What is a benefit of using it?
    * Types are known at compile time
    * Enforcement of invariants
- What are generics and parametric polymorphism? Which problems do they solve?
    * Mechanism that can abstract types over other types
    * Preventing code duplication
- What are traits? How are they used? How do they compare to interfaces? What are an auto trait and a blanket impl? What is a marker trait?
    * Traits are a mechanism to unify behavior of different types. 
    * They are used to hide irrelevant details about type but expose the relevant ones.
    * Traits are more flexible than interfaces. For example: one can implement a trait in a separate `impl` block outside of the file with the type definition. They are closer to typeclasses in this regard.
    * Traits that are automatically implemented for every class
    * Trait implementations for any type. An example is the `Sized` trait. Plays well with a negative impl mechanism.
- What are static and dynamic dispatches? Which should I use, and when?
    * Dynamic dispatch is signaled by the `dyn` keyword. It is usually a fat pointer specifying an object and vtable.
    * Static dyspatch is used in two forms. Argument marked my `impl` keyword means the same as generic parameter with respected constraint. Return type marked with `impl` means that the real type is obfuscated and only the respected interface is exposed. Used with lambda's a lot.
- What is a crate and what is a module in Rust? How do they differ? How are the used?
    * Crate is a compilation unit in rust
    * Module is a namespace
- What are move semantics? What are borrowing rules? What is the benefit of using them?
    * Move semantics, in contrast with copy semantics, is a set of rules preferring "moving" values instead of copying them.
    * Borrowing rules govern how many constant and mutable references can be active at the same time.
    * Some classes of errors are prevented by the compiler
- What is immutability? What is the benefit of using it?
    * Immutability is an inability of value to be mutated.
    * Provides stronger guarantees. Immutable value is safe to read from multiple threads. Easier to be sure about code invariants.
- What is cloning? What is copying? How do they compare?
    * Clone is some nontrivial way to construct a new object from the old one.
    * Copying is a simple, usually byte-wise, copy of the provided object
- What is RAII? How is it implemented in [Rust]? What is the benefit of using it?
    * RAII is a concept form C++. The idea is to put cleanup code in the object's destructor
    * It is implemented by the drop method. When an object is dropped the clean-up code is called.
- What is an iterator? What is a collection? How do they differ? How are they used?
    * Iterator is a mechanism that provides uniform access to a collection of elements if collection supports it.
    * Collection is a data structure that stores elements in some way and exposes methods to manipulate them.
    * Iterator does not own data, collection - does
    * Iterator is used to manipulate data with uniform API. Collection is used to effectively store and manipulate data
- What are macros? Which problems do they solve? What is the difference between declarative and procedural macro?
    * Macros are compile-time functions that manipulate source code on the AST level.
    * Some things like format strings are convenient to encode as macro and verify during compilation. Some boilerplate code can be eliminated with macro usage. The most evident example is trait deriving. Some traits are easily generated and macros provide an easy way to do so.
    * Procedural macro is a function on the AST token stream. Declarative macro is a simpler, more restrictive way to write procedural macro: pattern matching on AST and executing different code as a result.
- How code is tested in [Rust]? Where should you put tests and why?
    * There are special attributes for functions that are performing tests. There are special directories in the rust project reserved for tests. 
- Why [Rust] has `&str` and `String` types? How do they differ? When should you use them?
    * &str - reference to string slice. The String is a container owning the string data.
    * &str does not own data, String - does.
    * &str is better used as a string view. When data is not modified and has a clear expected lifetime.
- What are lifetimes? Which problems do they solve? Which benefits do they give?
    * lifetime is a mechanism to ensure that references are pointing to live data.
- Is [Rust] OOP language? Is it possible to use SOLID/GRASP? Does it have an inheritance?
    * Rust can be used as an OOP language, but it benefits greatly from its functional parts.
    * Not fully.
    * No, only traits can be inherited from each other.

After you're done notify your lead in an appropriate PR (pull request), and he will exam what you have learned.

_Additional_ articles, which may help to understand the above topic better:
- [Chris Morgan: Rust ownership, the hard way][1]
- [Adolfo Ochagavía: You are holding it wrong][12]
- [Vikram Fugro: Beyond Pointers: How Rust outshines C++ with its Borrow Checker][15]
- [Sabrina Jewson: Why the “Null” Lifetime Does Not Exist][16]
- [HashRust: A guide to closures in Rust][13]
- [Ludwig Stecher: Rusts Module System Explained][2]
- [Tristan Hume: Models of Generics and Metaprogramming: Go, Rust, Swift, D and More][3]
- [Jeff Anderson: Generics Demystified Part 1][4]
- [Jeff Anderson: Generics Demystified Part 2][5]
- [Bradford Hovinen: Demystifying trait generics in Rust][14]
- [Brandon Smith: Three Kinds of Polymorphism in Rust][6]
- [Jeremy Steward: C++ & Rust: Generics and Specialization][7]
- [cooscoos: &stress about &Strings][8]
- [Jimmy Hartzell: RAII: Compile-Time Memory Management in C++ and Rust][9]
- [Georgios Antonopoulos: Rust vs Common C++ Bugs][10]
- [Yurii Shymon: True Observer Pattern with Unsubscribe mechanism using Rust][11]
- [Clayton Ramsey: I built a garbage collector for a language that doesn't need one][17]




[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Rust]: https://www.rust-lang.org
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust FAQ]: https://prev.rust-lang.org/faq.html
[rustlings]: https://rustlings.cool

[1]: https://chrismorgan.info/blog/rust-ownership-the-hard-way
[2]: https://aloso.github.io/2021/03/28/module-system.html
[3]: https://thume.ca/2019/07/14/a-tour-of-metaprogramming-models-for-generics
[4]: https://web.archive.org/web/20220525213911/http://jeffa.io/rust_guide_generics_demystified_part_1
[5]: https://web.archive.org/web/20220328114028/https://jeffa.io/rust_guide_generics_demystified_part_2
[6]: https://www.brandons.me/blog/polymorphism-in-rust
[7]: https://www.tangramvision.com/blog/c-rust-generics-and-specialization#substitution-ordering--failures
[8]: https://cooscoos.github.io/blog/stress-about-strings
[9]: https://www.thecodedmessage.com/posts/raii
[10]: https://geo-ant.github.io/blog/2022/common-cpp-errors-vs-rust
[11]: https://web.archive.org/web/20230319015854/https://ybnesm.github.io/blah/articles/true-observer-pattern-rust
[12]: https://ochagavia.nl/blog/you-are-holding-it-wrong
[13]: https://hashrust.com/blog/a-guide-to-closures-in-rust
[14]: https://gruebelinchen.wordpress.com/2023/06/06/demystifying-trait-generics-in-rust
[15]: https://dev.to/vikram2784/beyond-pointers-how-rust-outshines-c-with-its-borrow-checker-1mad
[16]: https://sabrinajewson.org/blog/null-lifetime
[17]: https://claytonwramsey.github.io/2023/08/14/dumpster.html
