#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct TreeNode {
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
    pub key: u32,
    pub code: u32,
}

impl TreeNode {
    pub fn new_node() -> TreeNode {
        TreeNode {
            left: None,
            right: None,
            key: 0,
            code: 0,
        }
    }

    pub fn add_node(&mut self, node: TreeNode, position: u32) {
        let mask: u32 = 1 << position;
        if node.key & mask == mask { // Look at one bit at a time
            // The current bit is 1
            if position == 0 {
                // We are looking at the last bit of the huffman code
                match self.right {
                    None => {
                        // Insert the node at last (making a leaf)
                        self.right = Some(Box::new(node));
                    },
                    Some(_) => {
                        // There MUST never be a leaf already
                        // Throw a warning
                    }
                }
            } else {
                match self.right {
                    None => {
                        // Insert a dummy node so we can "move on"
                        // Call add_node recursive on this dummy node
                        let mut dummy = TreeNode::new_node();
                        dummy.add_node(node.clone(), position - 1);
                        self.right = Some(Box::new(dummy));
                    },
                    Some(ref mut right) => {
                        // There exists a node at this position in the tree,
                        // call the add_node recursive on this
                        // and move the position-var on to the right (subtract 1)
                        right.add_node(node.clone(), position - 1);
                    }
                }
            }
        } else {
            // The current bit is 0
            if position == 0 {
                // We are looking at the last bit of the huffman code
                match self.left {
                    None => {
                        // Insert the node at last (making a leaf)
                        self.left = Some(Box::new(node));
                    },
                    Some(_) => {
                        // There MUST never be a leaf already
                        // Throw a warning
                    }
                }
            } else {
                match self.left {
                    None => {
                        // Insert a dummy node so we can "move on"
                        // Call add_node recursive on this dummy node
                        let mut dummy = TreeNode::new_node();
                        dummy.add_node(node.clone(), position - 1);
                        self.left = Some(Box::new(dummy));
                    }
                    Some(ref mut left) => {
                        // There exists a node at this position in the tree,
                        // call the add_node recursive on this
                        // and move the position-var on to the right (subtract 1)
                        left.add_node(node.clone(), position - 1);
                    }
                }
            }
        }
    }
}
