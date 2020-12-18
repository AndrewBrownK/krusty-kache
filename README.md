Hi zencastr, here is the homework you requested. 

This readme file is an append-only sort of train of thought. Should you browse the git history, you can see both my development, and my thought processes every step of the way.

Here we go. Let's start with the requirements.

> We’d like you to build a caching service, something similar to Redis or Memcachd. The
purpose of this assignment is really an exploration in a problem, there is no right answer.
Perhaps there is a wrong answer :). We think that spending about 3 hours on this assignment
should get you to a good place, but we don’t want to define what is too much or too little.

Off the bat, among the languages I am familiar with, I gravitate towards rust. The bare metal performance and deliberate (yet safe) memory management should help give very predictable, high, and reliable performance. Additionally, there are a few key libraries I am familiar with that I know will suit this problem well.

Inferred feature goals (mostly based on redis and memcachd):
- key value store
- data is cached in memory. There is no definition/scope for storing data to disk 
- at reasonable limits, data may be dropped 
- the most ideal version of this cache can be distributed at scale
 
> 1. Build a standalone caching service (choose your language, maybe typescript?)
 
Rust should be pleasant for this, plus your team is familiar with it 

> 2. You should be able to add item(s), remove items, and fetch items

got it 

> 3. The data structure used to store these items is up to you

lovely

> 4. The API definitions and formats used to communicate with this API are up to you

off the top of my head, REST is very logical do to GET, PUT, DELETE, etc. A json payload seems fine. So far I'm undecided what to do for the key. Maybe I'll accept any reasonable string. Actually, forget JSON payload, since we don't care about the contents of what you are caching. We'll just use the HTTP body and not care what it is.

> 5. We expect that this service will be runnable, we’ll be able to connect to it, cache things
   and retrieve them.

Okies 

> 6. PRO TIP: Spending time on the cache internals would be better than spending time on
   the API

got it 
 
> 7. If you are writing this homework in typescript / javascript please make sure there is a
   valid package.json in the root of the repo

understood.

> 8. Upload code to a github repo and send back to your Zencastr contact

okay


----

I've added some tentative dependencies, but I've got to go to sleep for the night. I'll pick this up tomorrow.


--- 


Implementation plan:
- Get a web server with GET/POST/DELETE running
   - test it using curl
- Hook up the web server to the map 
- tag 1.0.0 
   - this version is MVP of homework
   - everything beyond is an exercise in fun
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















