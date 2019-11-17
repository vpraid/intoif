# intoif
This library provides two convenient traits that allow you to convert values into Option or Result
based on the provided predicate. It is somewhat similar to the boolinator crate, except you don't
need to create a boolean - the predicate will do it for you. This can be useful e.g. when writing a
long chain that ends with a fold which you want to convert into Option or Result.

```rust
use intoif::IntoOption;
use boolinator::Boolinator;

fn fizz_buzz(n: u32) -> String {
    [(3, "Fizz"), (5, "Buzz")]
        .iter()
        .filter_map(|(x, s)| (n % x == 0).as_some(*s))
        .collect::<String>()
        .none_if(String::is_empty)
        .unwrap_or_else(|| n.to_string())
}
```
