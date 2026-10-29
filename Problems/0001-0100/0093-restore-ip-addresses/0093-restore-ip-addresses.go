/*
============================================================================
93. 复原 IP 地址（中等）
来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/restore-ip-addresses/
分类：回溯 · 定长分段与剪枝

有效 IP 地址 正好由四个整数（每个整数位于 0 到 255 之间组成，且不能含有
前导 0），整数之间用 '.' 分隔。

例如："0.1.2.201" 和 "192.168.1.1" 是 有效 IP 地址，但是 "0.011.255.245"、
"192.168.1.312" 和 "192.168@1.1" 是 无效 IP 地址。

给定一个只包含数字的字符串 s ，用以表示一个 IP 地址，返回所有可能的有效 IP
地址，这些地址可以通过在 s 中插入 '.' 来形成。你 不能 重新排序或删除 s 中的
任何数字。你可以按 任何 顺序返回答案。

示例 1：
输入：s = "25525511135"
输出：["255.255.11.135","255.255.111.35"]

示例 2：
输入：s = "0000"
输出：["0.0.0.0"]

示例 3：
输入：s = "101023"
输出：["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]

提示：
- 1 <= s.length <= 20
- s 仅由数字组成
*/
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

/*
解法：回溯枚举四段（长度 1~3、无前导零、值不超过 255）【最优解】

为什么慢：最直接的想法是从 n-1 个空隙里任选 3 个插入点，共 C(n-1, 3) 种
切法，再逐段校验。切法本身不多，但它完全不做约束传播：切到 "999" 这样的段
还会继续往下切，白白展开一整棵子树。

换个角度：IP 只有固定四段，每段长度只能是 1~3，于是可以按"段"递归：每层
从 start 处取 1~3 个字符作为当前段，校验通过才往下走，取到第 4 段时要求
恰好走到字符串末尾。深度固定为 4，分支最多 3，搜索树极小。

关键点：
  · 段长超过 1 时不能以 '0' 开头，"0.011" 这类写法要整段砍掉
  · 段值边取边累加，一旦超过 255 就 break：再往后取只会更大，不可能回到合法
  · 第 4 段必须恰好消耗完所有字符；提前用完或还剩字符都不算解
  · 递归前先算剩余字符数 remain，必须满足 remain >= 剩余段数且
    remain <= 3 * 剩余段数，这一步能把长度明显不够或太长的输入直接剪没
  · 用切片 base 记录进入循环前的长度，回溯时 cur = cur[:base] 一次还原

以示例 1（s = "25525511135"）为例：
  前两段都取 255，剩 "11135" 要拆成两段
  第三段取 "11"，第四段 "135" → 255.255.11.135 ✔
  第三段改取 "111"，第四段 "35" → 255.255.111.35 ✔
  第三段取 "1113" 超过 255 被 break，其余分支同理剪掉

复杂度分析：
  时间 O(3^4 · n)：深度固定为 4、每层最多 3 个分支，叶子最多 81 个，
    每个叶子拼一次长度 O(n) 的字符串；n 再大也不会改变搜索树规模
  空间 O(4)：递归栈只有 4 层，cur 长度不超过 n + 3；结果集不计入
*/
func restoreIpAddresses(s string) []string {
	res := []string{}
	cur := make([]byte, 0, len(s)+3)

	var dfs func(start, part int)
	dfs = func(start, part int) {
		if part == 4 {
			// 四段用完且字符也正好用完，才是一个完整地址
			if start == len(s) {
				res = append(res, string(cur))
			}
			return
		}

		remain := len(s) - start
		left := 4 - part
		// 剩余字符太少填不满、太多装不下，这一支直接剪掉
		if remain < left || remain > 3*left {
			return
		}

		base := len(cur)
		value := 0
		for k := 1; k <= 3 && start+k <= len(s); k++ {
			// 多位数不允许前导零；再长也还是以 0 开头，直接停
			if k > 1 && s[start] == '0' {
				break
			}

			value = value*10 + int(s[start+k-1]-'0')
			// 已经超过 255，继续取只会更大
			if value > 255 {
				break
			}

			if part > 0 {
				cur = append(cur, '.')
			}
			cur = append(cur, s[start:start+k]...)

			dfs(start+k, part+1)

			// 一次砍掉"段 + 点"，回到进入循环前的长度
			cur = cur[:base]
		}
	}

	dfs(0, 0)
	return res
}

// 逐字符回验：四段、无前导零、每段在 [0, 255]、拼回去等于原串
func validIp(ip, s string) bool {
	parts := strings.Split(ip, ".")
	if len(parts) != 4 {
		return false
	}

	joined := ""
	for _, p := range parts {
		if len(p) == 0 || len(p) > 3 {
			return false
		}
		if len(p) > 1 && p[0] == '0' {
			return false
		}
		v, err := strconv.Atoi(p)
		if err != nil || v > 255 {
			return false
		}
		joined += p
	}
	return joined == s
}

func checkIp(s string, want []string) {
	got := restoreIpAddresses(s)

	sortedGot := append([]string(nil), got...)
	sort.Strings(sortedGot)
	sortedWant := append([]string(nil), want...)
	sort.Strings(sortedWant)

	if len(sortedGot) != len(sortedWant) {
		panic(fmt.Sprintf("%q 得到 %v，期望 %v", s, got, want))
	}
	for i := range sortedGot {
		if sortedGot[i] != sortedWant[i] {
			panic(fmt.Sprintf("%q 得到 %v，期望 %v", s, got, want))
		}
		// 每个输出都要能反解回原串，且四段都合法
		if !validIp(sortedGot[i], s) {
			panic(fmt.Sprintf("%q 产生了非法 IP %q", s, sortedGot[i]))
		}
	}
}

func main() {
	// 示例 1
	checkIp("25525511135", []string{"255.255.11.135", "255.255.111.35"})

	// 示例 2：四个零，唯一解是全单段 0
	checkIp("0000", []string{"0.0.0.0"})

	// 示例 3
	checkIp("101023", []string{"1.0.10.23", "1.0.102.3", "10.1.0.23",
		"10.10.2.3", "101.0.2.3"})

	// 前导零陷阱："010010" 不能拆成 01.00.1.0，只有下面两个解
	checkIp("010010", []string{"0.10.0.10", "0.100.1.0"})

	// 刚好四位的下界
	checkIp("1111", []string{"1.1.1.1"})

	// 四段都取满 255
	checkIp("255255255255", []string{"255.255.255.255"})

	// 长度不足 4：不可能凑出四段
	checkIp("123", []string{})
	checkIp("0", []string{})

	// 长度超过 12：四段每段最多 3 位，必然放不下
	checkIp("1234567890123", []string{})

	// 长度合法但只能靠"一位 + 两位"凑出四段
	checkIp("11111", []string{"1.1.1.11", "1.1.11.1", "1.11.1.1", "11.1.1.1"})

	// 含连续零的易错串：多位数段不能全零，5 个 0 凑不出四段
	checkIp("00000", []string{})

	fmt.Println("all tests passed")
}
