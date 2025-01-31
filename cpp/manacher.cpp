#include <vector>
#include <string>
#include <cstdint>
#include <algorithm>
#include <sstream>

using namespace std;

string manacher(string& s) {
    size_t s_len = s.length();
    size_t e_len = s_len * 2 + 1;
    vector<char> extended(e_len, 0);
    for (size_t i = 0; i < s_len; ++i) {
        extended[i * 2 + 1] = s[i];
    }

    size_t r = 0, p = 0;
    vector<size_t> dp(e_len, 0);

    for (size_t i = 0; i < e_len; ++i) {
        if (i <= r) {
            dp[i] = min(dp[p * 2 - i], r - i);
        }

        while (
            (i >= dp[i] + 1) && 
            (i + dp[i] + 1 < e_len) &&
            (extended[i - dp[i] - 1] == extended[i + dp[i] + 1])
        ) {
            dp[i] += 1;
        }

        if (r < i + dp[i]) {
            r = i + dp[i];
            p = i;
        }
    }

    size_t idx = distance(dp.begin(), max_element(dp.begin(), dp.end()));

    stringstream ss;
    for (int i = idx - dp[idx] + 1; i < idx + dp[idx]; i += 2) {
        ss << extended[i];
    }
    
    return ss.str();
}