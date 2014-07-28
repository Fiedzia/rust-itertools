#![feature(macro_rules)]
#![crate_name="itertools"]
#![crate_type="dylib"]

//! Itertools — extra iterator adaptors, functions and macros
//!
//! To use the macros in this crate, use the `phase(plugin)` attribute:
//!
//! ```
//! #![feature(phase)]
//! #[phase(plugin, link)] extern crate itertools;
//! ```
//!
//! I recommend shortening the crate name with something like:
//!
//! ```
//! use it = itertools;
//! ```
//! ## License 
//! Dual-licensed to be compatible with the Rust project.
//!
//! Licensed under the Apache License, Version 2.0
//! http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
//! http://opensource.org/licenses/MIT, at your
//! option. This file may not be copied, modified, or distributed
//! except according to those terms.
//!
//!

pub use adaptors::Interleave;
pub use adaptors::Product;
pub use adaptors::PutBack;
pub use adaptors::FnMap;
pub use boxiter::BoxIter;
pub use stride::Stride;
pub use times::Times;
pub use times::times;
mod adaptors;
mod boxiter;
mod stride;
mod times;

/// A trait for (x,y,z) ++ w => (x,y,z,w)
trait AppendTuple<X, Y> {
    fn append(self, x: X) -> Y;
}

/// 
pub fn append_tuple<X, Y, T: AppendTuple<X, Y>>((t, x): (T, X)) -> Y
{
    t.append(x)
}

macro_rules! impl_append_tuple(
    () => (
        impl<T> AppendTuple<T, (T, )> for () {
            fn append(self, x: T) -> (T, ) {
                (x, )
            }
        }
    );

    ($A:ident $(,$B:ident)*) => (
        impl_append_tuple!($($B),*)
        #[allow(uppercase_variables)]
        impl<$A, $($B,)* T> AppendTuple<T, ($A, $($B,)* T)> for ($A, $($B),*) {
            fn append(self, x: T) -> ($A, $($B,)* T) {
                let ($A, $($B),*) = self;
                ($A, $($B,)* x)
            }
        }
    );
)

impl_append_tuple!(A, B, C, D, E, F, G, H, I, J, K, L)

#[macro_export]
/// Create an iterator over the “cartesian product” of iterators.
///
/// ## Example
///
/// ```rust
/// // Iterate over the coordinates of a 4 x 4 grid
/// // from (0, 0), (0, 1), .. etc until (3, 3)
/// for (i, j) in iproduct!(range(0, 4i), range(0, 4i)) {
///    // ..
/// }
/// ```
pub macro_rules! iproduct(
    ($I:expr) => (
        ($I)
    );
    ($I:expr, $J:expr $(, $K:expr)*) => (
        {
            let it = ::itertools::Product::new($I, $J);
            $(
                let it = ::itertools::Product::new(it, $K)
                    .fn_map(::itertools::append_tuple);
            )*
            it
        }
    );
)

// Note: Instead of using struct Product, we could implement iproduct!()
// using .flat_map as well; however it can't implement size_hint.
// ($I).flat_map(|x| Repeat::new(x).zip($J))


/// `icompr` as in “iterator comprehension” allows creating a
/// mapped iterator with simple syntax, similar to set builder notation,
/// and directly inspired by Python. Supports an optional filter clause.
/// 
/// Syntax:
/// 
///  `icompr!(<expression> for <pattern> in <iterator>)`
///
/// or
///
///  `icompr!(<expression> for <pattern> in <iterator> if <expression>)`
///
/// Each element from the `<iterator>` expression is pattern matched
/// with the `<pattern>`, and the bound names are used to express the
/// mapped-to value.
///
/// ## Example
///
/// ```rust
/// let mut squares = icompr!(x * x for x in range(1i, 100));
/// ```
#[macro_export]
pub macro_rules! icompr(
    ($r:expr for $x:pat in $J:expr if $pred:expr) => (
        ($J).filter_map(|$x| if $pred { Some($r) } else { None })
    );
    ($r:expr for $x:pat in $J:expr) => (
        ($J).filter_map(|$x| Some($r))
    );
)

/// Extra iterator methods for arbitrary iterators
pub trait Itertools<A> : Iterator<A> {
    /// Like regular `.map`, but using a simple function pointer instead,
    /// so that the resulting `FnMap` iterator value can be cloned.
    fn fn_map<B>(self, map: fn(A) -> B) -> FnMap<A, B, Self> {
        FnMap::new(self, map)
    }

    /// Run the iterator to the end and consume all its elements
    ///
    /// ## Example
    ///
    /// ```rust
    /// ```
    ///
    fn drain(&mut self) {
        for _ in *self { /* nothing */ }
    }

    /// Alternate elements from two iterators until both
    /// are run out
    fn interleave<J: Iterator<A>>(self, other: J) -> Interleave<Self, J> {
        Interleave::new(self, other)
    }

    /// Assign to each reference in `iter` from this iterator, stopping
    /// at the shortest of the two iterators.
    ///
    /// Return the number of elements written.
    #[inline]
    fn write_to<'a, I: Iterator<&'a mut A>>(&mut self, iter: I) -> uint
    {
        let mut count = 0u;
        let mut iter = iter;
        for elt in *self {
            match iter.next() {
                None => break,
                Some(ptr) => *ptr = elt
            }
            count += 1;
        }
        count
    }
}

impl<A, T: Iterator<A>> Itertools<A> for T { }