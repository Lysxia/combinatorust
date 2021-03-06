#![feature(collections)]
pub mod iter;

#[cfg(test)]
mod tests {
    use iter::*;

    // Macro for testing iterators.
    macro_rules! iter_test {
        ($test_fn: ident, // Function name
         $([use $($i: ident)::*;],)* // Imports
         { $($s: stmt;)* }, // Definitions and initialization
         iter: $it: expr, // Iterator to test
         count: $x_count:expr) => { // Number of expected iterations
            #[test]
            fn $test_fn() {
                $(use $($i)::*;)*
                $($s;)*
                let mut count = 0;
                for x in $it {
                    println!("{:?}", x);
                    count += 1;
                }
                let x_count = $x_count;
                assert_eq![count, x_count]
            }
        }
    }

    fn choose(n: usize, k: usize) -> usize {
        if k == 0 || k == n { 1 } else { choose(n-1, k) + choose(n-1, k-1) }
    }

    iter_test!(
        combinations_count,
        { let n = 6usize; let k = 3usize; },
        iter: (0..n).collect::<Vec<usize>>().combinations(k),
        count: choose(n, k)
    );

    iter_test!(
        subsequences_count,
        [ use std::num::Int; ],
        { let n = 6usize; },
        iter: (0..n).collect::<Vec<usize>>().subsequences(),
        count: 2usize.pow(n)
    );

    iter_test!(
        permutations_count,
        { let n = 6usize; },
        iter: (0..n).collect::<Vec<usize>>().permutations_iter(),
        count: { let mut f = 1; for i in 2..(n + 1) { f *= i } f }
    );

    iter_test!(
        product_count,
        { let (n, m) = (5usize, 7usize); },
        iter: Product::new(0..n, 0..m),
        count: n * m
    );

    iter_test!(
        catalan_count,
        { let n = 6usize; },
        iter: Catalan::new(n + 1),
        count: choose(2 * n, n) / (n + 1)
    );
}

