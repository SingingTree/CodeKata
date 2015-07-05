# Kata 02: Karate Chop

A writeup by Bryce Guy.

## Rust

I decided to try out Rust first, as I've been using it recently, but had fallen off the wagon a little bit. Since I plan on using a couple of funtional languages which lend themselves to recursion, I'm using an iterative approach with rust.

### Issues

* Had an off by one error when testing the [1, 3, 5] test case when searching for 5. Was calculating the mid point incorrectly.
* Was failing to exclude the current index when iterating, resulting in endless loops under various circumstances.
* I overlooked that low may become greater than high without ever being equal. I initially had a equality check for low and high to break the iteration of the search. This needed to be changed to if low is equal or greater than high.
* Ran into issues with usize, which is unsigned, and the type used to index by Rust. Because it's possible for my high value to go into negatives, if it does so the program would panic. I decided to fix this with a check on one of the paths. It's gross, but prevents a lot of casting.
* Solved the issue of negative high by instead storing the low and a width. This also solves an overflow issue that could happen with really big slices (arrays)

### Thoughts

* I had to brush up on a few of my Rust techniques to code this, which was good in terms of learning, but it did slow me down a bit.
* Rusts typing is great, and if I weren't returning a -1 as per the Kata, I'd use Optional<usize>.
* I do like being able to build the tests into the same file as the code, and Rust's support for such things. It makes managing tests easier.
* Debugging Rust on Windows can be a bit of a task. It's hard to get debugging tools set up, and there is a bug with stack traces where every frame is reported to be main.

## Haskell

I started the Haskell code second. I aspire to program things in Haskell, and every time I try I get a little bit better, but I fear I'm not there yet, so this is another run at it.

### Issues

* Learning new concepts in Haskell is slow. How do I consturct an array? Not trivial to get my head around. And using a list for this exercise doesn't make a lot of sense because while you can do this with a list, a indexing into a list is O(n), so binary search is a bit naff compared to linear search for this scenario.
* Had an off by one error for the case where you need to look at the top half of the array. Unit tests caught it.
* Had an off by one error when detecting far end cases in even length arrays due to integer divison. Needed to change how the width was calculated for both greater than and less than cases. Unit tests helped me catch this one too (had to figure out how to use HUnit, though).

### Thoughts
