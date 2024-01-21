# Longest Substring Without Repeating Characters


# Intuition

We are given a numeric constraint metric (all unique), and we are searching for a subarray

These are good restrictions to take advantage of the sliding window algorithm.

Its also worth noting that since sliding windows involve a numeric constraint, hashmaps, dicts, and sets would be decent data type choices.


#### Initialize left and right pointers for window

#### Initialize dict to keep track of counts
We want this or a similar data type that has O(1) lookup times (by key in this case) and we can also test membership in  O(1)

#### While the right pointer iterates through the list, it should update our dictionary to increment a letter by one every time it is seen


Once we see our first duplicate, the count will be 2 which is helpful for defining our window constraint


#### While the letter at our right pointer is > 1 (duplicate)


We will just decrease the count of whatever letter left is pointing at

And increment left by 1

This will repeat until the duplicate letter has been removed

#### Once our inner loop has finished (or broken its condition)

We are sure that our current window does not contain any duplicates for the letters we are currently iterating on.

#### We can now update ans to be the max value of ans or right - left + 1

The last thing is to make sure that right is incremented after ans is updated.

#### When our loops end we will have stored the largest length of a valid subarray


```

def lengthOfLongestSubstring(s):

    left = right = ans= 0
    letter_counts = Counter()

    while right < len(s):
        right_letter = s[right]
        letter_counts[right_letter] += 1
        while letter_counts[right_letter] > 1:
            letter_counts[s[left]] -= 1
            left += 1
        ans = max(ans, right - left + 1)
        right += 1
    return ans
```








```

def lengthOfLongestSubstring(s):
    left = ans = 0
    seen = {}

    for right, letter in enumerate(s):
        if seen.get(letter, -1) >= left:
            left = seen[letter] + 1
        ans = max(ans , right - left + 1)
        seen[letter] = right
    return ans

```



# Optimized solution approach

This is roughly the same approach as above, with more time and thought for a clever implementation


In the above approach, we were storing and calculating the frequency of letters in our string.. but we did not actually need counts..

It was helpful to keep track of the count of our numbers, but if you think about it

There was nothing else useful provided from our frequencies, besides the distinction of uniqueness

#### We can check if something is unique or not without values

As we insert letters into our dict, we can check membership of the dict.

So any time we match a letter, we know our window our window needs to constrict or shrink to become valid again


#### Using indexes to change our window instead of values

Enumerate allows us to loop through our string getting both the index and the value at each iteration

We are able to take advantage of enumerate, use its temp variable for index as our right pointer for sliding window


This starts us off with 2 pointers at 0, with the right pointer iterating to the end of the iterable

#### Taking advantage of .get

This was a good case to provide an optional default value to dict.get()

We choose a default value that is lower than our left pointer..

This is because our right pointer assigns the index the letter was last seen at to our dictionary

By doing this, when we check membership we know where a letter was last seen and whether or not its valid

We can use that to reset our window, by re-assigning left to the index of our current duplicate + 1

#### This effectively shifts our window to the next valid window

Just like before, we want to calculate the max valid window length on each iteration of our outter loop



