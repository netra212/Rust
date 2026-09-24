### HashMap:
* key->value lookup table. 
* find value by key. 
* HashMap<K, V>.
* import with this: use std::collections::HashMap;
* stores key -> value relationship. 
* For example:
    "user_123" -> User
    "config.port" -> 8080
    "/api/users" -> RouteHandler

Important: A HashMap does not guarantee iteration order. 
When to use HashMap:
    "I know a key and want its associated data."


### HashSet:
* Unique values. 
* Membership/deduplication. 
* HashSet<T>.
* HashSet is basically a HashMap where we only care about keys. 
* Conceptually: HashMap<K, ()> Instead of storing: user->value. 
it answers:
    Does this value exist ?
* HashSet is just a wrapper around HashMap<T, ()>
* Q. "What's the point of that ?" you ask. "I could just store the keys in a Vec."
* Unique features of HashSet is that it is guaranteed to not have duplicate elements. That's the contract that any set collections fulfills.
* If we insert a value that is already present in the HashSet, (i.e. the new value is equal to the existing and they both have the same hash), then the new value will replace the old.
* 4 Operations of sets:
    -> `union`: get all the unique elements in both sets. 
    -> `difference`: get all the elements that are in the first set but not the second. 
    -> `intersection`: get all the elements that are only in both sets. 
    -> `symmetric_difference`: get all the elements that are in one set or the other, but not both. 
* Very Important Distributed-Systems use case. 
- Imagine receiving network messages:
    * request 1001
    * request 1002
    * request 1001
    * request 1003
    * request 1002
- Networks may retry messages. 
- HashSet<RequestId>

```
    if processed_requests.contains(&request_id) {
        return;
    }

    processed_requests.insert(request_id);

    process_request();
```
Another example of HashSet is:
- Imagine:
    * known_nodes
    * current_nodes
- then 
    * new_nodes = current_nodes - known_nodes


### LinkedList:
* Chain of nodes. 
* Insert/removes at ends. 
* LinkedList<T>.
* use std::collections::LinkedList;
* Doubly LinkedList: [A] <-> [B] <-> [C] <-> [D]

```
Node {
    previous,
    value,
    next
}
```

[A]   -----> [B] ----->  [C]  -----> [D]
0x1000     0x93ab       0x4200      0x8120


### Stack:
* LastIn, FirstOut
* Push/Pop
* Usually Vec<T>.
* Imagine:
```
push A

A

push B

B
A

push C

C
B
A
```

### Queue:
* FirstIn, FirstOut
* push_back/pop_front
* VecDeque<T>.
* Imagine people standing in a queue:
* `Alice` -> `Bob` -> `Charlie`
* 



### BinaryHeap:
* Priority Queue. 
* Get highest-priority item. 
* BinaryHeap<T>.
* A normal queue says:
    `first arrived` -> `first processed`
* A priority queue says:
    `most important` -> `first processed`
* A Binary heap is a max heap by default. 


NOTE:
### A HashMap says:
* "I optimize finding something when you know its key."

### A HashSet says:
* "I optimize answering whether something exists."

### A stack says:
* "The most recent thing matters first."

### A queue says:
* "The oldest thing matters first."

### A heap says:
* "The most important thing matters first."

### A LinkedList says:
* "I care about cheap changes around known position/endpoints more than random indexed access."

