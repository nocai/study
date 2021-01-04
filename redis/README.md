# Redis 手记

<!-- vim-markdown-toc GFM -->

* [简介](#简介)
  
  * [主要词条](#主要词条)ts
  
  Rtratsrafftsratsrattsasrararratsr〔方案選單〕 ttttsa
* [Redis 命令](#redis-命令)
  * [通用命令](#通用命令)
  * [Redis Key 命令](#redis-key-命令)
  * [字符串命令(Strings): key -> string](#字符串命令strings-key---string)
    * [APPEND: `APPEND key value`](#append-append-key-value)
    * [BITCOUNT: `BITCOUNT key [start end]`](#bitcount-bitcount-key-start-end)
    * [BITFIELD: `BITFIELD key [GET type offset] [SET type offset value] [INCRBY type offset increment] [OVERFLOW WRAP|SAT|FAIL]`](#bitfield-bitfield-key-get-type-offset-set-type-offset-value-incrby-type-offset-increment-overflow-wrapsatfail)
    * [BITOP: `BITOP operation destkey key [key ...]`](#bitop-bitop-operation-destkey-key-key-)
    * [BITPOS: `BITPOS key bit [start] [end]`](#bitpos-bitpos-key-bit-start-end)
    * [INCR: `INCR key`](#incr-incr-key)
    * [INCRBY: `INCRBY key increment`](#incrby-incrby-key-increment)
    * [INCRBYFLOAT: `INCRBYFLOAT key increment`](#incrbyfloat-incrbyfloat-key-increment)
    * [DECR: `DECR key`](#decr-decr-key)
    * [DECRBY: `DECRBY key decrement`](#decrby-decrby-key-decrement)
    * [GET: GET key](#get-get-key)
    * [MGET: `MGET key [key ...]`](#mget-mget-key-key-)
    * [GETBIT: `GETBIT key offset`](#getbit-getbit-key-offset)
    * [GETRANGE: `GETRANGE key start end`](#getrange-getrange-key-start-end)
    * [GETSET: GETSET key value](#getset-getset-key-value)
    * [SET: `SET key value [EX seconds] [PX milliseconds] [NX|XX]`](#set-set-key-value-ex-seconds-px-milliseconds-nxxx)
      * [选项](#选项)
    * [MSET: `MSET key value [key value ...]`](#mset-mset-key-value-key-value-)
    * [MSETNX: `MSETNX key value [key value ...]`](#msetnx-msetnx-key-value-key-value-)
    * [SETBIT: `SETBIT key offset value`](#setbit-setbit-key-offset-value)
    * [SETEX: `SETEX key seconds value`](#setex-setex-key-seconds-value)
    * [PSETEX: `PSETEX key milliseconds value`](#psetex-psetex-key-milliseconds-value)
    * [SETNX: `SETNX key value`](#setnx-setnx-key-value)
    * [SETRANGE: `SETRANGE key offset value`](#setrange-setrange-key-offset-value)
    * [STRLEN: `STRLEN key`](#strlen-strlen-key)
  * [列表命令(Lists，有序，可重复): key -> [k1, k1, k2, k3, ...]](#列表命令lists有序可重复-key---k1-k1-k2-k3-)
    * [BRPOP: `BRPOP key [key ...] timeout`](#brpop-brpop-key-key--timeout)
    * [BRPOPLPUSH: `BRPOPLPUSH source destination timeout`](#brpoplpush-brpoplpush-source-destination-timeout)
    * [LINDEX: `LINDEX key index`](#lindex-lindex-key-index)
    * [LINSERT: `LINSERT key BEFORE|AFTER pivot value`](#linsert-linsert-key-beforeafter-pivot-value)
    * [LLEN: `LLEN key`](#llen-llen-key)
    * [LPOP: `LPOP key`](#lpop-lpop-key)
    * [LPUSH: `LPUSH key value [value ...]`](#lpush-lpush-key-value-value-)
    * [LPUSHX: `LPUSHX key value`](#lpushx-lpushx-key-value)
    * [LRANGE: `LRANGE key start stop`](#lrange-lrange-key-start-stop)
      * [在不同编程语言里，关于求范围函数的一致性](#在不同编程语言里关于求范围函数的一致性)
      * [超过范围的下标](#超过范围的下标)
    * [LREM: `LREM key count value`](#lrem-lrem-key-count-value)
    * [LSET: `LSET key index value`](#lset-lset-key-index-value)
    * [LTRIM: `LTRIM key start stop`](#ltrim-ltrim-key-start-stop)
    * [RPOP: `RPOP key`](#rpop-rpop-key)
    * [RPOPLPUSHP `RPOPLPUSH source destination`](#rpoplpushp-rpoplpush-source-destination)
    * [RPUSH: `RPUSH key value [value ...]`](#rpush-rpush-key-value-value-)
    * [RPUSHX: `RPUSHX key value`](#rpushx-rpushx-key-value)
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

> Redis 命令十分丰富，包括的命令组有 Cluster、Connection、Geo、Hashes、HyperLogLog、
> Keys、Lists、Pub/Sub、Scripting、Server、Sets、Sorted Sets、Strings、Transactions
> 一共 14 个 redis 命令组两百多个 redis 命令。

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

- ping: 检测 redis 服务是否启动
- quit:关闭连接（connection）
- auth:简单的密码认证
- flushdb：删除当前选择数据库中的所有 key
- flushall：删除所有数据库中的所有 key

### Redis Key 命令

* AUTH: `AUTH password`

- BGREWRITEAOF: `BGREWRITEAOF`

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

- PEXPIRE: `PEXPIRE key milliseconds`

  这个命令和 `EXPIRE` 命令的作用类似，但是它以毫秒为单位设置 key 的生存时间，而不像 `EXPIRE` 命令那样，以秒为单位。

* EXPIREAT: `EXPIREAT key timestamp`

  EXPIREAT 的作用和 `EXPIRE`类似，都用于为`key`设置生存时间。不同在于`EXPIREAT`命令接受的时间参数是 UNIX 时间戳 Unix timestamp 。

* PEXPIREAT: `PEXPIREAT key milliseconds-timestamp`
  
  PEXPIREAT 这个命令和 `EXPIREAT` 命令类似，但它以毫秒为单位设置 `key` 的过期 unix 时间戳，而不是像 `EXPIREAT` 那样，以秒为单位。

* PERSIST: `PERSIST key`

  移除给定 `key` 的生存时间，将这个  `key`  从**『易失的』**(带生存时间 key )转换成『持久的』(一个不带生存时间、永不过期的 key )。

* KEYS: `KEYS pattern`

  查找所有符合给定模式pattern（正则表达式）的 key 。 时间复杂度为O(N)，N为数据库里面key的数量。

  **警告: KEYS 的速度非常快，但在一个大的数据库中使用它仍然可能造成性能问题，如果你需要从一个数据集中查找特定的 KEYS， 你最好还是用 Redis 的集合结构 SETS 来代替。**

  支持的正则表达模式：
  1. h?llo 匹配 hello, hallo 和 hxllo
  2. h*llo 匹配 hllo 和 heeeello
  3. h[ae]llo 匹配 hello 和 hallo, 但是不匹配 hillo
  4. h[^e]llo 匹配 hallo, hbllo, … 但是不匹配 hello
  5. h[a-b]llo 匹配 hallo 和 hbllo
  
  如果你想取消字符的特殊匹配（正则表达式，可以在它的前面加\。
  
* TTL: `TTL key`

  返回key剩余的过期时间。 这种反射能力允许Redis客户端检查指定key在数据集里面剩余的有效期。
  
  在Redis 2.6和之前版本，如果key不存在或者已过期时返回-1。
  
  从Redis2.8开始，错误返回值的结果有如下改变：
    1. 如果key不存在或者已过期，返回 -2
    2. 如果key存在并且没有设置过期时间（永久有效），返回 -1 。

* PTTL: `PTTL key`

  这个命令类似于TTL命令，但它以毫秒为单位返回 key 的剩余生存时间，而不是像TTL命令那样，以秒为单位。
  
* DEL: DEL key [key ...]
  
  删除指定的一批keys，如果删除中的某些key不存在，则直接忽略。

- MOVE: `MOVE key db`
  
  将当前数据库的 key 移动到给定的数据库 db 当中。 如果当前数据库(源数据库)和给定数据库(目标数据库)有相同名字的给定 key ，或者 key 不存在于当前数据库，那么 MOVE 没有任何效果。

  **因此，也可以利用这一特性，将 MOVE 当作锁(locking)原语(primitive)。**

* TYPE: `TYPE key`
  
  返回key所存储的value的数据结构类型，它可以返回string, list, set, zset 和 hash等不同的类型。
  
* RANDOMKEY: `RANDOMKEY`
  
  从当前数据库返回一个随机的key。

- EXISTS: `EXISTS key [key ...]`
  
  返回key是否存在

---

- RENAME: `RENAME key newkey`
  
  将key重命名为newkey，如果key与newkey相同，将返回一个错误。如果newkey已经存在，则值将被覆盖。
  
- RENAME: `RENAMENX key newkey`

  当且仅当 newkey 不存在时，将 key 改名为 newkey。当 key 不存在时，返回一个错误。

---

* dbsize：返回当前数据库中 key 的数目
* select(index)：按索引查询

- MIGRATE: `MIGRATE host port key destination-db timeout [COPY] [REPLACE]`

  将`key`原子性地从当前实例传送到目标实例的指定数据库上，一旦传送成功，`key`保证会出现在目标实例上，而当前实例上的`key`会被删除。

  这个命令是一个原子操作，它在执行的时候会阻塞进行迁移的两个实例，直到以下任意结果发生：**迁移成功，迁移失败**，等到超时。
  
  命令的内部实现是这样的：它在当前实例对给定`key`执行`DUMP`命令 ，将它序列化，然后传送到目标实例，目标实例再使用`RESTORE`对数据进行反序列化，并将反序列化所得的数据添加到数据库中；当前实例就像目标实例的客户端那样，只要看到`RESTORE`命令返回`OK`，它就会调用`DEL`删除自己数据库上的`key`。
  
  **timeout 参数以毫秒为格式**，指定当前实例和目标实例进行沟通的最大间隔时间。这说明操作并不一定要在`timeout`毫秒内完成，只是说数据传送的时间不能超过这个`timeout`数。
  
  MIGRATE 命令需要在给定的时间规定内完成`IO`操作。如果在传送数据时发生`IO`错误，或者达到了超时时间，那么命令会停止执行，并返回一个特殊的错误：`IOERR` 。

  当 IOERR 出现时，有以下两种可能：
  - key 可能存在于两个实例。
  - key 可能只存在于当前实例。
  
  **唯一不可能发生的情况就是丢失`key`**，因此，如果一个客户端执行 MIGRATE, 命令，并且不幸遇上 IOERR 错误，那么这个客户端唯一要做的就是检查自己数据库上的 key 是否已经被正确地删除。
  如果有其他错误发生，那么 MIGRATE 保证 key 只会出现在当前实例中。（当然，目标实例的给定数据库上可能有和 key 同名的键，不过这和 MIGRATE 命令没有关系）。

- OBJECT: `OBJECT subcommand [arguments [arguments ...]]`

  OBJECT 命令可以在内部调试(debugging)给出keys的内部对象，它用于检查或者了解你的keys是否用到了特殊编码 的数据类型来存储空间z。 当redis作为缓存使用的时候，你的应用也可能用到这些由OBJECT命令提供的信息来决定应用层的key的驱逐策略(eviction policies)
  
  OBJECT 支持多个子命令:
  - `OBJECT REFCOUNT` 该命令主要用于调试(debugging)，它能够返回指定key所对应value被引用的次数.
  - `OBJECT ENCODING` 该命令返回指定key对应value所使用的内部表示(representation)(译者注：也可以理解为数据的压缩方式).
  - `OBJECT IDLETIME` 该命令返回指定key对应的value自被存储之后空闲的时间，以秒为单位(没有读写操作的请求) ，这个值返回以10秒为单位的秒级别时间，这一点可能在以后的实现中改善
  
  对象可以用多种方式编码:
  - 字符串可以被编码为 raw (常规字符串) 或者int (用字符串表示64位无符号整数这种编码方式是为了节省空间).
  - 列表类型可以被编码为ziplist 或者 linkedlist. ziplist 是为了节省较小的列表空间而设计一种特殊编码方式.
  - 集合被编码为 intset 或者 hashtable. intset 是为了存储数字的较小集合而设计的一种特殊编码方式.
  - 哈希表可以被编码为 zipmap 或者hashtable. zipmap 是专为了较小的哈希表而设计的一种特殊编码方式
  - 有序集合被编码为ziplist 或者 skiplist 格式. ziplist可以表示较小的有序集合, skiplist 表示任意大小多的有序集合.

  一旦你做了一个操作让redis无法再使用那些节省空间的编码方式，它将自动将那些特殊的编码类型转换为普通的编码类型.

  ```bash
  redis> lpush mylist "Hello World"
  (integer) 4
  redis> object refcount mylist
  (integer) 1
  redis> object encoding mylist
  "ziplist"
  redis> object idletime mylist
  (integer) 10
  ```
  
  接下来的例子你可以看到redis一旦不能够实用节省空间的编码类型时编码方式的改变

  ```bash
  redis> set foo 1000
  OK
  redis> object encoding foo
  "int"
  redis> append foo bar
  (integer) 7
  redis> get foo
  "1000bar"
  redis> object encoding foo
  "raw"
  ```
  
- RESTORE: `RESTORE key ttl serialized-value [REPLACE]`
  
  反序列化给定的序列化值，并将它和给定的 `key` 关联。
  参数 `ttl` 以毫秒为单位为 key 设置生存时间；如果 `ttl` 为 0 ，那么不设置生存时间。
  `RESTORE` 在执行反序列化之前会先对序列化值的 `RDB` 版本和数据校验和进行检查，如果 `RDB` 版本不相同或者数据不完整的话，那么 `RESTORE` 会拒绝进行反序列化，并返回一个错误。

### 字符串命令(Strings): key -> string

二进制安全的字符串

> 一些通用命令前以 m 打头的命令为批量操作(mset, mget, msetnx)

#### APPEND: `APPEND key value`

如果`key`已经存在，并且值为字符串，那么这个命令会把`value`追加到原来值（value）的结尾。 如果`key`不存在，那么它将首先创建一个空字符串的`key`，再执行追加操作，这种情况`APPEND`将类似于`SET`操作。

#### BITCOUNT: `BITCOUNT key [start end]`

统计字符串被设置为1的bit数。  
一般情况下，给定的整个字符串都会被进行计数，通过指定额外的 `start` 或 `end` 参数，可以让计数只在特定的位上进行。
`start` 和 `end` 参数的设置和 `GETRANGE` 命令类似，都可以使用负数值：比如 `-1` 表示最后一个位，而 `-2` 表示倒数第二个位，以此类推。
不存在的 `key` 被当成是空字符串来处理，因此对一个不存在的 `key` 进行 `BITCOUNT` 操作，结果为 `0` 。

```bash
127.0.0.1:6379> set k foo
OK
127.0.0.1:6379> BITCOUNT k 0 -1
(integer) 16
127.0.0.1:6379> BITCOUNT k 0 0
(integer) 4
127.0.0.1:6379> BITCOUNT k 1 1
(integer) 6
127.0.0.1:6379>
```

#### BITFIELD: `BITFIELD key [GET type offset] [SET type offset value] [INCRBY type offset increment] [OVERFLOW WRAP|SAT|FAIL]`

用法见官方文档。

#### BITOP: `BITOP operation destkey key [key ...]`

用法见官方文档。

#### BITPOS: `BITPOS key bit [start] [end]`

用法见官方文档。

#### INCR: `INCR key`

对存储在指定key的数值执行原子的加1操作。

- 如果指定的key不存在，那么在执行incr操作之前，会先将它的值设定为0。
- 如果指定的key中存储的值不是字符串类型，或者存储的字符串类型不能表示为一个整数，那个执行这个命令时，服务器会返回一个错误。

这个操作仅限于64位的有符号整形数据。

**注意: 由于redis并没有一个明确的类型来表示整型数据，所以这个操作是一个字符串操作。
执行这个操作的时候，key对应存储的字符串被解析为10进制的64位有符号整型数据。
事实上，Redis 内部采用整数形式（Integer representation）来存储对应的整数值，所以对该类字符串值实际上是用整数保存，也就不存在存储整数的字符串表示（String representation）所带来的额外消耗。**

#### INCRBY: `INCRBY key increment`

将`key`对应的数字加`decrement`。如果`key`不存在，操作之前，`key`就会被置为`0`。如果`key`的`value`类型错误或者是个不能表示成数字的字符串，就返回错误。这个操作最多支持64位有符号的正型数字。

#### INCRBYFLOAT: `INCRBYFLOAT key increment`

通过指定浮点数key来增长浮点数(存放于string中)的值. 当键不存在时,先将其值设为0再操作.下面任一情况都会返回错误:

- key 包含非法值(不是一个string).
- 当前的key或者相加后的值不能解析为一个双精度的浮点值.(超出精度范围了)

如果操作命令成功, 相加后的值将替换原值存储在对应的键值上, 并以string的类型返回. string中已存的值或者相加参数可以任意选用指数符号,但相加计算的结果会以科学计数法的格式存储. 无论各计算的内部精度如何, 输出精度都固定为小数点后17位.

#### DECR: `DECR key`

对key对应的数字做减1操作。如果key不存在，那么在操作之前，这个key对应的值会被置为0。

如果key有一个错误类型的value或者是一个不能表示成数字的字符串，就返回错误。这个操作最大支持在64位有符号的整型数字。

查看命令INCR了解关于增减操作的额外信息。

#### DECRBY: `DECRBY key decrement`

将 `key` 对应的数字减decrement。如果 `key` 不存在，操作之前，`key` 就会被置为0。

如果 `key` 的 `value` **类型错误或者是个不能表示成数字的字符串**，就返回错误。这个操作**最多支持64位有符号的正型数字**。

查看命令INCR了解关于增减操作的额外信息。似。

#### GET: GET key

返回`key`的`value`。如果`key`不存在，返回特殊值`nil`。**如果`key`的`value`不是`string`，就返回错误，因为`GET`只处理`string`类型的`values`**。

#### MGET: `MGET key [key ...]`

返回所有指定的key的value。对于每个不对应string或者不存在的key，都返回特殊值nil。正因为此，这个操作从来不会失败。

#### GETBIT: `GETBIT key offset`

返回`key`对应的`string`在`offset`处的bit值 当`offset`超出了字符串长度的时候，这个字符串就被假定为由`0`比特填充的连续空间。当`key`不存在的时候，它就认为是一个空字符串，所以`offset`总是超出范围，然后`value`也被认为是由`0`比特填充的连续空间。到内存分配。

#### GETRANGE: `GETRANGE key start end`

警告：这个命令是被改成	GETRANGE	的，在小于2.0的Redis版本中叫`SUBSTR`。 返回`key`对应的字符串`value`的子串，这个子串是由`start`和`end`位移决定的（两者都在string内）。可以用负的位移来表示从string尾部开始数的下标。所以`-1`就是最后一个字符，`-2`就是倒数第二个，以此类推。
这个函数处理超出范围的请求时，都把结果限制在string内。

#### GETSET: GETSET key value

自动将key对应到value并且返回原来key对应的value。如果key存在但是对应的value不是字符串，就返回错误。

设计模式

`GETSET`可以和`INCR`一起使用实现支持重置的计数功能。举个例子：每当有事件发生的时候，一段程序都会调用`INCR`给key mycounter加1，但是有时我们需要获取计数器的值，并且自动将其重置为0。这可以通过`GETSET mycounter “0”`来实现：

#### SET: `SET key value [EX seconds] [PX milliseconds] [NX|XX]`

将键key设定为指定的“字符串”值。
如果 key 已经保存了一个值，那么这个操作会直接覆盖原来的值，并且忽略原始类型。
当set命令执行成功之后，之前设置的过期时间都将失效。

##### 选项

从2.6.12版本开始，redis为SET命令增加了一系列选项:

- EX seconds – Set the specified expire time, in seconds.
- PX milliseconds – Set the specified expire time, in milliseconds.
- NX – Only set the key if it does not already exist.
- XX – Only set the key if it already exist.
- EX seconds – 设置键key的过期时间，单位时秒
- PX milliseconds – 设置键key的过期时间，单位时毫秒
- NX – 只有键key不存在的时候才会设置key的值
- XX – 只有键key存在的时候才会设置key的值

注意: 由于SET命令加上选项已经可以完全取代`SETNX`, `SETEX`, `PSETEX`的功能，所以在将来的版本中，redis可能会不推荐使用并且最终抛弃这几个命令。

#### MSET: `MSET key value [key value ...]`

对应给定的keys到他们相应的values上。MSET会用新的value替换已经存在的value，就像普通的SET命令一样。如果你不想覆盖已经存在的values，请参看命令`MSETNX`。

**MSET是原子的，所以所有给定的keys是一次性set的。客户端不可能看到这种一部分keys被更新而另外的没有改变的情况。**

#### MSETNX: `MSETNX key value [key value ...]`

对应给定的keys到他们相应的values上。**只要有一个key已经存在，MSETNX一个操作都不会执行**。 由于这种特性，MSETNX可以实现要么所有的操作都成功，要么一个都不执行，这样可以用来设置不同的key，来表示一个唯一的对象的不同字段。

**MSETNX是原子的，所以所有给定的keys是一次性set的。客户端不可能看到这种一部分keys被更新而另外的没有改变的情况。**

#### SETBIT: `SETBIT key offset value`

设置或者清空key的value(字符串)在offset处的bit值。

那个位置的bit要么被设置，要么被清空，这个由value（只能是0或者1）来决定。当key不存在的时候，就创建一个新的字符串value。要确保这个字符串大到在offset处有bit值。参数offset需要大于等于0，并且小于232(限制bitmap大小为512)。当key对应的字符串增大的时候，新增的部分bit值都是设置为0。

#### SETEX: `SETEX key seconds value`

设置key对应字符串value，并且设置key在给定的seconds时间之后超时过期。这个命令等效于执行下面的命令：

```bash
SET mykey value
EXPIRE mykey seconds
```

SETEX是原子的，也可以通过把上面两个命令放到`MULTI/EXEC`块中执行的方式重现。相比连续执行上面两个命令，它更快，因为当Redis当做缓存使用时，这个操作更加常用。

#### PSETEX: `PSETEX key milliseconds value`

PSETEX和`SETEX`一样，唯一的区别是到期时间以毫秒为单位,而不是秒。

#### SETNX: `SETNX key value`

将key设置值为value，如果key不存在，这种情况下等同SET命令。 当key存在时，什么也不做。SETNX是”SET if Not eXists”的简写。

#### SETRANGE: `SETRANGE key offset value`

这个命令的作用是覆盖key对应的string的一部分，从指定的offset处开始，覆盖value的长度。如果offset比当前key对应string还要长，那这个string后面就补0以达到offset。不存在的keys被认为是空字符串，所以这个命令可以确保key有一个足够大的字符串，能在offset处设置value。

**注意，offset最大可以是229-1(536870911),因为redis字符串限制在512M大小。如果你需要超过这个大小，你可以用多个keys。**

#### STRLEN: `STRLEN key`

返回key的string类型value的长度。如果key对应的非string类型，就返回错误。

### 列表命令(Lists，有序，可重复): key -> [k1, k1, k2, k3, ...]

> Lists 命令大部分以`L`打头。如(lset, llen)
> 队列(先进先出)：lpush 左进+rpop 右出 / rpush 右进+lpop 左出
> 栈(先进后出)：lpush 左进+lpop 左出 / rpush 右进+rpop 右出

- BLPOP: `BLPOP key [key ...] timeout`

BLPOP是阻塞式列表的弹出原语。它是命令`LPOP`的阻塞版本，这是因为当给定列表内没有任何元素可供弹出的时候， 连接将被`BLPOP`命令阻塞。 当给定多个`key`参数时，按参数`key`的先后顺序依次检查各个列表，弹出第一个非空列表的头元素。

非阻塞行为

当`BLPOP`被调用时，如果给定`key`内至少有一个非空列表，那么弹出遇到的第一个非空列表的头元素，并和被弹出元素所属的列表的名字`key`一起，组成结果返回给调用者。

当存在多个给定`key`时，`BLPOP`按给定`key`参数排列的先后顺序，依次检查各个列表。 我们假设`key list1`不存在，而`list2`和`list3`都是非空列表。考虑以下的命令：

```bash
BLPOP list1 list2 list3 0
```

BLPOP保证返回一个存在于list2里的元素（因为它是从 list1 –> list2 –> list3 这个顺序查起的第一个非空列表）。

阻塞行为

如果所有给定`key`都不存在或包含空列表，那么`BLPOP`命令将阻塞连接， 直到有另一个客户端对给定的这些`key`的任意一个执行`LPUSH`或`RPUSH`命令为止。

一旦有新的数据出现在其中一个列表里，那么这个命令会解除阻塞状态，并且返回`key`和弹出的元素值。
当`BLPOP`命令引起客户端阻塞并且设置了一个非零的超时参数`timeout`的时候，若经过了指定的`timeout` 仍没有出现一个针对某一特定`key`的`push`操作，则客户端会解除阻塞状态并且返回一个 `nil` 的多组合值(multi-bulk value)。

**timeout 参数表示的是一个指定阻塞的最大秒数的整型值。当 timeout 为 0 是表示阻塞时间无限制。**

什么 key 会先被处理？是什么客户端？什么元素？优先顺序细节。

- 当客户端为多个 key 尝试阻塞的时候，若至少存在一个 key 拥有元素，那么返回的键值对(key/element pair)就是从左到右数第一个拥有一个或多个元素的key。 在这种情况下客户端不会被阻塞。比如对于这个例子 BLPOP key1 key2 key3 key4 0，假设 key2 和 key4 都非空， 那么就会返回 key2 里的一个元素。

- 当多个客户端为同一个 key 阻塞的时候，第一个被处理的客户端是等待最长时间的那个（即第一个因为该key而阻塞的客户端）。 一旦一个客户端解除阻塞那么它就不会保持任何优先级，当它因为下一个 BLPOP 命令而再次被阻塞的时候，会在处理完那些 被同个 key 阻塞的客户端后才处理它（即从第一个被阻塞的处理到最后一个被阻塞的）。
- 当一个客户端同时被多个 key 阻塞时，若多个 key 的元素同时可用（可能是因为事务或者某个Lua脚本向多个list添加元素）， 那么客户端会解除阻塞，并使用第一个接收到 push 操作的 key（假设它拥有足够的元素为我们的客户端服务，因为有可能存在其他客户端同样是被这个key阻塞着）。 从根本上来说，在执行完每个命令之后，Redis 会把一个所有 key 都获得数据并且至少使一个客户端阻塞了的 list 运行一次。 这个 list 按照新数据的接收时间进行整理，即是从第一个接收数据的 key 到最后一个。在处理每个 key 的时候，只要这个 key 里有元素， Redis就会对所有等待这个key的客户端按照“先进先出”(FIFO)的顺序进行服务。若这个 key 是空的，或者没有客户端在等待这个 key， 那么将会去处理下一个从之前的命令或事务或脚本中获得新数据的 key，如此等等。

当多个元素被 push 进入一个 list 时 BLPOP 的行为

有时候一个 list 会在同一概念的命令的情况下接收到多个元素：

- 像 LPUSH mylist a b c 这样的可变 push 操作。
- 在对一个向同一个 list 进行多次 push 操作的 MULTI 块执行完 EXEC 语句后。
- 使用 Redis 2.6 或者更新的版本执行一个 Lua 脚本。

当多个元素被 push 进入一个被客户端阻塞着的 list 的时候，Redis 2.4 和 Redis 2.6 或者更新的版本所采取行为是不一样的。

对于 Redis 2.6 来说，所采取的行为是先执行多个 push 命令，然后在执行了这个命令之后再去服务被阻塞的客户端。看看下面命令顺序。

```bash
Client A:   BLPOP foo 0
Client B:   LPUSH foo a b c
```

如果上面的情况是发生在 Redis 2.6 或更高版本的服务器上，客户端 A 会接收到 c 元素，因为在 LPUSH 命令执行后，list 包含了 c,b,a 这三个元素，所以从左边取一个元素就会返回 c。

相反，Redis 2.4 是以不同的方式工作的：客户端会在 push 操作的上下文中被服务，所以当 LPUSH foo a b c 开始往 list 中 push 第一个元素，它就被传送给客户端A，也就是客户端A会接收到 a（第一个被 push 的元素）。

Redis 2.4的这种行为会在复制或者持续把数据存入AOF文件的时候引发很多问题，所以为了防止这些问题，很多更一般性的、并且在语义上更简单的行为被引入到 Redis 2.6 中。

需要注意的是，一个Lua脚本或者一个 MULTI / EXEC 块可能会 push 一堆元素进入一个 list 后，再 删除这个 list。 在这种情况下，被阻塞的客户端完全不会被服务，并且只要在执行某个单一命令、事务或者脚本后 list 中没有出现元素，它就会被继续阻塞下去。

在一个 MULTI / EXEC 事务中的 BLPOP

BLPOP 可以用于流水线（pipeline，发送多个命令并且批量读取回复），特别是当它是流水线里的最后一个命令的时候，这种设定更加有意义。

在一个 MULTI / EXEC 块里面使用 BLPOP 并没有很大意义，因为它要求整个服务器被阻塞以保证块执行时的原子性，这就阻止了其他客户端执行一个 push 操作。 因此，一个在 MULTI / EXEC 里面的 BLPOP 命令会在 list 为空的时候返回一个 nil 值，这跟超时(timeout)的时候发生的一样。

如果你喜欢科幻小说，那么想象一下时间是以无限的速度在 MULTI / EXEC 块中流逝……

#### BRPOP: `BRPOP key [key ...] timeout`

BRPOP 是一个阻塞的列表弹出原语。 它是 RPOP 的阻塞版本，因为这个命令会在给定list无法弹出任何元素的时候阻塞连接。 该命令会按照给出的 key 顺序查看 list，并在找到的第一个非空 list 的尾部弹出一个元素。

请在 BLPOP 文档 中查看该命令的准确语义，因为 BRPOP 和 BLPOP 基本是完全一样的，除了它们一个是从尾部弹出元素，而另一个是从头部弹出元素。

#### BRPOPLPUSH: `BRPOPLPUSH source destination timeout`

BRPOPLPUSH是`RPOPLPUSH`的阻塞版本。 当`source`包含元素的时候，这个命令表现得跟`RPOPLPUSH`一模一样。 当`source`是空的时候，Redis将会阻塞这个连接，直到另一个客户端`push`元素进入或者达到 timeout 时限。 timeout 为 0 能用于无限期阻塞客户端。

查看`RPOPLPUSH`以了解更多信息。

#### LINDEX: `LINDEX key index`

返回列表里的元素的索引`index`存储在`key`里面。下标是从0开始索引的，所以`0`是表示第一个元素，`1` 表示第二个元素，并以此类推。 负数索引用于指定从列表尾部开始索引的元素。在这种方法下，`-1` 表示最后一个元素，`-2` 表示倒数第二个元素，并以此往前推。

当`key`位置的值不是一个列表的时候，会返回一个error。

#### LINSERT: `LINSERT key BEFORE|AFTER pivot value`

把`value`插入存于`key`的列表中在基准值`pivot`的前面或后面。

当 key 不存在时，这个list会被看作是空list，任何操作都不会发生。

当 key 存在，但保存的不是一个list的时候，会返回error。

#### LLEN: `LLEN key`

返回存储在`key`里的`list`的长度。 如果`key`不存在，那么就被看作是空`list`，并且返回长度为`0`。 当存储在`key`里的值不是一个`list`的话，会返回error。

#### LPOP: `LPOP key`

移除并且返回 key 对应的 list 的第一个元素。

#### LPUSH: `LPUSH key value [value ...]`

将所有指定的值插入到存于`key`的列表的头部。如果`key`不存在，那么在进行`push`操作前会创建一个空列表。如果`key`对应的值不是一个`list`的话，那么会返回一个错误。

可以使用一个命令把多个元素`push`进入列表，只需在命令末尾加上多个指定的参数。元素是从最左端的到最右端的、一个接一个被插入到`list`的头部。所以对于这个命令例子`LPUSH mylist a b c`，返回的列表是`c`为第一个元素，`b`为第二个元素，`a`为第三个元素。

#### LPUSHX: `LPUSHX key value`

只有当`key`已经存在并且存着一个`list`的时候，在这个`key`下面的`list`的头部插入`value`。 与`LPUSH` 相反，当`key`不存在的时候不会进行任何操作。

#### LRANGE: `LRANGE key start stop`

返回存储在`key`的列表里指定范围内的元素。`start`和`end`偏移量都是基于`0`的下标，即list的第一个元素下标是`0`（list的表头），第二个元素下标是`1`，以此类推。

**偏移量也可以是负数，表示偏移量是从list尾部开始计数**。例如，`-1`表示列表的最后一个元素，`-2` 是倒数第二个，以此类推。

##### 在不同编程语言里，关于求范围函数的一致性

需要注意的是，如果你有一个list，里面的元素是从0到100，那么 LRANGE list 0 10 这个命令会返回11个元素，即最右边的那个元素也会被包含在内。 在你所使用的编程语言里，这一点可能是也可能不是跟那些求范围有关的函数都是一致的。（像Ruby的 Range.new，Array#slice 或者Python的 range() 函数。）

##### 超过范围的下标

当下标超过list范围的时候不会产生error。 如果start比list的尾部下标大的时候，会返回一个空列表。 如果stop比list的实际尾部大的时候，Redis会当它是最后一个元素的下标。

#### LREM: `LREM key count value`

从存于`key`的列表里移除前`count`次出现的值为`value`的元素。 这个`count`参数通过下面几种方式影响这个操作：

- `count > 0`: 从头往尾移除值为`value`的元素。
- `count < 0`: 从尾往头移除值为`value`的元素。
- `count = 0`: 移除所有值为`value`的元素。

比如，`LREM list -2` “hello” 会从存于 list 的列表里移除最后两个出现的 “hello”。

**需要注意的是，如果list里没有存在key就会被当作空list处理，所以当 key 不存在的时候，这个命令会返回 0。**

#### LSET: `LSET key index value`

设置`index`位置的`list`元素的值为`value`。更多关于`index`参数的信息，详见`LINDEX`。
**当index超出范围时会返回一个error**。

#### LTRIM: `LTRIM key start stop`

修剪(trim)一个已存在的 list，这样 list 就会只包含指定范围的指定元素。start 和 stop 都是由0开始计数的， 这里的 0 是列表里的第一个元素（表头），1 是第二个元素，以此类推。

例如： LTRIM foobar 0 2 将会对存储在 foobar 的列表进行修剪，只保留列表里的前3个元素。

`start`和`end`也可以用负数来表示与表尾的偏移量，比如`-1`表示列表里的最后一个元素，`-2`表示倒数第二个，等等。

超过范围的下标并不会产生错误：如果`start`超过列表尾部，或者`start > end`，结果会是列表变成空表（即该`key`会被移除）。 如果`end`超过列表尾部，Redis 会将其当作列表的最后一个元素。

```bash
LTRIM 的一个常见用法是和 LPUSH / RPUSH 一起使用。 例如：
LPUSH mylist someelement
LTRIM mylist 0 99
```

这一对命令会将一个新的元素 push 进列表里，并保证该列表不会增长到超过100个元素。这个是很有用的，比如当用 Redis 来存储日志。 需要特别注意的是，当用这种方式来使用 LTRIM 的时候，操作的复杂度是 O(1) ， 因为平均情况下，每次只有一个元素会被移除。

#### RPOP: `RPOP key`

移除并返回在于`key`的`list`的最后一个元素。

#### RPOPLPUSHP `RPOPLPUSH source destination`

原子性地返回并移除存储在`source`的列表的最后一个元素（列表尾部元素），并把该元素放入存储在`destination`的列表的第一个元素位置（列表头部）。

例如：假设`source`存储着列表`a,b,c`，`destination`存储着列表`x,y,z`。 执行`RPOPLPUSH`得到的结果是`source`保存着列表`a,b`，而`destination`保存着列表`c,x,y,z`。

如果`source`不存在，那么会返回`nil`值，并且不会执行任何操作。 如果`source`和`destination`是同样的，那么这个操作等同于移除列表最后一个元素并且把该元素放在列表头部，**所以这个命令也可以当作是一个旋转列表的命令**。

#### RPUSH: `RPUSH key value [value ...]`

向存于`key`的列表的尾部插入所有指定的值。如果`key`不存在，那么会创建一个空的列表然后再进行`push`操作。 当`key`保存的不是一个列表，那么会返回一个错误。

可以使用一个命令把多个元素打入队列，只需要在命令后面指定多个参数。元素是从左到右一个接一个从列表尾部插入。比如命令`RPUSH mylist a b c`会返回一个列表，其第一个元素是`a`，第二个元素是`b`，第三个元素是`c`。

#### RPUSHX: `RPUSHX key value`

将值`value`插入到列表`key`的表尾, **当且仅当`key`存在并且是一个列表**。**和`RPUSH`命令相反,`当`key`不存在时，RPUSHX`命令什么也不做**。

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
