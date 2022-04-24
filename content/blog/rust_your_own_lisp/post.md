---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--AL977crl--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/hpy6695aug72kxxj300s.jpg
date: 2019-06-04T12:00:00.000Z
title: Rust Your Own Lisp
description: An overview of my Rust translation of orangeducks Build Your Own Lisp
tags:
  - rust
  - beginners
  - lisp
---

This is a fuller walk-through of the code I talked about in a previous post, [Solving Problems by Avoiding Them](https://dev.to/deciduously/solving-problems-by-avoiding-them-58dm).

The project is a translation of [Build Your Own Lisp](http://www.buildyourownlisp.com/) by [orangeduck](http://theorangeduck.com/page/about) into [Rust](https://www.rust-lang.org/). His book is fantastic, both as an introduction to C and an introduction to writing an interpreter.

This post is nowhere close to a replacement for that text, by a long shot - go read the book. It's excellent. In translating to Rust, though, there are a few necessary differences worth noting. This post does not include the code in its entirety but rather examples of each concept, and may be useful for anyone attempting a similar project or translation of their own in Rust. I've also removed most debug logging for clarity. The full implementation can be found in [this repo](https://github.com/deciduously/blispr).

I learned a lot about C, interpreters, and Rust from this project, and highly recommend the exercise. For better or worse (probably worse), I've called this implementation `blispr`.

## Rustyline

First thing's first, we've got to collect us some strings. I highly recommend [`rustyline`](https://github.com/kkawakam/rustyline), a pure-Rust `readline` implementation. You get line editing, keyboard commands, and command history out of the box. This is all you have to do:

```rust
fn repl(e: &mut Lenv) -> Result<()> {
    println!("Blispr v0.0.1");
    println!("Use exit(), Ctrl-C, or Ctrl-D to exit prompt");

    let mut rl = Editor::<()>::new();
    if rl.load_history("./.blispr-history.txt").is_err() {
        println!("No history found.");
    }

    loop {
        let input = rl.readline("blispr> ");

        match input {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                print_eval_result(eval_str(e, &line));
            }
            Err(ReadlineError::Interrupted) => {
                info!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                info!("CTRL-D");
                break;
            }
            Err(err) => {
                warn!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("./.blispr-history.txt")?;
    Ok(())
}

fn print_eval_result(v: BlisprResult) {
    match v {
        Ok(res) => println!("{}", res),
        Err(e) => eprintln!("Error: {}", e),
    }
}

```

One thing to note is that I'm not propagating the error that `eval_str` might throw up to the caller here with `?` - I don't want blispr evaluation errors to crash the repl. Anything that can happen inside `eval_str()` I just want to inform the user about with `eprintln!()` and loop again. The `&mut Lenv` getting passed through is the global environment - more on that below.

The bulk of evaluation is hinted at in the `Ok()` arm of the `match` - the meat of the work is happening in `eval_str()`:

```rust
pub fn eval_str(e: &mut Lenv, s: &str) -> BlisprResult {
    let parsed = BlisprParser::parse(Rule::blispr, s)?.next().unwrap();
    let mut lval_ptr = lval_read(parsed)?;
    lval_eval(e, &mut *lval_ptr)
}
```

This is it, this is the entire interpreter. This function does all of the steps required to evaluate a programming language given in text string form. The first line stores the parse tree to `parsed`. This tags our input string with semantic grammatical tags that we'll define below. The next line reads that tree into an AST at `lval_ptr`, which represents the whole program as a lisp value that can be evaluated recursively. Finally we return the result of fully evaluating that AST with `lval_eval`, which ensures this there are no further evaluations that can happen. Any errors that happened along the way were caught with the `?` operator - below we'll see what that `Result<T>` alias represents.

## Lval

To represent the AST, I used a Rust `enum` called `Lval`:

```rust
// The recursive types hold their children in a `Vec`
type LvalChildren = Vec<Box<Lval>>;
// This is a function pointer type
pub type LBuiltin = fn(&mut Lval) -> BlisprResult;

// There are two types of function - builtin and lambda
#[derive(Clone)]
pub enum LvalFun {
    Builtin(String, LBuiltin), // (name, function pointer)
    Lambda(HashMap<String, Box<Lval>>, Box<Lval>, Box<Lval>), // (environment, formals, body), both should be Qexpr
}

// The main type - all possible Blispr values
#[derive(Debug, Clone, PartialEq)]
pub enum Lval {
    Fun(LvalFun),
    Num(i64),
    Sym(String),
    Sexpr(LvalChildren),
    Qexpr(LvalChildren),
}
```

Each variant carries its contents with it. As we read the text each element is going to be converted into the proper type of `Lval`. For example, a string like `"4"` is going to be parsed into `Lval::Num(4)`. Now this value can be used in the context of a larger evaluation. I've also implemented [`fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) for this type, which is responsible for defining the output string to be finally displayed to the user. With the auto-derived `Debug` trait we get something like `Lval::Num(4)`, and with `Display` we just get `4`:

```rust
impl fmt::Display for Lval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lval::Blispr(_cells) => write!(f, "<toplevel>"),
            Lval::Fun(lf) => match lf {
                LvalFun::Builtin(name, _) => write!(f, "<builtin: {}>", name),
                LvalFun::Lambda(_, formals, body) => write!(f, "(\\ {} {})", formals, body),
            },
            Lval::Num(n) => write!(f, "{}", n),
            Lval::Sym(s) => write!(f, "{}", s),
            Lval::Sexpr(cell) => write!(f, "({})", lval_expr_print(cell)),
            Lval::Qexpr(cell) => write!(f, "{{{}}}", lval_expr_print(cell)),
        }
    }
}

fn lval_expr_print(cell: &[Box<Lval>]) -> String {
    let mut ret = String::new();
    for i in 0..cell.len() {
        ret.push_str(&format!("{}", cell[i]));
        if i < cell.len() - 1 {
            ret.push_str(" ");
        }
    }
    ret
}
```

We have numbers, symbols, functions (two different types of function - more on those later on), and two types of expression list - s-expressions and q-expressions. S-expressions will be evaluated as code, looking for a function in the first position, and q-expressions are evaluated as just lists of data. The whole program that's read in is going to be one big containing `Lval::Sexpr`, and we just need to evaluate it until we only have a result needing no further evaluation, either a `Num`, `Sym`, or `Qexpr`.

As a simple example, `"+ 1 2"` is going to get stored as `Sexpr(Sym("+"), Num(1), Num(2))`. When this `Sexpr` is evaluated, it will first look up `+` in the environment and find a function pointer to the built-in addition function: `Sexpr(Fun(Builtin("+"), Num(1), Num("2")))`. Then this `Sexpr` will be evaluated as a function call, yielding `Num(3)`, which cannot be evaluated further.

This code makes use of the `Box` pointer type, which is a smart pointer to a heap-allocated value. Because an `Lval` can hold many different types of data, the size of a given `Lval` is not known at compile-time. By only storing pointers to values on the heap, we can build lists of them. Because these `Box`es adhere to Rust's ownership and borrowing semantics, Rust is going to manage cleaning them up for us when they are no longer needed. This is how we'll manage our memory over the lifetime of the program - with quite a bit less ceremony than the corresponding C! To build a new one, we use a constructor. For example:

```rust
pub fn lval_num(n: i64) -> Box<Lval> {
    Box::new(Lval::Num(n))
}
```

There's one of these for each variant. Calling this will allocate the appropriate space with `Box::new()` on the heap and return the pointer. No need to futz with a destructor - the `Box` will drop itself as soon as it can.

The containing types start out with an empty `Vec` of children, and can be manipulated with `lval_add` and `lval_pop`:

```rust
// Add lval x to lval::sexpr or lval::qexpr v
pub fn lval_add(v: &mut Lval, x: &Lval) -> Result<()> {
    match *v {
        Lval::Sexpr(ref mut children)
        | Lval::Qexpr(ref mut children)
        | Lval::Blispr(ref mut children) => {
            children.push(Box::new(x.clone()));
        }
        _ => return Err(BlisprError::NoChildren),
    }
    Ok(())
}

// Extract single element of sexpr at index i
pub fn lval_pop(v: &mut Lval, i: usize) -> BlisprResult {
    match *v {
        Lval::Sexpr(ref mut children)
        | Lval::Qexpr(ref mut children)
        | Lval::Blispr(ref mut children) => {
            let ret = (&children[i]).clone();
            children.remove(i);
            Ok(ret)
        }
        _ => Err(BlisprError::NoChildren),
    }
}
```

Both of these functions mutate their first argument in place, either removing or adding a child.

## Errors

One difference from the book's implementation is that I don't have a separate specific `Lval::Err` AST variant for handling errors in our program. Instead, I built a separate error type and leverage `Result<T, E>`-style error handling throughout:

```rust
#[derive(Debug)]
pub enum BlisprError {
    DivideByZero,
    EmptyList,
    FunctionFormat,
    NoChildren,
    NotANumber,
    NumArguments(usize, usize),
    ParseError(String),
    ReadlineError(String),
    WrongType(String, String),
    UnknownFunction(String),
}
```

To simplify the type signatures used throughout, I have a few type aliases:

```rust
pub type Result<T> = std::result::Result<T, BlisprError>;
pub type BlisprResult = Result<Box<Lval>>;
```

The majority of evaluation functions are going to return a `Result<Box<Lval>, BlisprError>`, now I can just type `BlisprResult`. The few functions here and there that don't have a success type of `Box<Lval>` can still use this new `Result<T>` alias instead of the more verbose built-in `Result<T, E>`, and the error type will automatically always be this `BlisprError`.

In order to be able to use this throughout our entire program, I've provided `impl From<E> for BlisprError` for a few other types of errors that are thrown, like `std::io::Error` and `pest::error::Error` for example:

```rust
impl<T> From<pest::error::Error<T>> for BlisprError
where
    T: Debug + Ord + Copy + Hash,
{
    fn from(error: pest::error::Error<T>) -> Self {
        BlisprError::ParseError(format!("{}", error))
    }
}

impl From<std::io::Error> for BlisprError {
    fn from(error: std::io::Error) -> Self {
        BlisprError::ParseError(error.to_string())
    }
}
```

This way I can still use the `?` operator on function calls that return these other error types inside functions that return a `BlisprResult`, and any errors returned will be automatically converted to the proper `BlisprError` for me. Instead of storing specific error-type `Lval`s during our evaluation that are carried through the whole computation and finally printed out, all errors are bubbled up through the type system, but you still get the full `pest`-generated error carried along:

```lisp
blispr> eval {* 2 3)
Parse error:  --> 1:12
  |
1 | eval {* 2 3)
  |            ^---
  |
  = expected expr
```

Full disclosure: to write the `pest::error::Error<T>` block, I just wrote what I wanted, i.e. `BlisprError::ParseError(format!("{}", error))` and appeased the compiler. There is likely a better way to go about this but it works!

## Parsing

The book uses the author's own parser combinator library called [mpc](https://github.com/orangeduck/mpc). If I were to tackle another similar problem in C, I'd likely reach for it again. Rust, however, has its own strong ecosystem for parsing. Some of the heavyweights in this space are [nom](https://github.com/Geal/nom), [combine](https://github.com/Marwes/combine), and [pest](https://github.com/pest-parser/pest). For this project I opted for pest, to stay as close to the source material as possible. Whereas `nom` and `combine` will have you defining your own [parser combinators](https://dev.to/deciduously/parser-combinators-are-easy-4bjm), with `pest` you provide a PEG (or [Parsing Expression Grammar](https://en.wikipedia.org/wiki/Parsing_expression_grammar)), separately from your code. Pest then uses Rust's powerful custom derive tooling to create a parse for your grammar automatically.

Here's the grammar I used for this language:

```pest
COMMENT = _{ "/*" ~ (!"*/" ~ ANY)* ~ "*/" }
WHITESPACE = _{ (" " | NEWLINE ) }

num = @{ int }
    int = { ("+" | "-")? ~ digit+ }
    digit = { '0'..'9' }

symbol = @{ (letter | digit | "_" | arithmetic_ops | "\\" | comparison_ops | "&")+ }
    letter = { 'a' .. 'z' | 'A' .. 'Z' }
    arithmetic_ops = { "+" | "-" | "*" | "/" | "%" | "^" }
    comparison_ops = { "=" | "<" | ">" | "!" }

sexpr = { "(" ~ expr* ~ ")" }

qexpr = { "{" ~ expr* ~ "}" }

expr = { num | symbol | sexpr | qexpr }

blispr = { SOI ~ expr* ~ EOI }
```

This is stored in its own file called `blispr.pest` alongside the source code. Each line refines a parse rule. I find this exceedingly readable, and easy to tweak. Starting from the bottom, we see a unit of valid `blispr` consists of one or more `expr`s between the Start of Input (SOI) and End of Input (EOI). An `expr` is any of the options given. It can handle comments and whitespace for you. I also enjoy how the grammar maintained completely separately from any Rust code. It's easy to get this working with Rust:

```rust
use pest::{iterators::Pair, Parser};

#[cfg(debug_assertions)]
const _GRAMMAR: &str = include_str!("blispr.pest");

#[derive(Parser)]
#[grammar = "blispr.pest"]
pub struct BlisprParser;
```

Now we can use the `BlisprParser` struct to parse string input into a parse tree with `parse()`. In order to evaluate it, though, we need to build a a big `Lval` AST:

```rust
fn lval_read(parsed: Pair<Rule>) -> BlisprResult {
    match parsed.as_rule() {
        Rule::blispr => {
            let mut ret = lval_blispr();
            read_to_lval(&mut ret, parsed)?;
            Ok(ret)
        }
        Rule::expr => lval_read(parsed.into_inner().next().unwrap()),
        Rule::sexpr => {
            let mut ret = lval_sexpr();
            read_to_lval(&mut ret, parsed)?;
            Ok(ret)
        }
        Rule::qexpr => {
            let mut ret = lval_qexpr();
            read_to_lval(&mut ret, parsed)?;
            Ok(ret)
        }
        Rule::num => Ok(lval_num(parsed.as_str().parse::<i64>()?)),
        Rule::symbol => Ok(lval_sym(parsed.as_str())),
        _ => unreachable!(), // COMMENT/WHITESPACE etc
    }
}
```

We pass the parse tree from `pest` into `lval_read`, which will recursively build the AST for us. This function looks at the top-level rule and takes an appropriate action, either allocating a new `Lval` variant or adjusting the children of . Then every child in the parse tree is added as a child to this containing `Lval`, passing through `lval_read()` itself to turn it into the correct `Lval`. The rule for `qexpr` is similar, and the other rules just create the corresponding `Lval` from the type given. The one weird one is `Rule::expr` - this is a sort of meta-rule that matches any of the valid expression types, so it's not its own lval, just wrapping one of a more specific type. We just use `next()` to pass the actual rule found back into `lval_read()`.

The variants contianng children use a helper which skips surrounding brackets, and just adds the actual children to the new `Lval`:

```rust
fn read_to_lval(mut v: &mut Lval, parsed: Pair<Rule>) -> Result<()> {
    for child in parsed.into_inner() {
        if is_bracket_or_eoi(&child) {
            continue;
        }
        lval_add(&mut v, &*lval_read(child)?)?;
    }
    Ok(())
}
```

The final result of `lval_read()` will be a single `Lval` containing the entire parsed program, saved in `lval_ptr`. Then we call `lval_eval()`, which will also return a `BlisprResult` after reducing this tree to its most evaluated form. If the evaluation is successful we just print out the result, and if any error was raised we print that error instead.

## Environment

Before we dig into how `lval_eval()` does its mojo lets pause and talk about the environment. This is how symbols are able to correspond to functions and values - otherwise `"+"` would just be that character, but we need to to specifically correspond to the addition function.

Jury's out on whether or not I have the right idea, here, but I also handled this differently from the book. The original text has you create a `struct` that holds two arrays and a counter, one for keys and the other for values. To perform a lookup, you find the index of that key and then return the value at that same index in the values. This struct is built before the program enters the loop, and is passed in manually to every single function that gets called.

Instead, I've opted for a [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) data structure instead of two separated arrays with matching indices:

```rust
pub type LEnvLookup = HashMap<String, Box<Lval>>;

#[derive(Debug, PartialEq)]
pub struct Lenv<'a> {
    lookup: LEnvLookup,
    pub parent: Option<&'a Lenv<'a>>,
}
```

The `Lenv` itself holds the lookup table and optionally a reference to a parent.

I've got some helper methods for getting, setting, and enumerating the contents:

```rust
impl Lenv {

 // ..

 pub fn get(&self, k: &str) -> BlisprResult {
        match self.lookup.get(k) {
            Some(v) => Ok(v.clone()),
            None => {
                // if we didn't find it in self, check the parent
                // this will recur all the way up to the global scope
                match &self.parent {
                    None => Err(BlisprError::UnknownFunction(k.to_string())),
                    Some(p_env) => p_env.get(k),
                }
            }
        }
    }

    // Returns an Lval containing Symbols with each k,v pair in the local env
    pub fn list_all(&self) -> BlisprResult {
        let mut ret = lval_qexpr();
        for (k, v) in &self.lookup {
            lval_add(&mut ret, &lval_sym(&format!("{}:{}", k, v)))?;
        }
        Ok(ret)
    }

    // add a value to the local env
    pub fn put(&mut self, k: String, v: Box<Lval>) {
        let current = self.lookup.entry(k).or_insert_with(|| v.clone());
        if *v != **current {
            // if it already existed, overwrite it with v
            *current = v;
        }
    }
}
```

Getting a value from the environment will return a brand new `Lval` with a copy of what's stored, and printing out the contents will also return a ready-made `Lval::Qexpr` containing `Symbol`s corresponding to each entry. We'll come back to initialization after talking a bit about evaluation.

Environments optionally hold a parent environment, and if the lookup fails in this one it will attempt the parent environment.

## Eval

The `lval_eval()` function called in `eval_str()` is where the real crunching happens. This will take an `Lval` (that is, an AST) and recursively evaluate it to a final `Lval`. Most types of `Lval` are already evaluated fully - but any `S-Expression` found will need to be evaluated, and any `Symbol` gets looked up in the environment.

Before looking at the Rust, let's break it down in English:

1. Check the type of Lval:

   a. Fun | Num | Qexpr - we're done - return lval as is.

   b. Symbol - Do an environment lookup with `Lenv::get()` - e.g., for `Sym("+")`, see if we have a function pointer stored at name `"+"`. Return result of lookup, which will already be an `Lval`.

   c. Sexpr - Evaluate the S-Expression.

2. If we made it to this step, we're working with an S-Expression. Everything else has already returned. Before going further, fully evaluate all children with `lval_eval()`.

3. Check the length of the S-Expression:

   a. 0 - empty S-Expression - return as-is

   b. 1 - single expression - pop that expression and return the result of calling `lval_eval()` on it

   c. Multiple expressions (function call) - pop the first expression and attempt to use it as a function on the rest of the children

Here's what that looks like in Rust:

```rust
// Fully evaluate an `Lval`
pub fn lval_eval(e: &mut Lenv, v: &mut Lval) -> BlisprResult {
    let child_count;
    let mut args_eval;
    match v {
        Lval::Blispr(forms) => {
            // If it's multiple, evaluate each and return the result of the last
            args_eval = eval_cells(e, forms)?;
            let forms_len = args_eval.len()?;
            return Ok(lval_pop(&mut args_eval, forms_len - 1)?);
        }
        Lval::Sym(s) => {
            // If it's a symbol, perform an environment lookup
            let result = e.get(&s)?;
            // The environment stores Lvals ready to go, we're done
            return Ok(result);
        }
        Lval::Sexpr(ref mut cells) => {
            // If it's a Sexpr, we're going to continue past this match
            // First recursively evaluate each child with lval_eval()
            // grab the length and evaluate the children
            child_count = cells.len();
            args_eval = eval_cells(e, cells)?;
        }
        // if it's not a sexpr, we're done, return as is
        _ => {
            return Ok(Box::new(v.clone()));
        }
    }
    if child_count == 0 {
        // It was a Sexpr, but it was empty.  We're done, return it
        Ok(Box::new(v.clone()))
    } else if child_count == 1 {
        // Single expression
        lval_eval(e, &mut *lval_pop(v, 0)?)
    } else {
        // Function call
        // We'll pop the first element off and attempt to call it on the rest of the elements
        let fp = lval_pop(&mut args_eval, 0)?;
        lval_call(e, *fp, &mut *args_eval)
    }
}
```

The step that fully evaluates all the children of an S-Expression before tackling the expression itself uses a helper:

```rust
// Given a slice of boxed Lvals, return a single evaluated sexpr
fn eval_cells(e: &mut Lenv, cells: &[Box<Lval>]) -> BlisprResult {
    cells.iter().fold(Ok(lval_sexpr()), |acc, c| {
        match acc {
            Ok(mut lval) => {
                lval_add(&mut lval, &*lval_eval(e, &mut c.clone())?)?;
                Ok(lval)
            }
            // it's just a Result so we can bubble errors out of the fold
            Err(_) => unreachable!(),
        }
    })
}
```

This is written as a `fold` using an empty `Lval::Sexpr` as the accumulator, using `lval_add` to add each new result to it.

## Function calling

This gets us almost all the way there - there's one last missing step, which is `lval_call()`.

This language has two kinds of functions: builtins and user-defined lambdas. Builtins are implemented in Rust and part of the executable itself. These are stored in the environment when it's created:

```rust
fn add_builtin(&mut self, name: &str, func: LBuiltin) {
    self.put(name.to_string(), lval_builtin(func, name))
}

pub fn new(lookup: Option<LEnvLookup>, parent: Option<&'a Lenv<'a>>) -> Self {
        let mut ret = Self {
            lookup: lookup.unwrap_or_default(),
            parent,
        };

        // Register builtins
        // The "stub" fns are dispatched separately - the function pointer stored is never called
        // these are the ones the modify the environment

        // Definiton
        ret.add_builtin("\\", builtin_lambda);
        ret.add_builtin("def", builtin_put_stub);

        // etc, lots and lots of builtins

        ret.add_builtin("max", builtin_max);

        ret
}
```

Each name stores a function pointer to a Rust function. These functions manipulate lvals directly. For example, this is `builtin_head`, which returns the first element of an `Lval::Qexpr`:

```rust
pub fn builtin_head(v: &mut Lval) -> BlisprResult {
    let mut qexpr = lval_pop(v, 0)?;
    match *qexpr {
        Lval::Qexpr(ref mut children) => {
            if children.is_empty() {
                return Err(BlisprError::EmptyList);
            }
            debug!("builtin_head: Returning the first element");
            Ok(children[0].clone())
        }
        _ => Err(BlisprError::WrongType(
            "qexpr".to_string(),
            format!("{:?}", qexpr),
        )),
    }
}
```

Mathematical operations all use the same function. They all accept a list of any length of `Lval::Num`s and will successively apply a binary operation to a running result and the next number until the list is consumed:

```rust
fn builtin_op(mut v: &mut Lval, func: &str) -> BlisprResult {
    let mut child_count;
    match *v {
        Lval::Sexpr(ref children) => {
            child_count = children.len();
        }
        _ => return Ok(Box::new(v.clone())),
    }

    let mut x = lval_pop(&mut v, 0)?;

    // If no args given and we're doing subtraction, perform unary negation
    if (func == "-" || func == "sub") && child_count == 1 {
        let x_num = x.as_num()?;
        return Ok(lval_num(-x_num));
    }

    // consume the children until empty
    // and operate on x
    while child_count > 1 {
        let y = lval_pop(&mut v, 0)?;
        child_count -= 1;
        match func {
            "+" | "add" => {
                apply_binop!(add, x, y)
            }
            "-" | "sub" => {
                apply_binop!(sub, x, y)
            }
            "*" | "mul" => {
                apply_binop!(mul, x, y)
            }
            "/" | "div" => {
                if y.as_num()? == 0 {
                    return Err(BlisprError::DivideByZero);
                } else {
                    apply_binop!(div, x, y)
                }
            }
            "%" | "rem" => {
                apply_binop!(rem, x, y)
            }
            "^" | "pow" => {
                let y_num = y.as_num()?;
                let x_num = x.as_num()?;
                let mut coll = 1;
                for _ in 0..y_num {
                    coll *= x_num;
                }
                x = lval_num(coll);
            }
            "min" => {
                let x_num = x.as_num()?;
                let y_num = y.as_num()?;
                if x_num < y_num {
                    x = lval_num(x_num);
                } else {
                    x = lval_num(y_num);
                };
            }
            "max" => {
                let x_num = x.as_num()?;
                let y_num = y.as_num()?;
                if x_num > y_num {
                    x = lval_num(x_num);
                } else {
                    x = lval_num(y_num);
                };
            }
            _ => unreachable!(),
        }
    }
    Ok(x)
}
```

This is a long function - but it'd be even longer without the macro I defined:

```rust
macro_rules! apply_binop {
    ( $op:ident, $x:ident, $y:ident ) => {
        match (*$x, *$y) {
            (Lval::Num(x_num), Lval::Num(y_num)) => {
                $x = lval_num(x_num.$op(y_num));
                continue;
            }
            _ => return Err(BlisprError::NotANumber),
        }
    };
}
```

This makes some of the Lval type checking quicker to type! It handles making sure both arguments are `Lval::Num` before trying to do something numeric with them, as in `apply_binop!(add, x, y)`. This was my first brush with defining Rust macros, and it was a serious help.

These are fairly easy to call. Because the environment stores these ans function pointers you can simply call the function. My solution is a little hacky, because a few builtins require access to an environment, which builtin functions don't have - these special cases are dispatched separately, and everything else is just called with `fp()`:

```rust
LvalFun::Builtin(name, fp) => match name.as_str() {
        "eval" => builtin_eval(e, args),
        "def" => builtin_def(e, args),
        "printenv" => builtin_printenv(e),
        // Otherwise, just apply the actual stored function pointer
        _ => fp(args),
},
```

Calling a `Lambda` is a little trickier. We need to build a new environment, add any local bindings to it, and then either call the new function or return a new, partially applied lambda if not all locals were given. The machinery here is verbose - see [this line](https://github.com/deciduously/blispr/blob/2d8aa15cf7ba8cfc624cf9663fd024dda1df9f72/src/eval.rs#L436) for the code in context.

That's all of our pieces. With all this in place `lval_eval()` can handle a whole bunch of stuff, and this language actually approaches usable. This language implementation is not complete, but it's a great playground for learning about how languages work!
