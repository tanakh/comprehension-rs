/// Iterator comprehension
///
/// The syntax is similar to [Haskell's list comprehension](https://wiki.haskell.org/List_comprehension).
///
/// Basic syntax is as: `[<expr>; <pattern>, ...]`
///
/// # Examples
///
/// `<var> <- <expr>` pattern binds items of `expr` to `var`.
/// `expr` must have `.into_iter()` method.
///
/// ```
/// use comprehension::iter;
///
/// iter![x * x; x <- 0..10];
/// // => [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]
/// ```
///
/// `<expr>` pattern filters item.
/// `expr` must have type `bool`.
///
/// ```
/// use comprehension::iter;
///
/// fn gcd(a: i32, b: i32) -> i32 {
///     if b == 0 { a } else { gcd(b, a % b) }
/// }
///
/// iter![(i, j); i <- 1.., j <- 1..i, gcd(i, j) == 1].take(10);
/// // => [(1, 1), (2, 1), (2, 2), (2, 3), (2, 4), (3, 1), (3, 2), (3, 3), (3, 4), (3, 5)]
/// ```
///
/// `let <pat> = <expr>` pattern introduces a binding.
///
/// ```
/// use comprehension::iter;
///
/// iter![(i, j); i <- 1.., let k = i * i, j <- 1..=k].take(10);
/// // => [(1, 1), (2, 1), (2, 2), (2, 3), (2, 4), (3, 1), (3, 2), (3, 3), (3, 4), (3, 5)]
/// ```
///
/// If there is no binding to iterator, just one element will be returned (same as Haskell's behaviour).
///
/// ```
/// use comprehension::iter;
///
/// iter![1; ];      // => [1]
/// iter![1; false]; // => []
/// iter![1; true];  // => [1]
/// ```
///
#[macro_export]
macro_rules! iter {
    ($val:expr; $var:ident <- $it:expr $(, $($rest:tt)*)?) => {
        $it.into_iter().flat_map(move |$var| $crate::iter!($val; $($($rest)*),*))
    };
    ($val:expr; let $p:pat = $e:expr $(, $($rest:tt)*)?) => {
        {
            let $p = $e;
            $crate::iter!($val; $($($rest)*),*)
        }
    };
    ($val:expr; $pred:expr $(, $($rest:tt)*)?) => {
        std::iter::once(())
            .take(if $pred {1} else {0})
            .flat_map(move |_| $crate::iter!($val; $($($rest)*),*))
    };
    ($val:expr; ) => {
        std::iter::once($val)
    };
}

/// Vector comprehension
///
/// `vect![...]` is same as `iter![...].collect::<Vec<_>>()`
///
#[macro_export]
macro_rules! vect {
    ($($rest:tt)*) => {
        $crate::iter![$($rest)*].collect::<Vec<_>>()
    };
}

/// Sum of iterator comprehension
///
/// `sum![...]` is same as `iter![...].sum()` excepting output type will be inferred.
///
/// ```
/// use comprehension::sum;
///
/// let s = sum![i; i <- 1..=10]; // this compiles
/// // let s = iter![i; i <- 1..=10].sum(); // this does not compile
/// ```
///
#[macro_export]
macro_rules! sum {
    ($($rest:tt)*) => {
        $crate::sum($crate::iter![$($rest)*])
    };
}

#[doc(hidden)]
pub fn sum<T, I>(it: I) -> T
where
    T: std::iter::Sum<T>,
    I: Iterator<Item = T>,
{
    it.sum()
}

/// Product of iterator comprehension
///
/// `product![...]` is same as `iter![...].product()` excepting output type will be inferred.
///
/// ```
/// use comprehension::product;
///
/// let s = product![i; i <- 1..=10]; // this compiles
/// // let s = iter![i; i <- 1..=10].product(); // this does not compile
/// ```
///
#[macro_export]
macro_rules! product {
    ($($rest:tt)*) => {
        $crate::product($crate::iter![$($rest)*])
    };
}

#[doc(hidden)]
pub fn product<T, I>(it: I) -> T
where
    T: std::iter::Product<T>,
    I: Iterator<Item = T>,
{
    it.product()
}

#[test]
fn test_iter() {
    assert_eq!(vect![1; ], vec![1]);
    assert_eq!(vect![1; true], vec![1]);
    assert!(vect![1; false].is_empty());

    assert_eq!(
        vect![x * x; x <- 0..10],
        vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81],
    );

    assert_eq!(
        vect![x * y; x <- 1..=3, y <- 1..=3],
        vec![1, 2, 3, 2, 4, 6, 3, 6, 9],
    );

    assert_eq!(
        vect![x * y; x <- vec![1, 2, 3], y <- vec![1, 2, 3]],
        vec![1, 2, 3, 2, 4, 6, 3, 6, 9],
    );

    let mat = vec![vec![1, 2, 3]; 3];
    assert_eq!(
        vect![cell; row <- mat, cell <- row],
        vec![1, 2, 3, 1, 2, 3, 1, 2, 3],
    );

    assert_eq!(vect![x * x; x <- 0..10, x % 2 == 0], vec![0, 4, 16, 36, 64]);

    assert_eq!(
        vect![(*i, j); i <- &[1, 2], j <- 1..=3],
        vec![(1, 1), (1, 2), (1, 3), (2, 1), (2, 2), (2, 3)],
    );

    assert_eq!(
        iter![(*i, j); i <- &[1, 2], j <- 1..]
            .take(5)
            .collect::<Vec<_>>(),
        vec![(1, 1), (1, 2), (1, 3), (1, 4), (1, 5),],
    );

    assert_eq!(
        iter![vect![(*i, j); i <- &[1, 2] ]; j <- 1..]
            .take(5)
            .collect::<Vec<_>>(),
        vec![
            vec![(1, 1), (2, 1)],
            vec![(1, 2), (2, 2)],
            vec![(1, 3), (2, 3)],
            vec![(1, 4), (2, 4)],
            vec![(1, 5), (2, 5)],
        ],
    );

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    assert_eq!(
        iter![(i, j); i <- 1.., j <- 1..i, gcd(i, j) == 1]
            .take(10)
            .collect::<Vec<_>>(),
        vec![
            (2, 1),
            (3, 1),
            (3, 2),
            (4, 1),
            (4, 3),
            (5, 1),
            (5, 2),
            (5, 3),
            (5, 4),
            (6, 1),
        ],
    );

    assert_eq!(
        iter![(i, j); i <- 1.., let k = i * i, j <- 1..=k ]
            .take(10)
            .collect::<Vec<_>>(),
        vec![
            (1, 1),
            (2, 1),
            (2, 2),
            (2, 3),
            (2, 4),
            (3, 1),
            (3, 2),
            (3, 3),
            (3, 4),
            (3, 5)
        ],
    );

    assert_eq!(
        iter![c.to_ascii_uppercase(); c <- "Hello".chars()].collect::<String>(),
        "HELLO"
    );

    assert_eq!(55, sum![i; i <- 1..=10]);
    assert_eq!(3628800, product![i; i <- 1..=10]);
}
