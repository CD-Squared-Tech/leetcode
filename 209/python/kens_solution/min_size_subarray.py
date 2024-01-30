def min_size_arr(target, nums):
    total = 0
    min_length = 0
    left = 0
            

    for i in range(len(nums)):
        total += nums[i]
        while total >= target:
            if min_length == 0 or (i+1) - left < min_length:
                min_length = (i+1)-left
            total -= nums[left]
            left += 1

    return min_length