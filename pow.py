from hashlib import new
import math


class Solution(object):
    def gcd(self, a, b):
        if a == b:
            return a
        if a % b == 0:
            return b
        return self.gcd(self, b, a % b)

    def decimalToFraction(self, n):
        denum = 10000
        num = n * 10000

        while True:
            commonDenom = self.gcd(self, num, denum)
            if commonDenom == 1:
                break
            num = num / commonDenom
            denum = denum / commonDenom

        return [int(num), int(denum)]

    def nthRoot(self, base, rootExp):
        def computeGuess(guess):
            firstTerm = (guess * (rootExp - 1)) / rootExp
            secondTerm = base / (rootExp * self.myPow(self, base, rootExp - 1))
            return firstTerm + secondTerm

        guess1 = base
        newGuess = base
        while True:
            newGuess = computeGuess(guess1)
            if abs(newGuess / guess1) < 1.0001:
                break
            guess1 = newGuess

        return newGuess

    def myPow(self, x, n):
        """
        :type x: float
        :type n: int
        :rtype: float
        """
        if n == 0:
            return 1

        if n < 0:
            return 1 / self.myPow(self, x, -n)

        return x * self.myPow(self, x, n - 1)


print(Solution.nthRoot(Solution, 16, 4))
