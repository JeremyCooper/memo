# memo
Simple memoization function wrapper

Usage: 
----
Create memoization context with `MemoBox::new(*function-or-closure*)`
To call function with memoization active use MemoBox::call on your spawned instance.  
  
There is built in functionality for serializing/deserializing to/from bincode format.
Simply call MemoBox::des(), and MemoBox::ser(), respectively. You will need to satisfy the
Read and Write traits, again, respectively.

Dependencies:  
----
serde  
bincode  
  
Tested on:  
----
Arch Linux  
macOS  
