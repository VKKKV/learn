#include <unordered_set>
#include <string>
#include <unordered_map>
#include <iostream>
#include <vector>
using namespace std;

class Solution
{
    double findMedianSortedArrays(vector<int> &nums1, vector<int> &nums2)
    {
        int len = nums1.size() + nums2.size();
        int i = 0, j = 0;
        int pre = 0, cur = 0;
        for (int k = 0; k <= len / 2; k++)
        {
            pre = cur;
            if (i == nums1.size())
            {
                cur = nums2[j++];
            }
            else if (j == nums2.size())
            {
                cur = nums1[i++];
            }
            else if (nums1[i] < nums2[j])
            {
                cur = nums1[i++];
            }
            else
            {
                cur = nums2[j++];
            }
        }
        if (len % 2 == 0)
        {
            return (pre + cur) / 2.0;
        }
        else
        {
            return cur;
        }
    }
};
