class Solution(object):
    def __init__(self):
        return

    def isPalindrome(self, s):
        return s == s[::-1]

    def longestPalindrome(self, s):
        if len(s) == 1:
            return 1

        window_start = 0
        window_end = 2
        longest = 1

        while True:
            if self.isPalindrome(s[window_start:window_end]):
                
