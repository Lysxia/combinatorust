// TODO
// - Combinations
pub struct Combinations<T>;

// - Sub-sets/sequences (more efficient than counter to 2^n and gives
// directly the subsequence as a vector)
pub struct Subsequences<T>;

// - Tuples
// A product is a variant of iter::FlatMap with an almost constant iterator
pub struct Product<A, B, I, J> where
    I: Iterator<Item=A>,
    J: Clone + Iterator<Item=B>,
{
    iter_b_const: J,
    cur_a: Option<A>,
    iter_a: I,
    iter_b: J,
}

pub trait ProductIterator<A, B, J>: Sized + Iterator<Item=A> where
    J: Clone + Iterator<Item=B>,
{
    fn iter_mult(mut self, j: J) -> Product<A, B, Self, J>
    {
        return Product {
            iter_b_const: j.clone(),
            cur_a: self.next(),
            iter_a: self,
            iter_b: j.clone() }
    }
}

impl<A, B, I, J> ProductIterator<A, B, J> for I where
    I: Iterator<Item=A>,
    J: Clone + Iterator<Item=B>,
{}

impl<A, B, I, J> Iterator for Product<A, B, I, J> where
    A: Clone,
    B: Clone,
    I: Iterator<Item=A>,
    J: Clone + Iterator<Item=B>,
{
    type Item = (A, B);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        loop {
            match self.cur_a.as_ref() {
                None => return None,
                Some(x) => {
                    for y in self.iter_b {
                        return Some((x.clone(), y.clone()))
                    }
                }
            }
            self.cur_a = self.iter_a.next();
            self.iter_b = self.iter_b_const.clone();
        }
    }
}

// - Partitions

#[test]
fn product_size() {
    let n = 5us;
    let m = 7us;
    let i = 0..n;
    let j = 0..m;
    let p = i.iter_mult(j);
    assert![p.count() == n * m];
}
