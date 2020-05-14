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
rust:
- low level lexer: https://github.com/rust-lang/rust/blob/master/src/librustc_lexer/src/lib.rs
- higher level lexer: https://github.com/rust-lang/rust/blob/master/src/librustc_parse/lexer/mod.rs
Notes: these are hand-written 

High level description of how the rust compiler works: https://rustc-dev-guide.rust-lang.org/overview.html

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

DONE: Java implementation, challenge questions, check the answers, Rust implementation

1. The lexical grammars of Python and Haskell are not regular. What does that mean, and why aren’t they?

- context sensitive, due to significant indentation (for both)
- python adds indent and dedent pseudo-tokens

important alternatives being type 0 (no rules!), context-free but not regular, or context-sensitive


2. Aside from separating tokens — distinguishing print foo from printfoo — spaces aren’t used for much in most languages. However, in a couple of dark corners, a space does affect how code is parsed in CoffeeScript, Ruby, and the C preprocessor. Where and what effect does it have in each of those languages?

not sure for coffeescript or c preprocessor

- coffescript has ruby-like ambiguity around function calls, which spaces disambiguate
- C preprocessor has an ambiguity between function macros and regular text macros that start with an open paren:

```
#define MACRO1 (p) (p)
#define MACRO2(p) (p)
```
the second is a function macro, so the spaces are significant!

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

answer:
- could be made cleaner by starting the nesting level at 1 instead of 0
- because it stores the level of nesting, it makes the language no longer _regular_

### Representing Code

we're generating some Java code, as strings, using Java... fun!

see jlox-java/tool/GenerateAst.java

1. Earlier, I said that the |, *, and + forms we added to our grammar metasyntax were just syntactic sugar. Given this grammar:

expr → expr ( "(" ( expr ( "," expr )* )? ")" | "." IDENTIFIER )*
     | IDENTIFIER
     | NUMBER

Produce a grammar that matches the same language but does not use any of that notational sugar.

expr → expr x
expr → IDENTIFIER
expr → NUMBER

x → x x
x → tuple
x → "." IDENTIFIER

tuple → "()"
tuple → "(" elist ")"

elist → expr
elist → expr "," elist

Bonus: What kind of expression does this bit of grammar encode?

number literals, object property access, and function calls, including args that are also function calls, and also trailing etcs on them. somewhat unfortunate that both numbers and identifiers can appear in the expression box, since it's weird to see arg.7.8.9.10

ident
5
ident.ident.ident()
5.ident
5()
5()()
5(arg, arg, 5.6.8)


2. The Visitor pattern lets you emulate the functional style in an object-oriented language. Devise a corresponding pattern in a functional language. It should let you bundle all of the operations on one type together and let you define new types easily.

(SML or Haskell would be ideal for this exercise, but Scheme or another Lisp works as well.)

visitor pattern: 
abstract class, with subclasses
  want to make a new kind of thing-that-deals-with-subclasses, but not stick the behavior in the subclasses themselves
  because it, seemingly, belongs in it's own class (makes sense!)
So, you make a function 'accept' in the abstract class
  and override it in the subclasses
  each subclass calling a method like 'visitMySubClass' in their version of 'accept'
  as a way of dispatching the 'right' version of the method when you write it in your implementing class
  then the implementing class implements the 'visitSubClass' methods to deal with each kind of subclass
  and a 'dispatcher' method that calls the 'accept' method of the abstract class, (and has a type signature that accepts the abstract class)

note: imo, this looks a lot like 'dependency inversion', to my naive eyes

So, in a functional world, what does the corresponding pattern look like?

> It should let you bundle all of the operations on one type together and let you define new types easily.

So... like making classes in Lisp?

I... don't know if I understand what the code would do
or what problem it would solve...
Let's take a stab anyhow.

un-dependency-inverter?
operate-on-data

or is this just trying to illustrate that this isn't a problem in Lisps? idk, I'm not sure I quite get it

<T> -> computed

make something into a T?
interfaces?

feels very much like a guessing the password issue. Maybe it would make sense if I were writing it in Scheme or Haskell?

Answer:
> One way is to create a record or tuple containing a function pointer for each operation. In order to allow defining new types and passing them to existing code, these functions need to encapsulate the type entirely -- the existing code isn't aware of it, so it can't type check. You can do that by having the functions be closures that all close over the same shared object, "this", basically.

I... don't know if I can visualize what such code would look like tbh

3. In Reverse Polish Notation (RPN), the operands to an arithmetic operator are both placed before the operator, so 1 + 2 becomes 1 2 +. Evaluation proceeds from left to right. Numbers are pushed onto an implicit stack. An arithmetic operator pops the top two numbers, performs the operation, and pushes the result. Thus, this:

(1 + 2) * (4 - 3)

in RPN becomes:

1 2 + 4 3 - *

Define a visitor class for our syntax tree classes that takes an expression, converts it to RPN, and returns the resulting string.

jlox/RpnPrinter.java


// TODO: translate ch. 5 into rust

### Parsing Expressions

jlox/Parser.java

// TODO: translate ch. 6 into rust

1. In C, a block is a statement form that allows you to pack a series of statements where a single one is expected. The comma operator is an analogous syntax for expressions. A comma-separated series of expressions can be given where a single expression is expected (except inside a function call’s argument list). At runtime, the comma operator evaluates the left operand and discards the result. Then it evaluates and returns the right operand.

Add support for comma expressions. Give them the same precedence and associativity as in C. Write the grammar, and then implement the necessary parsing code.

2. Likewise, add support for the C-style conditional or “ternary” operator ?:. What precedence level is allowed between the ? and :? Is the whole operator left-associative or right-associative?

3. Add error productions to handle each binary operator appearing without a left-hand operand. In other words, detect a binary operator appearing at the beginning of an expression. Report that as an error, but also parse and discard a right-hand operand with the appropriate precedence.

