### Rust core Concepts Cheat Sheet:
- Ownershiping:
  
  > The ownership concept is one of the rust mechanics to prevents memory leak or concurrent erros. This works arround the approach of if u create a variable, this is the data ownership. But, what this means?
  - Explaining in the simple way, every time you create a data, this needs a var ownership, and every time on this var scope ends, the data's destructed on the memory.

- Shadowing:
  
  > The shadowing is the capacity of rewrite a var destructing the last when another rises.
  > Look, if u make: "let x: i32 = 10" and, in another local on the scope block makes "let x: i32 = x + 1", the first x is destructed and recreated in another memory slot.

