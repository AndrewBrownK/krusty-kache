Hello and welcome to the [Krusty Kache](https://www.youtube.com/watch?v=vE2ETqUGj6Q) 🦀🦀🦀
---

This is a practice project to both represent my coding abilities, and grow more accustom to Rust (which I'm mostly new to).

This readme file is a sort of train of thought. Should you browse the git history, you can see both my development, and my thought processes every step of the way.

Fair warning reinforcement: This readme is very stream of conscious.

Project Definition
---

> A caching service, something similar to Redis or Memcachd.

Caching service heavily gravitates me towards Rust. The bare metal performance and deliberate (yet safe) memory management should help give very predictable, high, and reliable performance. Additionally, there are a few key libraries I am familiar with that I know will suit this problem well.

Inferred feature goals (mostly based on redis and memcachd):
- key value store
- data is cached in memory. There is no definition/scope for storing data to disk 
- at reasonable limits, data may be dropped 
- the most ideal version of this cache can be distributed at scale

> - Should be able to add item(s), remove items, and fetch items
> - The API definitions and formats used to communicate with this API are somewhat arbitrary

Off the top of my head, HTTP/REST is very logical tool with GET, PUT, DELETE, etc. We'll use the path's slug as the resource id (key in key-value). We'll just use the HTTP body as the value of the resource (value in key-value). We don't particularly care about the format of the body too much. Arbitrarily though, let's constrain it to UTF-8 instead of just any byte stream.

----

Implementation plan:
---
- Get a web server with GET/POST/DELETE running
   - test it using curl
- Hook up the web server to the map 
- tag 1.0.0 
   - this version is MVP
   - everything beyond is beyond MVP
- Add some tests in Rust, and also measure performance 
- tag 1.0.1
- add command line options and/or other configuration
   - number of threads 
   - limit to key size 
   - limit to value size
- tag 1.1.0
- Implement reaction to resource limitations
   - number of cache entries limit
   - memory usage limit (if there's an easy way to measure this)
- tag 2.0.0
