# Brain Scan
<html>
<br>
<h1>Brain Scan</h1>

<h2>What is this? Why the foul language?</h2>
<p>This project is a interpreter for the language brainfuck. Brainfuck is an esoteric programming language that was designed in the mid 90s by the computer scientist Urban Muller, with the goal of having as small as a compiler as possible. Inspired by the FALSE esoteric programming language, the original compiler for brainfuck was sized at 296 bytes! It is aptly named as it is an extremely confusing language to write in due to its complete lack of readability. The language is made of only 8 commands which are listed below. I started this project because I originally wanted to create a compiler, but an interpreter is an amazing start. I wrote this in Rust because I love the Rust language a lot. This was a lot of fun and I hope you enjoy.</p>

<ul>
    <li><h3>&gt; = increases memory pointer, or moves the pointer to the right 1 block.</h3></li>
    <li><h3>&lt; = decreases memory pointer, or moves the pointer to the left 1 block.</h3></li>
    <li><h3>+</h3><h3> = increases value stored at the block pointed to by the memory pointer</h3></li>
    <li><h3>- = decreases value stored at the block pointed to by the memory pointer</h3></li>
    <li><h3>[ = like c while(cur_block_value != 0) loop.</h3></li>
    <li><h3>] = if block currently pointed to's value is not zero, jump back to corresponding [</h3></li>
    <li><h3>, = like c getchar(). input 1 character.</h3></li>
    <li><h3>. = like c putchar(). print 1 character to the console</h3></li>
</ul>

<h3><a href="/hello_world.bf">Included in hello_world.bf is hello world written in brainfuck</a></h3>
</html>
