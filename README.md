# Reading 'Crafting Interpreters'

Bob Nystrom's http://craftinginterpreters.com/
Notes will go here in this Readme until I need more pages for clarity

## Languages

I'm following along with the book, so I'll do the implementations in Java and C. 

I'm also practicing Rust, and want to practice my translation skills, so I'll do the Java and C implementations in Rust too.


## Status

- jlox-java: Finished! Didn't do every challenge, but jlox-java works!
- jlox-rust: Finished Chapter 8, starting chapter 9
- clox-c: finished chapter 22 on locals, haven't completed all the challenges
- clox-rust: unstarted. not 100% clear that transliteration will be smooth, but we'll give it a go. Helpfully, rustc is cleverer than clang about modules, so we don't have as much #include dance (though, we'll still have some macros to write!)

### jlox-java

```sh
./jlox-java/rebuild.sh
```

### jlox-rust

```sh
Cargo run
```

### clox-c

```sh
clang -o main.out *.c && ./main.out
```

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

Note: added jlox-java/rebuild.sh later on, which does this + the code-generation from running tool/GenerateAst.

3. Do the same thing for C. To get some practice with pointers, define a doubly-linked list of heap-allocated strings. Write functions to insert, find, and delete items from it. Test them.

Note: these practice files are moved to clox-c/practice

`clox-c/hello.c`

`clang clox-c/hello.c -o clox-c/hello.out`

`./clox-c/hello.out`

`clox-c/dll.c`

`clang clox-c/dll.c -o clox-c/dll.out`

`./clox-c/dll.out`


Build and run clox:
```
clang clox-c/*.c -o main.out
./clox-c/main.out
```
And if that gets slow, I'll think about learning / using CMake


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

In progress: Rust for ch 8

- DONE statements to the 'generator'
- DONE this will actually mean adding an stmt.rs file I think
- DONE parser updates for statements
- DONE interpreter updates for statements
- DONE adding assignment expressions
- DONE environment to keep track of state
- DONE block statements

Fought it hard, but ended up using Rc and RefCell for the environment and the interpreter reference to the environment. I thought I could get away with Box, but apparently, no.

DONE Getting a parser error, probably not doing the parsing right (the parser error was using leftparen and rightparen instead of leftbrace and rightbrace, like a bozo)

Rust version of the interpreter still panics in bad ways, which is the current worst thing

Challenges:
1 evaluate expressions in the REPL

Add a check for the whole program, and switch to calling the evaluate expression path if the program is a single expression

questions: 
- should this logic go in the parser or the interpreter?
- should it be a global setting on the parser?
- should it allow all kinds 'statements without ;', or just expression statements?

Possible: if the parser sees just one expression statement, and it's at the end of parsing an expression statement, then it can recover from the error and the expression into an expression statement, then pass it to the interpreter.

Passed a 'loose' option to the parser to get the expressions to pass the parsing without semicolons. Making the intepreter print out the result of expression statements, but only in repl mode, is too much annoying work. Committing all this to ch-8-1 branch and not master.

2 remove initialization to nil, and make runtime access of undefined an error

- Still need a way to define a variable without assignment.
- So, need to have some kind of signal or marker for undefined variables
- Can still have 'null' literally, so have to be careful that we're not swallowing a 'real' null
- set up a single-member enum, initialized the value to that, handled it in the variable 'get' logic in environment.

3 deal with strange scope program in conflicting_scope.lox

> What does the following program do?
> 
> var a = 1;
> {
>   var a = a + 2;
>   print a;
> }
> What did you expect it to do? Is it what you think it should do? What does analogous code in other languages you are familiar with do? What do you think users will expect this to do?

It prints 3, and leaves the outer-scoped 'a' with the value 1.

This sorta makes sense, since the lookup happens 'before' the local variable is created. weird, though, because maybe it should be an error for accessing the uninitialized 'a' before it's created. To 'fix' this behavior to raise this error, maybe you have to register the declaration on the lhs, create that 'slot' before you evaluate the expression on the rhs. Solving challenge #2 should help with this.

Solving challenge 2 did indeed help, all that challenge three took was adding a single line to define the variable before evaluating the initializer in `visitVarStmt` in the interpreter.


## Chapter 9

Note: Desugaring is another way we could have implemented the parsing for the challenges in chapter 8. e.g. instead of having the parser and intepreter deal with 'looseness', it could construct an 'artificial' print statement.

### Challenges
1. A few chapters from now, when Lox supports first-class functions and dynamic dispatch, then we technically won’t need branching statements built into the language. Show how conditional execution can be implemented in terms of those. Name a language that uses this technique for its control flow.

guess: does haskell do this? various lisps? I don't think they have 'if', but instead use the cool match expressions, whcih I assume is what is meant by dynamic dispatch here.

answer: apparently, it's smalltalk, and the 'true' and 'false' functions on a class. So, you send a message to the class with a block to execute, and it knows what to do

2. Likewise, looping can be implemented using those same tools, provided our interpreter supports an important optimization. What is it, and why is it necessary? Name a language that uses this technique for iteration.

