#include <string>
#include <iostream>

using namespace std;
//中心扩散
//c++ substr(start,length)返回从start开始的length个字符
//java substring(start,end)返回从start到end-1的字符
class Solution
{
public:
    string longestPalindrome(string s)
    {
        int length = s.length();
        int maxStart = 0;
        int maxLength = 0;
        for (int i = 0; i < length; i++)
        {
            // j=0表示中心节点只有 i，j=1，表示中心节点有两个 i，i+1;
            for (int j = 0; j <= 1; j++)
            {
                int l = i;
                int r = i + j;

                while (l >= 0 && r < length && s[l] == s[r])
                {
                    l--;
                    r++;
                }

                // 回溯到回文字符串的起始和结束位置
                l++;
                r--;

                // 比较并保存最长的字符串起始位置和长度。
                if (maxLength < r - l + 1)
                {
                    maxLength = r - l + 1;
                    maxStart = l;
                }
            }
        }

        return s.substr(maxStart, maxLength);
    }
};
int main()
{
    Solution s;
    string i;
    cout << s.longestPalindrome("cbbd") << endl;
    return 0;
}