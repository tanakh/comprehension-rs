# comprehension-rs

Iterator comprehension in Rust

# Usage

The syntax is derived from [Haskell's list comprehension](https://wiki.haskell.org/List_comprehension). This library use iterators instead of lists.

```rust
// this returns the iterator generates `[0, 1, 4, ..., 81]`
iter![x * x; x <- 0..10];
```

You can also use patterns in generators,

```rust
iter![x * y; (x, y) <- vec![(1, 1), (2, 3), (4, 5)]];
// => [1, 6, 20]
```

filtering values,

```rust
iter![(i, j); i <- 1.., j <- 1..i, gcd(i, j) == 1].take(10)
// => [(1, 1), (2, 1), (2, 2), (2, 3), (2, 4), (3, 1), (3, 2), (3, 3), (3, 4), (3, 5)]
```

and let bindings.

```rust
iter![(i, j); i <- 1.., let k = i * i, j <- 1..=k].take(10);
// => [(1, 1), (2, 1), (2, 2), (2, 3), (2, 4), (3, 1), (3, 2), (3, 3), (3, 4), (3, 5)]
```

Some useful variants are provided.

`vect!` returns `Vec`:

```rust
// just same as iter![].collect::<Vec<_>>()
vect![x * x; x <- 0..10];
```

`sum!` return sum of iterator:

```rust
let t = sum![x; x <- 1..=10]; // => 55

// same as this:
// let t = iter![x; x <- 1..=10].sum()

// but this does not compiles (need type annotation).
let t = iter![x; x <- 1..=10].sum::<i32>()

// `sum!` can infer the return type, so it has non-trivial functionality.
```

Also has `product!` macro:

```rust
let t = product![x; <- 1..=10]; // => 3628800
```

