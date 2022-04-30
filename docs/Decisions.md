# Decision justifications
- All files will be h2 headers.
- The location of the code will be given by a description instead of a line number to reduce technical dept as otherwise we would need to update this file any time code is changed further up in any of the files.

## backend/write_serde.rs

### Beginning of `get_write_data`:

The file is read again in this function because it may have been modified since last read and if we just used the one at first write it would overwrite any modifications. 

#### But what happens now if the file is modified?
It will be written to the file but the hash will not match up with the previous messages. This is not as big of a problem as the hashes should show this conflict that can be sorted out later and because both messages are still there they can be displayed appropriately.

#### But what about the performance impact?
You will not be spamming messages and this function is only called when a message is actually posted. Other processes like syncing that will be executed at this point will take time anyway.

## CLI/write.rs

### `ask_for_parser`
To print out every possible option of the parser I used the `strum` crate to iterate over every possible option and implemented std::fmt::Display for it (with a match statement). The reason for doing it this way is so that it's easier to maintain because one doesn't have to change far off bits of the code when modifying the enumerator (and the Display impl's match statement would give a warning when you modify it)
