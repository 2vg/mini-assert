# mini-assert

[mini-assert](https://github.com/2vg/mini-assert) is minimal library for assertion.

this library will give u more easy-to-read, colored console output.

## install

in `Cargo.toml`,

```
mini_assert = { git = "https://github.com/2vg/mini-assert" }
```

or, if u have `cargo-edit`, u can use `cargo add https://github.com/2vg/mini-assert`

## usage
currently, [mini-assert](https://github.com/2vg/mini-assert) give `assert_equal!` macro.(or, `assert_eq!`. just a alias of `assert_equal!`)

`assert_equal!(<expected value>, <actual value>, <assertion name, but this is option>)`

```rust
use mini_assert::assert_eq;

fn main() {
    assert_eq!("QUICK brown FOX", "QUlCK blue F0X");
}
```

![](https://user-images.githubusercontent.com/17700125/77382350-82e02780-6dc3-11ea-9a9f-38ad48f39d40.PNG)

<hr>

```rust
use mini_assert::assert_eq;

#[derive(Debug, PartialEq)]
struct A {
    name: String,
    age: i32,
    b: B
}

#[derive(Debug, PartialEq)]
struct B {
    sub: i32
}

fn main() {
    let a = A{
        name: String::from("user"),
        age: 20,
        b: B{
            sub: 0
        }
    };

    let b = A{
        name: String::from("et"),
        age: 21,
        b: B{
            sub: 1
        }
    };

    /// no assertion name
    assert_eq!(a, b);

    /// with assertion name
    assert_eq!(a, b, "test assertion");
}
```

![](https://user-images.githubusercontent.com/17700125/77382259-444a6d00-6dc3-11ea-9355-85b734b03252.PNG)

## todo
- [ ] refactor
- [ ] add more function
- [ ] add file name and line pos info (is this need...? i have no idea)