recursion, tail call optimization (so it doesn't blow the stack!)

Answer: yes, nailed it. Also, tail call optimization means discarding the old call stack when you have a tail call, so, the more you know.

3. Unlike Lox, most other C-style languages also support break and continue statements inside loops. Add support for break statements.

The syntax is a break keyword followed by a semicolon. It should be a syntax error to have a break statement appear outside of any enclosing loop. At runtime, a break statement causes execution to jump to the end of the nearest enclosing loop and proceeds from there. Note that the break may be nested inside other blocks and if statements that also need to be exited.

woof. okay, so parsing is not that hard, it's just another kind of statement. Interpreting, visitBreakStmt would have to know how to jump out of a loop. hmm. 

Ideas: 
- add a marker like 'is breaking' and skip other things if it's true, set to true in visit break, set to false at the end of visitWhile

Answer: correct to assume this sucks, it's a lot of changes. Implemented with exceptions in the answer guide, which, eh. Sucks. Wait, couldn't it lead to undefined behavior? E.g. if there's some environment cleanup that needs to happen, and it gets skipped?

## Chapter 10

functions are very cool

Also, it looks like the fibonacci program is wrong right now. Do I have a bug, or is it because of the way that environments are defined?

`executeBlock` was super messed up... not sure what happened to it, but it was v borked

Bug fixed, unclear exactly how I messed it up that bad. oh well.

### Chapter 10 Challenges

1. Our interpreter carefully checks that the number of arguments passed to a function matches the number of parameters it expects. Since this check is done at runtime on every call, it has a real performance cost. Smalltalk implementations don’t have that problem. Why not?

Hmm, maybe I need to learn smalltalk :|. My guess: method calls are messages sent to functions, so arity isn't a runtime check?

Smalltalk has different call syntax for different arities. To define a method that takes multiple arguments, you use keyword selectors. Each argument has a piece of the method name preceding instead of using commas as a separator. For example, a method like:

```
list.insert("element", 2)
To insert "element" as index 2 would look like this in Smalltalk:
```

```
list insert: "element" at: 2
```

Smalltalk doesn't use a dot to separate method name from receiver. More interestingly, the "insert:" and "at:" parts both form a single method call whose full name is "insert:at:". Since the selectors and the colons that separate them form part of the method's name, there's no way to call it with the wrong number of arguments. You can't pass too many or two few arguments to "insert:at:" because there would be no way to write that call while still actually naming that method.

Answer: 

2. Lox’s function declaration syntax performs two independent operations. It creates a function and also binds it to a name. This improves usability for the common case where you do want to associate a name with the function. But in functional-styled code, you often want to create a function to immediately pass it to some other function or return it. In that case, it doesn’t need a name.

Languages that encourage a functional style usually support “anonymous functions” or “lambdas”—an expression syntax that creates a function without binding it to a name. Add anonymous function syntax to Lox so that this works:

```
fun thrice(fn) {
  for (var i = 1; i <= 3; i = i + 1) {
    fn(i);
  }
}

thrice(fun (a) {
  print a;
});
// "1".
// "2".
// "3".
```

How do you handle the tricky case of an anonymous function expression occurring in an expression statement:

```
fun () {};
```

Guess: Instead of the environment definition part of visitFunctionStatement, we want to have a visitfunExpr that just evaluates to a function expression. (not doing this, because I'm a little bored). So, add a new expr to GenerateAst.java, add it to the grammar (hmm) add it to the parser, and evaluation creates a LoxFunction, which can be stored or called. I _think_ you can just ignore anonymous function expressions that occur in an expression statement, since they don't get used, but that's an optimization. Is it hard to parse for some reason? Maybe because it gets picked up by the function statement parser, so it'll error. That is maybe 'right', but if this is to be treated as legit (which I guess it should be) then the grammar really treats the fn name as optional.

3. Is this program valid?

```
fun scope(a) {
  var a = "local";
}
```

In other words, are a function’s parameters in the same scope as its local variables, or in an outer scope? What does Lox do? What about other languages you are familiar with? What do you think a language should do?

Guess: right now, it's valid, and lox just overwrites it. That seems okay - it's what javascript does, and it is used to e.g. add default values. It's maybe a little bit of a footgun, but not the worst. Immutable variables would probably make this an illegal name collision.

## Chapter 11

The chapter starts off with an example `showA`, which... doesn't fail as it should given my version of jlox! I think this is probably a bug in my implementation of blocks or environments, but I can't tell.

Problem was, I didn't implement all the code in the functions chapter, specifically, missed the bit about closures. :/

### Challenges

1. Why is it safe to eagerly define the variable bound to a function’s name when other variables must wait until after they are initialized before they can be used?

Guess: function definitions referring to themselves isn't a bug - recursive variable definitions don't work because they can't have a base case.

answer: no, it's because actually starting using the function will happen later, after definition is finished, not right away. It's about timing, not about power.

2. How do other languages you know handle local variables that refer to the same name in their initializer, like:

var a = "outer";
{
  var a = a;
}
Is it a runtime error? Compile error? Allowed? Do they treat global variables differently? Do you agree with their choices? Justify your answer.

Guess: I don't think I hate this as much as the author, though it does seem like a footgun. Node allows it, ruby doesn't have bare blocks, neither does python. I think this is allowed in Rust, though I should check.

3. Extend the resolver to report an error if a local variable is never used.

- Add to the representation of the scope stack in the resolver whether or not a variable has been referenced
- mark true when referenced
- when stack is popped, check that all values are true
    might be good to change to tokens rather than strings in the Map if so, so that the errors can be helpful.

Answer: enum instead of boolean in the scope map

4. Our resolver calculates which environment the variable is found in, but it’s still looked up by name in that map. A more efficient environment representation would store local variables in an array and look them up by index.

Extend the resolver to associate a unique index for each local variable declared in a scope. When resolving a variable access, look up both the scope the variable is in and its index and store that. In the interpreter, use that to quickly access a variable by its index instead of using a map.

Guess: Fancy! have to track the current count in each scope, which sounds more and more like a full-fledged class. Implementation doesn't seem all that hard, except maybe getAt has a new dependency on the scope? Actually, all the environment `get` and `set` stuff has to be updated to use array indices. Note, this also makes debugging the environment much harder :/

## Chapter 12: Classes

Challenges:
1. We have methods on instances, but there is no way to define “static” methods that can be called directly on the class object itself. Add support for them. Use a class keyword preceding the method to indicate a static method that hangs off the class object:

```
class Math {
  class square(n) {
    return n * n;
  }
}

print Math.square(3); // Prints "9".
```

You can solve this however you like, but the “metaclasses” used by Smalltalk and Ruby are a particularly elegant approach. Hint: Make LoxClass extend LoxInstance and go from there.

2. Most modern languages support “getters” and “setters”—members on a class that look like field reads and writes but that actually execute user-defined code. Extend Lox to support getter methods. These are declared without a parameter list. The body of the getter is executed when a property with that name is accessed:

```
class Circle {
  init(radius) {
    this.radius = radius;
  }

  area {
    return 3.141592653 * this.radius * this.radius;
  }
}

var circle = Circle(4);
print circle.area; // Prints roughly "50.2655".
```

3. Python and JavaScript allow you to freely access an object’s fields from outside of its own methods. Ruby and Smalltalk encapsulate instance state. Only methods on the class can access the raw fields, and it is up to the class to decide which state is exposed. Most statically typed languages offer modifiers like private and public to control which parts of a class are externally accessible on a per-member basis.

What are the trade-offs between these approaches and why might a language prefer one or the other?


## Chapter 13

1. Lox only supports single inheritance—a class may have a single superclass and that’s the only way to reuse methods across classes. Other languages have explored a variety of ways to more freely reuse and share capabilities across classes: mixins, traits, multiple inheritance, virtual inheritance, extension methods, etc.

If you were to add some feature along these lines to Lox, which would you pick and why? If you’re feeling courageous (and you should be at this point), go ahead and add it.

Guess:

Multiple inheritance is kind of a bad idea, but mixins is a decent approach. I like ruby's definition-order-as-precedence, but there's still ways it's a footgun. Rust's traits and impls are kinda cool. How would they work in lox? Maybe not well.

2. In Lox, as in most other object-oriented languages, when looking up a method, we start at the bottom of the class hierarchy and work our way up—a subclass’s method is preferred over a superclass’s. In order to get to the superclass method from within an overriding method, you use super.

The language BETA takes the opposite approach. When you call a method, it starts at the top of the class hierarchy and works down. A superclass method wins over a subclass method. In order to get to the subclass method, the superclass method can call inner, which is sort of like the inverse of super. It chains to the next method down the hierarchy.

The superclass method controls when and where the subclass is allowed to refine its behavior. If the superclass method doesn’t call inner at all, then the subclass has no way of overriding or modifying the superclass’s behavior.

Take out Lox’s current overriding and super behavior and replace it with BETA’s semantics. In short:

When calling a method on a class, prefer the method highest on the class’s inheritance chain.

Inside the body of a method, a call to inner looks for a method with the same name in the nearest subclass along the inheritance chain between the class containing the inner and the class of this. If there is no matching method, the inner call does nothing.

For example:

```
class Doughnut {
  cook() {
    print "Fry until golden brown.";
    inner();
    print "Place in a nice box.";
  }
}

class BostonCream < Doughnut {
  cook() {
    print "Pipe full of custard and coat with chocolate.";
  }
}

BostonCream().cook();
This should print:

Fry until golden brown.
Pipe full of custard and coat with chocolate.
Place in a nice box.
```

(Skipping, but it seems straightforward: change the lookup order in findMethod, treat `inner` much the same way that `super` is treated, except that it's easier to resolve. For cases where there is no subclass, or the subclass doesn't implement the refinement, either add in an empty function, skip it - do nothing (in visitInnerExpr).

This seems like a counterintuitive way for inheritance to work, but I guess that's why it's just one niche/unused language where it's the pattern.

3. In the chapter where I introduced Lox, I challenged you to come up with a couple of features you think the language is missing. Now that you know how to build an interpreter, implement one of those features.

- objects are kind of possible with class instances. Making them more like js objects, using a dynamic / calculated property name, would make them pretty versatile.
- arrays would be nice
- random would be nice (built like the `clock`)
- basically, standard library would be cool
- string interpolation, regex, errors

String interpolation is a funky feature, since it reaches deep into the lexer. Inside of strings are other expressions, interpolation expressions. Then, you have to visit each of those expresions, evaluate them, and stringify them in order to build the interpolated string expression. Neat that languages have this.

# Part 2

## Chapter 14

Bytecode, wheeeeeeeeeeeeeeee!!!

C is a very different language than Java. Less safe, (maybe less expressive?) but also, free from some of the annoying bits of Java. Loading related files remains a PitA.

### Challenges

1. Our encoding of line information is hilariously wasteful of memory. Given that a series of instructions often correspond to the same source line, a natural solution is something akin to run-length encoding of the line numbers.

Devise an encoding that compresses the line information for a series of instructions on the same line. Change writeChunk() to write this compressed form, and implement a getLine() function that, given the index of an instruction, determines the line where the instruction occurs.

Hint: It’s not necessary for getLine() to be particularly efficient. Since it is only called when a runtime error occurs, it is well off the critical path where performance matters.

Guess:
make each entry in the lines array an (instruction_count). Then, as you encode the bytes, increment the instruction count for the line until the end of the line, until the next line. getLine() takes the instruction count and adds up the lines instruction count until it gets to the line. That makes it getLine() roughly linear in _previous bytecode instructions_, but I think that's sorta fine?

2. Because OP_CONSTANT only uses a single byte for its operand, a chunk may only contain up to 256 different constants. That’s small enough that people writing real-world code will hit that limit. We could use two or more bytes to store the operand, but that makes every constant instruction take up more space. Most chunks won’t need that many unique constants, so that wastes space and sacrifices some locality in the common case to support the rare case.

To balance those two competing aims, many instruction sets feature multiple instructions that perform the same operation but with operands of different sizes. Leave our existing one-byte OP_CONSTANT instruction alone, and define a second OP_CONSTANT_LONG instruction. It stores the operand as a 24-bit number, which should be plenty.

Implement this function:

void writeConstant(Chunk* chunk, Value value, int line) {
  // Implement me...
}

It adds value to chunk’s constant array and then writes an appropriate instruction to load the constant. Also add support to the disassembler for OP_CONSTANT_LONG instructions.

Defining two instructions seems to be the best of both worlds. What sacrifices, if any, does it force on us?

Guess: check the size of the constants array, if it's at max, then switch to writing OP_CONSTANT_LONGs. See branch long_constants for the implementation.

Sacrifice: maintaining a larger instruction set imposes maintenance burden, which reduces ability to optimize code. Easy to make a big ol' bug, easy to accidentally make slowdown-types of mistakes.

Better answers and better C code in the answer guide, author also chose little-endian as opposed to my bigendian choice.

Long const is pretty complicated! bitshifting? crazy! Who wants to keep track of that? _might_ be worth it, but maybe not!

3. Our reallocate() function relies on the C standard library for dynamic memory allocation and freeing. malloc() and free() aren’t magic. Find a couple of open source implementations of them and explain how they work. How do they keep track of which bytes are allocated and which are free? What is required to allocate a block of memory? Free it? How do they make that efficient? What do they do about fragmentation?

Hardcore mode: Implement reallocate() without calling realloc(), malloc(), or free(). You are allowed to call malloc() once, at the beginning of the interpreter’s execution, to allocate a single big block of memory which your reallocate() function has access to. It parcels out blobs of memory from that single region, your own personal heap. It’s your job to define how it does that.

This is an interesting distraction, but a distraction nonetheless, so I won't.

## Chapter 15

Executing instructions in the tiny VM.

Notes:
- remember in C, #define with a function basically inlines the function, by replacing things that look like the the function call with the literal code. That's _faster_ in many cases, because it removes the indirection and overhead of a function call. It's got pitfalls, so we're using it in the `run()` function in particular.
- we're also using fairly standard #include and #ifndef/ #define as a once-only header mechanism
- Also we're starting to do pointer stuff, since we're building the stack. Cool. 
- It's a little hard to see the whole design space of programming languages, and what other options there are for pieces of this. I guess playing with the pieces once they're working is one way to get that sense, but maybe good to truck through most of the book first is a good idea.
- we're also doing some _funky_ preprocessor stuff. See #define BINARY_OP and the book notes around it.
- "Even though the arithmetic operators take operands—which are found on the stack—the arithmetic bytecode instructions do not." This is somewhat remarkable - we're turning the language into a stack-calculator, which, I guess I didn't realize was possible.


### Challenges
1. What bytecode instruction sequences would you generate for the following expressions:

1 * 2 + 3
1 + 2 * 3
3 - 2 - 1
1 + 2 * 3 - 4 / -5

(Remember that Lox does not have a syntax for negative number literals, so the -5 is negating the number 5.)

A: see main.c at commit 923b1ed20760e2fa7ffcd176f5aefda6e2e99fe6 for solutions

2. If we really wanted a minimal instruction set, we could eliminate either OP_NEGATE or OP_SUBTRACT. Show the bytecode instruction sequence you would generate for:

4 - 3 * -2

First, without using OP_NEGATE. Then, without using OP_SUBTRACT.

Given the above, do you think it makes sense to have both instructions? Why or why not? Are there any other redundant instructions you would consider including?

A: 
without negate:
4 - 3 * ( 0 - 2)
without subtract:
4 + (-3) * -2

fewer instructions (smaller bytecode) when we support more ops.
but, the inner loop gets a little longer / more complicated
that cost we pay again and again. Would need information about programs in order
to tell which is actually right - it's just a pain for language implementers to
miss one or the other, not language users - so, whichever is actually faster is
better. Likely better to support it, though that might be forgetting the lesson
from RISC.

From the mouth of the man himself:

> I do think it makes sense to have both instructions. The overhead of dispatching is pretty high, so you want instructions as high level as possible, you want to fill your opcode space, and you want common operations to encode as a single instruction when possible.

> Given how common both negation and subtraction are, and given that we've got plenty of room in our opcode set, it makes perfect sense to have instructions for both.

> I would also consider specialized instructions to load common number constants like zero and one. It might be worth having instructions to increment and decrement a number too.

3. Our VM’s stack has a fixed size, and we don’t check if pushing a value overflows it. This means the wrong series of instructions could cause our interpreter to crash or go into undefined behavior. Avoid that by dynamically growing the stack as needed.

What are the costs and benefits of doing so?

- nice to grow the stack _some_ since otherwise we can't run big programs
- stack is an implicit limit on depth of recursion, so infinite recursion gets a
    stack overflow
- definitely nice to error instead of crashing or going into undefined behavior
- cost of dynamic growing is a check on the stack size each time we add something
    to the stack (plus the complexity + penalty when we resize the stack -
    there's now an implementation-dependent performance hit when we do certain
    operations)

Reused the ValueArray type to achieve, but not cleanly. Bad code in
947f725bf11c7e96c85f8661eae65ac328056e86, cleaning up so that my mistakes don't
haunt me forever

4. To interpret OP_NEGATE, we pop the operand, negate the value, and then push the result. That’s a simple implementation, but it increments and decrements ip unnecessarily, since the stack ends up the same height in the end. It might be faster to simply negate the value in place on the stack and leave ip alone. Try that and see if you can measure a performance difference.

```
case OP_NEGATE:   *(vm.stackTop - 1) = -*(vm.stackTop - 1); break;
```

- I could attempt to measure a perf difference; but I assume it's pretty small

Are there other instructions where you can do a similar optimization?

- We could do more direct work on the stack - add directly to the pointer,
instead of doing two pops for a binary op. Seems like it wouldn't be much
faster, but then again, maybe I'm missing something.

## Chapter 16 Scanning On Demand 

1. Many newer languages support string interpolation. Inside a string literal, you have some sort of special delimiters—most commonly ${ at the beginning and } at the end. Between those delimiters, any expression can appear. When the string literal is executed, the inner expression is evaluated, converted to a string, and then merged with the surrounding string literal.

For example, if Lox supported string interpolation, then this:

```
var drink = "Tea";
var steep = 4;
var cool = 2;
print "${drink} will be ready in ${steep + cool} minutes.";
```
Would print:

`Tea will be ready in 6 minutes.`

What token types would you define to implement a scanner for string interpolation? What sequence of tokens would you emit for the above string literal?

What tokens would you emit for:

`"Nested ${"interpolation?! Are you ${"mad?!"}"}"`

Consider looking at other language implementations that support interpolation to see how they handle it.

HMMMMM.

Seems super hard in clox, but maybe easier in jlox, since we'd have the objects
point to each other.

Maybe we'd emit something like

STRING
INTERP
STRING
INTERP
STRING

and they'd encompass each other?

Makes dealing with strings in the compiler much harder, but hey, maybe that's
the compiler's problem. Overlapping tokens? :|

Alternatively... maybe we manipulate the source, to leave slots more like a fmt
string?

> author answer 

string before interpolation gets special 'TOKEN_STRING_INTERP', which ends at
the interpolation start - other tokens get emitted until the end of the
interpolation. Then the last bit gets a normal 'string' token.

Seems like a reasonable answer, no idea how other langs do it. Tagged template
literals make it seem like js handles '\`' delimited strings differently.

2. Several languages use angle brackets for generics and also have a >> right shift operator. This led to a classic problem in early versions of C++:

`vector<vector<string>> nestedVectors;`

This would produce a compile error because the >> was lexed to a single right shift token, not two > tokens. Users were forced to avoid this by putting a space between the closing angle brackets.

Later versions of C++ are smarter and can handle the above code. Java and C# never had the problem. How do those languages specify and implement this?

C++ answer:
http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2005/n1757.html

Apparently the C# and Java specs don't spec this right, and the parsers do hax
to keep things working - C# by making two > tokens from the bat, and Java by
treating a >> token as two `>`s if it needs to. Wild.

3. Many languages, especially later in their evolution, define “contextual keywords”. These are identifiers that act like reserved words in some contexts but can be normal user-defined identifiers in others.

For example, await is a keyword inside an async method in C#, but in other methods, you can use await as your own identifier.

Name a few contextual keywords from other languages, and the context where they are meaningful. What are the pros and cons of having contextual keywords? How would you implement them in your language’s front end if you needed to?

`async/await` is the big one here, I guess `case` is sorta contextual inside of
`switch` too, sorta? idk, they're both reserved, it's a syntax error outside of
a switch I suppose. Now that I think about it, tons of reserved words only work
in certain contexts, that's the point of the grammar, duhhhh. Usually those are
implemented at the scanner level, contextual keywords in this sense are actually
dealt with in the compiler / interpreter - they get treated as identifiers in
the interim :o

### ch 17 challenges

1.  (-1 + 2) * 3 - -4

> Write a trace of how those functions are called. Show the order they are called, which calls which, and the arguments passed to them.

- compile (called by interpret, called by repl in main.c)
  - given a blank chunk and the source, inits a scanner for the source
  - advances once, over the '('
- calls expression() with no args
  - calls parsePrecedence with PREC_ASSIGNMENT
  - advances over the `-`
  - gets the prefixrule for `(` (since that's previous)
  - prefixrule is 'grouping'
  - calls grouping
    - calls expression
    - calls parsePrecedence with PREC_ASSIGNMENT
        - advances over the `1`
        - gets prefix rule for `-` (unary)
        - calls `unary`
          - calls parsePrecedence with PREC_UNARY
          - advances over the `+`
          - gets 'number' as the prefix rule
          - calls `number`
            - emits a constant 
          - PREC_UNARY is more than the precedence of `+` (PREC_TERM) so we skip
              the loop
          - emits the op_negate byte
      - PREC_ASSIGNMENT is lower than PREC_TERM of `+`
        - so we enter the while loop and advance to `2`
        - pull the infixRule from `+`, which is `binary`
        - and call binary()
          - we call parsePrecedence with PREC_TERM + 1, which I guess is
              PREC_FACTOR
              - advance takes us over `(`
              - number() prefix rule
                - just emits the constant 2
              - token right paren has PREC_NONE, which is less than PREC_FACTOR,
                  so we skip the while loop
          - then we emit an OPP_ADD
    - consumes the right token, which advances to `*`
  - PREC_ASSIGNMENT is less than PREC_FACTOR from the token_star lookup in the
      table, so we enter the while loop
      - advance to the 3
      - get the infixRule from `*`, binary, call it
        - calls parsePrecedence with PREC_FACTOR + 1, PREC_UNARY
            - advances over the `-`
            - gets the prefix rule for `3`, which is number
              - calls it, adds constant op
            - PREC_FACTOR is more than PREC_TERM, which is the precedence of the
                `-`, so we skip the while loop
        - then adds the OP_MULTIPLY
  - PREC_ASSIGNMENT is less than PREC_TERM, so we run the while loop again
    - advance over the second `-`
    - get the prefix rule for `-`, which is 
... and so on


2. The ParseRule row for TOKEN_MINUS has both prefix and infix function pointers. That’s because - is both a prefix operator (unary negation) and an infix one (subtraction).

In the full Lox language, what other tokens can be used in both prefix and infix positions? What about in C or another language of your choice?
- the minus operator
- (I struggled, but the left paren is apparently the other)
- in other languages, there's a '+' prefix operator

3. You might be wondering about complex “mixfix” expressions that have more than two operands separated by tokens. C’s conditional or “ternary” operator, ?: is a widely-known one.

Add support for that operator to the compiler. You don’t have to generate any bytecode, just show how you would hook it up to the parser and handle the operands.

Yikes! Gonna skip the mixfix operator, but it seems hard.

### 18: Types of values

1. We could reduce our binary operators even further than we did here. Which other instructions can you eliminate, and how would the compiler cope with their absence?

- greater and less than could be collapsed into subtract and equal, at the cost
    of complexity and more ops
- like we did before, add and substract can be collapsed into just subtract
- similarly, I think multiply can be collapsed into divide

A: negate and subtract are redundant

2. Conversely, we can improve the speed of our bytecode VM by adding more specific instructions that correspond to higher-level operations. What instructions would you define to speed up the kind of user code we added support for in this chapter?

- The binary operators we left out (bang equal, greater equal, less equal)

A: 
- small integer constant loads
- incrementing and decrementing by one
- doubling
- comparison with zero (maybe also true / false?)

### 19: Strings

### 20: Hash Tables

### 21: Global Variables

### 22: Local Variables

TODO: Implement the optimization for OP_POPN, mentioned at the 22.4 heading.

1. Our simple local array makes it easy to calculate the stack slot of each local variable. But it means that when the compiler resolves a reference to a variable, we have to do a linear scan through the array.

Come up with something more efficient. Do you think the additional complexity is worth it?

- We could keep a map
- Probably not worth it in our little C implementation, because it's a huge PITA
- and it's at compile time, not runtime, so it doesn't *matter* as much
- but, in a language with easy maps (like we had in jlox) it's pretty nice to
    use them
- key question: how large is our array likely to get? In lox programs, we can't have more than 256 locals, so it is moot. In other langs (or if we had larger programs), there could be a lot of named things. Compilation is already linear in program length, but this would make that particularly painful.


2. How do other languages handle code like this:

```
var a = a;
```

What would you do if it was your language? Why?

- often, resolve to an outer-scoped version of that variable
- so I'd probably do that, since it's what people are accustomed to
- but, idunno, shadowing isn't all that great

3. Many languages make a distinction between variables that can be reassigned and those that can’t. In Java, the final modifier prevents you from assigning to a variable. In JavaScript, a variable declared with let can be assigned, but one declared using const can’t. Swift treats let as single-assignment and uses var for assignable variables. Scala and Kotlin use val and var.

Pick a keyword for a single-assignment variable form to add to Lox. Justify your choice, then implement it. An attempt to assign to a variable declared using your new keyword should cause a compile error.

- name "const" implies no more reassignment
- though, it may imply constancy, which for mutable objects is bad
- maybe "once" or "invar" like 'invariable'

Implementation: bool in the local / global struct? a whole new type? Probably
mark it in the struct, though I forget how globals are represented, so it'd be
annoying there.

4. Extend clox to allow more than 256 local variables to be in scope at a time.

number of locals is fixed by the UINT8_COUNT in the array in the Compiler
struct. But also, I think we're keeping a uint8 to index into that array? would
have to bump up the sizes of all those, or something (e.g. replace the array with something else).

### 23: Jumping Back and Forth

Notes: had stupid bugs with my for loops, it was an issue with the lox code (not the compiler), and I had trouble debugging because I am dumb!

1. In addition to if statements, most C-family languages have a multi-way switch statement. Add one to clox. The grammar is:

```
switchStmt     → "switch" "(" expression ")"
                 "{" switchCase* defaultCase? "}" ;
switchCase     → "case" expression ":" statement* ;
defaultCase    → "default" ":" statement* ;
```

To execute a switch statement, first evaluate the parenthesized switch value expression. Then walk the cases. For each case, evaluate its value expression. If the case value is equal to the switch value, execute the statements under the case and then exit the switch statement. Otherwise, try the next case. If no case matches and there is a default clause, execute its statements.

To keep things simpler, we’re omitting fallthrough and break statements. Each case automatically jumps to the end of the switch statement after its statements are done.

2. In jlox, we had a challenge to add support for break statements. This time, let’s do continue:

```
continueStmt   → "continue" ";" ;
```

A continue statement jumps directly to the top of the nearest enclosing loop, skipping the rest of the loop body. Inside a for loop, a continue jumps to the increment clause, if there is one. It’s a compile-time error to have a continue statement not enclosed in a loop.

Make sure to think about scope. What should happen to local variables declared inside the body of the loop or in blocks nested inside the loop when a continue is executed?

3. Control flow constructs have been mostly unchanged since Algol 68. Language evolution since then has focused on making code more declarative and high level, so imperative control flow hasn’t gotten much attention.

For fun, try to invent a useful novel control flow feature for Lox. It can be a refinement of an existing form or something entirely new. In practice, it’s hard to come up with something useful enough at this low expressiveness level to outweigh the cost of forcing a user to learn an unfamiliar notation and behavior, but it’s a good chance to practice your design skills.

- hard to beat 'goto', but maybe a conditional goto
- 'match' seems more powerful than switch, since it's value-based and the patterns are helpful / more reliable
- so, maybe a value-based pattern matching that can do some logic? idk, a bit hard without types
- but maybe sort of like the regex-based match that python and ruby have?

### 24: Calls and Functions

Notes:
- frame pointer is a pointer into the stack, to the start, where all the variable locations are offset
- we can't calculate the absolute position of all the variable locations at compile time (bc we don't know in advance what the context will be, i.e. what the state of the stack will be ahead of time), but we do know that on a relative basis, i.e. minus the frame pointer.
- call a function? just set the instruction pointer!
- but what about arguments and return values?
- and what about the return address (where to go once the function finishes executing?)

(to solve these, we implement the call stack and use frames)

interesting: can use the C call stack and a linked list instead of an array for storing compiler things; in the previous challenge we stored things in arrays, but didn't necessarily need to (i.e. the loop array could probably have been a linked list?)

- sharing the mutable views into the stack via callframes is neat. It will be painful with Rust, unless I figure out something clever.

#### Challenges


1. Reading and writing the ip field is one of the most frequent operations inside the bytecode loop. Right now, we access it through a pointer to the current CallFrame. That requires a pointer indirection which may force the CPU to bypass the cache and hit main memory. That can be a real performance sink.

Ideally, we’d keep the ip in a native CPU register. C doesn’t let us require that without dropping into inline assembly, but we can structure the code to encourage the compiler to make that optimization. If we store the ip directly in a C local variable and mark it register, there’s a good chance the C compiler will accede to our polite request.

This does mean we need to be careful to load and store the local ip back into the correct CallFrame when starting and ending function calls. Implement this optimization. Write a couple of benchmarks and see how it affects the performance. Do you think the extra code complexity is worth it?

--SKIP--

2. Native function calls are fast in part because we don’t validate that the call passes as many arguments as the function expects. We really should, or an incorrect call to a native function without enough arguments could cause the function to read uninitialized memory. Add arity checking.

a. how do we know how many args a native fn takes? well, we can define it when we defineNative or whatever
b. how do we check it? well, we can add the check into the call, just like for ObjFunction calls
c. how do we store the arity? same as with ObjFunction probably

3. Right now, there’s no way for a native function to signal a runtime error. In a real implementation, this is something we’d need to support because native functions live in the statically typed world of C but are called from dynamically typed Lox land. If a user, say, tries to pass a string to sqrt(), that native function needs to report a runtime error.

Extend the native function system to support that. How does this capability affect the performance of native calls?

a. how do we do this checking? well... we could check the types of the args like we check the arity, or similar
b. before we actually call the C function, we check that the arg types are correct, and issue a runtimeError if not
c. performance of native calls would be slower! we have to do the arg typechecking, which costs some ops

4. Add some more native functions to do things you find useful. Write some programs using those. What did you add? How do they affect the feel of the language and how practical it is?

- reading args and files seems really useful
- ditto writing files
- maybe like a generic syscall interface, that can make other syscalls if you want them
- loading your own native fn / supporting writing interop code seems potentially useful 
- maybe a builtin for http requests and responses, a la Bun -- what else do you do with scripts?

## 25: Closures

So, in the language itself, using a closure doesn't feel like much -- it 'just happens'. This, despite programming courses taking time aside in order to discuss them explicitly. I wonder -- do the classes discuss them because they have to be implemented so carefully at the language level? Is that an artifact of knowledge on the part of the teachers, or is it because the semantics are confusing to a beginner?

Doing multiple passes seems like an obvious way to do this implementation. Check which variables get closed over, and then you can mark them to be allocated on the heap and associated with the corresponding function. Curious to see what the single-pass version from Lua/Lox looks like!

Closures mean that the ObjFunctions that we allocated in the compile step and only referenced in the vm are not going to work as neatly as before. Functions can be defined dynamically with different values 'inside' -- the closure.

#### Challenges

1. Wrapping every ObjFunction in an ObjClosure introduces a level of indirection that has a performance cost. That cost isn’t necessary for functions that do not close over any variables, but it does let the runtime treat all calls uniformly.

Change clox to only wrap functions in ObjClosures that need upvalues. How does the code complexity and performance compare to always wrapping functions? Take care to benchmark programs that do and do not use closures. How should you weight the importance of each benchmark? If one gets slower and one faster, how do you decide what trade-off to make to choose an implementation strategy?

- Gah, I need to set up benchmarking if I want to do this.
- in larger programs, there are likely to be more cases that use closures
- the complexity seems a little annoying, so nice to make it simpler
- the perf: every fn call involves additional pointer lookups for every `OP_CALL`.  
  - `OP_CALL` already has to do some jumping
  - the pointer indirection perf hit depends a lot on architecture / cache...

2. Read the design note below. I’ll wait. Now, how do you think Lox should behave? Change the implementation to create a new variable for each loop iteration.

- Reading the design note, I agree it's a bit confusing, but I also don't really want to implement this!

3. A famous koan teaches us that “objects are a poor man’s closure” (and vice versa). Our VM doesn’t support objects yet, but now that we have closures we can approximate them. Using closures, write a Lox program that models two-dimensional vector “objects”. It should:

- Define a “constructor” function to create a new vector with the given x and y coordinates.

- Provide “methods” to access the x and y coordinates of values returned from that constructor.

- Define an addition “method” that adds two vectors and produces a third.

This is cool! lox-programs/closures-are-objects.lox

a little tricky since we don't have any collection types, so things have to be very functional

## 26: Garbage Collection

Exciting! Reachability / liveness analysis. If it's a root or can be reached from a root, it's alive!

Naive GC algorithm: traverse from roots, find all live things, free everything else. Mark-Sweep does this in two passes.

do we have a sample program that takes a lot of memory, to benchmark against? let's try.  
- lox-programs/memory-hog.lox
- and to bench it, `memory_profile ./main.out lox-programs/memory-hog.lox`

Results:

- Before garbage collector: 431.62 MB peak memory usage
- After garbage collector: 2.90 MB

Pretty big difference!

Conceptually, we are keeping a tricolor list as we GC. We implement this with a worklist, which is more or less the frontier of the graph traversal, where we've noticed the nodes but haven't yet processed them. We'll need some way to check if they've been 'seen', and then add and kick things to/from the worklist, and when it's empty, we're done.

- mark and sweep is conceptually simple, the trick is making sure to get all the details right
- e.g. don't forget the weak refs in the vm.strings hash table

When to run GC? Tradeoff: bigger, less frequent runs or smaller, more frequent runs?
- GC tuning is a whole arcane art
- this book doesn't have great answers
- so, it uses a reasonable guess: fewer GC runs when the heap is bigger


#### Challenges

1. The Obj header struct at the top of each object now has three fields: type, isMarked, and next. How much memory do those take up (on your machine)? Can you come up with something more compact? Is there a runtime cost to doing so?

A: the type enum and marked bool could be compacted, at the runtime cost of a bitmask. I don't think the pointer should get compacted in. A bool is a byte uncompacted. One byte per object doesn't seem like too high a cost vs. the bitmask, but it's a little hard to tell without benching.

2. When the sweep phase traverses a live object, it clears the isMarked field to prepare it for the next collection cycle. Can you come up with a more efficient approach?

A: in theory, we could try to unmark things whenever they leave the roots or (potentially) if they are no longer pointed to by some other object. Refcounting is at least one way to do it, but maybe it's possible to still mark/sweep but maintaining the marks on lots of the objects and 'unmarking' when they are e.g. removed from roots or something else stops pointing to them. Without the refcounts, I think there might be some penalties to this if the object graph is highly connected, otherwise it's probably pretty low cost?

3. Mark-sweep is only one of a variety of garbage collection algorithms out there. Explore those by replacing or augmenting the current collector with another one. Good candidates to consider are reference counting, Cheney’s algorithm, or the Lisp 2 mark-compact algorithm.

Reference counting:
- reference counting: instead of an isMarked bool field, maintain a counter on each object
- when objects are initialized, increment their count for incoming pointers (roots, objects pointing in)
- when objects go out of scope, decrement the count of everything they point to
- if the count goes to 0, collect the object (and decrement the count of what it pointed to)
- could get big chains of freed memory all at once, but mostly doesn't need as much work to GC

Cheney's algorithm:
- stop and copy
- from space and to space -- old heap and new heap
- copy over all the live objects, similar to the mark phase of mark and sweep, but copying as you go
- then, just free the whole old heap
- the pointer management seems annoying - have to 'forward' the pointers from the from-space to the to-space

Mark-compact:
- https://en.wikipedia.org/wiki/Mark%E2%80%93compact_algorithm
- somehow combines mark/sweep with Cheney's algorithm
- mark the reachable objects first
- move them to the beginning of the heap
- compacting the heap reduces heap fragmentation, so should get better cache performance
- have to relocate the pointers to the objects (wiki describes a 'break table' with relocation records)
- Lisp 2: After standard marking, the algorithm proceeds in the following three passes: 1) Compute the forwarding location for live objects. Keep track of a free and live pointer and initialize both to the start of heap. If the live pointer points to a live object, update that object's forwarding pointer to the current free pointer and increment the free pointer according to the object's size. Move the live pointer to the next object End when the live pointer reaches the end of heap. 2) Update all pointers. For each live object, update its pointers according to the forwarding pointers of the objects they point to. 3) Move objects. For each live object, move its data to its forwarding location.
- Compressor algorithm: In a first pass, the mapping is computed for all objects in the heap. In a second pass, each object is moved to its new location (compacted to the beginning of the heap), and all pointers within it are modified according to the relocation map.

