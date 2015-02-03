use std::*;
use std::slice::ElementSwaps;

/// An iterator that iterates through combinations of k elements in a list of n
// The i-th cell of dest can contain an element from src with index
// j between i and n-k+i. indices[i] records the difference n-k+i-j.
// The Option distinguishes the first call to .next() which initializes
// dest, and the subsequent ones, which modify dest before returning it
// as a slice.
pub struct Combinations<'a, T> where T: 'a
{
    src: &'a [T],
    dest: Vec<T>,
    indices: Vec<usize>,
    first: bool,
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
            dest: self[0..k].to_vec(),
            indices: is,
            first: true,
        }
    }
}

/// Iterate through combinations of k elements.
///
/// Each iteration yields a slice of length `k` with distinct elements from the
/// stored sequence in the same order. If `n` is the length of the source slice,
/// there are (`n` choose `k`) combinations.
///
/// Calls to `.next()` after `None` has been output give undefined results.
/// (In practice, currently, it keeps returning `None`)
impl<'a, 'b, T> Iterator for Combinations<'a, T> where T: 'a + Clone
{
    type Item = &'b [T];
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let Combinations {
            src,
            ref mut dest,
            ref mut indices,
            ref mut first,
        } = *self;
        let n = src.len();
        let k = dest.len();
        if *first { *first = false; return Some(&dest[]) }
        let i_opt = indices.iter().rposition(|&j| { j != 0 });
        match i_opt {
            None => None,
            Some(i) => {
                let h = indices[i];
                let m = n - h;
                let r = (m - k + i)..m;
                for j in indices[i..].iter_mut() { *j = h - 1; }
                (&mut dest[i..]).clone_from_slice(&src[r]);
                Some(&dest[])
            }
        }
    }
}

/// Sub-sets/sequences
///
/// Gives directly the subsequences as slices of an internal vector.
// Invariant: dest and indices have the same length
pub struct Subsequences<'a, T> where T: 'a {
    src: &'a [T],
    dest: Vec<T>,
    indices: Vec<usize>,
    first: bool,
}

pub trait SubsequencesIterator<T> {
    fn iter_subseq<'a>(&'a self) -> Subsequences<'a, T>;
}

impl<'t, T> SubsequencesIterator<T> for &'t [T] where T: 't {
    fn iter_subseq<'a>(&'a self) -> Subsequences<'a, T> {
        Subsequences {
            src: self,
            dest: Vec::new(),
            indices: Vec::new(),
            first: true,
        }
    }
}

/// Iterate through the subsequences of a stored sequence.
///
/// If `n` is the length of the source slice, there are `2^n` subsequences.
///
/// Resets after returning `None`.
impl<'a, 'b, T> Iterator for Subsequences<'a, T> where T: 'a + Clone {
    type Item = &'b [T];
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let Subsequences {
            src,
            ref mut dest,
            ref mut indices,
            ref mut first,
        } = *self;
        // The first call returns an empty slice
        if *first { *first = false; return Some(&dest[]) }
        let n = src.len();
        let i = indices.last().map_or(0, |&i| { i+1 });
        // Push an element while we can
        if i < n {
            indices.push(i);
            dest.push(src[i].clone());
            return Some(&dest[])
        }
        // The end of the input is reached,
        // pop and increment the previous index
        indices.pop();
        dest.pop();
        match (indices.last_mut(), dest.last_mut()) {
            (None, None) => { *first = true; return None }, // Wrap around
            (Some(i), Some(x)) => {
                *i += 1;
                *x = src[*i].clone();
            },
            _ => assert![false, "Should not happen!"]
        }
        Some(&dest[])
    }
}

/// Permutations
///
/// The advantage of this implementation over the standard one from `std`
/// is that the permutations are not copied.
/// Instead, an immutable slice into the vector is returned.
// Simply reuse the standard implementation elements
pub struct Permutations<'a, T> where T: 'a {
    dest: Vec<T>,
    swaps: ElementSwaps,
}

pub trait PermutationsIterator<T> {
    fn iter_permutations<'a>(&'a self) -> Permutations<'a, T>;
}

impl<'t, T> PermutationsIterator<T> for &'t [T] where T: 't + Clone {
    fn iter_permutations<'a>(&'a self) -> Permutations<'a, T> {
        Permutations {
            dest: self.to_vec(),
            swaps: ElementSwaps::new(self.len()),
        }
    }
}

impl<'a, 'b, T> Iterator for Permutations<'a, T> where T: 'a + Clone {
    type Item = &'b [T];
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let Permutations {
            ref mut dest,
            ref mut swaps,
        } = *self;
        match swaps.next() {
            None => None,
            Some((0, 0)) => Some(&dest[]),
            Some((a, b)) => { dest.swap(a, b); Some(&dest[]) },
        }
    }
}

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
    let (n, m) = 5us, 7us;
    let i = 0..n;
    let j = 0..m;
    let p = i.iter_mult(j);
    assert![p.count() == n * m];
}
