use std::mem;
pub struct List {
    head: Link,
}
#[derive(Clone,Default)]
enum Link {
    #[default]
    Empty,
    More(Box<Node>),
}
#[derive(Clone)]
struct Node {
    elem: i32,
    next: Link,
}


impl Drop for List{
  fn drop(&mut self){
    while matches!(self.head,Link::More(_)){
      self.pop_node();
    }
  }
}
impl List {

    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
         self.pop_node().map(|n|n.elem)
    }

    fn pop_node(&mut self)->Option<Node>{
    match mem::replace(&mut self.head,Link::Empty){
      Link::Empty=>None,
      Link::More(mut node)=>{
        self.head = node.next;
        node.next=Link::Empty;
        Some(*node)
      }
    }
    }
}

#[cfg(test)]
mod test{
    use crate::first::List;
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
    fn test_drop(){
     { let mut list = List::new();

      for i in 1..=100000{
        list.push(i)
      }
      }
      
    }
}