Generational GC
- younger objects tend to live shorter
- keep track of generations, GC the older generations less frequently
- in essence, keep two heaps with two GCs, one that runs frequently and one that is less frequent
- note suggests the types of the GCs is sometimes different too


## 27: Classes and Instances

Should be easier, apparently. Empty classes, at least, are mostly just boilerplate.

Instances are also mostly boilerplate, except that getting and setting values requires a bit of thought.

#### Challenges

1. Trying to access a non-existent field on an object immediately aborts the entire VM. The user has no way to recover from this runtime error, nor is there any way to see if a field exists before trying to access it. It’s up to the user to ensure on their own that only valid fields are read.

How do other dynamically typed languages handle missing fields? What do you think Lox should do? Implement your solution.

- Javascript: return an undefined
- Python: raise (catchable) AttributeError if missing on get
- Ruby: undefined method (all fields are methods?): could either catch or implement method_missing

what other langs are relevant? idk

- implementing returning a Null is doable, so Lox could be Javascripty
  - in vm.c, swap the runtimeError with a push(NIL_VAL);
- Ruby and Python's solutions require implementing userspace error handling, which I don't wanna do

2. Fields are accessed at runtime by their string name. But that name must always appear directly in the source code as an identifier token. A user program cannot imperatively build a string value and then use that as the name of a field. Do you think they should be able to? Devise a language feature that enables that and implement it.

