# Reading 'Crafting Interpreters'

Bob Nystrom's http://craftinginterpreters.com/

Notes will go here in this Readme until I need more pages for clarity

## Languages

I'm following along with the book, so I'll do the implementations in Java and C. 

I'm also practicing Rust, and want to practice my translation skills, so I'll do the Java and C implementations in Rust too.

## Challenges

### introduction

1. There are at least six domain-specific languages used in the little system I cobbled together to write and publish this book. What are they?

- markdown
- jinja
- make
- sass
- html
- requirements.txt
- gitignore, git scripts
- iml? xml

Missed: CSS

2. Get a “Hello, world!” program written and running in Java. Set up whatever Makefiles or IDE projects you need to get it working. If you have a debugger, get comfortable with it and step through your program as it runs.


`jlox-java/hello.java`

`java -cp jlox-java/* hello`

Compiling and running jlox:

`javac jlox/*.java`
`java -cp ./ jlox.Lox`

3. Do the same thing for C. To get some practice with pointers, define a doubly-linked list of heap-allocated strings. Write functions to insert, find, and delete items from it. Test them.

`clox-c/hello.c`

`clang clox-c/hello.c -o clox-c/hello.out`

`./clox-c/hello.out`

`clox-c/dll.c`

`clang clox-c/dll.c -o clox-c/dll.out`

`./clox-c/dll.out`

### A Map of the Territory

1. Pick an open source implementation of a language you like. Download the source code and poke around in it. Try to find the code that implements the scanner and parser. Are they hand-written, or generated using tools like Lex and Yacc? (.l or .y files usually imply the latter.)

python


2. Just-in-time compilation tends to be the fastest way to implement a dynamically-typed language, but not all of them use it. What reasons are there to not JIT?

My answers:
- more unexpected behavior and bugs possible
- unpredictable runtimes
- harder to do

Correct answers:
- it's harder
- ties you to a specific cpu architecture (curious, I assume that means JIT devs end up building multiple JITs)
- bytecode is compact, so maybe there's a memory advantage to not do this
- some platforms don't allow it (iOS, for instance)

3. Most Lisp implementations that compile to C also contain an interpreter that lets them execute Lisp code on the fly as well. Why?

guess: 
- lisp authors often use the repl in order to evaluate snippets of code as they write. It's helpful!

correct: 
- Most Lisps support macros -- code that is executed at compile time, so the implementation needs to be able to evaluate the macro itself while in the middle of compiling. You could do that by compiling the macro and then running that, but that's a lot of overhead.

### The Lox Language


1. Write some sample Lox programs and run them (you can use the implementations of Lox in my repository). Try to come up with edge case behavior I didn’t specify here. Does it do what you expect? Why or why not?

2. This informal introduction leaves a lot unspecified. List several open questions you have about the language’s syntax and semantics. What do you think the answers should be?

what happens when a for loop definition leaves out some piece? error!

Is nil falsy? yes.

3. Lox is a pretty tiny language. What features do you think it is missing that would make it annoying to use for real programs? (Aside from the standard library, of course.)

- umm, there are no arrays or objects?
- I guess we'll just implement lists ourselves then.
- No way to get random or pseudorandom numbers to build a decent hash function
- for a dynamic language, annoying not to be able to get other args (or specify arity)
- anonymous functions for passing callbacks
- timing (scratch that there's `clock()`!)
- facilities for converting between data types
- string interpolation syntax
- regular expressions
- errors!

### Scanning

1. The lexical grammars of Python and Haskell are not regular. What does that mean, and why aren’t they?

- context sensitive, due to significant indentation (for both)

important alternatives being type 0 (no rules!), context-free but not regular, or context-sensitive


2. Aside from separating tokens — distinguishing print foo from printfoo — spaces aren’t used for much in most languages. However, in a couple of dark corners, a space does affect how code is parsed in CoffeeScript, Ruby, and the C preprocessor. Where and what effect does it have in each of those languages?

not sure for coffeescript or c preprocessor

spaces affecting parsing in Ruby... 
- tuples vs. function calls
- more places?

3. Our scanner here, like most, discards comments and whitespace since those aren’t needed by the parser. Why might you want to write a scanner that does not discard those? What would it be useful for?

Keeping comments is key if
- comments can _do_ stuff
- you are targeting another high level representation, and preserving the comments is important
- e.g. syntax highlighting, formatting, transpiling...

4. Add support to Lox’s scanner for C-style /* ... */ block comments. Make sure to handle newlines in them. Consider allowing them to nest. Is adding support for nesting more work than you expected? Why?

- newlines seem like they're handled
- nesting levels weren't that hard, but maybe my implementation is fragile? idk, I just count up when hit more and count down when we end
