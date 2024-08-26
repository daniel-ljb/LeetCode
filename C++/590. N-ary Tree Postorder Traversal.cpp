/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};
*/


// dfs
class Solution {
public:
    vector<int> postorder(Node* root) {
        if (root == nullptr) return {};
        std::vector<int> result;
        
        function<void(Node*)> dfs = [&](Node* node) {
            for (Node* child : node->children) {
                dfs(child);
            }
            result.push_back(node->val);
        };

        dfs(root);

        return result;
    }
};

// stack
class Solution2 {
public:
    vector<int> postorder(Node* root) {
        if (root == nullptr) return {};
        std::vector<int> result;
        std::vector<Node*> stack = {root};
        while (stack.size() > 0) {
            Node* current = stack.back();
            stack.pop_back();
            result.push_back(current->val);
            stack.insert(stack.end(), current->children.begin(), current->children.end());
        }
        std::reverse(result.begin(), result.end());
        return result;
    }
};