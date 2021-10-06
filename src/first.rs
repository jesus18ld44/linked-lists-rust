// pub makes it accesible from outside
// pub enum List {
//     Empty,
//     ElemEmpty(i32),
//     ElemNotEmpty(i32, Box<List>),
//     //                ^^^ recursive structures must be boxed
// }
use std::mem;

struct Node {
    elem: i32,
    next: Link,
}


enum Link {
    Empty,
    More(Box<Node>),
}



pub struct List {
    head: Link,
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty)
        }
    }
}

impl List {
    // Firstly we want to construct a list
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // we want to get the head of the list by value.
        // we can't do that through the shared reference we get through &self.head
        // the only way we can move stuff is to replace it
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        assert_eq!(list.pop(),Some(4));


    }
}
