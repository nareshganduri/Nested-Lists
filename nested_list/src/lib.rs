#![allow(dead_code)]

use std::fmt;

#[derive(PartialEq)]
pub struct NestedList<T>(pub Vec<NestedListItem<T>>);

impl<T> NestedList<T> {
    pub fn new() -> Self {
        NestedList(vec![])
    }

    pub fn flatten(self) -> Self {
        use crate::NestedListItem::*;

        let mut flat = Self::new();

        for item in self.0.into_iter() {
            match item {
                Item(x) => {
                    flat.0.push(Item(x));
                }
                List(x) => {
                    let mut sub = x.flatten();

                    flat.0.append(&mut sub.0);
                }
            }
        }

        flat
    }
}

impl<T> fmt::Debug for NestedList<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

#[derive(PartialEq)]
pub enum NestedListItem<T> {
    Item(T),
    List(NestedList<T>),
}

impl<T> fmt::Debug for NestedListItem<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NestedListItem::Item(ref x) => write!(f, "{:?}", x),
            NestedListItem::List(ref x) => write!(f, "{:?}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NestedList;

    #[test]
    fn test_flatten() {
        use crate::NestedListItem::*;

        let list = NestedList(vec![
            Item(5),
            Item(6),
            List(NestedList(vec![
                Item(4),
                Item(3),
                List(NestedList(vec![
                    Item(2),
                    Item(3),
                ])),
                Item(1),
            ])),
            Item(9),
            List(NestedList(vec![
                Item(8),
                Item(3),
            ]))
        ]);

        let flat = NestedList(vec![
            Item(5), Item(6), Item(4), Item(3), Item(2), Item(3), Item(1),
            Item(9), Item(8), Item(3)
        ]);

        assert_eq!(list.flatten(), flat);
    }
}