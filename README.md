# Reading 'Crafting Interpreters'

Bob Nystrom's http://craftinginterpreters.com/

Notes will go here in this Readme until I need more pages for clarity

## Languages

I'm following along with the book, so I'll do the implementations in Java and C. 

I'm also practicing Rust, and want to practice my translation skills, so I'll do the Java and C implementations in Rust too.


## Status

jlox-java: Finished! Didn't do every challenge, but jlox-java works!
jlox-rust: Finished Chapter 8, starting chapter 9
clox-c: Finished chapters 13 and 14, haven't completed the ch 14 challenges
clox-rust: unstarted. not 100% clear that transliteration will be smooth, but we'll give it a go. Helpfully, rustc is cleverer than clang about modules, so we don't have as much #include dance (though, we'll still have some macros to write!)

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
