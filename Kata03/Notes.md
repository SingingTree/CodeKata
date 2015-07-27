## Kata03: How Big? How Fast?

A writeup by Bryce Guy.

In the spirit of the Kata I'm going to be brain dumping my thoughts on each question, and will note if I use any external references or calculations.

### How Big?

- roughly how many binary digits (bit) are required for the unsigned representation of:
  - 1,000: Counting out powers of two using my fingers: 2^10 ~ 1000, so 10 bits (I know, I know, I should know off the top of my head all the powers of 2, but one has to check with some good honest finger counting)
  - 1,000,000: Based on the last answer to get three more zeroes I'd think I need 10 more bits, so 20 bits.
  - 1,000,000,000: 30 bits, same reasoning as above
  - 1,000,000,000,000: 40 bits, as above
  - This is 8 times the last answer, since 2^3 = 8, I'd want 3 more bits to give me the value, so 43 bits.
- My town has approximately 20,000 residences. How much space is required to store the names, addresses, and a phone number for all of these (if we store them as characters)?
  - How long can each of these fields be? Time to make up some figures!
  - Decompose names into 3 fields, first, middle, and last. Give 64 characters for each (was thinking between 50 and 100, so why not go with the power of 2 in there).
  - I imagine addresses could be quite long, so I'm going to say 1024 characters, to provide a large buffer.
  - Phone numbers are pretty static in length. Since I'm from NZ I'm used to at most 10 digits for a number, with an international calling code on the front. The US phone numbers in my phone also appear to cap out at 10 with a calling code. Without researching (becuase that's cheating), I'm going to say that 10 + 3 for calling code is all I need, but take 16 characters for each phone number anyway, to keep with my power of 2 theme, and incase I'm over looking something.
  - So 64 * 3 + 1024 + 16 = 192 + 1024 + 16 = 1232 characters of space for each person. 20,000 * 1232 = 24,640,000 characters (I cheated and used a calc for that one).
  - Now it depends on what characters I'm using. Let's make it UTF-8, because that's a cool thing I think I should use.
  - I cheated here and checked how many bytes UTF-8 maxes out at: it's 4 bytes. So each character will take up between one and 4 bytes. For the sake of a quick estimate I'm not going to waffle about trimming the space for certain characters (phone number may always contain 0-9, it could be argued).
  - So between 24,640,000 and 24,640,000 * 4 bytes or ~ 24 megabytes to ~96 megabytes.
- I’m storing 1,000,000 integers in a binary tree. Roughly how many nodes and levels can I expect the tree to have? Roughly how much space will it occupy on a 32-bit architecture?
  - Given a 32 bit architecture lets say ints are 32 bits = 4 bytes (assuming 8 bit bytes, [which is mostly safe](https://isocpp.org/wiki/faq/intrinsic-types#very-large-bytes)). We have 1,000,000 ints, so we need 4,000,000 bytes just for storage of ints.
  - Each node in the binary tree is super simple and has storage for two pointers and an int. 32 bit architecture: 32 bit pointers. We need a node for each int, so 1,000,000 nodes, 2 pointers each, 32 bits (4 bytes) = 1,000,000 * 2 * 4 = 8,000,000 bytes for the pointers in the tree.
  - 12,000,000 bytes (ish) of space needed to hold the thing then. (4,000,000 for each int, 8,000,000 for the pointers -- that overhead!)
  - How many nodes? 1,000,000, one for each int
  - How many levels? I was writing this out and though it was log base 2 of n, where n is the number of nodes. I think that's close, but am not 100% sure. Napkin math suggests then I'd need about 20 levels. However, more napkin math: the first level of such a tree gives me 1 node (2^0), the second level gives me 2 more nodes (2^1), third 4 more (2^2). So the number of total nodes is a finite geometric series (I remember some math!). I don't remember enough math to really explain this properly without cheating and looking somewhere else, but what I do know is that the final term the sequence that makes up the series will dominate the others (e.g. in 2^0 + 2^1 + 2^2, the 2^2 term is greater than the sum of all the others). I think this means that the final level of the tree must have capacity > total nodes / 2 (though this level may not be full, but it must still have such a capcity).
  - Based on the above I think I need 19 levels, as 2^19 would give me more than half the nodes required. Hacky napkin logic: 2^10 = 1024, 2^9 = 512, 2^10 * 2^9 = 2^19, 1024 * 512 > 500,000. Thus my last level in a 19 level tree would have more than half the capacity needed for the whole thing. And if my above reasoning holds then it'd be enough.
