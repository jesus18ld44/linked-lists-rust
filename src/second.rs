#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T: std::fmt::Debug + std::fmt::Display> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref().map(|node| &*node);
            &node.elem
        })
    }
}

impl<T: std::fmt::Debug + std::fmt::Display> List<T> {
    // Firstly we want to construct a list
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {next: self.head.as_deref().map(|node| &*node) }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        
        self.head.as_ref().map(|node| {
            &node.elem
        })

    }

    pub fn print_list(&self) {
        for (i, node) in self.iter().enumerate() {
            println!("elem:{}\t{}", i, node);
        }
    }

}


#[cfg(test)]
mod tests {
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

    #[test]
    fn peek() {
        let mut list = List::new();
        list.push("a");
        list.push("b");
        list.push("c");

        println!("{:?}", list);

        assert_eq!(list.peek(), Some(&"c"));
        assert_eq!(list.peek(), Some(&"c"));

    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push("a");
        list.push("b");
        list.push("c");

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some("c"));
        assert_eq!(iter.next(), Some("b"));
        assert_eq!(iter.next(), Some("a"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push("a");
        list.push("b");
        list.push("c");

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&"c"));
        assert_eq!(iter.next(), Some(&"b"));
        assert_eq!(iter.next(), Some(&"a"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn print() {
        let mut list = List::new();
        list.push("a");
        list.push("b");
        list.push("c");

        list.print_list();
    }
}
