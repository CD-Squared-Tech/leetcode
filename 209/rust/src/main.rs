fn main() {}

pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let (mut left, mut right, mut sum, n) = (0, 0, 0, nums.len());
    let mut ans = n + 1;
    while right < n {
        while right < n && sum < target {
            sum += nums[right];
            right += 1;
        }
        while sum >= target {
            ans = usize::min(ans, right - left);
            sum -= nums[left];
            left += 1;
        }
    }
    if ans == n + 1 {
        ans = 0;
    }

    ans as i32
}

#[cfg(test)]
mod tests {
    use crate::min_sub_array_len;

    #[test]
    pub fn test1() {
        let ansr = min_sub_array_len(7, [2, 3, 1, 2, 4, 3].to_vec());
        assert_eq!(ansr, 2)
    }
}
