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
  - Given a 32 bit architecture lets say ints are 32 bits = 4 bytes (assuming 8 bit bytes, [which is mostly safe](https://isocpp.org/wiki/faq/intrinsic-types#very-large-bytes)). We have 1,000,000 ints, so swe need 4,000,000 bytes just for storage of ints.
