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

* Lots of fighting with Haskell, but good to learn.
* It's type system is totally rad, and monads are powerful (though my code here scarcely scratches the surface).
* I'm not sure how I should be structuring my Haskell code, and feel like I may be losing style points.

## Java

I spend my works days coding Java, but figured this would be a good one to talk to me colleagues about. Also a chance to shake out some cobwebs in areas I don't hit in my day to day. I decided to try out using Gradle, which I haven't done much with before.

I decided to do both a recursive and iterative solution, with the recursive first. I figure these are the same as the above two solutions, in how I think about them, but the language is different enough to make it a little exciting.

### Issues

* After having drilled the code from the previous two I didn't seem to make any mistakes. The integer division is not wildly intuitive to me mind. I need to sit down occasionally and think about the ramifications of integer division for certain cases, but no big thing.

### Thoughts

* Gradle seems all right. Another make tool, with cool bits (no XML, plays nice with Maven and ivy) and not cool bits (I get to learn another set of syntax to script a build tool, language specific).
* Generics in Java are less fun than their counterparts in Rust and Haskell. Having them attached to a class instead of simply being able to attach them to a method is cumbersome. And I know 'object oriented design blah blah' and that there are implementation factors in Java, but from a purely how do I like to code these things perspective, I like it less.
* Parametrized JUnit tests seem rad, if not syntactically a little unintuitive to me.

## Ruby

I learned ruby years and years ago. I want to get back into it a bit, so I figured I'd try here.

### Issues

* Having to relearn bits and bobs from ruby
* Not super sure about the style, but think I've come up with something approximating an appropriate directory layout

### Thoughts

* Not sure how I feel about all the tooling, but I think RSpec is really rad. After having done some BDD stuff in other languages, I find RSpec much quicker to just get stuff done

## Scala

Scala! I'd done some Scala in the past, it has a lot of rad things, and a few things I don't like. But all in all I think Scala is pretty cool and I can just choose not to use the gross bits for the most part. Even then it feels like I have everything and a few kitchen sinks at times. Anywho, I wanted to brush up. I also wanted to try and play around with sbt again.

### Issues

* Sbt feels way more clunky than I want. Really long times to download deps to build simple programs (admittedly it's a one off). [Incompatibility between Scala versions](http://stackoverflow.com/questions/24103043/why-does-sbt-give-cant-expand-macros-compiled-by-previous-versions-of-scala-f). The documentation for the latest version of sbt (0.13 as of writing) details using an incompatible version of scala: ```scalaVersion := "2.11.4"```
* Returning to Scala it's hard to keep up with language changes. I implemented my solution using view bounds. I ended up with a swag of google results suggesting these when I was dealing with typing issues in my solution. Indeed, even the official docs in places detail view bounds [without noting their deprecation](http://docs.scala-lang.org/tutorials/FAQ/context-and-view-bounds.html).  However, after completing my solution it came to my attention that these are deprecated. I ended up fixing this using context bounds, which seem more complicated for the scenario I'm dealing with, and still don't think I fully understand.

### Thoughts

* I remeber Scala fondly, but I feel I've lost some of that gloss. The way Rust and Haskell implement types seems to much more concisely achieve what Scala does and involved much less pain on my behalf.
* Having everything and the kitchen sink is fantastic in terms of flexibility, but I fear in some cases it requires more work to have everything play nice together (classes and traits, in this case).
