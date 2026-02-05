# Redis Handbook

## Table of Contents
- [Basic Redis Commands](#basic-redis-commands)
- [Redis Data Structures](#redis-data-structures)
- [Redis Expiry](#redis-expiry)

---

## Basic Redis Commands

### Connection & Server Commands

```bash
# Connect to Redis server
redis-cli

# Connect to specific host and port
redis-cli -h localhost -p 6379

# Authenticate (if password is set)
AUTH password

# Ping server
PING
# Response: PONG

# Check server info
INFO

# Select database (0-15 by default)
SELECT 0

# Get current database size
DBSIZE

# Flush current database
FLUSHDB

# Flush all databases
FLUSHALL
```

### Key Management Commands

```bash
# Set a key
SET mykey "Hello"

# Get a key
GET mykey
# Response: "Hello"

# Check if key exists
EXISTS mykey
# Response: 1 (true) or 0 (false)

# Delete a key
DEL mykey

# Delete multiple keys
DEL key1 key2 key3

# Get all keys matching pattern
KEYS pattern
KEYS *          # All keys
KEYS user:*     # All keys starting with "user:"

# Rename a key
RENAME oldkey newkey

# Get key type
TYPE mykey
# Response: string, list, set, zset, hash, stream

# Get random key
RANDOMKEY

# Scan keys (cursor-based iteration)
SCAN 0 MATCH user:* COUNT 100
```

---

## Redis Data Structures

### 1. Strings
Strings are the most basic Redis data type, representing a sequence of bytes.

**Commands:**
```bash
# Set value
SET name "John"

# Set with expiry (seconds)
SETEX session:123 3600 "user_data"

# Set if not exists
SETNX mykey "value"

# Get value
GET name
# Response: "John"

# Set multiple keys
MSET key1 "value1" key2 "value2" key3 "value3"

# Get multiple keys
MGET key1 key2 key3

# Append to existing value
APPEND name " Doe"
GET name
# Response: "John Doe"

# Get string length
STRLEN name
# Response: 8

# Increment (for numeric strings)
SET counter 10
INCR counter        # Increment by 1
# Response: 11

INCRBY counter 5    # Increment by 5
# Response: 16

DECR counter        # Decrement by 1
DECRBY counter 3    # Decrement by 3

# Increment float
INCRBYFLOAT price 0.5

# Get substring
GETRANGE name 0 3
# Response: "John"

# Set range
SETRANGE name 0 "Jane"
GET name
# Response: "Jane Doe"
```

**Use Cases:**
- Session storage
- Caching HTML fragments
- Counters (page views, likes)
- Rate limiting

---

### 2. Lists
Ordered collections of strings, sorted by insertion order. Lists can act as stacks or queues.

**Commands:**
```bash
# Push to left (head)
LPUSH mylist "world"
LPUSH mylist "hello"

# Push to right (tail)
RPUSH mylist "!"

# Get list range
LRANGE mylist 0 -1
# Response: ["hello", "world", "!"]

# Get list length
LLEN mylist
# Response: 3

# Pop from left
LPOP mylist
# Response: "hello"

# Pop from right
RPOP mylist
# Response: "!"

# Get element at index
LINDEX mylist 0

# Set element at index
LSET mylist 0 "new_value"

# Insert before/after element
LINSERT mylist BEFORE "world" "beautiful"

# Remove elements
LREM mylist 2 "value"  # Remove first 2 occurrences
LREM mylist -1 "value" # Remove last occurrence
LREM mylist 0 "value"  # Remove all occurrences

# Trim list to specified range
LTRIM mylist 0 99      # Keep first 100 elements

# Blocking pop (wait for element)
BLPOP mylist 30        # Wait 30 seconds
BRPOP mylist 30
```

**Use Cases:**
- Message queues
- Activity feeds
- Latest N items (with LTRIM)
- Task lists

---

### 3. Sets
Unordered collection of unique strings.

**Commands:**
```bash
# Add members
SADD myset "apple"
SADD myset "banana" "orange"

# Get all members
SMEMBERS myset
# Response: ["apple", "banana", "orange"]

# Check membership
SISMEMBER myset "apple"
# Response: 1 (true)

# Get set size
SCARD myset
# Response: 3

# Remove member
SREM myset "banana"

# Pop random member
SPOP myset

# Get random member (without removing)
SRANDMEMBER myset
SRANDMEMBER myset 2    # Get 2 random members

# Move member between sets
SMOVE source_set dest_set "member"

# Set operations
SADD set1 "a" "b" "c"
SADD set2 "b" "c" "d"

# Union
SUNION set1 set2
# Response: ["a", "b", "c", "d"]

# Intersection
SINTER set1 set2
# Response: ["b", "c"]

# Difference
SDIFF set1 set2
# Response: ["a"]

# Store result
SUNIONSTORE result set1 set2
SINTERSTORE result set1 set2
SDIFFSTORE result set1 set2
```

**Use Cases:**
- Tags
- Unique visitor tracking
- Friend relationships
- IP blacklists

---

### 4. Sorted Sets (ZSets)
Similar to Sets but each member has a score used for sorting.

**Commands:**
```bash
# Add members with scores
ZADD leaderboard 100 "player1"
ZADD leaderboard 200 "player2" 150 "player3"

# Get range by rank (ascending)
ZRANGE leaderboard 0 -1
# Response: ["player1", "player3", "player2"]

# Get range with scores
ZRANGE leaderboard 0 -1 WITHSCORES

# Get range by rank (descending)
ZREVRANGE leaderboard 0 -1 WITHSCORES

# Get range by score
ZRANGEBYSCORE leaderboard 100 200
ZRANGEBYSCORE leaderboard -inf +inf  # All members

# Get count
ZCARD leaderboard
# Response: 3

# Get score
ZSCORE leaderboard "player1"
# Response: 100

# Get rank
ZRANK leaderboard "player1"    # Ascending rank (0-based)
ZREVRANK leaderboard "player1" # Descending rank

# Increment score
ZINCRBY leaderboard 50 "player1"

# Remove member
ZREM leaderboard "player1"

# Remove by rank
ZREMRANGEBYRANK leaderboard 0 0  # Remove lowest ranked

# Remove by score
ZREMRANGEBYSCORE leaderboard 0 100

# Count members in score range
ZCOUNT leaderboard 100 200

# Lexicographic operations (for same score)
ZRANGEBYLEX myzset [a [z
```

**Use Cases:**
- Leaderboards
- Priority queues
- Time-series data (using timestamp as score)
- Rate limiting with sliding window

---

### 5. Hashes
Maps between string fields and string values, perfect for representing objects.

**Commands:**
```bash
# Set field
HSET user:1000 name "John Doe"
HSET user:1000 email "john@example.com" age "30"

# Get field
HGET user:1000 name
# Response: "John Doe"

# Get all fields and values
HGETALL user:1000
# Response: {"name": "John Doe", "email": "john@example.com", "age": "30"}

# Get multiple fields
HMGET user:1000 name email

# Set multiple fields
HMSET user:1000 name "John" age "31" city "NYC"

# Check if field exists
HEXISTS user:1000 name
# Response: 1

# Delete field
HDEL user:1000 age

# Get all field names
HKEYS user:1000

# Get all values
HVALS user:1000

# Get number of fields
HLEN user:1000

# Increment numeric field
HINCRBY user:1000 age 1
HINCRBYFLOAT user:1000 balance 10.50

# Set if field doesn't exist
HSETNX user:1000 status "active"

# Scan hash fields
HSCAN user:1000 0 MATCH na*
```

**Use Cases:**
- User profiles
- Product details
- Session data
- Configuration settings

---

### 6. Additional Data Structures

#### Bitmaps
```bash
# Set bit
SETBIT mybitmap 0 1
SETBIT mybitmap 2 1

# Get bit
GETBIT mybitmap 0
# Response: 1

# Count set bits
BITCOUNT mybitmap

# Bitwise operations
BITOP AND destkey key1 key2
BITOP OR destkey key1 key2
BITOP XOR destkey key1 key2
```

**Use Cases:**
- User online status
- Feature flags
- Real-time analytics

#### HyperLogLog
```bash
# Add elements
PFADD hll a b c d e f g

# Get cardinality (approximate count of unique elements)
PFCOUNT hll
# Response: 7

# Merge HyperLogLogs
PFMERGE dest hll1 hll2
```

**Use Cases:**
- Unique visitor counting
- Cardinality estimation

#### Streams
```bash
# Add entry
XADD mystream * sensor-id 1234 temperature 19.8

# Read entries
XRANGE mystream - +
XREAD COUNT 2 STREAMS mystream 0

# Consumer groups
XGROUP CREATE mystream mygroup 0
XREADGROUP GROUP mygroup consumer1 COUNT 1 STREAMS mystream >
```

**Use Cases:**
- Event sourcing
- Activity streams
- Message queues

---

## Redis Expiry

Redis allows you to set a time-to-live (TTL) on keys, after which they are automatically deleted.

### Setting Expiry

```bash
# Set expiry in seconds
EXPIRE mykey 60

# Set expiry in milliseconds
PEXPIRE mykey 60000

# Set expiry at specific Unix timestamp (seconds)
EXPIREAT mykey 1735737600

# Set expiry at specific Unix timestamp (milliseconds)
PEXPIREAT mykey 1735737600000

# Set key with expiry
SET session:123 "data" EX 3600       # Expire in 3600 seconds
SET session:123 "data" PX 3600000    # Expire in 3600000 milliseconds
SETEX session:123 3600 "data"        # Alternative syntax

# Set only if no expiry exists
SET mykey "value" EXAT 1735737600 NX
```

### Checking Expiry

```bash
# Get remaining time in seconds
TTL mykey
# Response: 
#   Positive number: seconds remaining
#   -1: key exists but has no expiry
#   -2: key doesn't exist

# Get remaining time in milliseconds
PTTL mykey

# Example
SET temp "value" EX 100
TTL temp
# Response: 100 (or less, depending on time elapsed)
```

### Removing Expiry

```bash
# Remove expiry (make key persistent)
PERSIST mykey

# Verify
TTL mykey
# Response: -1 (no expiry)
```

### Expiry Behavior

```bash
# Updating a key removes its expiry (for most commands)
SET mykey "value1" EX 60
SET mykey "value2"      # Expiry is removed!
TTL mykey
# Response: -1

# These commands preserve expiry:
# - LPUSH, RPUSH, SADD, ZADD, HSET, etc.
# - GETSET (deprecated, use SET with GET option)
# - INCR, DECR, etc.

# Example with INCR
SET counter 10 EX 60
INCR counter
TTL counter
# Response: ~60 (expiry preserved)

# Example with SET (removes expiry)
SET mykey "value" EX 60
SET mykey "newvalue"
TTL mykey
# Response: -1 (expiry removed)

# To update and keep expiry, use SET with KEEPTTL option
SET mykey "newvalue" KEEPTTL
```

### Expiry Precision

- Redis checks for expired keys in two ways:
  1. **Passive expiry**: When a client tries to access a key, Redis checks if it's expired
  2. **Active expiry**: Redis randomly tests keys and deletes expired ones

```bash
# Configure active expiry (in redis.conf)
# hz 10  # How many times per second to check for expired keys
```

### Advanced Expiry Patterns

#### Sliding Expiration (Session Timeout)
```bash
# Every time user is active, reset expiry
SET session:user123 "data" EX 1800

# On each user activity:
EXPIRE session:user123 1800
```

#### Conditional Expiry
```bash
# Set expiry only if key has no expiry
# (Use Lua script or check TTL first)
if redis.call("TTL", KEYS[1]) == -1 then
    redis.call("EXPIRE", KEYS[1], ARGV[1])
end
```

#### Cache with Auto-Refresh
```bash
# Set cache with expiry
SET cache:data "value" EX 300

# On read, if TTL is low, trigger refresh
TTL cache:data
# If < 60 seconds, refresh in background
```

### Expiry Use Cases

1. **Session Management**
   ```bash
   SETEX session:abc123 3600 "user_data"
   ```

2. **Cache Invalidation**
   ```bash
   SET cache:user:1000 "cached_data" EX 600
   ```

3. **Rate Limiting**
   ```bash
   SET rate:limit:user123 1 EX 60
   INCR rate:limit:user123
   ```

4. **Temporary Data**
   ```bash
   SETEX verification:code:user123 300 "123456"
   ```

5. **Distributed Locks**
   ```bash
   SET lock:resource "token" NX EX 10
   ```

### Important Notes

- Expiry is always set per key, not per field in a hash
- Renaming a key transfers the expiry time
- Using PERSIST removes the expiry
- Most write operations remove expiry unless using KEEPTTL
- Expiry times are stored in milliseconds internally
- Expired keys are eventually deleted, not instantly (lazy + periodic)

---

## Best Practices

1. **Use appropriate data structures** based on your access patterns
2. **Set expiry on cache keys** to prevent memory bloat
3. **Use SCAN instead of KEYS** in production (non-blocking)
4. **Pipeline commands** to reduce network round trips
5. **Use Redis transactions (MULTI/EXEC)** for atomic operations
6. **Monitor memory usage** with INFO memory
7. **Use connection pooling** in production applications
8. **Consider persistence options** based on durability needs (RDB vs AOF)

---

## Performance Tips

```bash
# Use pipelining for multiple commands
# Instead of:
SET key1 "value1"
SET key2 "value2"
SET key3 "value3"

# Use pipeline (pseudo-code):
pipeline.set("key1", "value1")
pipeline.set("key2", "value2")
pipeline.set("key3", "value3")
pipeline.execute()

# Use MGET/MSET for multiple keys
MSET key1 "val1" key2 "val2" key3 "val3"
MGET key1 key2 key3

# Use SCAN for large keyspaces
SCAN 0 MATCH pattern* COUNT 100
```

---

## Common Patterns

### Counter Pattern
```bash
INCR page:views:homepage
INCRBY user:1000:points 50
```

### Rate Limiting Pattern
```bash
# Fixed window
SET rate:user123 1 EX 60 NX
INCR rate:user123
# Check if > threshold

# Sliding window using sorted set
ZADD requests:user123 <timestamp> <request_id>
ZREMRANGEBYSCORE requests:user123 0 <60_seconds_ago>
ZCARD requests:user123  # Count requests in last 60 seconds
```

### Cache-Aside Pattern
```bash
# 1. Try to get from cache
value = GET cache:key

# 2. If miss, get from database
if value is null:
    value = database.query()
    SET cache:key value EX 300

# 3. Return value
```

### Pub/Sub Pattern
```bash
# Subscribe to channel
SUBSCRIBE news:sports

# Publish message
PUBLISH news:sports "Team won!"

# Pattern subscription
PSUBSCRIBE news:*
```

---

## Resources

- [Official Redis Documentation](https://redis.io/documentation)
- [Redis Commands Reference](https://redis.io/commands)
- [Try Redis Online](https://try.redis.io/)