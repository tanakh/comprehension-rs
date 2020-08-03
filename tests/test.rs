use comprehension::*;

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

    let t = vect![(x, y); x <- 1..=3, y <- 1..=3];
    assert_eq!(vect![x * y; (x, y) <- t], vec![1, 2, 3, 2, 4, 6, 3, 6, 9]);
}
