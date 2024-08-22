class Solution {
public:
    vector<string> findItinerary(vector<vector<string>>& tickets) {
        std::unordered_map<std::string, std::vector<std::string>> flights;

        for (auto ticket : tickets) {
            auto it = flights.find(ticket[0]);
            if (it != flights.end()) {
                it->second.push_back(ticket[1]);
            }
            else {
                flights[ticket[0]] = {ticket[1]};
            }
        }

        for (auto& [_, destinations] : flights) {
            sort(destinations.rbegin(), destinations.rend());
        }
        
        vector<string> itinerary;

        function<void(const string&)> dfs = [&](const string& airport) {
            while (!flights[airport].empty()) {
                string next = flights[airport].back();
                flights[airport].pop_back();
                dfs(next);
            }
            itinerary.push_back(airport);
        };

        dfs("JFK");
        reverse(itinerary.begin(), itinerary.end());

        return itinerary;
    }
};