use crate::nibbles::Nibbles;
use std::mem;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node {
    Empty,
    Leaf(LeafNode),
    Extension(ExtensionNode),
    Branch(BranchNode),
    Hash(HashNode),
}

impl Node {
    pub fn swap(&mut self, mut n: Node) -> Node {
        mem::swap(self, &mut n);
        n
    }
    pub fn take(&mut self) -> Node {
        self.swap(Node::Empty)
    }
}

#[test]
fn test_swap() {
    let mut node = Node::Leaf(LeafNode::new(
        Nibbles::from_raw(b"123", true),
        b"123".to_vec(),
    ));
    let cp = node.clone();
    assert_eq!(node.take(), cp);
    assert_eq!(node, Node::Empty);
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct LeafNode {
    pub(crate) key: Nibbles,
    pub(crate) value: Vec<u8>,
}

impl LeafNode {
    pub fn new(key: Nibbles, value: Vec<u8>) -> Self {
        LeafNode { key, value }
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    pub fn get_key(&self) -> &Nibbles {
        &self.key
    }

    pub fn into_node(self) -> Node {
        Node::Leaf(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BranchNode {
    children: Box<[Node; 16]>,
    value: Option<Vec<u8>>,
}

impl BranchNode {
    pub fn new() -> Self {
        BranchNode {
            children: Box::new([
                Node::Empty,
                Node::Empty,
                Node::Empty,
                Node::Empty,
                Node::Empty,
                Node::Empty,
                Node::Empty,
                Node::Empty,
                Node::Empty,
                Node::Empty,
                Node::Empty,
                Node::Empty,
                Node::Empty,
                Node::Empty,
                Node::Empty,
                Node::Empty,
            ]),
            value: None,
        }
    }

    pub fn at_children(&self, i: usize) -> &Node {
        &self.children[i]
    }

    pub fn child_mut(&mut self, i: usize) -> &mut Node {
        &mut self.children[i]
    }

    pub fn insert(&mut self, i: usize, n: Node) {
        if i == 16 {
            match n {
                Node::Leaf(leaf) => {
                    self.value = Some(leaf.value);
                }
                _ => panic!("The n must be leaf node"),
            }
        } else {
            self.children[i] = n
        }
    }

    pub fn get_value(&self) -> Option<&[u8]> {
        match &self.value {
            Some(v) => Some(v),
            None => None,
        }
    }

    pub fn set_value(&mut self, value: Option<Vec<u8>>) {
        self.value = value
    }

    pub fn into_node(self) -> Node {
        Node::Branch(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExtensionNode {
    pub(crate) prefix: Nibbles,
    pub(crate) node: Box<Node>,
}

impl ExtensionNode {
    pub fn new(prefix: Nibbles, node: Node) -> Self {
        ExtensionNode {
            prefix,
            node: Box::new(node),
        }
    }

    pub fn get_prefix(&self) -> &Nibbles {
        &self.prefix
    }

    pub fn get_node(&self) -> &Node {
        &self.node
    }

    pub fn set_node(&mut self, n: Node) {
        self.node = Box::new(n)
    }

    pub fn into_node(self) -> Node {
        Node::Extension(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct HashNode {
    hash: Vec<u8>,
}

impl HashNode {
    pub fn new(hash: &[u8]) -> Self {
        HashNode {
            hash: hash.to_vec(),
        }
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    pub fn into_node(self) -> Node {
        Node::Hash(self)
    }
}
