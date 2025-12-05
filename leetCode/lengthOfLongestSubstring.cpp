#include <unordered_set>
#include <string>
#include <unordered_map>
#include <iostream>
using namespace std;

class Solution
{
    int lengthOfLongestSubstring(string s)
    {

        int len = s.length();
        if (len == 0)
            return 0;

        unordered_map<char, int> map;

        int maxLen = 0;
        int left = 0;
        for (int i = 0; i < len; i++)
        {

            if (map.find(s[i]) != map.end())
            {
                left = max(left, map[s[i]] + 1);
            }
            map[s[i]] = i;
            maxLen = max(maxLen, i - left + 1);
        }
        return maxLen;
    }
};