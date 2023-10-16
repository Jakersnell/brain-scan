# BrainScan Interpreter
## Disclaimer
Please note that the name "Brainfuck" is the historical and accepted name of the esoteric programming language covered by this project. 
The name is not intended to offend and is used strictly in a technical and educational context.
## What is this? Why the foul language?

This project is a interpreter for the language brainfuck.  Brainfuck is an esoteric programming language that was designed in the mid 90s by the computer scientist Urban Muller, with the goal of having as small as a compiler as possible.  Inspired by the FALSE esoteric programming language, the original compiler for brainfuck was sized at 296 bytes!  It is aptly named as it is an extremely confusing language to write in due to its complete lack of readability.  The language is made of only 8 commands which are listed below. I started this project because I originally wanted to create a compiler, but an interpreter is an amazing start. I wrote this in Rust because I love the Rust language a lot. This was a lot of fun and I hope you enjoy.

* ### '>'  increases memory pointer, or moves the pointer to the right 1 block.
* ### '<'  decreases memory pointer, or moves the pointer to the left 1 block.
* ### '+'  increases value stored at the block pointed to by the memory pointer
* ### '-'  decreases value stored at the block pointed to by the memory pointer
* ### '['  defines the beginning of a loop block
* ### ']'  if block currently pointed to's value is not zero, jump back to corresponding loop entry
* ### ','  input 1 character into the current block pointed to by the memory pointer.
* ### '.'  print 1 character to the console from the current block pointed to by the memory pointer.


### [Included in hello_world.bf is hello world written in brainfuck](/hello_world.bf)
