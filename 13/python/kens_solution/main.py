def roman_to_int(s):
    roman_dict = {
        "I":1,
        "V":5,
        "X":10,
        "L":50,
        "C":100,
        "D":500,
        "M":1000,
        "IV":4,
        "IX":9,
        "XL":40,
        "XC":90,
        "CD":400,
        "CM":900
    }

    left = total = 0
    right = 1

    while right < len(s) and left < len(s):
        if roman_dict[s[left]] < roman_dict[s[right]]:
            total -= roman_dict[s[left]]
            left+= 1
            right+=1
        else:
            total += roman_dict[s[left]]
            left+=1
            right+=1
    total += roman_dict[s[right-1]]

    return total



print(roman_to_int("DCXXI"))