
# Example 1:
# Input: s = "abcabcbb"
# Output: 3

# Example 2:

# Input: s = "bbbbb"
# Output: 1

# Example 3:

# Input: s = "pwwkew"
# Output: 3

#r1: (67%, 100%) r2: (76% , 100%)  r3: (94%, 100%)
def lengthOfLongestSubstring(s):
    left = ans = 0
    letters_seen = {}

    for right, letter in enumerate(s):
        if letters_seen.get(letter, -1) >= left:
            left = letters_seen[letter] + 1

        ans = max(ans, right - left + 1)
        letters_seen[letter] = right
    return ans

# print(lengthOfLongestSubstring("abcabcbb"))
# print(lengthOfLongestSubstring("bbbbb"))
# print(lengthOfLongestSubstring("pwwkew"))