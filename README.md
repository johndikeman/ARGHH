ARGHH [![Build Status](https://travis-ci.org/microwaveabletoaster/ARGHH.svg?branch=master)](https://travis-ci.org/microwaveabletoaster/ARGHH)
=====

ARGHH stands for ARGHH: A Rustic Step Implementation. it's an recursive onomatopoeic backronym, because it's the noise that you make when you read what it stands for.

GETTING STARTED WITH ARGHH
==========================
### sike! it's time for a history lesson
a little while ago, I wrote [STEP](https://github.com/microwaveabletoaster/STEPlang). It's functional, but it never really reached the point that I wanted it to. Recently I decided to pick the project back up, but instead of working with the code that I already had, I decided to just write the whole darn thing again- but this time, in Rust- a fast, safe systems programming language that the nerds at Mozilla built. 

### okay here we go

The most distinct feature of STEP/ARGHH that sets is apart from other stack-based programming languages is a ~~confusing as hell~~ revolutionary syntax that requires explicit typing of the order of operations. What does this look like?

`0. this gets run first .0 1. this gets run second .1 2. you get the idea .2`

this means you can do clever stuff like reusing operators, since everything within one operation gets run regardless of whether or not it's within another operation. watch!

`0. this gets run once 1. this gets run twice .1 .0`

this allows you to make your code sweet and dense, like a pound cake. everything that isn't a reserved operator (we'll get to those) is pushed directly onto the stack. there's only two types in ARGHH- strings and integers. No, ARGHH doesn't support floating point numbers, because if you do something you shouldn't do it half way, even if you're a number.

the main method is the entry point of your ARGHH program. methods are defined with a smol waffle (#)

`# main 0. this code ACTUALLY gets run .0`

the default active stack is also named `main`, coincidentally.

armed with these tidbits of information, you are relatively prepared to program in ARGHH.

### reserved operators

the secret sauce that makes ARGHH tick. sidenote: I wrote a programming language, so I get to name all the symbols. that's like, certainly in the rules somewhere, so deal with it.

### +
This is called a frightened starfish, and it adds the top two numbers on the stack together. It will cry if you try to use it with strings, but support for that is planned.

### - 
This is called a half-worm, and it will subtract the top thing on the stack from the second to top thing on the stack.

### /
This is called a confused tree, and it will divide the second to top thing on the stack by the top thing on the stack.

### *
This is called a butthole, and it will multiply the top two things on the stack.

### % 
This is called a disappointment, and it will find the remainder of the second to top thing on the stack divided by the top thing on the stack.

### output
This will pop a value from the stack and write it so stdout.

### goto
This operator sets the operator index to the value that it pops from the stack. If that value happens to not be a number, your program will meet an untimely end. Example:

`0. heya output .0 1. 0 goto .1`

This program prints "heya" forever.

### end
If you decide that your beautiful program has experienced enough of the world, you can break its fragile little neck with `end`. Don't let emotions cloud your decision. It's for the best.

### dupe
This operator duplicates the top value on the stack.

### spawn
This operator pops a value from the stack and creates a new stack that's named after the value, then makes it the active stack.

### switch
This operator pops a value from the stack and makes it the active stack. It HAS to be `spawn`ed first so if you didn't do that your program will die. You don't have to switch to a newly spawned stack, the spawn operator does that for you.

### =>
This is called a ruined peasant home, and it is used to move values from stack to stack. example:

`# main 0. newstack spawn 0 newstack main => .0`

this code makes a new stack called `newstack`, pushes 0 onto it, then moves the 0 from `newstack` to `main`.

### <=
This is called a failed shovel, and it does the same thing as the ruined peasant home, just backwards.

### input
This operator will read a line from stdin and push it to the stack.

### <
This is called a confused seagull, and it will push "yea" onto the stack if the top operator is greater than the second from the top operator and will push "nope" if it isn't. These operators will work with numbers and strings interchangably- the value that is compared when you're using a string is the length of the string.

### >
This is called a really bad arrowhead, and it does the same thing as the confused seagull- just backwards.

### ==
This is called a cracked lip, and it will push "yea" onto the stack if the top values equal each other. This handles strings in the same way as the confused seagull and really bad arrowhead, except if you try to compare two strings the comparison will be based on the actual value, not just the length.

### !
This is called a surprise, and it will push "yea" onto the stack if the top value is "nope" and vice versa.

### !!
This is called either a wingzing or a special surprise, and it will pop a value from the stack and invoke the method of that name, if there is one. If there isn't one, you'd better have the funeral arrangements ready for your program. example:

`# other_method 0. other output .0 # main 0. other_method !! .0`

This code prints "other". Since method definitions are handled before the other operators, methods can be defined in any order and be called from anywhere in the program.

### ?
This is called a curlyfry, and it's a special operator in ARGHH. You can stick it with any operator, and that operator will only be run if the value popped from the stack is "yea". example:

`# main 0. balls yea ?output .0`

this program prints "balls". How vulgar.




ROADMAP
=======

:white_check_mark: output

:white_check_mark: input

:white_medium_square: pause

:white_check_mark: dupe

:white_medium_square: rand

:white_check_mark: goto

:white_check_mark: +, -, *, /, %

:white_check_mark: <, >, ==, !

:white_check_mark: ?

:white_check_mark: end

:white_check_mark: alternate stacks

:white_medium_square: string operations

:white_medium_square: importing files

:white_check_mark: methods

:white_medium_square: tests

:white_medium_square: kind types:kissing_heart:

:white_medium_square: finish the documentation
