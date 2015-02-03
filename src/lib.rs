pub mod iter;

mod tests {
    use iter::*;

    #[test]
    fn combination_count() {
        let n = 6us;
        let k = 3us;
        let v = (0..n).collect::<Vec<usize>>();
        let s = &v[];
        let c = s.iter_comb(k);
        assert![c.count() == 20];
    }

    #[test]
    fn subsequences_count() {
        use std::num::Int;

        let n = 6us;
        let v = (0..n).collect::<Vec<usize>>();
        let s = &v[];
        let c = s.iter_subseq();
        assert![c.count() == 2us.pow(n)];
    }

    #[test]
    fn permutations_count() {
        let n = 6us;
        let v = (0..n).collect::<Vec<usize>>();
        let s = &v[];
        let c = s.iter_permutations();
        let mut f = 1;
        for i in 2..(n + 1) { f *= i; } // factorial
        assert![c.count() == f]
    }

    #[test]
    fn product_count() {
        let (n, m) = (5us, 7us);
        let i = 0..n;
        let j = 0..m;
        let p = i.iter_mult(j);
        assert![p.count() == n * m];
    }
}

