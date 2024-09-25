class Solution:
    def sumPrefixScores(self, words: List[str]) -> List[int]:
        original_order, sorted_words = zip(*sorted(enumerate(words), key=lambda x:x[1]))
        answer = [0] * len(words)
        prefixes = []
        prefix_counts = []
        for i, word in enumerate(sorted_words):
            while len(prefixes) < len(word):
                prefixes.append("")
                prefix_counts.append(0)
            add_point = len(word)
            for j in range(len(word)):
                if word[j] == prefixes[j]:
                    prefix_counts[j] += 1
                else:
                    add_point = j
                    break
            k = len(prefixes) - add_point
            for _ in range(k):
                amount = prefix_counts.pop()
                prefixes.pop()
                for l in range(amount):
                    answer[i-1-l] += amount
            
            for m in range(add_point, len(word)):
                prefixes.append(word[m])
                prefix_counts.append(1)

        for _ in range(len(prefix_counts)):
            amount = prefix_counts.pop()
            prefixes.pop()
            for l in range(amount):
                answer[len(words)-1-l] += amount
        
        _, original_order_answer = zip(*sorted(zip(original_order, answer)))
        
        return original_order_answer
