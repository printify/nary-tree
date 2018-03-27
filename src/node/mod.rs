use tree::Tree;
use tree::core::NodeId;

pub struct Node<T> {
    data: T,
    parent: Option<NodeId>,
    prev_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    first_child: Option<NodeId>,
    last_child: Option<NodeId>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node {
            data,
            parent: None,
            prev_sibling: None,
            next_sibling: None,
            first_child: None,
            last_child: None,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn replace_data(&mut self, mut data: T) -> T {
        ::std::mem::swap(&mut data, self.data_mut());
        data
    }

    pub fn parent(&self) -> Option<&NodeId> {
        self.parent.as_ref()
    }

    pub fn prev_sibling(&self) -> Option<&NodeId> {
        self.prev_sibling.as_ref()
    }

    pub fn next_sibling(&self) -> Option<&NodeId> {
        self.next_sibling.as_ref()
    }

    pub fn first_child(&self) -> Option<&NodeId> {
        self.first_child.as_ref()
    }

    pub fn last_child(&self) -> Option<&NodeId> {
        self.last_child.as_ref()
    }

    pub(crate) fn set_prev_sibling(&mut self, prev_sibling: Option<NodeId>) {
        self.prev_sibling = prev_sibling;
    }

    pub(crate) fn set_next_sibling(&mut self, next_sibling: Option<NodeId>) {
        self.next_sibling = next_sibling;
    }

    pub(crate) fn set_first_child(&mut self, first_child: Option<NodeId>) {
        self.first_child = first_child;
    }

    pub(crate) fn set_last_child(&mut self, last_child: Option<NodeId>) {
        self.last_child = last_child;
    }
}

pub struct NodeMut<'a, T: 'a> {
    pub(crate) node_id: NodeId,
    pub(crate) tree: &'a mut Tree<T>,
}

impl<'a, T: 'a> NodeMut<'a, T> {
    pub(crate) fn new(node_id: NodeId, tree: &'a mut Tree<T>) -> NodeMut<'a, T> {
        NodeMut {
            node_id,
            tree,
        }
    }

    pub fn parent(&mut self) -> Option<NodeMut<T>> {
        // todo: fix when non-lexical-lifetimes comes out
        let parent_id;
        {
            let node = unsafe {
                self.tree.get_unchecked(&self.node_id)
            };
            parent_id = node.parent.clone()?;
        }
        let parent = unsafe {
            self.tree.get_unchecked_mut(&parent_id)
        };
        Some(parent)
    }

    pub fn append() {
        unimplemented!()
    }

    pub fn prepend() {
        unimplemented!()
    }

    pub fn remove_first() {
        unimplemented!()
    }

    pub fn remove_last() {
        unimplemented!()
    }
}

