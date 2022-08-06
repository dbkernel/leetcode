# BigdataExample

**要求**

编程实现以下各任务, 请尽量优化你的程序以⾼效处理海量数据, 请注重代码⻛格与程序结构, 请以注释对程序的⾏为进⾏解释.

你可以使⽤语⾔⾃带的基础库, 如 C 中可使⽤ libc 提供的函数, C++ 中可以使⽤ STL. 可以假设内存⼤到⾜以容纳所有数据.

**任务 1**

基于以下 C 程序框架, 我们希望你实现 `task1()` 这个函数, 它把 rows 中所有满⾜ `b >= 10 && b < 50 并且 a == 1000 || a == 2000 || a == 3000` 的⾏的内容都打印到终端.

```cpp
typedef struct Row
{
    int a;
    int b;
} Row;

/*!
* Task 1.
*
* Find out all the rows that sastify below conditions:
*
* ((b >= 10 && b < 50) &&
* (a == 1000 || a == 2000 || a == 3000))
*
* Print them to the terminal, one row per line, for example:
*
* 1000,20
* 1000,23
* 2000,16
*
* @param nrows The total number of rows.
* @param rows The rows, for example rows[0] is the first row.
*/
void task1(const Row *rows, int nrows)
{
    /*
     * TODO: implement this function.
     */
}
```

**任务 2**

在任务 1 的基础上, 如果输⼊的参数 rows 已经按照 (a,b) 进⾏过排序, 请实现 task2() , 对 task1() 的执⾏性能进⾏优化. ⽰例输⼊ (再次提醒, 实际输⼊为海量数据):

```cpp
Row sorted_rows[] =
{
    { 1000, 31 },
    { 1000, 72 },
    { 1500, 12 },
    { 1500, 34 },
    { 2000, 22 },
    { 2000, 33 },
};
```

⽰例输出:

```cpp
1000,31
2000,22
2000,33
```

**任务 3**

在任务 2 的基础上, 我们期望你打印出的匹配⾏是按照 b 列进⾏排序的, 请实现 task3() . ⽰例输出:

```cpp
2000,22
1000,31
2000,33
```

**任务 4**

在任务 3 的基础上, 如果 a 列上的匹配条件不只是 `1000,2000,3000` , ⽽是扩充成 `1000,2000,3000,...,98000,99000` , 你在任务 3 中进⾏的实现是否⾜够优化? 请针对这⼀场景实现 task4() , 再次提醒, 程序要能⾼效处理海量数据.
