pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn len(&self) -> usize {
        let mut size = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            size += 1;
            current = &node.next;
        }
        size
    }

    pub fn push(&mut self, data: T) {
        let next = self.head.take();
        self.head = Some(Box::new(Node { data, next }));
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take();
        match head {
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.data)
    }

    // NOTE: bonus methods (push_back & pop_back)
    pub fn push_back(&mut self, data: T) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(Box::new(Node { data, next: None }));
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let mut current = &mut self.head;
        loop {
            match current {
                Some(node) if node.next.is_none() => return current.take().map(|last| last.data),
                Some(node) => current = &mut node.next,
                None => return None,
            }
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut result = SimpleLinkedList::new();
        let mut current = &self.head;
        while let Some(node) = current {
            result.push(node.data.clone());
            current = &node.next;
        }
        result
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        item.iter().fold(SimpleLinkedList::new(), |mut list, item| {
            list.push(item.clone());
            list
        })
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut result = vec![];
        let mut current = self.head;
        while let Some(node) = current {
            result.push(node.data);
            current = node.next;
        }
        result.reverse();
        result
    }
}
