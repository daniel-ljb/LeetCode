class Solution {
public:
    vector<string> fullJustify(vector<string>& words, int maxWidth) {
        vector<string> result, current;
        int numberOfLetters = 0;
        
        for (string word : words) {
            if (numberOfLetters + current.size() + word.size() > maxWidth) {
                for (int i = 0; i < maxWidth - numberOfLetters; i++) {
                    current[i % (current.size() == 1 ? 1 : current.size() - 1)] += " ";
                }
                string currentAsString = "";
                for (string w : current) {
                    currentAsString += w;
                }
                result.push_back(currentAsString);
                current.clear();
                numberOfLetters = 0;
            }
            current.push_back(word);
            numberOfLetters += word.size();
        }
        
        string currentAsString = "";
        for (string w : current) {
            currentAsString += w + " ";
        }
        currentAsString.erase(currentAsString.size() - 1);
        while (currentAsString.size() < maxWidth) {
            currentAsString += " ";
        }
        result.push_back(currentAsString);

        return result;
    }
};