- this is a very basic kind of reflection
- it can be handy for lots of situations, enables a lot of semi-metaprogramming relatively cheaply
- it's probably worth it in a dynamic language to be able to do this, if not other reflection

implementation:
- need an alternative syntax (probably something like the [] index operator; maybe `instance.[constructed string expression]`
- or maybe it's a total alternative to the dot, just a `[]` accessor
- when we hit that version, we need to pop the string from the stack instead of reading it from the constants table in the vm case for GET_PROPERTY. hmmm. Maybe just a separate operator? dull, but maybe the way to get it done

3. Conversely, Lox offers no way to remove a field from an instance. You can set a field’s value to nil, but the entry in the hash table is still there. How do other languages handle this? Choose and implement a strategy for Lox.

- python has `delete`
- js has `delete`
- ruby has `undef`! (TIL) -- unclear exactly what it's doing

removing a field from an instance, why do it? save memory? Just... don't add the field to the instance. I'm not sure it's a good idea, really

The people want structs

4. Because fields are accessed by name at runtime, working with instance state is slow. It’s technically a constant-time operation—thanks, hash tables—but the constant factors are relatively large. This is a major component of why dynamic languages are slower than statically typed ones.

How do sophisticated implementations of dynamically typed languages cope with and optimize this?

- instead of names, it's compiled (or JITd) into array accesses and objects are 'mini-structs'
- sometimes there's some vtable business going on (ruby!)
- there's hot path and then there's times when the object doesn't match the expected shape, and code gets deoptimized

## 28: Methods and Initializers

Cool, closing in on a really useful language! Let's get there!

- no field declarations, so anything in a class is a method (nice!)
- creating the methods is similar to creating closures w/ upvalues, in that we'll have dynamic opcodes that do the defining
  - OP_CLASS for the class
  - OP_METHOD for each method
- can't compile methods from strings at runtime, it's always going to be a pointer to bytecode that's in some chunk already
- the ops ensure that the methods have the right pointers to/from the class, and the names are associated with the pointer to the function, so that the calls can work. It's about the binding, not about the function compilation (w.r.t. these runtime vm ops).

- method access and invokation are separate, functions are first class, so we have to be able to 'fetch' out the method with a dot, separately from calling it
- there's also a `this` binding that should happen for the instance. hence we 'bind' the methods when we access them so that the instance is there -- it's a new object like a closure with upvalues, except it's a bound method

- optimization is neat! we get to skip a heap allocation with the special-cased OP_INVOKE which pre-empts the normal path that would take a GET_PROPERTY and a CALL.

#### Challenges

1. The hash table lookup to find a class’s init() method is constant time, but still fairly slow. Implement something faster. Write a benchmark and measure the performance difference.

- hmm... special case the init method? 
- keep methods in a different kind of data structure instead of a hashtable (one where init performs especially well)? Maybe just assume that there are few methods in a class, and keep name/pointer tuples in an array, with init first, and check against strings instead of hashtable lookup? If there are just a few methods it'd probably be faster

2. In a dynamically typed language like Lox, a single callsite may invoke a variety of methods on a number of classes throughout a program’s execution. Even so, in practice, most of the time a callsite ends up calling the exact same method on the exact same class for the duration of the run. Most calls are actually not polymorphic even if the language says they can be.

How do advanced language implementations optimize based on that observation?

- A: I'm not sure I understand what this means. a.somemethod() in the source code could be invoking somemethod on different classes? or, sometimes somemethod gets reassigned or something?
- I assume that instead of setting up dynamic dispatches like this, it could in effect be rewritten to a function, and then maybe inlined in the hot path, but idk. This sounds like JIT stuff to me

3. When interpreting an OP_INVOKE instruction, the VM has to do two hash table lookups. First, it looks for a field that could shadow a method, and only if that fails does it look for a method. The former check is rarely useful—most fields do not contain functions. But it is necessary because the language says fields and methods are accessed using the same syntax, and fields shadow methods.

That is a language choice that affects the performance of our implementation. Was it the right choice? If Lox were your language, what would you do?

- hmm, I think it's a pretty simple mental model right now
- but, the option is to ban fields with the same name as methods, and throw a compile-time error for them
- that'd allow the optimization, and _likely_ remove a class of errors from programs (shadowing a method seems usually bad...)
- but... this isn't really the language anyone is reaching for, speed-wise, so maybe the perf penalty is okay. Same with reflection in Ruby etc, I think

## 29: SuperClasses

- a little fiddling to get things to resolve right. There's some real cleverness in how we've set up the calling convention and our tables, to make these just fall out so easily.
- nice to have things mostly working!

Note: I segfault on lox-programs/degenerate-inheritance.lox. That probably means I fucked up my implementation somehow...

Debugging, because it's good for me.

it was stupid, I was emitting a duplicate OP_GET_SUPER outside of the else clause :facepalm:

#### Challenges

1. A tenet of object-oriented programming is that a class should ensure new objects are in a valid state. In Lox, that means defining an initializer that populates the instance’s fields. Inheritance complicates invariants because the instance must be in a valid state according to all of the classes in the object’s inheritance chain.

The easy part is remembering to call super.init() in each subclass’s init() method. The harder part is fields. There is nothing preventing two classes in the inheritance chain from accidentally claiming the same field name. When this happens, they will step on each other’s fields and possibly leave you with an instance in a broken state.

If Lox was your language, how would you address this, if at all? If you would change the language, implement your change.

- we don't declare the fields in Lox, instead it's a bag of data
- stomping on superclass fields seems like it's almost guaranteed to happen
- 'claiming' a field in init could be special-cased, somehow
- but... what would the semantics be that would be sensible? 
  - setting the field from the child class seems like it's really useful, like, that's a core thing that child classes should be able to do
  - I don't see what semantics could make this a better situation? Visibility into the fields of a parent class seems wise.

2. Our copy-down inheritance optimization is valid only because Lox does not permit you to modify a class’s methods after its declaration. This means we don’t have to worry about the copied methods in the subclass getting out of sync with later changes to the superclass.

Other languages, like Ruby, do allow classes to be modified after the fact. How do implementations of languages like that support class modification while keeping method resolution efficient?

- hypothesis: do the update when the class is redefined
- classes keep track of ancestors and descendants
- when the 'end' of a class redefinition happens, update the chain of method tables to have the correct function pointers
- there's probably some bookkeeping about which version of a method you use, like, from which class do you inherit it currently, and how far above you in the tree is it
- then updates could be fairly reasonable

3. Reverse the inheritance pattern (swap inner for super, highest method in the chain wins)

- guess at the steps:
  - instead of copying the method table, we have to... do something else... geez
  - I guess instead of children keeping track of superclasses, parents have to keep track of their (immediate) children
  - so that the `inner()` invocations can find the right child function
  - uggg it feels so gross!

## 30: Optimization

- benchmark first
- then optimize
- then test

Benchmark 1: Zoo
- class with lots of fields
- method for each field access
- loop to invoke the methods and add up and print all the values
- expectation: most of the time is spent in the callframe setup stuff; the method invokation is still somewhat expensive

Before optimization, time for sum > 100000000: 12-13s
- results: OP_GET_GLOBAL(17%), OP_GET_PROPERTY(12%), and big spender OP_INVOKE(42%) of time spent by ops
- all of these are spending most of their time in... `tableGet()`

This lookup optimization business was asked about in an earlier challenge. I think there are a few ways to go, but maybe using a linear scan through a small array for small tables is still a decent idea.

Looking deeper, there's a mod operation that's somehow very expensive, so we can optimize that. We can bitmask instead of % to wrap the key.

Seems like it improved things a bit (10-11s on my system, down from 12-13) but not a lot. Maybe a different thing is slow for me!

Nan-boxing has a more significant effect (7s or so, down from 10s). 

However, all of this is a bit mooted by a missing piece. I was clanging without optimizing, but passing the -O flags makes things way faster. O1 => 3.5-3.7s or so, O2 => 3.2-3.4s or so, O3 => 3.1-3.4s.


