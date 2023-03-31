use crate::rps::RPSMove;

pub enum TreeNode {
    Value((RPSMove, RPSMove)),
    Children(
        (RPSMove, RPSMove),
        (Box<Option<TreeNode>>, Box<Option<TreeNode>>),
    ),
}

pub fn build_tree(top_nodes: Vec<(RPSMove, RPSMove)>, depth: usize) {
    for node in top_nodes {
        let new_node = TreeNode::Children(node, (Box::new(None), Box::new(None)));
    }
}

pub fn expand_node(node: &mut TreeNode, remaining_choices: &mut Vec<RPSMove>) {
    match node {
        TreeNode::Value(_) => todo!(),
        TreeNode::Children((move_1, move_2), (left_node, right_node)) => {
            
            let ideal_move;
            
            if move_1.get_char() == 'S'

            left_node = Box::new(TreeNode::Children(move_1, ()))
        }
    }
}
