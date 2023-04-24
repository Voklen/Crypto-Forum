The GitArk keys are called "GitArkRepo<original IPNS link>" so that you can do `gitark <IPNS link>` and it can find the key from the link to be able to operate on it.
This is what happens when editing someone elses repo, if you want to create your own repo we could make it:

# Use "GitArkRepo<IPNS link>"
For this one it would remain consistent but the clone of would be pointing to itself

## Benefit
Get the key from the link easily

## Wokraround
Check if the IPNS link maches the one the key controls to determine if you own it

# Use "GitArkOwnedRepo<repo name>"
The reason we use the other method is to avoid name conflicts and to be able to find the key from the link

## Benefit
Allows easy identification of repos where your the owner and ones where you're not

## Workaround
Whenever there's a GUI version of the app it'll pass the repo name instead of the link, and if we really had to get the link we'd iterate through the keys to get the right one

# Use "GitArkOwnedRepo<IPNS link>"
This would need two checks

## Benefit
Allows easy identification of repos where your the owner and ones where you're not while still being able to use the link

## Workaround
You'd need to do two checks when resolving a link to a key, one for "GitArkRepo" and one for "GitArkOwnedRepo"

# Conclusion
"GitArkRepo<IPNS link>", for the 1:1 key and link conversion as well as consistency.
