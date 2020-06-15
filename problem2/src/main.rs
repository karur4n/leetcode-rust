fn main() {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn recur_add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() {
        if carry > 0 {
            return Some(Box::new(ListNode {
                next: None,
                val: carry,
            }));
        }
        return None;
    }

    let (l1val, l1next) = match l1 {
        None => (0, None),
        Some(unbox_l1) => (unbox_l1.val, unbox_l1.next),
    };

    let (l2val, l2next) = match l2 {
        None => (0, None),
        Some(unbox_l2) => (unbox_l2.val, unbox_l2.next),
    };

    let sum = l1val + l2val + carry;

    let (sum, carry) = match sum {
        0..=9 => (sum, 0),
        _ => (sum - 10, sum / 10),
    };

    return Some(Box::new(ListNode {
        next: recur_add_two_numbers(l1next, l2next, carry),
        val: sum,
    }));
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        recur_add_two_numbers(l1, l2, 0)
    }
}
