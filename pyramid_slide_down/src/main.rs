use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
enum PyramidError {
    InvalidPyramid,
    EmptyPyramid,
}

type NodeRef = Rc<RefCell<BTreeNode>>;

#[derive(Debug, Clone)]
struct BTreeNode {
    value: u16,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
}

impl BTreeNode {
    fn new_leaf(value: u16) -> NodeRef {
        Rc::new(RefCell::new(Self {
            value,
            left: None,
            right: None,
        }))
    }

    fn new(value: u16, left: NodeRef, right: NodeRef) -> NodeRef {
        Rc::new(RefCell::new(Self {
            value,
            left: Some(left),
            right: Some(right),
        }))
    }

    fn from_pyramid(pyramid: &[Vec<u16>]) -> Result<NodeRef, PyramidError> {
        if pyramid.is_empty() {
            return Err(PyramidError::EmptyPyramid);
        }

        let mut prev_row: Vec<NodeRef> = Vec::new();

        for row in pyramid.iter().rev() {
            let mut current_row: Vec<NodeRef> = Vec::new();

            for (j, &value) in row.iter().enumerate() {
                let node = if prev_row.is_empty() {
                    BTreeNode::new_leaf(value)
                } else {
                    // Node j's children are prev_row[j] (left) and prev_row[j+1] (right)
                    let left = Rc::clone(&prev_row[j]);
                    let right = Rc::clone(&prev_row[j + 1]);
                    BTreeNode::new(value, left, right)
                };
                current_row.push(node);
            }

            prev_row = current_row;
        }

        prev_row
            .into_iter()
            .next()
            .ok_or(PyramidError::InvalidPyramid)
    }
}

fn recursive_slide(node_ref: &NodeRef, memo: &mut HashMap<*const RefCell<BTreeNode>, u16>) -> u16 {
    let ptr = Rc::as_ptr(node_ref);

    if let Some(&cached) = memo.get(&ptr) {
        return cached;
    }

    let result = {
        let node = node_ref.borrow();
        match (&node.left, &node.right) {
            (Some(left), Some(right)) => {
                let left_sum = recursive_slide(left, memo);
                let right_sum = recursive_slide(right, memo);
                node.value + left_sum.max(right_sum)
            }
            _ => node.value,
        }
    };

    memo.insert(ptr, result);
    result
}

fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
    let mut memo = HashMap::new();
    let btree = BTreeNode::from_pyramid(pyramid).unwrap();
    recursive_slide(&btree, &mut memo)
}

fn main() {
    let small = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
    println!("{}", longest_slide_down(&small));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        let small = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
        assert_eq!(
            longest_slide_down(&small),
            23,
            "It should work for small pyramids"
        );
    }

    #[test]
    fn test_small_tricky() {
        let small = vec![
            vec![10],
            vec![10, 20],
            vec![10, 10, 20],
            vec![10, 90, 10, 20],
        ];
        assert_eq!(
            longest_slide_down(&small),
            130,
            "It should work for small, tricky pyramids"
        );
    }

    #[test]
    fn test_medium() {
        let medium = vec![
            vec![75],
            vec![95, 64],
            vec![17, 47, 82],
            vec![18, 35, 87, 10],
            vec![20, 4, 82, 47, 65],
            vec![19, 1, 23, 75, 3, 34],
            vec![88, 2, 77, 73, 7, 63, 67],
            vec![99, 65, 4, 28, 6, 16, 70, 92],
            vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
            vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
            vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
            vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
            vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
            vec![63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
            vec![4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23],
        ];
        assert_eq!(
            longest_slide_down(&medium),
            1074,
            "It should work for medium pyramids"
        );
    }
}
