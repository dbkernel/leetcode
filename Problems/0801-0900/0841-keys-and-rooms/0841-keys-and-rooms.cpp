/*
841. 钥匙和房间

有 n 个房间，房间按从 0 到 n - 1 编号。最初，除 0 号房间外的其余所有房间都被锁住。
你的目标是进入所有的房间。然而，你不能在没有获得钥匙的时候进入锁住的房间。

当你进入一个房间，你可能会在里面找到一套不同的钥匙，每把钥匙上都有对应的房间号，即表示钥匙可以打开的房间。你可以拿上所有钥匙去解锁其他房间。
给你一个数组 rooms 其中 rooms[i] 是你进入 i 号房间可以获得的钥匙集合。如果能进入 所有 房间返回 true，否则返回 false。

示例 1：
输入：rooms = [[1],[2],[3],[]]
输出：true
解释：
我们从 0 号房间开始，拿到钥匙 1。
之后我们去 1 号房间，拿到钥匙 2。
然后我们去 2 号房间，拿到钥匙 3。
最后我们去了 3 号房间。
由于我们能够进入每个房间，我们返回 true。

示例 2：
输入：rooms = [[1,3],[3,0,1],[2],[0]]
输出：false
解释：我们不能进入 2 号房间。

提示：
n == rooms.length
2 <= n <= 1000
0 <= rooms[i].length <= 1000
1 <= sum(rooms[i].length) <= 3000
0 <= rooms[i][j] < n
所有 rooms[i] 的值 互不相同

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/keys-and-rooms

思路：
当 x 号房间中有 y 号房间的钥匙时，我们就可以从 x 号房间去往 y 号房间。
 如果我们将这 n 个房间看成有向图中的 n 个节点，那么上述关系就可以看作是图中的 x 号点到 y 号点的一条有向边。
这样一来，问题就变成了给定一张有向图，询问从 0 号节点出发是否能够到达所有的节点。
*/

#include <iostream>
#include <cassert>
using namespace std;

/*
方法一：深度优先搜索 DFS（Depth First Search）

我们可以使用深度优先搜索的方式遍历整张图，统计可以到达的节点个数，并利用数组 vis 标记当前节点是否访问过，以防止重复访问。

复杂度分析
- 时间复杂度：O(n+m)，其中 n 是房间的数量，m 是所有房间中的钥匙数量的总数。
- 空间复杂度：O(n)，其中 n 是房间的数量。主要为栈空间的开销。
*/
class Solution {
public:
    vector<int> vis; // 标记当前节点是否访问过
    int num; // 访问过的节点数量

    void dfs(vector<vector<int>>& rooms, int x) {
        vis[x] = true;
        num++;
        for (auto& it : rooms[x]) { // 当前房间里有多把钥匙，每一把都是一个访问分支
            if (!vis[it]) {
                dfs(rooms, it);
            }
        }
    }

    bool canVisitAllRooms(vector<vector<int>>& rooms) {
        int len = rooms.size();
        num = 0;
        vis.resize(len); // 此时才能确定 vis 数组的大小，因此需要 resize 重建。
        dfs(rooms, 0); // 从 0 号开始
        return num == len; // 如果 num 等于 len，说明从 0 号节点出发能够到达所有的节点。
    }
};

/*
方法二：广度优先搜索 BFS（Breadth First Search）

我们也可以使用广度优先搜索的方式遍历整张图，统计可以到达的节点个数，并利用数组 vis 标记当前节点是否访问过，以防止重复访问。

复杂度分析
- 时间复杂度：O(n+m)，其中 n 是房间的数量，m 是所有房间中的钥匙数量的总数。
- 空间复杂度：O(n)，其中 n 是房间的数量。主要为队列的开销。
*/
class Solution {
public:
    bool canVisitAllRooms(vector<vector<int>>& rooms) {
        int len = rooms.size(), num = 0; // num 表示访问过的节点数量
        vector<int> vis(len); // 标记当前节点是否访问过
        queue<int> que; // 为了保证广度优先遍历，需要用到队列存储平行节点

        // 先访问 0 号，因此先入队
        vis[0] = true;
        que.emplace(0);

        // 遍历访问其他节点
        while (!que.empty()) {
            // 取出队头元素
            int x = que.front();
            que.pop();

            num++; // 访问节点数+1
            for (auto& it : rooms[x]) { // 遍历当前节点指向的下一级节点
                if (!vis[it]) { // 确保当前节点没有访问过
                    vis[it] = true;
                    que.emplace(it); // 入队
                }
            }
        }
        return num == len; // 如果 num 等于 len，说明从 0 号节点出发能够到达所有的节点。
    }
};






