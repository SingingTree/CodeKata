# Kata 04: Data Munging

## To what extent did the design decisions you made when writing the original programs make it easier or harder to factor out common code?

I cheated a little and started writing the second set of code to be more general purpose. To do this I made certain values into function parameters. Also, just having things broken up into functions aids in having discrete units of code that are a bit more modular. If the code for each program were written inside just a main function I think it'd be a bit harder to merge the two sets of code.

The first set of code lacked some of the parameterisation of the second, so it took a little bit more work to adapt, but wasn't particularly painful.

## Was the way you wrote the second program influenced by writing the first?

Absolutely. I saw the second problem as very similar to the first, so the code was written in a very similar way.

## Is factoring out as much common code as possible always a good thing? Did the readability of the programs suffer because of this requirement? How about the maintainability?

I'd say it's not always a good thing. In this case I don't think the impact has been particularly bad for readability or comprehension. However, for arbitrary cases, if code becomes too general I think it can be harder to read or understand. In this scenario there's overhead in that one of the solutions needs specific trimming behaviour, while another does not. This adds an overhead to understanding the general purpose code that is not required for one of the cases. Here I think it acceptable, but depending on the situation I don't always think it so.
