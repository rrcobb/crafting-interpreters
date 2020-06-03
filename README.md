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

### Parsing Expressions

jlox/Parser.java

1. In C, a block is a statement form that allows you to pack a series of statements where a single one is expected. The comma operator is an analogous syntax for expressions. A comma-separated series of expressions can be given where a single expression is expected (except inside a function call’s argument list). At runtime, the comma operator evaluates the left operand and discards the result. Then it evaluates and returns the right operand.

Add support for comma expressions. Give them the same precedence and associativity as in C. Write the grammar, and then implement the necessary parsing code.

2. Likewise, add support for the C-style conditional or “ternary” operator ?:. What precedence level is allowed between the ? and :? Is the whole operator left-associative or right-associative?

- right to left associative
- low precedence (above comma, below equality)

3. Add error productions to handle each binary operator appearing without a left-hand operand. In other words, detect a binary operator appearing at the beginning of an expression. Report that as an error, but also parse and discard a right-hand operand with the appropriate precedence.

Parser.java L155

- tool/GenerateAst.java
- Expr.java (which Java generates with GenerateAst.java
- AstPrinter.java
- RpnPrinter.java
- Parser.java

Notes:
- jlox-java Expr subclasses implement the Visitor pattern, so that e.g. AstPrinter can implement for each type but dispatch to the correct version with a type signature that uses a Generic.
- In Rust, we don't need to do this - all the 'subclasses' will just impl a trait, so a function can just accept a type with that trait - see https://blog.rust-lang.org/2015/05/11/traits.html

Actually, seems like my traits knowledge is too weak, so made the visitor pattern anyway. It could be because I can't quite see how to do the types right for Expr, but w/e it's working okay now.

Broken: ast_generator.rs
working, ish: expr.rs, ast_printer.rs

Fixed a bunch of things, but the parser is now compiling.

parser issues:
- error handling

## Evaluating Expressions

Challenges:
1. Allowing comparisons on types other than numbers could be useful. The syntax is shorter than named function calls and might have a reasonable interpretation for some types like strings. Even comparisons among mixed types, like `3 < "pancake"` could be handy to enable things like heterogeneous ordered collections. Or it could lead to bugs and confused users.

Would you extend Lox to support comparing other types? If so, which pairs of types do you allow and how do you define their ordering? Justify your choices and compare them to other languages.

guess: comparison sounds dangerous, cut it

more honestly: if there are nice types and traits, then ordering things seems like a nice trait to have. Otherwise, you're just overloading the operators, which... idk, seems like a weird way for a language to work.

Ruby's method call syntax is nice, because it places operator overloading firmly within a 'normal' framework (even though the infix operators and assignment get some special weirdness in parsing).

types that seem orderable:
- strings against other strings (maybe length or alpha)
- ... idk, booleans? what other types do we have? only the three kinds of literals, I think...
- soo, maybe we need more types in order to think about overloading the > operator. If we had ints and floats, that usually seems pretty natural.

answer:
python has numeric comparison and string comparison (lexicographic), and a cool comparison of sets (partial ordering by subset / superset)

Seems obnoxious until you're doing lots of set comparisons, probably

2. Many languages define + such that if either operand is a string, the other is converted to a string and the results are then concatenated. For example, "scone" + 4 would yield scone4. Extend the code in visitBinaryExpr() to support that.

I could, but I won't! It's so baaaad!

Okay, I did, and it should be purged

3. What happens right now if you divide a number by zero? What do you think should happen? Justify your choice. How do other languages you know handle division by zero and why do they make the choices they do?

Change the implementation in visitBinaryExpr() to detect and report a runtime error for this case.

Guess: Currently yields Java Infinity, which is _wild_. I didn't know that about Java!

Fixed so it throws.

Implementation is not really so hard, but missing the error handling from before really is coming back to bite.

I should go through and figure out what error types ought to propagate where, but I am not sure yet.

Rust interpreter implemented, though it's all panicky
Also fixed a parser bug with the unary grouping logic, it was dumb, and it is possibly dumber now

But it works

## Chapter 8 Statements and State

Lots to do in this chapter, so much going on

- expression statements and print statements
- expr statement: expr;
- print statement: print expr;

one of the deep differences between statements and expressions is that expressions produce a value, and statements don't.


Todo: make the ternary operator work again, fix the grammar to reflect it properly
