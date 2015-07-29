## Kata03: How Big? How Fast?

A writeup by Bryce Guy.

In the spirit of the Kata I'm going to be brain dumping my thoughts on each question, and will note if I use any external references or calculations.

### How Big?

- Roughly how many binary digits (bit) are required for the unsigned representation of:
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
  - I'd like to do a formal proof of the this all, think I could, but I need other resources, so not right now!

### How Fast?

- My copy of Meyer’s Object Oriented Software Construction has about 1,200 body pages. Assuming no flow control or protocol overhead, about how long would it take to send it over an async 56k baud modem line?
  - TIL what I body page is (had to look that one up, tsk, tsk).
  - Unless I'm missing something this is open ended in terms of how much text is on each page (or we could send the pages as images, but let's not on 56k).
  - So let me guess how many chars per page, and be gentle, for I have no freakin' idea. Let's say each page has on average 40 lines, and each line on average 100 characters. Then we have 40 * 100 * 1200 = 4000 * 1200 = 4,800,000 characters (is that a sensible amount, I don't know!).
  - I'm going to assume that the characters in this book are all drawn from good ole ASCII. We're going to send them via UTF-8, because we're living in the future, but they'll all be a single byte because they're those drawn from ASCII. So 4,800,000 bytes or ~4,800 kilo bytes.
  - 56kb/s = 7kB/s (my modem never hit this, but let's think of an ideal world).
  - 4,800 / 7 = 686 (rounding up) seconds. Or about 11 and a half minutes.
- My binary search algorithm takes about 4.5mS to search a 10,000 entry array, and about 6mS to search 100,000 elements. How long would I expect it to take to search 10,000,000 elements (assuming I have sufficient memory to prevent paging).
  - Based on the idea that the binary search has some constant time component, C, then I'm going to try and figure out how the size of the input impacts the result.
  - I know that the time binary search takes typicall grows as a function of log n, where n is the input size.
  - So I'm going to use the following formula for time taken. T = C + V * log base 2(n), where T is time taken, and V is some constant that modifies the result of the log.
  - (1) For the first time 4.5 = C + V * log (10,000). (Very) Napkin math: 4.5 = C + V * 13.2
  - (2) Second time: 6 = C + V * log (100,000). Ballpark math: 6 = C + V * 16.5
  - Rearranging (1) gives C = 4.5 - V * 13.2. Substituting into (2) gives 6 = 4.5 - V * 13.2 + V * 16.5 = 4.5 + V(16.5 - 13.2) = 4.5 + V * 3.3, rearranging 1.5 / 3.3 = V, gives us a super rough value of .4 for V.
  - Plugging V back into (1) gives 4.5 = C + .4 * 13.2 ~= C + 6, which suggests C is about -1.5, which is weird. But I'm gonna see how this pans out.
  - Plugging V and C into (2) 6 = -1.5 + .4 * 16.5 ~= -1.5 + 7.5. There's a lot of fudging here by me, and C being a negative doesn't really make me happy.
