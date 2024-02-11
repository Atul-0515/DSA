/*
    Persistent List Implementation That Functional Programmer come to know and love;
*/
use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;
#[derive(Debug, Eq, PartialEq, Clone)]
struct Node<T> {
    elm: T,
    next: Link<T>,
}
impl<T> Node<T> {
    fn new(elm: T) -> Self {
        Self {elm, next: None}
    }
}

struct PersistentList<T> {
    head: Link<T>,
}
impl<T> PersistentList<T> {
    fn new() -> Self {
        Self { head: None }
    }
    fn prepend(&self, elm: T) -> Self {
        let new_node = Rc::new(Node {
            elm,
            next: self.head.clone(),
        });
        Self { head: Some(new_node) }
    }

    fn tail(&self) -> Self {
        Self { head: self.head.as_ref().and_then(|x| x.next.clone()) }
    }

    fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elm)
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            &node.elm
        })
    }
}

impl<T> Drop for PersistentList<T> {
    fn drop(&mut self) {
        let mut curr_link = self.head.take();
        while let Some(node) = curr_link {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                curr_link = node.next.take();
            } else {
                break;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let node1 = Node { elm: 1, next: None };
        let node2 = Node::new(1);
        assert_eq!(node1, node2);

        let list = PersistentList::new();
        assert_eq!(list.is_empty(), true);
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.is_empty(), false);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.is_empty(), true);
        assert_eq!(list.head(), None);

        let list = list.tail();
        assert_eq!(list.head(), None);
        assert_eq!(list.is_empty(), true);
    }

    #[test]
    fn iter() {
        let list = PersistentList::new().prepend(1).prepend(2).prepend(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    
}