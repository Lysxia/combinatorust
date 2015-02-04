//! Iterators over combinatorial derivatives of slices

use std::*;
use std::slice::ElementSwaps;

/// An iterator over combinations of `k` elements in a list of `n`.
// The i-th cell of dest can contain an element from src with index
// j between i and n-k+i. indices[i] records the difference n-k+i-j.
// The bool distinguishes the first call to .next() from the subsequent ones.
pub struct Combinations<'a, T> where T: 'a {
    src: &'a [T],
    dest: Vec<T>,
    indices: Vec<usize>,
    first: bool,
}

pub trait CombinationsIterator<T> {
    fn combinations<'a>(&'a self, k: usize) -> Combinations<'a, T>;
}

impl<T: Clone> CombinationsIterator<T> for [T] {
    fn combinations<'a>(&'a self, k: usize) -> Combinations<'a, T> {
        let is = iter::repeat(self.len()-k).take(k).collect::<Vec<usize>>();
        Combinations {
            src: self,
            dest: self[0..k].to_vec(),
            indices: is,
            first: true,
        }
    }
}

/// Iterate through combinations of `k` elements.
///
/// Each iteration yields a slice of length `k` with distinct elements from the
/// stored sequence in the same order. If `n` is the length of the source slice,
/// there are (`n` choose `k`) combinations.
///
/// Calls to `.next()` after `None` has been output give undefined results.
/// (In practice, currently, it keeps returning `None`)
impl<'a, 'b, T> Iterator for Combinations<'a, T> where T: 'a + Clone {
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
        if *first { *first = false; return Some(dest) }
        let i_opt = indices.iter().rposition(|&j| { j != 0 });
        match i_opt {
            None => None,
            Some(i) => {
                let h = indices[i];
                let m = n - h + 1;
                let r = (m - k + i)..m;
                for j in indices[i..].iter_mut() { *j = h - 1; }
                dest[i..].clone_from_slice(&src[r]);
                Some(dest)
            }
        }
    }
}

/// An iterator over subsets/subsequences.
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
    fn subsequences<'a>(&'a self) -> Subsequences<'a, T>;
}

impl<T> SubsequencesIterator<T> for [T] {
    fn subsequences<'a>(&'a self) -> Subsequences<'a, T> {
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
        if *first { *first = false; return Some(dest) }
        let n = src.len();
        let i = indices.last().map_or(0, |&i| { i+1 });
        // Push an element while we can
        if i < n {
            indices.push(i);
            dest.push(src[i].clone());
            return Some(dest)
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
        Some(dest)
    }
}

/// An operator over permutations.
///
/// The advantage of this implementation over the standard one from `std::slice`
/// is that the permutations are not copied.
/// Instead, an immutable slice into the vector is returned.
///
/// The order in which the permutations are returned is not guaranteed
/// to be the same as the one from the standard `Permutations` iterator.
///
/// Resets after returning `None`.
// Simply reuse the standard implementation elements
pub struct Permutations<'a, T> where T: 'a {
    dest: Vec<T>,
    swaps: ElementSwaps,
}

pub trait PermutationsIterator<T> {
    fn permutations_iter<'a>(&'a self) -> Permutations<'a, T>;
}

impl<T> PermutationsIterator<T> for [T] where T: Clone {
    fn permutations_iter<'a>(&'a self) -> Permutations<'a, T> {
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
            Some((0, 0)) => Some(dest),
            Some((a, b)) => { dest.swap(a, b); Some(dest) },
        }
    }
}

/// An iterator over pairs of elements.
///
/// A `Product` is a variant of `std::iter::FlatMap` with a constant iterator.
///
/// Not very useful, as it can simply be replaced with a nested loop...
pub struct Product<I: Iterator, J> {
    iter_b_const: J,
    cur_a: Option<I::Item>,
    iter_a: I,
    iter_b: J,
}

impl<I: Sized + Iterator, J: Clone + Iterator> Product<I, J> where
    I::Item: Clone
{
    pub fn new(mut i: I, j: J) -> Self
    {
        Product {
            iter_b_const: j.clone(),
            cur_a: i.next(),
            iter_a: i,
            iter_b: j }
    }
}

impl<I: Iterator, J: Clone + Iterator> Iterator for Product<I, J> where
    I::Item: Clone
{
    type Item = (I::Item, J::Item);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        // This loops infinitely if self.iter_b is empty
        // and self.iter_a is infinite.
        loop {
            match (self.cur_a.as_ref(), self.iter_b.next()) {
                (Some(x), Some(y)) => return Some((x.clone(), y)),
                (None, _) => return None,
                _ => {}
            }
            self.cur_a = self.iter_a.next();
            self.iter_b = self.iter_b_const.clone();
        }
    }
}

/// An iterator over binary trees.
///
/// There are `choose(2 * n, n) / (n + 1)` trees with `n + 1` leaves.
pub struct Catalan {
    indices: Vec<usize>,
    first: bool,
}

impl Catalan {
    pub fn new(n: usize) -> Catalan {
        Catalan {
            indices: (0..n-1).collect(),
            first: true,
        }
    }
}

/// Iterate through binary trees with n leaves.
///
/// The returned slices represent sequences of nodes from the traversals of
/// binary trees, where a label is the index of the leftmost leaf in the
/// subtree rooted at the corresponding node.
///
/// Alternatively, starting with a sequence of leaves:
///
///     0 1 2 3 4 5 6
///
/// the slice
/// 
///     0 1 2 3 3 4
///
/// can be associated with a prefix expression, obtained by inserting an
/// operator at the corresponding positions between the leaves:
///
///     + 0 + 1 + 2 + + 3 + 4 5 6
///
/// Brackets reveal the tree structure (with a LISP flavour):
///
///     (+ 0 (+ 1 (+ 2 (+ (+ 3 (+ 4 5)) 6))))
///
/// That is equivalent to the infix expression:
///
///     0 + (1 + (2 + ((3 + (4 + 5)) + 6)))
///
/// # Algorithm for `next()`
///
/// From this state (`j != 0`, `*`: cells left intact):
///
///     0 0 0 ... 0 0 0 ... 0 j * ...
///
/// the next one is
///
///     0 1 2 ... j-2 j-1 j-1 ... j-1 j-1 * ...
///
impl<'a> Iterator for Catalan {
    type Item = &'a [usize];
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let Catalan { ref mut indices, ref mut first } = *self;
        if *first { *first = false; return Some(indices) }
        match indices.iter().position(|&j| { j != 0 }) {
            None => None,
            Some(i) => {
                let j = indices[i];
                indices.move_from((1..j).collect(), 0, j);
                for k in indices[j..(i + 1)].iter_mut() { *k = j-1 }
                Some(indices)
            }
        }
    }
}

// - Partitions

