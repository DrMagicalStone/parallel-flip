use std::{fmt::Display, iter::Map, marker::PhantomData, ops::{Add, Index, IndexMut, Neg, Sub}, write};

/// A simple (mod 4) algebra which means 3 (mod 4) + 1 == 0
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Cyclic4 {
    value: u8,
}

impl From<usize> for Cyclic4 {
    fn from(value: usize) -> Self {
        Self::new(value as u8)
    }
}

impl Into<usize> for Cyclic4 {
    fn into(self) -> usize {
        self.value as usize
    }
}

impl Into<u8> for Cyclic4 {
    fn into(self) -> u8 {
        self.value
    }
}

impl Display for Cyclic4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (mod 4)", self.value)
    }
}

impl Cyclic4 {
    pub const fn new(value: u8) -> Self {
        Cyclic4 { value: value % 4 }
    }

    pub fn prev(self) -> Self {
        Self::new(self.value + 3)
    }

    pub fn next(self) -> Self {
        Self::new(self.value + 1)
    }

    pub fn opposite(self) -> Self {
        Self::new(self.value + 2)
    }
}

impl Add for Cyclic4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Cyclic4::new(self.value + rhs.value)
    }
}

impl Sub for Cyclic4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + - rhs
    }
}

impl Neg for Cyclic4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Cyclic4::new(4 - self.value)
    }
}

/// A linear container whose length is 4 accepts Cyclic4 as its index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Cyclic4Indexed<I, T> where I: Into<Cyclic4> + From<Cyclic4> { pub value: [T; 4], _marker: PhantomData<fn(I)> }

impl<I, T> Cyclic4Indexed<I, T> where I: Into<Cyclic4> + From<Cyclic4> {
    pub fn new(value: [T; 4]) -> Self {
        Self { value, _marker: PhantomData }
    }

    #[inline]
    pub fn iter_enumerated(&self) -> Map<std::iter::Enumerate<std::slice::Iter<'_, T>>, fn((usize, &T)) -> (I, &T)> {
        self.value.iter().enumerate().map(|(i, v)| {
            (I::from(Cyclic4::from(i)), v)
        })
    }

    #[inline]
    pub fn iter_mut_enumerated(&mut self) -> Map<std::iter::Enumerate<std::slice::IterMut<'_, T>>, fn((usize, &mut T)) -> (I, &mut T)> {
        self.value.iter_mut().enumerate().map(|(i, v)| {
            (I::from(Cyclic4::from(i)), v)
        })
    }
}

impl<I, T> IntoIterator for Cyclic4Indexed<I, T> where I: Into<Cyclic4> + From<Cyclic4> {
    type Item = T;

    type IntoIter = std::array::IntoIter<T, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl<'a, I, T> IntoIterator for &'a Cyclic4Indexed<I, T> where I: Into<Cyclic4> + From<Cyclic4> {
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.value).into_iter()
    }
}

impl<'a, I, T> IntoIterator for &'a mut Cyclic4Indexed<I, T> where I: Into<Cyclic4> + From<Cyclic4> {
    type Item = &'a mut T;

    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.value).into_iter()
    }
}

impl<I, T> Index<I> for Cyclic4Indexed<I, T> where I: Into<Cyclic4> + From<Cyclic4> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        &self.value[index.into().value as usize]
    }
}

impl<I, T> IndexMut<I> for Cyclic4Indexed<I, T> where I: Into<Cyclic4> + From<Cyclic4> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.value[index.into().value as usize]
    }
}