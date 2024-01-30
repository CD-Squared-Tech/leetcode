def minSubArrayLen(target, nums):
    left = right = curr = 0
    ans = float('inf')

    while right < len(nums):
        curr += nums[right]

        while curr >= target:
            curr -= nums[left]
            ans = min(ans , right - left + 1)
            left += 1
        right += 1
    return ans if ans < float('inf') else 0



if __name__ == '__main__':
    print(minSubArrayLen(7, [2,3,1,2,4,3]))




















# Example 1:

# Input: target = 7, nums = [2,3,1,2,4,3]
# Output: 2


# Example 2:

# Input: target = 4, nums = [1,4,4]
# Output: 1

# Example 3:

# Input: target = 11, nums = [1,1,1,1,1,1,1,1]
# Output: 0