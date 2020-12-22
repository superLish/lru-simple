### LRU算法
LRU(Least recently used)最近最少使用算法，是一种常用的缓存淘汰算法，主要思想就是将最近最少使用的数据淘汰掉，保留最近使用过的数据。
当缓存满了的时候，需要选择一种策略将部分缓存淘汰掉，LRU的出发点就是淘汰掉最近最少使用的那部分缓存。


### 实现思路
最简单的思路就是为每个缓存数据添加一个时间戳，用来记录最近一次访问的时间，当缓存满了的时候，淘汰数据时遍历所有缓存数据，选择时间戳最老的那个淘汰掉。
想法简单，但性能太差。代码可见[lru_time](../src/lru_time.rs)。

![image](lru.png)
