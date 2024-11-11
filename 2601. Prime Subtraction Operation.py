from bisect import bisect_left

class Solution:
    def primeSubOperation(self, nums: List[int]) -> bool:
        def generate_primes(n):
            if n < 2:
                return [2]

            is_prime = [True] * (n + 1)
            is_prime[0] = is_prime[1] = False

            for i in range(2, int(n**0.5) + 1):
                if is_prime[i]:
                    for multiple in range(i * i, n + 1, i):
                        is_prime[multiple] = False

            primes = [num for num, prime in enumerate(is_prime) if prime]
            return primes

        primes = generate_primes(max(nums))
        primes.append(2*primes[-1])
        for i in range(1, len(nums)):
            i = len(nums) - i - 1
            if nums[i] >= nums[i+1]:
                nums[i] -= primes[bisect_left(primes, nums[i] - nums[i+1] + 1)]
                if nums[i] <= 0: return False
              
        return True
