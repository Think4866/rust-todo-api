# Excercise Retrospective
# Date: 2023-02-16

## What I learned building this simple API in Rust

My takeaway: Rust is a programming language that you have to absorb by doing and it makes you a more safety-conscious programmer.
The tutorial I completed did not explain much about what the code was doing so I was left to research much of the code on my own.
In the process of researching I learned that safety is a big part of Rust.
I would choose to build more APIs in Rust because I like the safety aspect of the language.

The convention of using a `prelude.rs` file to import commonly used modules was new to me.
Setting up the `Cargo.toml` file was also new to me.
Editing the `Cargo.toml` file felt a lot like editing a `Gemfile` in Ruby.
The approach I would take to building out another API in Rust would be to organize my dependencies pessimistically.
The tutorial I followed broke when using `>=` and `~` for key dependencies in the `Cargo.toml` file.
Generally speaking I would stick to using `^` for dependencies in Rust as the default behavior of looking for an exact equivalent version worked best.

Using match statements felt intuitive enough to handle the different types of errors that could occur.
It was nice knowing each time I ran `cargo build` that the code would work as expected.
Oftentimes, working in Node.js, I would have to run the code multiple times with logging to understand it, depending on the situation.
The `unwrap()` method was also a nice way to handle errors, because it would crash the program if something didn't have a value but I expected it to.

## SeaORM

I think that I would choose SeaORM for another API project because of the features it offers.
The ability to generate models from a database schema is a nice feature.
I think that my tendency would be to make more use of the `sea_orm::query` module to build out more complex queries.
For simple tasks, like this API demonstrates, the `sea_orm::entity` module is sufficient.
`ActiveModel` is a nice abstraction that makes it easy to know if a model is writable or not.

## Actix Web Framework

Pragmatism is the word that comes to mind when I think about Actix.
The specification of the API was simple enough that I didn't need to use any of the more advanced features of Actix.
I normally reach for TypeScript when considering type-safe APIs, but I think that Actix is a good alternative.
In combination with `serde` I think that Actix is a good choice for building out APIs in Rust.
Particularly, the type-safety offered by Implementors of `FromRequest` and `Responder` is a nice feature.
Cases like bad JSON payloads and missing query parameters were handled by Actix out of the box in a nice way.
Having application data stored with `App::app_data()` makes setting up the API state easy.

## Futures

I found futures to be easy to understand, and I'm glad that Rust has first-class async support.
All the libraries I used did not depend on a specific async runtime, which was nice.