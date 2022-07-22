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
I chose to hard code the options instead of iterating over every variant of the enumerator because:
- It does not require an external crate
- The enumerator variant names are the ones shown to the user (so cannot include spaces or non-ASCII characters)
- The downside of having to modify other bits of code when adding a new parser shouldn't impact this too much as forgetting to modify this is easily spotted due to, well, not being able to use the new parser
