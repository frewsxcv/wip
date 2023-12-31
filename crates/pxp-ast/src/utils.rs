use std::slice::Iter;
use std::slice::IterMut;

use crate::node::Node;
use pxp_span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct CommaSeparated<T> {
    pub inner: Vec<T>,
    pub commas: Vec<Span>, // `,`
}

impl<T> CommaSeparated<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.inner.iter_mut()
    }
}

impl<T: Node> Node for CommaSeparated<T> {
    fn children(&mut self) -> Vec<&mut dyn Node> {
        self.inner.iter_mut().map(|x| x as &mut dyn Node).collect()
    }
}

impl<T> IntoIterator for CommaSeparated<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
