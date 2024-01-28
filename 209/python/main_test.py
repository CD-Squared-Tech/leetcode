import unittest
from main import minSubArrayLen


class TestMinSubArrayLen(unittest.TestCase):
    
    def test_example1(self):
        target = 7
        nums = [2,3,1,2,4,3]
        self.assertEqual(minSubArrayLen(target, nums), 2)

    def test_example2(self):
        target = 4
        nums = [1,4,4]
        self.assertEqual(minSubArrayLen(target, nums), 1)
    
    def test_example3(self):
        target = 11
        nums = [1,1,1,1,1,1,1,1]
        self.assertEqual(minSubArrayLen(target, nums), 0)




if __name__ == '__main__':
    unittest.main()