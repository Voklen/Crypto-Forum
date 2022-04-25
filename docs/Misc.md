## General program structure
Ths project can be used as both a rust library and compiled into a interactive command-line program. The library code is all in `backend/` and consolidated in `lib.rs`\
`main.rs` then simply uses said rust library to make a command-line application to interact with.
```
┌─────────┐     
│ main.rs │     
└┬───────┬┘     
┌▽──┐  ┌▽────┐
│CLI/│─ᐅ│lib.rs│
└────┘  └┬─────┘
        ┌▽──────┐    
        │backend/│    
        └────────┘    
```
So nothing in `backend/` will ever depend on `CLI/` (you could completely remove `CLI/` and `main.rs` and the code will still work as a library)