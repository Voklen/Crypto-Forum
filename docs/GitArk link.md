A Gitark link is an IPNS link controlled by the owner that points to a recent version of the repo on IPFS.

This is all well and good, but what if the owner goes offline? Well because of the way the repo is built anyone can validate a given repo. Therefore whenever someone wants to post a comment they can create their own repo with that one file changed. So the question is: How do you get all the user's repos?

Users must be validated by moderators before they can post anyway so when a user is validated we can put a public key for the forum _AND_ an IPNS link in the file of valid users. Then when you get the main IPNS link repo, you have a look at the IPNS links of the validated users to see if there are any more recent ones.

In this case, why don't we just make the IPNS link point to the first version of the repo and find the most recent one from there? Because having the link point quite close to the HEAD is faster than having it be right at the bottom.

# Ideas
- Every so often the valid users list can be rearranged to put the most active and constantly online posters at the top and then only the top x can be checked 
- Maybe have a line in the file which separates the ones to be checked on repo load from the ones to be checked on ping
