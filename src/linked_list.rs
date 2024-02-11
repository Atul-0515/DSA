// Singly Linked List Implementation


type Link<T> = Option<Box<Node<T>>>;
#[derive(PartialEq, Eq, Clone, Debug)]
struct Node<T> {
    elm: T,
    next: Link<T>,
}
impl<T> Node<T> {
    fn new(elm: T) -> Self {
        Self {elm, next: None}
    }
}

struct LinkedList<T> {
    head: Link<T>,
}
impl<T> LinkedList<T> {
    fn new() -> Self {
        Self {head: None}
    }

    fn push_front(&mut self, elm: T) {
        let new_node = Box::new(Node {
            elm,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elm
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elm)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elm)
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}


/* 
    Looping is done in Rust using Iterator Trait
    Signature of Iterator Trait
        pub trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
        }

    Implementation of Iterator in Rust
        - IntoIter
            - Easy Because we just need wrapper around linked_list and call pop_front over and over.
        - Iter
        - IterMut
*/

struct IntoIter<T>(LinkedList<T>);
impl<T> LinkedList<T> {
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<T> LinkedList<T> {
    fn iter(&self) ->  Iter<T>{
        Iter {next: self.head.as_deref()}
    }
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

struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<T> LinkedList<T> {
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {next: self.head.as_deref_mut()}
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elm
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut curr_link = self.head.take();
        while let Some(mut node) = curr_link {
            curr_link = node.next.take();
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let node = Node {elm: 5, next: None};
        assert_eq!(Node::<i32>::new(5), node);

        let mut list = LinkedList::new();
        for i in 1..=3 {
            list.push_front(i);
        }

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek() {
        let mut list = LinkedList::new();
        list.push_front(1); list.push_front(2); list.push_front(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.pop_front(), Some(3));

        if let Some(val) = list.peek_mut() {
            *val *= 100;
        }
        assert_eq!(list.peek(), Some(&200));
    }

    #[test]
    fn is_empty() {
        let mut list = LinkedList::new();
        assert_eq!(list.is_empty(), true);

        list.push_front(1); list.push_front(2);
        assert_eq!(list.is_empty(), false);
        
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.is_empty(), false);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.is_empty(), true);

        assert_eq!(list.pop_front(), None);
        assert_eq!(list.is_empty(), true);
    }

    #[test]
    fn into_iter() {
        let mut list = LinkedList::new();
        for i in 1..=3 {
            list.push_front(i);
        }

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = LinkedList::new();
        for i in 1..=3 {
            list.push_front(i);
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        assert_eq!(list.pop_front(), Some(3));
    }
    
    #[test]
    fn iter_mut() {
        let mut list = LinkedList::new();
        for i in 1..=3 {
            list.push_front(i);
        }

        let mut iter = list.iter_mut();
        
        while let Some(val) = iter.next() {
            *val *= 100;
        }

        list.push_front(4);
        
        let mut iter2 = list.iter_mut();
        assert_eq!(iter2.next(), Some(&mut 4));
        assert_eq!(iter2.next(), Some(&mut 300));
        assert_eq!(iter2.next(), Some(&mut 200));
        assert_eq!(iter2.next(), Some(&mut 100));
        assert_eq!(iter2.next(), None);
        
    }
}