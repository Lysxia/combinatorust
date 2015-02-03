use std::*;

/// Iterator for combinations of k elements in a list of n
// The i-th cell of dest can contain an element from src with index
// j between i and n-k+i. indices[i] records the difference n-k+i-j.
// The Option distinguishes the first call to .next() which initializes
// dest, and the subsequent ones, which modify dest before returning it
// as a slice.
pub struct Combinations<'a, T> where T: 'a
{
    src: &'a [T],
    dest_opt: Option<Vec<T>>,
    indices: Vec<usize>,
}

pub trait CombinationsIterator<T>
{
    fn iter_comb<'a>(&'a self, k: usize) -> Combinations<'a, T>;
}

impl<'t, T> CombinationsIterator<T> for &'t [T] where T: 't + Clone
{
    fn iter_comb<'a>(&'a self, k: usize) -> Combinations<'a, T> {
        let is = iter::repeat(self.len()-k).take(k).collect::<Vec<usize>>();
        Combinations {
            src: *self,
            dest_opt: None,
            indices: is,
        }
    }
}

impl<'a, 'b, T> Iterator for Combinations<'a, T> where T: 'a + Clone
{
    type Item = &'b [T];
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let Combinations {
            src,
            ref mut dest_opt,
            ref mut indices,
        } = *self;
        let n = src.len();
        let k = indices.len();
        let mut dest = match dest_opt.clone() {
            None => {
                let dest = &src[..k];
                *dest_opt = Some(dest.to_vec());
                return Some(dest) },
            Some(dest) => dest };
        let i_opt = indices.iter().rposition(|&j| { j != 0 });
        match i_opt {
            None => None,
            Some(i) => {
                let ii = indices[i];
                let m = n-ii;
                let r = m-k+i..m;
                for j in indices[i..].iter_mut() { *j = ii-1; }
                (&mut dest[i..]).clone_from_slice(&(self.src)[r]);
                Some(&dest[])
            }
        }
    }
}

/// Sub-sets/sequences
///
/// More efficient than counter to 2^n and gives directly the subsequence
/// as a vector.
pub struct Subsequences<T>;

/// Product of iterators
///
/// A product is a variant of iter::FlatMap with a constant iterator
/// Not very useful, as this is simply a nested loop...
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
        Product {
            iter_b_const: j.clone(),
            cur_a: self.next(),
            iter_a: self,
            iter_b: j }
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
        loop { // This loops infinitely if self.iter_b is empty
            match (self.cur_a.as_ref(), self.iter_b.next()) {
                (Some(x), Some(y)) => return Some((x.clone(), y.clone())),
                (None, _) => return None,
                _ => {}
            }
            self.cur_a = self.iter_a.next();
            self.iter_b = self.iter_b_const.clone();
        }
    }
}

// - Partitions

#[test]
fn combination_size() {
    let n = 6us;
    let k = 3us;
    let v = (0..n).collect::<Vec<usize>>();
    let s = &v[];
    let c = s.iter_comb(k);
    assert![c.count() == 20];
}

#[test]
fn product_size() {
    let n = 5us;
    let m = 7us;
    let i = 0..n;
    let j = 0..m;
    let p = i.iter_mult(j);
    assert![p.count() == n * m];
}
