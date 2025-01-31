#include <vector>
#include <string>

using namespace std;

vector<size_t> get_pi(const string& pattern) {
    size_t begin = 1, matched = 0, pattern_len = pattern.length();
    vector<size_t> pi(pattern_len, 0);
    // pi[i]: pattern[..i]에서 Prefix == Suffix의 최대 길이
    while (begin + matched < pattern_len) {
        if (pattern[begin + matched] == pattern[matched]) {
            matched += 1;
            pi[begin + matched - 1] = matched;
        } else {
            if (matched == 0) begin += 1;
            else {
                begin += matched - pi[matched - 1];
                matched = pi[matched - 1];
            }
        }
    }
    return pi;
}

vector<size_t> kmp(const string& txt, const string& pattern) {
    size_t txt_len = txt.length(), pattern_len = pattern.length();
    vector<size_t> pi = get_pi(pattern);
    vector<size_t> found;
    size_t begin = 0, matched = 0;

    while (begin <= txt_len - pattern_len) {
        if (matched < pattern_len && txt[begin + matched] == pattern[matched]) {
            matched += 1;
            if (matched == pattern_len) found.push_back(begin);
        } else {
            if (matched == 0) begin += 1;
            else {
                begin += matched - pi[matched - 1];
                matched = pi[matched - 1];
            }
        }
    }

    return found;
}