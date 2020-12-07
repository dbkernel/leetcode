/*
Given an array of integers, return indices of the two numbers such that they add up to a specific target.
You may assume that each input would have exactly one solution, and you may not use the same element twice.
Example:
Given nums = [2, 7, 11, 15], target = 9,
Because nums[0] + nums[1] = 2 + 7 = 9,
return [0, 1].
*/
class Solution
{
public:
    vector<int> twoSum(vector<int> &nums, int target)
    {
        vector<int> vi;
        unordered_map<int, int> hmap;
        for (auto i = 0; i < nums.size(); i++)
        {
            if (hmap.find(target - nums[i]) != hmap.end())
            {
                vi.push_back(hmap[target - nums[i]]);
                vi.push_back(i);
                return vi;
            }
            hmap.insert(pair<int, int>(nums[i], i));
        }
        return vi;
    }
};
