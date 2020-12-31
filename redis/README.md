# Redis 手记

<!-- vim-markdown-toc GFM -->

* [简介](#简介)
  * [主要词条](#主要词条)
* [Redis 命令](#redis-命令)
  * [通用命令](#通用命令)
  * [Redis Key 命令](#redis-key-命令)
  * [字符串命令(Strings): key -> string](#字符串命令strings-key---string)
  * [列表命令(Lists，有序，可重复): key -> [k1, k1, k2, k3, ...]](#列表命令lists有序可重复-key---k1-k1-k2-k3-)
  * [集合命令(Sets，无序，不可重复): key -> [k1, k2, k3, k4, ...]](#集合命令sets无序不可重复-key---k1-k2-k3-k4-)
  * [有序集合(SortedSets): key -> [(score1, k1), (score2, k2), (score3, k3), ...]](#有序集合sortedsets-key---score1-k1-score2-k2-score3-k3-)
  * [哈希(Hashes): key -> {k1:v1, k2:v2, ...}](#哈希hashes-key---k1v1-k2v2-)
  * [持久化](#持久化)
* [二级目录](#二级目录)

<!-- vim-markdown-toc -->

## 简介

> Redis 是一个开源（BSD 许可）的，内存中的数据结构存储系统，它可以用作数据库、缓存和消息中间件。 它支持多种类型的数据结构，如 字符串（strings）， 散列（hashes）， 列表（lists）， 集合（sets）， 有序集合（sorted sets） 与范围查询， bitmaps， hyperloglogs 和 地理空间（geospatial） 索引半径查询。 Redis 内置了 复制（replication），LUA 脚本（Lua scripting）， LRU 驱动事件（LRU eviction），事务（transactions） 和不同级别的 磁盘持久化（persistence）， 并通过 Redis 哨兵（Sentinel）和自动 分区（Cluster）提供高可用性（high availability）。

### 主要词条

1. 开源
2. 内在数据结构存储
3. 可用作数据库，缓存，消息中间件
4. 5 大基本数据类型：Strings, Hashes, Lists, Sets, SortedSets
5. 3 个特殊类型：BitMaps, Hyperloglog, Geospatial
6. 支持复制(replication), Lua 脚本, LRU 驱动事件, 事务，磁盘持久化
7. 支持集群，提供高可用。Redis 哨兵（Sentinel）和自动分区（Cluster）

## Redis 命令

> Redis 命令十分丰富，包括的命令组有 Cluster、Connection、Geo、Hashes、HyperLogLog、Keys、Lists、Pub/Sub、Scripting、Server、Sets、Sorted Sets、Strings、Transactions 一共 14 个 redis 命令组两百多个 redis 命令。

### 通用命令

* 客户端连接

```bash
$ redis-cli -h
redis-cli 6.0.9

Usage: redis-cli [OPTIONS] [cmd [arg [arg ...]]]
  -h <hostname>      Server hostname (default: 127.0.0.1).
  -p <port>          Server port (default: 6379).
  -a <password>      Password to use when connecting to the server.
                     You can also use the REDISCLI_AUTH environment
                     variable to pass this password more safely
                     (if both are used, this argument takes precedence).
  --pass <password>  Alias of -a for consistency with the new --user option.

$ redis-cli -h 127.0.0.1 -p 6379 # 交互式命令
127.0.0.1:6379>

$ redis-cli -h 127.0.0.1 -p 6379 get hello # 命令方式
(nil)
```

* ping: 检测 redis 服务是否启动
* quit:关闭连接（connection）
* auth:简单的密码认证

### Redis Key 命令

* AUTH: `AUTH password`

* APPEND: `APPEND key value`

  如果`key`已经存在，并且值为字符串，那么这个命令会把`value`追加到原来值（value）的结尾。 如果`key`不存在，那么它将首先创建一个空字符串的`key`，再执行追加操作，这种情况`APPEND`将类似于`SET`操作。

* BGREWRITEAOF: `BGREWRITEAOF`

  Redis `BGREWRITEAOF` 命令用于异步执行一个 `AOF（AppendOnly File）`文件重写操作。重写会创建一个当前AOF文件的体积优化版本。
  
  即使 `BGREWRITEAOF` 执行失败，也不会有任何数据丢失，因为旧的AOF文件在 `BGREWRITEAOF` 成功之前不会被修改。
  
  AOF 重写由 Redis 自行触发， `BGREWRITEAOF`仅仅用于手动触发重写操作。
  
  具体内容:
  1. 如果一个子Redis是通过磁盘快照创建的，AOF重写将会在RDB终止后才开始保存。这种情况下BGREWRITEAOF任然会返回OK状态码。从Redis 2.6起你可以通过INFO命令查看AOF重写执行情况。
  2. 如果只在执行的AOF重写返回一个错误，AOF重写将会在稍后一点的时间重新调用

  从 Redis 2.4 开始，AOF重写由 Redis 自行触发，BGREWRITEAOF仅仅用于手动触发重写操作

* DUMP: `DUMP key`
  
  序列化给定 key ，并返回被序列化的值，使用 RESTORE 命令可以将这个值反序列化为 Redis 键。
  
  序列化生成的值有以下几个特点：
  1. 它带有 64 位的校验和，用于检测错误，RESTORE 在进行反序列化之前会先检查校验和。
  2. 值的编码格式和 RDB 文件保持一致。
  3. RDB 版本会被编码在序列化值当中，如果因为 Redis 的版本不同造成 RDB 格式不兼容，那么 Redis 会拒绝对这个值进行反序列化操作。
  
  序列化的值不包含任何时间信息。

* EXPIRE: `EXPIRE key seconds`

  设置`key`的过期时间，超过时间后，将会自动删除该`key`。在Redis的术语中一个key的相关超时是不确定的。

  超时后只有对`key`执行`DEL`命令或者SET命令或者GETSET时才会清除。 这意味着，从概念上讲所有改变key的值的操作都会使他清除。 例如，`INCR`递增`key`的值，执行`LPUSH`操作，或者用`HSET`改变`hash`的`field`所有这些操作都会触发删除动作。
  
  使用`PERSIST`命令可以清除超时，使其变成一个永久的`key`。

  如果`key`被`RENAME`命令修改，相关的超时时间会转移到新`key`上面。

  如果`key`被`RENAME`命令修改，比如原来就存在`Key_A`,然后调用`RENAME Key_B Key_A`命令，这时不管原来`Key_A`是永久的还是设置为超时的，都会由`Key_B`的有效期状态覆盖。

* EXPIREAT: `EXPIREAT key timestamp`

  EXPIREAT 的作用和 `EXPIRE`类似，都用于为`key`设置生存时间。不同在于`EXPIREAT`命令接受的时间参数是 UNIX 时间戳 Unix timestamp 。

* KEYS: `KEYS pattern`
  查找所有符合给定模式pattern（正则表达式）的 key 。 时间复杂度为O(N)，N为数据库里面key的数量。
* del key: 用于在 key 存在时删除 key

```bash
127.0.0.1:6379> DEL k1
(integer) 1
127.0.0.1:6379> DEL kk
(integer) 0
```

* exists(key)：确认一个 key 是否存在
* del(key)：删除一个 key
* type(key)：返回值的类型
* keys(pattern)：返回满足给定 pattern 的所有 key
* randomkey：随机返回 key 空间的一个 key
* rename(oldname, newname)：将 key 由 oldname 重命名为 newname，若 newname 存在则删除 newname 表示的 key
* dbsize：返回当前数据库中 key 的数目
* expire：设定一个 key 的活动时间（s）
* ttl：获得一个 key 的活动时间
* select(index)：按索引查询
* move(key, dbindex)：将当前数据库中的 key 转移到有 dbindex 索引的数据库
* flushdb：删除当前选择数据库中的所有 key
. flushall：删除所有数据库中的所有 key

### 字符串命令(Strings): key -> string

> 一些通用命令前以 m 打头的命令为批量操作(mset, mget, msetnx)

* set(key, value)：给数据库中名称为 key 的 string 赋予值 value
* get(key)：返回数据库中名称为 key 的 string 的 value
* del(key1, key2...)：删除指定的一批 keys，如果删除中的某些 key 不存在，则直接忽略
* getset(key, value)：给名称为 key 的 string 赋予上一次的 value
* mget(key1, key2,…, key N)：返回库中多个 string（它们的名称为 key1，key2…）的 value
* setnx(key, value)：如果不存在名称为 key 的 string，则向库中添加 string，名称为 key，值为 value
* setex(key, time, value)：向库中添加 string（名称为 key，值为 value）同时，设定过期时间 time
* mset(key1, value1, key2, value2,…key N, value N)：同时给多个 string 赋值，名称为 key i 的 string 赋值 value i
* msetnx(key1, value1, key2, value2,…key N, value N)：如果所有名称为 key i 的 string 都不存在，则向库中添加 string，名称 key i 赋值为 value i
* incr(key)：名称为 key 的 string 增 1 操作
* incrby(key, integer)：名称为 key 的 string 增加 integer
* decr(key)：名称为 key 的 string 减 1 操作
* decrby(key, integer)：名称为 key 的 string 减少 integer
* append(key, value)：名称为 key 的 string 的值附加 value
* ***substr(key, start, end)：返回名称为 key 的 string 的 value 的子串，原 key 的值不改变

### 列表命令(Lists，有序，可重复): key -> [k1, k1, k2, k3, ...]

> Lists 命令大部分以*l*打头。如(lset, llen)
> 队列(先进先出)：lpush 左进+rpop 右出 / rpush 右进+lpop 左出
> 栈(先进后出)：lpush 左进+lpop 左出 / rpush 右进+rpop 右出

* rpush(key, value)：在名称为 key 的 list 尾添加一个值为 value 的元素
* lpush(key, value)：在名称为 key 的 list 头添加一个值为 value 的 元素
* llen(key)：返回名称为 key 的 list 的长度
* lrange(key, start, end)：返回名称为 key 的 list 中 start 至 end 之间的元素（下标从 0 开始，下同）
* ltrim(key, start, end)：截取名称为 key 的 list，保留 start 至 end 之间的元素
* lindex(key, index)：返回名称为 key 的 list 中 index 位置的元素
* lset(key, index, value)：给名称为 key 的 list 中 index 位置的元素赋值为 value
* lrem(key, count, value)：删除 count 个名称为 key 的 list 中值为 value 的元素。count 为 0，删除所有值为 value 的元素，count>0 从头至尾删除 count 个值为 value 的元素，count<0 从尾到头删除|count|个值为 value 的元素。 lpop(key)：返回并删除名称为 key 的 list 中的首元素 rpop(key)：返回并删除名称为 key 的 list 中的尾元素 blpop(key1, key2,… key N, timeout)：lpop 命令的 block 版本。即当 timeout 为 0 时，若遇到名称为 key i 的 list 不存在或该 list 为空，则命令结束。如果 timeout>0，则遇到上述情况时，等待 timeout 秒，如果问题没有解决，则对 keyi+1 开始的 list 执行 pop 操作
* brpop(key1, key2,… key N, timeout)：rpop 的 block 版本
* rpoplpush(srckey, dstkey)：返回并删除名称为 srckey 的 list 的尾元素，并将该元素添加到名称为 dstkey 的 list 的头部

### 集合命令(Sets，无序，不可重复): key -> [k1, k2, k3, k4, ...]

> 大部分命令以*S*打头

* sadd(key, member)：向名称为 key 的 set 中添加元素 member
* srem(key, member) ：删除名称为 key 的 set 中的元素 member
* spop(key) ：随机返回并删除名称为 key 的 set 中一个元素
* smove(srckey, dstkey, member) ：将 member 元素从名称为 srckey 的集合移到名称为 dstkey 的集合
* scard(key) ：返回名称为 key 的 set 的基数
* sismember(key, member) ：测试 member 是否是名称为 key 的 set 的元素
* sinter(key1, key2,…key N) ：求交集
* sinterstore(dstkey, key1, key2,…key N) ：求交集并将交集保存到 dstkey 的集合
* sunion(key1, key2,…key N) ：求并集
* sunionstore(dstkey, key1, key2,…key N) ：求并集并将并集保存到 dstkey 的集合
* sdiff(key1, key2,…key N) ：求差集
* sdiffstore(dstkey, key1, key2,…key N) ：求差集并将差集保存到 dstkey 的集合
* smembers(key) ：返回名称为 key 的 set 的所有元素
* srandmember(key) ：随机返回名称为 key 的 set 的一个元素

### 有序集合(SortedSets): key -> [(score1, k1), (score2, k2), (score3, k3), ...]

> 大部分命令以`Z`打头

* `ZADD key [NX|XX] [CH] [INCR] score member [score memer ...]`

  向名称为 key 的 zset 中添加元素 member，score 用于排序。如果该元素已经存在，则根据 score 更新该元素的顺序。

* `ZRANGE key start stop [WITHSCORES]`

  返回存储在有序集合 key 中的指定范围的元素。 返回的元素可以认为是按得分从最低到最高排列。 如果得分相同，将按字典排序

  1. 当你需要元素从最高分到最低分排列时，请参阅 ZREVRANGE（相同的得分将使用字典倒序排序）
  2. 参数`start`和`stop`都是基于零的索引，即 0 是第一个元素，1 是第二个元素，以此类推。 它们也可以是负数，表示从有序集合的末尾的偏移量，其中-1 是有序集合的最后一个元素，-2 是倒数第二个元素，等等。
  3. `start`和`stop`都是全包含的区间`[start, stop]`，因此例如`ZRANGE myzset 0 1`将会返回有序集合的第一个和第二个元素。超出范围的索引不会产生错误。 如果`start`参数的值大于有序集合中的最大索引，或者`start > stop`，将会返回一个空列表。 如果`stop`的值大于有序集合的末尾，Redis 会将其视为有序集合的最后一个元素
  4. 可以传递`WITHSCORES`选项，以便将元素的分数与元素一起返回。这样，返回的列表将包含`value1,score1,...,valueN,scoreN`，而不是`value1,...,valueN`。 客户端类库可以自由地返回更合适的数据类型（建议：具有值和得分的数组或记录）

* `ZREVRANGE key start stop [WITHSCORES]`

  同`ZRANGE`,成员的位置按`score`值递减(从大到小)来排列。具有相同`score`值的成员按字典序的反序排列。

* `ZRANGEBYSCORE key min max [WITHSCORES] [LIMIT offset count]`

  如果 M 是常量（比如，用 limit 总是请求前 10 个元素），你可以认为是 O(log(N))。

  返回 key 的有序集合中的分数在 min 和 max 之间的所有元素（包括分数等于 max 或者 min 的元素）。元素被认为是从低分到高分排序的。

  具有相同分数的元素按字典序排列（这个根据 redis 对有序集合实现的情况而定，并不需要进一步计算）。

  1. 可选的`LIMIT`参数指定返回结果的数量及区间（类似 SQL 中`SELECT LIMIT offset, count`）。注意，如果`offset`太大，定位`offset`就可能遍历整个有序集合，这会增加`O(N)`的复杂度。

  2. 可选参数 WITHSCORES 会返回元素和其分数，而不只是元素。这个选项在 redis2.0 之后的版本都可用

  3. 区间及无限
     1. min 和 max 可以是`-inf`和`+inf`，这样一来，你就可以在不知道有序集的最低和最高`score`值的情况下，使用`ZRANGEBYSCORE`这类命令
     2. 默认情况下，区间的取值使用闭区间(小于等于或大于等于)，你也可以通过给参数前增加(符号来使用可选的开区间(小于或大于)

* `ZRANK key member`

  返回有序集 key 中成员 member 的排名。其中有序集成员按 score 值递增(从小到大)顺序排列。排名以 0 为底，也就是说，score 值最小的成员排名为 0。

  使用 ZREVRANK 命令可以获得成员按 score 值递减(从大到小)排列的排名。

* `ZREVRANK key member`

  返回有序集 key 中成员 member 的排名，其中有序集成员按 score 值从大到小排列。排名以 0 为底，也就是说，score 值最大的成员排名为 0。

  使用 ZRANK 命令可以获得成员按 score 值递增(从小到大)排列的排名。

* `ZREM key member [member ...]`

  删除名称为 key 的 zset 中的元素 member

* `ZINCRBY key increment member`

  为有序集 key 的成员`member`的`score`值加上增量`increment`。

  如果`key`中不存在`member`，就在 key 中添加一个`member`，`score`是`increment`（就好像它之前的 score 是 0.0）。

  如果`key`不存在，就创建一个只含有指定`member`成员的有序集合

  1. 当 key 不是有序集类型时，返回一个错误
  2. score 值必须是字符串表示的整数值或双精度浮点数，并且能接受 double 精度的浮点数。也有可能给一个负数来减少 score 的值

### 哈希(Hashes): key -> {k1:v1, k2:v2, ...}

> 以`H`打头
> 比较适合保存对象，结构体。如 use:001 -> {name: "name", age: 1}

* `HSET key field value`

  设置`key`指定的哈希集中指定字段的值。

  如果`key`指定的哈希集不存在，会创建一个新的哈希集并与`key`关联。

  如果字段在哈希集中存在，它将被重写。

* `HMSET key field value [field value ...]`

  设置 `key` 指定的哈希集中指定字段的值。该命令将重写所有在哈希集中存在的字段。
  如果 `key` 指定的哈希集不存在，会创建一个新的哈希集并与 key 关联

* `HSETNX key field value`

  只在 `key` 指定的**哈希集中不存在指定的字段**时，设置字段的值。
  如果 `key` 指定的**哈希集**不存在，会创建一个新的哈希集并与 key 关联。如果字段已存在，该操作无效果。

* HGETALL: `HGETALL key`

  返回 key 指定的哈希集中所有的字段和值。返回值中，每个字段名的下一个是它的值，所以返回值的长度是哈希集大小的两倍 

* `HGET key field`

  返回 key 指定的哈希集中该字段所关联的值

* `HMGET key field [field ...]`

  返回 key 指定的哈希集中指定字段的值。

  对于哈希集中不存在的每个字段，返回 nil 值。

* `HEXISTS key field`

  返回 `hash` 里面 `field` 是否存在

* `HLEN key`

  返回 `key` 指定的哈希集包含的字段的数量。

* HKEYS: `HKEYS key`

* HVALS: `HVALS key`

* HDEL: `HDEL key field [field ...]`

  从 `key` 指定的哈希集中移除指定的域。在哈希集中不存在的域将被忽略。

  如果 `key` 指定的哈希集不存在，它将被认为是一个空的哈希集，该命令将返回 0。

* HINCRBY: `HINCRBY key field increment`

  增加`key`指定的哈希集中指定字段的数值。如果 key 不存在，会创建一个新的哈希集并与 key 关联。如果字段不存在，则字段的值在该操作执行前被设置为 0。`HINCRBY` 支持的值的范围限定在 64 位 有符号整数

* HINCRBYFLOAT: `HINCRBYFLOAT key field increment`

  为指定 key 的 hash 的 field 字段值执行 float 类型的 increment 加。如果 field 不存在，则在执行该操作前设置为 0。

  如果出现下列情况之一，则返回错误：

  1. field 的值包含的类型错误(不是字符串)。
  2. 当前 field 或者 increment 不能解析为 float 类型。

### 持久化

Redis 提供了不同级别的持久化方式:

* RDB持久化方式能够在指定的时间间隔能对你的数据进行快照存储.
* AOF持久化方式记录每次对服务器写的操作,当服务器重启的时候会重新执行这些命令来恢复原始的数据,AOF命令以redis协议追加保存每次写的操作到文件末尾.Redis还能对AOF文件进行后台重写,使得AOF文件的体积不至于过大.
* 如果你只希望你的数据在服务器运行的时候存在,你也可以不使用任何持久化方式.
* 你也可以同时开启两种持久化方式, 在这种情况下, 当redis重启的时候会优先载入AOF文件来恢复原始的数据,因为在通常情况下AOF文件保存的数据集要比RDB文件保存的数据集要完整.
* 最重要的事情是了解RDB和AOF持久化方式的不同,让我们以RDB持久化方式开始:

## 二级目录

1. This is a list item with two paragraphs. Lorem ipsum dolor
    sit amet, consectetuer adipiscing elit. Aliquam hendrerit
    mi posuere lectus.

    Vestibulum enim wisi, viverra nec, fringilla in, laoreet
    vitae, risus. Donec sit amet nisl. Aliquam semper ipsum
    sit amet velit.

2. Suspendisse id sem consectetuer libero luctus adipiscing.

* This is a list item with two paragraphs. Lorem ipsum dolor

  sit amet, consectetuer adipiscing elit. Aliquam hendrerit
  mi posuere lectus.

  Vestibulum enim wisi, viverra nec, fringilla in, laoreet
  vitae, risus. Donec sit amet nisl. Aliquam semper ipsum
  sit amet velit.

* Suspendisse id sem consectetuer libero luctus adipiscing.

