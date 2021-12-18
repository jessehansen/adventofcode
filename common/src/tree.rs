pub struct Tree<T> {
    root: Node<T>,
}

pub struct Node<T> {
    pub data: T,
    children: Vec<Node<T>>,
}

impl<T> Tree<T> {
    pub fn traverse<F: FnMut(&Node<T>) -> bool>(&self, visit: &mut F) {
        if !visit(&self.root) {
            self.root.traverse(visit);
        }
    }
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node {
            data,
            children: vec![],
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    fn traverse<F: FnMut(&Node<T>) -> bool>(&self, visit: &mut F) -> bool {
        for child in &self.children {
            if visit(child) {
                return true;
            }
            if child.traverse(visit) {
                return true;
            }
        }
        false
    }
}
