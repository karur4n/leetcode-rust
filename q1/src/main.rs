fn main() {}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        let left = nums[i];

        for j in (i + 1)..nums.len() {
            let right = nums[j];

            if left + right == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    panic!("Pair not found");
}

#[test]
fn test_two_sum() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![2, 7, 11, 15], 17), vec![0, 3]);
}
