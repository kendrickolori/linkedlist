type Link<T> = Option<Box<Node<T>>>;
pub struct List<T> {
    head: Link<T>,
}

#[derive(Clone)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.head.is_some() {
            self.pop_node();
        }
    }
}
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|n| n.elem)
    }

    fn pop_node(&mut self) -> Option<Node<T>> {
        match self.head.take() {
            None => None,
            Some(mut node) => {
                self.head = node.next;
                node.next = None;
                Some(*node)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

pub struct ListIntoIter<T>(List<T>);
impl<T> Iterator for ListIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIntoIter(self)
    }
}

#[cfg(test)]
mod test {
    use crate::second::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(None, list.pop());

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn test_drop() {
        {
            let mut list = List::new();

            for i in 1..=10000000 {
                list.push(i)
            }
        }
    }

    #[test]
    fn test_peek() {
        let mut list: List<i32> = List::new();
        list.push(2i32);
        assert_eq!(Some(&2i32), list.peek());
        list.push(45i32);
        list.peek_mut().map(|val| *val = 45);
        assert_eq!(Some(&45), list.peek());
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
