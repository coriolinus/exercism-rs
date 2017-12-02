use std::mem;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn len(&self) -> usize {
        if let Some(ref next) = self.next {
            1 + next.len()
        } else {
            1
        }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        if let Some(ref head) = self.head {
            head.len()
        } else {
            0
        }
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            item: element,
            next: mem::replace(&mut self.head, None),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take();
        old_head.map(|boxed_node| {
            // explicit unboxing is required
            // see https://stackoverflow.com/a/38243247/504550
            let node = *boxed_node;
            self.head = node.next;
            node.item
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();

        let mut option_node = &self.head;
        while let &Some(ref node) = option_node {
            option_node = &node.next;
            reversed.push(node.item.clone());
        }

        reversed
    }
}


impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(items: &[T]) -> Self {
        let mut sll = SimpleLinkedList::new();

        for item in items.iter() {
            sll.push(item.clone())
        }

        sll
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut output = Vec::with_capacity(self.len());

        while self.len() > 0 {
            output.push(self.pop().unwrap());
        }

        output.reverse();
        output
    }
}
