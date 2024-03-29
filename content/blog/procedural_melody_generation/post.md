---
cover_image: https://thepracticaldev.s3.amazonaws.com/i/iuakwwcexql5u0th7g
date: 2019-12-05T12:00:00.000Z
title: Procedural Melody Generation in Rust
description: Procedurally generate melodies by synthesizing your own sound waves in Rust using test-driven development.
tags:
  - beginners
  - rust
  - tutorial
  - music
---

## Teaching Numbers How To Sing

> Everything is music. When I go home, I throw knickers in the oven and it's music. Crash, boom, bang!

_- [Winona Ryder](https://en.wikipedia.org/wiki/Winona_Ryder) as [Björk](https://en.wikipedia.org/wiki/Bj%C3%B6rk) on [SNL](https://en.wikipedia.org/wiki/Saturday_Night_Live)'s [Celebrity Rock 'N' Roll Jeopardy!](<https://en.wikipedia.org/wiki/Celebrity_Jeopardy!_(Saturday*Night_Live)>) - [2002](https://en.wikipedia.org/wiki/2002) - [YouTube](https://youtu.be/R3V94ZtmdbQ?t=190)*

In this [post](<(https://en.wikipedia.org/wiki/Blog)>), we'll [throw](https://en.wikipedia.org/wiki/Throwing) something [random](https://en.wikipedia.org/wiki/Random_number_generation) into, [well](https://en.wikipedia.org/wiki/Well), a [math](https://en.wikipedia.org/wiki/Mathematics)-[oven](https://en.wikipedia.org/wiki/Subroutine) and [_viola_](https://en.wikipedia.org/wiki/Viola), [music](https://en.wikipedia.org/wiki/Music)! We'll just skip the [crash](<https://en.wikipedia.org/wiki/Crash_(computing)>).

In other words, we're going to teach our [computers](https://en.wikipedia.org/wiki/Personal_computer) to ["sing"](https://en.wikipedia.org/wiki/Singing) using [idiomatic](https://en.wikipedia.org/wiki/Programming_idiom) [Rust](<https://en.wikipedia.org/wiki/Rust_(programming_language)>), backed by a little light [physics](https://en.wikipedia.org/wiki/Physics) and [music theory](https://en.wikipedia.org/wiki/Music_theory).

The [one-liner](https://en.wikipedia.org/wiki/One-liner_program) in the cover image [procedurally generates](https://en.wikipedia.org/wiki/Procedural_generation) a [melody](https://en.wikipedia.org/wiki/Melody) using [tools assumed to be present](https://en.wikipedia.org/wiki/Unix_philosophy) on a standard [desktop](https://en.wikipedia.org/wiki/Desktop_computer) [Linux distribution](https://en.wikipedia.org/wiki/Linux_distribution) like [Ubuntu](https://en.wikipedia.org/wiki/Ubuntu). The melody produced will be composed of notes along a single [octave](https://en.wikipedia.org/wiki/Octave) in a hardcoded [key](<https://en.wikipedia.org/wiki/Key_(music)>) ([A major](https://en.wikipedia.org/wiki/A_major)):

{# {% youtube uLhQQSKhTok %} #}

By the end of this post our program will:

1. Support [86](<https://en.wikipedia.org/wiki/86_(number)>) different [key signatures](https://en.wikipedia.org/wiki/Key_signature) with [minimal effort](https://en.wikipedia.org/wiki/Music_and_mathematics).
1. Support a full [108](<https://en.wikipedia.org/wiki/108_(number)>)-key extended [piano](https://en.wikipedia.org/wiki/Piano) [keyboard](https://en.wikipedia.org/wiki/Musical_keyboard), allowing the user to pick a range.
1. Produce any arbitrary [tone](https://en.wikipedia.org/wiki/Musical_tone) we ask for.
1. Compile and run on [Windows](https://en.wikipedia.org/wiki/Microsoft_Windows), [MacOS](https://en.wikipedia.org/wiki/MacOS), or [Linux](https://en.wikipedia.org/wiki/Linux) with no extra code ([I tried](https://en.wikipedia.org/wiki/Nerd) all three).
1. Encourage further [extension](https://en.wikipedia.org/wiki/Scalability) with lots of Rust-y goodness.

[C# minor](https://en.wikipedia.org/wiki/C-sharp_minor) has a funky, dark kind of vibe - [Lullaby](<https://en.wikipedia.org/wiki/Lullaby_(The_Cure_song)>) by [The Cure](https://en.wikipedia.org/wiki/The_Cure), [Message in a Bottle](<https://en.wikipedia.org/wiki/Message_in_a_Bottle_(song)>) by [The Police](https://en.wikipedia.org/wiki/The_Police), [Feel It Still](https://en.wikipedia.org/wiki/Feel_It_Still) by [Portugal, The Man](https://en.wikipedia.org/wiki/Portugal._The_Man), a bunch of [others](https://en.wikipedia.org/wiki/C-sharp_minor#Notable_songs). Your computer could be the next [Dolly Parton](https://en.wikipedia.org/wiki/Dolly_Parton) ([Jolene](<https://en.wikipedia.org/wiki/Jolene_(song)>)):

```txt
$ ./music -b C#2 -o 4 -s minor
.: Cool Tunes :.
Generating music from the C# minor scale
Octaves: 2 - 6
[ C# D# E F# G# A B C# ]
```

However, at the end of the day, it's just the thing in the cover image.

While the complete runnable source code is embedded within this post, the full project can also be found on [GitHub](https://github.com/deciduously/music) along with some [pre-compiled binaries](https://github.com/deciduously/music/releases/tag/v0.1.0). Feel free to make a PR!

## Table of Contents

- [Preamble](#preamble)
- [The Meme](#the-meme)
- [The Program](#the-program)
  - [Project Structure](#project-structure) - [Dependencies](#dependencies) - [Test-Driven Development](#test-driven-development) - [Entry Point](#entry-point) - [Traits](#traits)
  - [Random Numbers](#random-numbers)
  - [Mapping Numbers To Notes](#mapping-numbers-to-notes) - [A Little Physics](#a-little-physics) - [Sine Waves](#sine-waves) - [Pitch](#pitch) - [Singing](#singing) - [A Little Music Theory](#a-little-music-theory) - [Scientific Pitch Notation](#scientific-pitch-notation) - [Intervals](#intervals) - [Scales](#scales) - [Key](#key) - [Circle of Fifths](#circle-of-fifths) - [Diatonic Modes](#diatonic-modes) - [Non Heptatonic Scales](#non-heptatonic-scales)
    - [Generating Music](#generating-music)
      - [Cents](#cents)
      - [Random Notes](#random-notes)
      - [User Parameters](#user-parameters)
- [Challenges](#challenges)

## Preamble

_[top](#table-of-contents)_

This tutorial is aimed at [beginners](https://en.wikipedia.org/wiki/Novice) (and up) who are comfortable solving problems with at least one [imperative](https://en.wikipedia.org/wiki/Imperative_programming), [object-oriented](https://en.wikipedia.org/wiki/Object-oriented_programming) [language](https://en.wikipedia.org/wiki/Programming_language). It does not matter if that's [JavaScript](https://en.wikipedia.org/wiki/JavaScript) or [Python](<https://en.wikipedia.org/wiki/Python_(programming_language)>) or [Object Pascal](https://en.wikipedia.org/wiki/Object_Pascal), I just assume you know the [basic](<https://en.wikipedia.org/wiki/Syntax_(programming_languages)>) [building](<https://en.wikipedia.org/wiki/Semantics_(computer_science)>) [blocks](https://en.wikipedia.org/wiki/Standard_library) of [creating a program](https://en.wikipedia.org/wiki/Computer_programming) in an object-oriented style. If you already know Rust some of this will be skimmable, but this is primarily a post about the problem space and not "how to use Rust".

You do not need any prior knowledge of physics or music theory, but there will be a tiny smattering of [elementary algebra](https://en.wikipedia.org/wiki/Elementary_algebra). I promise it's quick.

I have two disclaimers before getting started:

1. [There are](https://en.wikipedia.org/wiki/Existence) [257](<https://en.wikipedia.org/wiki/257_(number)>) [links](https://en.wikipedia.org/wiki/Hyperlink) [here](https://en.wikipedia.org/wiki/Boston), [199](<https://en.wikipedia.org/wiki/199_(number)>) [of them](<https://en.wikipedia.org/wiki/Element_(mathematics)>) [to](https://en.wikipedia.org/wiki/Codomain) [Wikipedia](https://en.wikipedia.org/wiki/Main_Page). [If](<https://en.wikipedia.org/wiki/Conditional_(computer_programming)>) [you're](https://en.wikipedia.org/wiki/You) [that](https://en.wikipedia.org/wiki/Autodidacticism) [kind](https://en.wikipedia.org/wiki/Impulsivity) [of](https://en.wikipedia.org/wiki/Preposition_and_postposition) [person](https://en.wikipedia.org/wiki/Person), [set](https://en.wikipedia.org/wiki/Innovation) [rules](https://en.wikipedia.org/wiki/Law).
1. Further to Point 1, most of this I learned myself on Wikipedia, some of it while writing this post. The rest is what I remember from [high school](<https://en.wikipedia.org/wiki/High_school_(North_America)>) as a [band geek](https://en.wikipedia.org/wiki/Euphonium), which was over [ten years](https://en.wikipedia.org/wiki/Decade) [ago](https://en.wikipedia.org/wiki/Past). I do believe it's generally on the mark, but I am making no claims of authority. If you see something, [say something](https://en.wikipedia.org/wiki/Allen_Kay#Advertisements).

## The Meme

_[top](#table-of-contents)_

This post was inspired by this [meme](https://en.wikipedia.org/wiki/Internet_meme) I saw when I was _attempting_ to casually browse [Reddit](https://en.wikipedia.org/wiki/Reddit):

![the meme](https://i.redd.it/uirqnamnjpz31.jpg)

I couldn't let myself just scroll past that one, [clearly](https://en.wikipedia.org/wiki/Diatribe). Here's a version of the [`bash`](<https://en.wikipedia.org/wiki/Bash_(Unix_shell)>) [pipeline](<https://en.wikipedia.org/wiki/Pipeline_(Unix)>) at the bottom with slightly different hard-coded values, taken from [this blog post](https://blog.robertelder.org/bash-one-liner-compose-music/) by [Robert Elder](https://www.robertelder.org/) that explores it:

```bash
cat /dev/urandom | hexdump -v -e '/1 "%u\n"' | awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }' | xxd -r -p | aplay -c 2 -f S32_LE -r 16000
```

The linked blog post is considerably more brief and assumes a greater degree of background knowledge than this one, but that's not to discredit it at as a fantastic source. That write-up and Wikipedia were all I needed to complete this translation, and I had absolutely not a clue how this whole thing worked going in.

I've gotta be honest - I didn't even try the `bash` and immediately dove into the pure Rust solution. Nevertheless, it serves as a [solid](https://en.wikipedia.org/wiki/Solid) [30,000ft](https://en.wikipedia.org/wiki/Flight_level) [roadmap](https://en.wikipedia.org/wiki/Plan):

1. `cat /dev/urandom`: Get a stream of random binary data.
1. `hexdump -v -e '/1 "%u\n"'`: Convert binary to 8-bit base-10 integers (0-255).
1. `awk '{ split("0,2,4,5,7,9,11,12",a,","); for (i = 0; i < 1; i+= 0.0001) printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i)) }'`: Map integers to pitches and return sound wave samples.
1. `xxd -r -p`: Convert hexadecimal samples back to binary.
1. `aplay -c 2 -f S32_LE -r 16000`: Play back binary samples as sound wave.

Don't worry at all if some or all of this is incomprehensible. You don't need to have a clue how any of it works yet. This program is not a direct translation of that [code](https://en.wikipedia.org/wiki/Source_code), and I'm not going to elaborate much on what any of the specific commands in the pipeline mean (read the linked post for that), just the relevant logic. By the time we're done, you'll be able to pick apart the whole thing yourself.

If you'd like the challenge of implementing this yourself from scratch in your own language, **stop right here**. If you get stuck, this should all apply to whatever you've got going unless you've gone real funky with it - in which case, it sounds cool and you should show me.

[¡Vámonos!](https://en.wikipedia.org/wiki/Party)

## The Program

_[top](#table-of-contents)_

### Project Structure

_[top](#table-of-contents)_

Before getting started, ensure you have at least the default stable Rust toolchain [installed](https://www.rust-lang.org/tools/install). If you've previously installed `rustup` at any point, just issue `rustup update` to grab the latest stable build. This code was written with `rustc` [version 1.39.0](https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html) (released [November 4](https://en.wikipedia.org/wiki/November_4), [2019](https://en.wikipedia.org/wiki/2019)), but should compile on any version compatible with [Rust 2018](https://doc.rust-lang.org/edition-guide/rust-2018/index.html).

Then, spin up a new library project:

```txt
$ cargo new music --lib
```

Open your new `music` project directory in the environment of your choice. If you're not already sure what to use with Rust, I recommend [Visual Studio Code](https://code.visualstudio.com/) with the [Rust Language Server](https://github.com/rust-lang/rls) installed for in-editor development support. If you have `rustup` present, the [VS Code RLS extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) has a one-click set up that's worked for me without a hitch on both Linux and Windows 10.

#### Dependencies

_[top](#table-of-contents)_

We'll use a few crates, which is the Rust term for external libraries. Two of them give us functionality not found in the Rust standard library:

- [`rand`](https://docs.rs/rand/0.7.2/rand/) - [Random number generation](https://en.wikipedia.org/wiki/Random_number_generation)
- [`rodio`](https://docs.rs/rodio/0.10.0/rodio/) - [Audio signal playback](https://en.wikipedia.org/wiki/Audio_signal)

`rand` is in place of [`/dev/urandom`](https://en.wikipedia.org/wiki//dev/random), and `rodio` will cover and [`aplay`](https://linux.die.net/man/1/aplay). We can replace [`hexdump`](https://en.wikipedia.org/wiki/Hex_dump), [`xxd`](https://www.systutorials.com/docs/linux/man/1-xxd/), and the `awk` logic built-in stuff. The `rand` crate provides several different random number generators (RNGs), and the one perfect for this application isn't included by default. We have to specifically add it to the configuration, so its declaration is split out to deifne multiple keys.

The other two are just for programmer comfort. I also use [`pretty_assertions`](https://docs.rs/pretty_assertions/0.6.1/pretty_assertions/) to make the [test runner](https://en.wikipedia.org/wiki/Unit_testing) output a little prettier and [`structopt`](https://github.com/TeXitoi/structopt) to get a minimal-effort CLI.

In `Cargo.toml`:

```toml
[dependencies]

rodio = "0.10"
structopt = "0.3"

# the section below is equivalent TOML to:
# rand = { features = [ "small_rng" ], version = "0.7" }
# it's a style preference
[dependencies.rand]

features = [ "small_rng" ]
version = "0.7"

[dev-dependencies]

pretty_assertions = "0.6"
```

#### Test Driven Development

_[top](#table-of-contents)_

Cargo has auto-created a file at `src/lib.rs` to define your library, but hold on - we're going to write this program using [Test-Driven Development](https://en.wikipedia.org/wiki/Test-driven_development), or TDD. This means we're going to define the expected behavior of new functionality _before_ attempting the implementation. Here's an example of a test we'll write later:

```rust
#[test]
fn test_add_interval() {
    use Interval::*;
    assert_eq!(Unison + Unison, Unison);
    assert_eq!(Unison + Maj3, Maj3);
    assert_eq!(Maj2 + Min3, Perfect4);
    assert_eq!(Octave + Octave, Unison);
    assert_eq!(Tritone + Tritone, Unison);
    assert_eq!(Maj7 + Min3, Maj2);
}
```

Each test is just a plain Rust function. In it we use a feature of our library and assert that the result matches the expected result that we hardcode. In this test, we're specifying the expected behavior when adding musical intervals together with the `+` operator. This way, we can tell immediately if the code we write actually matches the specification. As our code evolves we'll immediately notice if we break functionality that worked previously.

The Rust toolchain has a test runner built-in, so this all works out of the box. Every function marked `#[test]` will be executed during an invocation of `cargo test`, so we can see anywhere our expectations are not met in the whole program.

All of our tests will live in their own separate module. Create a new file at `src/test.rs`:

```rust
use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_cool_greeting() {
    assert_eq!(GREETING, ".: Cool Tunes :.");
}
```

If the two arguments to `assert_eq!()` are not equal, this test will fail and you'll get pretty-printed output showing you the difference between the two. I generally put the test code in the first argument and the hardcoded expected value in the second.

This test is importing a constant, `GREETING`, from our library, and expecting it to be the string `Cool Tunes (tm)`. This code will fail to compile, though - there's no such `super::GREETING` constant available to test! The `super` part means "one module higher" - `test` is a child module of the `music` library we're writing, so the crate root in `lib.rs` corresponds to `super` here. You could also say `crate::*` or `music::*`. Now open up `src/lib.rs` and replace the contents with this:

```rust
#[cfg(test)]
mod test;

pub const GREETING: &str = ".: Cool Tunes :.\n";
```

The `#[cfg(test)]` tag tells the compiler to only build the `test` module when we're using the test runner. The compiler won't even look at it when using `cargo run`.

Now we can give `cargo test` a go - the first build will take the longest as it gathers and builds dependencies for the first time:

![test fail](https://thepracticaldev.s3.amazonaws.com/i/4wgtozis0bfoxnmedvrp.png)

Whoops - no need to include a newline with the greeting string, we'll pass it to [`println!()`](https://doc.rust-lang.org/std/macro.println.html) in the program which includes one:

```diff
  #[cfg(test)]
  mod test;

- pub const GREETING: &str = ".: Cool Tunes :.\n";
+ pub const GREETING: &str = ".: Cool Tunes :.";
```

Let's try this again:

![test pass](https://thepracticaldev.s3.amazonaws.com/i/ajubi9o41dsfvkfwcnqa.png)

Good to go! Throughout this post new sections of code will be preceded by a test with he `#[test]` tag that defines the behavior we're aiming for. These tests should all go in `src/test.rs`.

#### Entry Point

_[top](#table-of-contents)_

Finally, create a directory called `src/bin`. This optional module is where Cargo will by default expect an executable, if present. Place a file at `src/bin/music.rs` - this filename will be the name of the executable, so when distributed you'd execute `./music` to run the code in `main()`:

```rust
use music::*;

fn main() {
    println!("{}", GREETING);
}
```

Give it a go with `cargo run`:

```txt
$ cargo run
   Compiling music-rebuild v0.1.0 (/home/you/code/music)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/music`
.: Cool Tunes :.
```

The _coolest_ tunes. You can see right above the output the actual name of the executable file being run - you can find it right in your project's `target` directory:

![executable screenshot](https://thepracticaldev.s3.amazonaws.com/i/15gwbnp3j15lb2a8fhps.png)

Your `music/src` directory should look like the following:

```txt
~/code/music $ tree src
src
├── bin
│   └── music.rs
├── lib.rs
└── test.rs

1 directory, 3 files
```

This is a good time for an initial commit:

```txt
$ git add .
$ git commit -m "Initial Commit"
```

You can run a faster compilation pass with `cargo check` if you just want the compiler to verify your code's integrity, not produce a binary.

#### Traits

If you're already familiar with developing in Rust, you can probably skip right to [Random Numbers](#random-numbers).

If you are brand new to the language, you should expect to spend a little longer with the code in this post to extract the relevant bits. I'm not going to devote much time in general to Rust-specific topics, as there is a vast amount of great material already available devoted to that, but out of all of Rust's interesting properties this is the big one you'll need to know about to follow along with this program.

Most of this code is compartmentalized using [Rust traits](https://doc.rust-lang.org/book/ch10-02-traits.html), which collect bits of composable functionality (my type "has-a" `ScreenWidget` trait that implements those methods). It's OOP, Jim, [but not as we know it](https://en.wikipedia.org/wiki/Star_Trekkin%27). In this post, you can think of them like interfaces in more traditional [class-based OOP](https://en.wikipedia.org/wiki/Class-based_programming) languages. They're a little more powerful, but that analogy does fit and is enough to get you up and running.

One big difference from "regular" object-oriented programming is that this is all we get. There's no such thing as inheritance (my type "is-a" more specific `ScreenWidget` type and inherits or overrides those methods). As a result, composition of functionality features heavily in Rust code in the form of `impl SomeTrait for MyType {}` blocks, with collections of method definitions inside.

The compiler can infer types in many situations, and can auto-fill these trait implementations for us in many cases with a `#[derive(..)]` tag. In this case, the default `value` is also the `Default` value for the primitive type `i32`, which for all the numeric types is `0` (or `0.0`). When that's what we want in this context too, we can ask the compiler to auto-generate the above code with this syntax:

```rust
#[derive(Default)]
struct MyType {
    value: i32,
}
```

Writing this code is nearly equivalent to the former in terms of the output machine code. This syntax is a [macro](<https://en.wikipedia.org/wiki/Macro_(computer_science)>) that will expand to the full Rust code for any `impl Trait` block being derived blocks before your program is compiled as if it had been fully written out. In general, a struct can derive a trait as long as all of its members implement that trait, either derived or hand-implemented, because the compiler will just call that method for whatever type it needs. The auto-derived `Default` implementation looks like this when your code reaches the compiler:

```rust
impl Default for MyType {
    fn default() -> Self {
        Self { value: i32::default() }
    }
}
```

Now we can use `MyType::default()` to construct an object of this type - the following two statements store the same object to `obj`:

```rust
let obj = MyType::default();
// or
let obj: MyType = MyType { value: 0 }
```

It's up to the specific type to decide what happens, as long as the input and output types match. Whenever you get lost just remember - it's [traits all the way down](https://en.wikipedia.org/wiki/Turtles_all_the_way_down).

We can also define methods that aren't associated with any trait with, e.g.:

```rust
impl MyType {
    fn some_specific_thing(&self) {
        // ..
    }
}
```

### Random Numbers

_[top](#table-of-contents)_

The first part of the one-liner is `cat /dev/urandom | hexdump -v -e '/1 "%u\n"'`, which gets a source of random bytes (8-bit binary values) and shows them to the user formatted as base-10 integers.

When I sat down to write this program, I decided to knock out this functionality first mostly because I immediately knew how. The `rand` crate can give us random 8-bit integers out of the box by using the so-called ["turbofish"](https://docs.serde.rs/syn/struct.Turbofish.html) syntax to specify a type: `random::<u8>()` will produce a random [unsigned](https://en.wikipedia.org/wiki/Signedness) [8 bit](https://en.wikipedia.org/wiki/8-bit) integer ([`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) with the default generator settings.

To match the one-liner exactly, we could write an [`Iterator`](https://doc.rust-lang.org/std/iter/index.html) implementation with a `next()` method like this - no need to copy this code to your project, we don't use it again:

```rust
impl Iterator for RandomBytes {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        Some(random::<Self::Item>())
    }
}
```

If we endlessly call this method, we'll get output that matches `cat /dev/urandom | hexdump -v -e '/1 "%u\n"'` exactly:

```rust
fn main() {
    let mut rands = RandomBytes::new();
    loop {
        println!("{}", rands.next().unwrap());
    }
}
```

I'm not bothering to show you the full runnable snippet - try to build the `RandomBytes` struct yourself if you'd like. In a `bash` one-liner you've got to take your randomness where you can get it, but the `rand` crate provides a richer set of tools. Before streaming in something random, we need to think about what exactly it is we're randomizing.

In this application, we want to pick a musical note from a set of valid choices at random. The `awk` code does this with the modulo operator: `list[n % listLength]`. That will take a random index that's ensured to be a valid list member. See if you can spot the corresponding section of the cover image code.

We get to use the [`rand::seq::SliceRandom`](https://docs.rs/rand/0.7.2/rand/seq/trait.SliceRandom.html) trait here. This gives us a `choose()` method that we can call on any [slice](https://doc.rust-lang.org/std/slice/index.html) to pull a random member.

So, there's no need for a `RandomBytes` iterator. Instead, we need to define a list of notes and call `[notes].choose(&mut RNG)` on it to get a specific note to play.

### Mapping Numbers To Notes

_[top](#table-of-contents)_

Take a closer look at step 3 of the pipeline. This code closely resembles the core logic we ultimately end up with:

```bash
split("0,2,4,5,7,9,11,12",a,",");
for (i = 0; i < 1; i += 0.0001)
    printf("%08X\n",
           100 * sin(1382 * exp((a[$1 % 8] / 12) * log(2)) * i))
```

This is probably still not too helpful for most - there's [magic numbers](<https://en.wikipedia.org/wiki/Magic_number_(programming)>) and [sines](https://en.wikipedia.org/wiki/Sine) and [logarithms](https://en.wikipedia.org/wiki/Logarithm) (oh, my) - and its written in freakin' [`AWK`](https://en.wikipedia.org/wiki/AWK). Don't despair if this still doesn't mean much (or literally anything) to you.

We can glean a bit of information at a glance, though, and depending on your current comfort with this domain you may be able to kind of understand the general idea here. It looks like we're going to tick up floating point values by ten-thousandths from zero to one (`0.0`, `0.0001`, `0.0002`, etc.) with `for (i = 0; i < 1; i += 0.0001)`, and do... I don't know, some math on each value. In that math we're using both `i`, the current fractional part from 0 to 1, and `$1`, which is the random 8-bit integer being piped in. Specifically, we're indexing into a list `a`: `a[$1 % 8]`. In other words, we're using the random byte `0-255` to select an index `0-7` from this list.

The list is defined with `split("0,2,4,5,7,9,11,12",a,",");`, which means split the first parameter string input by the third parameter `","`, and store the resulting list of elements to the second parameter `a` (`awk` is terse). After we do the math, we're going to print it out as an 8-digit hex number: `printf("%08X\n", someResult)` - this [`printf`](https://en.wikipedia.org/wiki/Printf_format_string) formatter means we want a [0-padded](<https://en.wikipedia.org/wiki/Npm_(software)#Notable_breakages>) number that's 8 digits long in [upper-case](https://en.wikipedia.org/wiki/Letter_case) [hexadecimal](https://en.wikipedia.org/wiki/Hexadecimal). The [base 10](https://en.wikipedia.org/wiki/Decimal) integer [`42`](<https://en.wikipedia.org/wiki/Phrases_from_The_Hitchhiker%27s_Guide_to_the_Galaxy#Answer_to_the_Ultimate_Question_of_Life,_the_Universe,_and_Everything_(42)>) would be printed as `0000002A`.

TL;DR for each ten-thousandth between 0 and 1 `i`, select a value `n` from `[0,2,4,5,7,9,11,12]` and return the result of `100 * sin(1382 * exp((n / 12) * log(2) * i)`.

If you recognize this formula, awesome! You can probably skim the next section. If not, it's still [not time to panic](https://en.wikipedia.org/wiki/Phrases_from_The_Hitchhiker%27s_Guide_to_the_Galaxy#Don't_Panic). We just need to get some fundamentals out of the way.

#### A Little Physics

_[top](#table-of-contents)_

[Sound](https://en.wikipedia.org/wiki/Sound) is composed physically of [vibrations](https://en.wikipedia.org/wiki/Vibration). These vibrations cause perturbations in some [medium](https://en.wikipedia.org/wiki/Transmission_medium), which radiate out from the source of the vibration, and those perturbations cause [tiny oscillating variations](https://en.wikipedia.org/wiki/Sound_pressure) in local atmospheric pressure. These variations are what we experience as sound. When we're talking about [hearing](https://en.wikipedia.org/wiki/Hearing) a sound with our [ears](https://en.wikipedia.org/wiki/Ear), the medium is usually [air](https://en.wikipedia.org/wiki/Atmosphere_of_Earth).

##### Sine Waves

_[top](#table-of-contents)_

Sound propagates as a [wave](https://en.wikipedia.org/wiki/Wave). In [reality](https://en.wikipedia.org/wiki/Reality) a sound contains many components but for this program we can talk about a super-simplified version that can be represented as a single [sine wave](https://en.wikipedia.org/wiki/Sine_wave):

![sine waves](https://upload.wikimedia.org/wikipedia/commons/6/6d/Sine_waves_different_frequencies.svg)

If the x-axis is time, a sine wave represents a recurring action with a smooth (or analog) oscillation between peaks. Lots of physical phenomena are analog in nature - picture a ball getting tossed, rising and then falling. The ball passes through every point in between the highest point it hits and the ground, so we can measure at any arbitrary instant an exact fractional height. It doesn't fall from 8 meters to 7 meters all at once, it passes through 7.9, 7.8, 7.7, and all infinitesimally small heights in between too. It's the same with sound.

Instead of height above the ground on the y axis, we have a [pressure gradient](https://en.wikipedia.org/wiki/Sound_pressure) from an equilibrium. The air is getting rapidly pushed and pulled by this vibration across space as a wave. It's still a physical phenomenon - a pressure gradient rises to a peak and then falls back to equilibrium and then below to an opposite peak, oscillating back and forth. It doesn't just magically become a different higher value all at once. A guitar string wobbling passes through each point in space between the two extremes it's tensing to and from, so the vibrations it causes oscillate in kind.

You can actually use [math](https://en.wikipedia.org/wiki/Fourier_transform) to represent multi-component sound waves as a single wave - the ability to do so is what enables the whole field of [telecommunications](https://en.wikipedia.org/wiki/Telecommunication). We're not going to touch that today, partially because I don't actually know how to perform a Fourier transform myself (yet). One single sine wave is enough of a signal to produce a tone, so we can keep it simple.

There are two interesting properties of a sine wave: the [amplitude](https://en.wikipedia.org/wiki/Amplitude), which measures the current deviation from the 0 axis for a given _x_, and the [frequency](https://en.wikipedia.org/wiki/Frequency), which is how close together these peaks at maximal amplitudes are, or how frequently this recurring thing happens. The combination of the two dictate how we perceive the sound. The amplitude will be perceived as [volume](https://en.wikipedia.org/wiki/Loudness) and the frequency as [pitch](<https://en.wikipedia.org/wiki/Pitch_(music)>).

You can do cool things like frequency modulation and amplitude modulation to encode your signal as modulations of one of these properties:

![modulation](https://upload.wikimedia.org/wikipedia/commons/a/a4/Amfm3-en-de.gif)

This is how FM and AM radio process incoming sound signals to broadcast them to your radio, which can then perform the reverse and play back the original sound. We also don't do any of that today, but you could experiment with these functions with this as a base.

##### Pitch

_[top](#table-of-contents)_

The standard unit for frequency is the [Hertz](https://en.wikipedia.org/wiki/Hertz), abbreviated `Hz`, which measures the _number of cycles per second_. One cycle here is the distance (or time) between two peaks on the graph, or the time it takes to go all the way around the circle once:

![cycle gif](https://media.giphy.com/media/F5rQlfTXqCJ8c/giphy.gif)

According to my super scientific smartphone stopwatch observations, this gif is chugging along at a whopping 0.2Hz.

Recall above that we saw we're going to run a loop like this: `for (i = 0; i < 1; i += 0.0001)`. In that loop, the math we process includes the function `sin()`. If one were to, say, calculate a bunch of points along a single cycle of a sine wave like this one, it sure seems like just such a loop could get the job done.

The higher the frequency, or closer together the peaks representing maximum positive amplitudes, the higher the pitch.

![frequency](https://upload.wikimedia.org/wikipedia/commons/e/ea/Wave_frequency.gif)

Sound is a continuous spectrum of frequency, but when we make music we tend to prefer [notes](https://en.wikipedia.org/wiki/Musical_note) at set frequencies, or pitches. I'm using [fundamental frequency](https://en.wikipedia.org/wiki/Fundamental_frequency) and pitch interchangeably, because for this application specifically they are, but go Wiki-diving if you want to learn about the distinction and nuance at play here. The nature of sound is super cool but super complex and outside of the scope of this post - we just want to hear some numbers sing, we don't need to hear a full orchestra.

One of the super cool things about it is the [octave](https://en.wikipedia.org/wiki/Octave). Octaves just sound related, you know?

It turns out the relationship is physical - to increase any pitch by an octave, you double the frequency. Not only that, this fixed ratio actually holds for any arbitrary smaller or larger interval as well. This system is called ["equal temperament"](https://en.wikipedia.org/wiki/Equal_temperament) - every pair of adjacent notes has the same ratio, regardless of how you define "adjacent". To get halfway to the next octave, you multiply by 1.5 instead of 2.

To start working with concrete numbers, we need some sort of standard to start multiplying from. Some of the world has settled on [440Hz](<https://en.wikipedia.org/wiki/A440_(pitch_standard)>) - it's [ISO](https://en.wikipedia.org/wiki/International_Organization_for_Standardization) [16](https://www.iso.org/standard/3601.html), at least. It's also apparently called "The Stuttgart Pitch", which is funny.

![stuttgart](https://i.imgflip.com/3h0y3g.jpg)

We can keep track of Hertz with a double-precision floating-point value:

```rust
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Hertz(f64);
```

This is just a floating point value, but I didn't just assign an alias like `type Hertz = f64`. Instead, I made my very own fully-fledged new type. A lot of this program will involve type conversions and unit conversions, but they will all be explicit and defined in places we expect. When manipulating our increasing set of abstractions we don't want to have to think about things like floating point accuracy - it should just work as we expect. The [tuple struct](https://doc.rust-lang.org/1.37.0/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types) syntax is perfect for this, when the underlying value is just a single value but there may be complex relationships with other types.

Luckily, the compiler can actually derive a number of things for us straight from the inner value. For the rest, we'll provide our own implementations that destructure the tuple:

```rust
#[test]
fn test_subtract_hertz() {
    assert_eq!(Hertz(440.0) - Hertz(1.0), Hertz(439.0))
}
```

```rust
use std::ops::Sub;

impl Sub for Hertz {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
```

This allows us to subtract two `Hertz` values with the subtraction operator `-`, and get a `Hertz` back. We can also give ourselves some more helpful conversion traits - this gets us both the defined `from()` and the type-inferred `into()` in both directions with `f64`:

```rust
impl From<Hertz> for f64 {
    fn from(h: Hertz) -> Self {
        h.0
    }
}

impl From<f64> for Hertz {
    fn from(f: f64) -> Self {
        Self(f)
    }
}
```

There are a lot of unit conversions throughout this program but _all_ of them are explicit and defined where we expect them. This does add to our boilerplate, but reduces the element of surprise - my LEAST favorite element in programming. Next, we need a way to represent a pitch:

```rust
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pitch(Hertz);
```

I didn't take `Default` this time - the default pitch is not 0Hz. We want our new `Pitch` type to default to A440, but also accept any arbitrary value:

```rust
#[test]
fn test_new_pitch() {
    assert_eq!(Pitch::default(), Pitch(Hertz(440.0)));
    assert_eq!(Pitch::new(MIDDLE_C), Pitch(Hertz(261.626)));
}
```

The following code gets us there:

```rust
pub const STANDARD_PITCH: Hertz = Hertz(440.0);
pub const MIDDLE_C: Hertz = Hertz(261.626);

// ..

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pitch(Hertz);

impl Pitch {
    pub fn new(frequency: Hertz) -> Self {
        Self(frequency)
    }
}

impl Default for Pitch {
    fn default() -> Self {
        Self(STANDARD_PITCH)
    }
}
```

Verify it all with `cargo test`:

```txt
running 3 tests
test test::test_cool_greeting ... ok
test test::test_subtract_hertz ... ok
test test::test_new_pitch ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

I won't keep prompting you to do so, but the prevailing wisdom is to run it after adding every test and watch it fail even before adding the implementation. Then you can watch it fail in incrementally different ways as you get closer to the correct code.

##### Singing

_[top](#table-of-contents)_

Knowing what frequency to use to produce a given pitch is all well and good, but we need to actually make the sound. When we sing with our [voice](https://en.wikipedia.org/wiki/Human_voice), our [speech organs](https://en.wikipedia.org/wiki/Speech_organ) vibrate to produce complex multiple-component sound waves of differing frequencies. We can program ourselves a little one-frequency "speechbox" that produces a wave programmatically instead of by physically vibrating.

To do so, we're going to perform an [analog-to-digital conversion](https://en.wikipedia.org/wiki/Analog-to-digital_converter). That's a super fancy term for something that isn't that complicated conceptually. We're going to [graph](https://en.wikipedia.org/wiki/Graph_of_a_function) the function of a single cycle of the target sine wave and [sample](<https://en.wikipedia.org/wiki/Sampling_(signal_processing)>) it. If you already know how we're doing this part, feel free to skip this explanation.

A sine wave, as we've seen, is smooth. However, what's a graph but a visualization of a function. There's some function `mySineWave(x)` that's this wave when we put in a bunch of fractional numbers between _0_ and _1_. The `for (i = 0; i < 1; i += 0.0001)` loop is doing exactly that, calculating a series of adjacent points at a fixed interval (`0.0001`) that satisfy the function of this wave. That's our analog-to-digital conversion - we've taken something smooth, a sine wave, and made it digital, or made up of discrete points. For `Pitch::default()`, this cycle repeats 440 times each second.

The [sample rate](<https://en.wikipedia.org/wiki/Sampling_(signal_processing)#Sampling_rate>) of an audio stream is how many points to store for each one of these cycles, or is how high-fidelity this "digital snapshot" of the wave is. Lots of applications use a [44.1KHz](https://en.wikipedia.org/wiki/44,100_Hz) sample rate - a bit higher than 10KHz like the example. According to the [sampling theorem](https://en.wikipedia.org/wiki/Nyquist%E2%80%93Shannon_sampling_theorem), the threshold for ensuring you've captured a sufficient sample from an analog signal is that the sample rate must be greater than twice the frequency you you're sampling. Humans can hear about 20Hz to 20,000Hz. This means we need at least 40,000 samples, and 44,100 exceeds that. The `rodio` crate defaults to 48KHz, which per that same link is the standard for professional digital audio equipment.

The maximum amplitude this struct can represent is the maximum wave that fits in whatever type is used for the sample, because that's the biggest _x_ will ever be in either direction - `1` or `-1`. This code uses an `f32`, or single-precision 4-byte float.

The `rodio` crate actually has a built-in [`rodio::source::SineWave`](https://docs.rs/rodio/0.10.0/rodio/source/struct.SineWave.html) that takes a frequency in Hertz but as an unsigned integer. Go ahead and throw a quick conversion in for our `Pitch` type:

```rust
// lib.rs
use rodio::source::SineWave;

impl From<Pitch> for f64 {
    fn from(p: Pitch) -> Self {
        p.0.into()
    }
}

impl From<Pitch> for SineWave {
    fn from(p: Pitch) -> Self {
        SineWave::new(f64::from(p) as u32)
    }
}
```

This code should produce an A440 tone when executed with `cargo run`:

```rust
// bin/music.rs
use rodio::{Sink, source::SineWave, default_output_device};

fn main() {
    let device = default_output_device().unwrap();
    let sink = Sink::new(&device);
    let source =  SineWave::from(Pitch::default());
    sink.append(source);
    sink.sleep_until_end();
}
```

I'll briefly cover the other tidbits: `default_output_device()` attempts to find the running system's currently configured default audio device, and a [`Sink`](https://docs.rs/rodio/0.10.0/rodio/struct.Sink.html) is an abstraction for handling multiple sounds. It works like an audio track. You can `append()` a new `Source` of sound, and the first one appended starts the track. A newly appended track will play after whatever is playing finishes, but a `rodio::source::SineWive` is an infinite source.

Finally, we have to `sleep_until_end()` the thread until the sound completes playing (which for `SineWave` is never), or else the program will move right along and exit. You'll have to kill this run with `Ctrl-C`, this sound will play forever.

By simply modulating the pitch passed to `SineWave`, we could generate any pitch we want. That's what the one-liner does, it's selecting an offset to pass from the list `[0,2,4,5,7,9,11,12]`, so we know that sequence works. And, like, _cool_, I guess. We can do a lot better, though. What's so special about these numbers?

#### A Little Music Theory

_[top](#table-of-contents)_

While it's great to have a voice we can sing with with, I'm sure we'd all prefer it if our program learned how to sing on key. To get oriented, A440 is the A above Middle C on a piano:

![piano](https://upload.wikimedia.org/wikipedia/commons/thumb/2/2e/Piano_Frequencies.svg/2560px-Piano_Frequencies.svg.png)

##### Scientific Pitch Notation

_[top](#table-of-contents)_

Instead of frequencies in Hertz, it's much easier to manipulate pitches in terms of [Scientific Pitch Notation](https://en.wikipedia.org/wiki/Scientific_pitch_notation), another fancy name for a simple concept. The piano keyboard above was labelled according to this standard. The A440 pitch is denoted `"A4"` in this system. We're going to want to parse them from strings:

```rust
#[test]
fn test_new_piano_key() {
    use Accidental::*;
    use NoteLetter::*;
    assert_eq!(
        PianoKey::default(),
        PianoKey {
            note: Note {
                letter: C,
                accidental: None
            },
            octave: 0
        }
    );
    assert_eq!(
        PianoKey::new("A4").unwrap(),
        PianoKey {
            note: Note {
                letter: A,
                accidental: None
            },
            octave: 4
        }
    );
    assert_eq!(
        PianoKey::new("G♭2").unwrap(),
        PianoKey {
            note: Note {
                letter: G,
                accidental: Some(Flat)
            },
            octave: 2
        }
    );
    assert_eq!(
        PianoKey::new("Gb2").unwrap(),
        PianoKey {
            note: Note {
                letter: G,
                accidental: Some(Flat)
            },
            octave: 2
        }
    );
    assert_eq!(
        PianoKey::new("F#8").unwrap(),
        PianoKey {
            note: Note {
                letter: F,
                accidental: Some(Sharp)
            },
            octave: 8
        }
    );
}
```

We also want to reject invalid letters - we can use `#[should_panic]` to indicate that a panic is the expected behavior. No need to bother defining a real match:

```rust
#[test]
#[should_panic]
fn test_reject_piano_key_too_high() {
    assert_eq!(PianoKey::new("A9").unwrap(), PianoKey::default());
}

#[test]
#[should_panic]
fn test_reject_piano_key_invalid_letter() {
    assert_eq!(PianoKey::new("Q7").unwrap(), PianoKey::default());
}
```

Additionally, we want to go the other way. We need a `to_string()` or some such:

```rust
#[test]
fn test_piano_key_to_str() {
    assert_eq!(PianoKey::default().to_string(), "C0".to_string());
    assert_eq!(PianoKey::new("A#4").unwrap().to_string(), "A#4".to_string());
    assert_eq!(PianoKey::new("Bb5").unwrap().to_string(), "B♭5".to_string())
}
```

A more robust system would also accept multiple accidentals and coerce, e.g. `E#` -> `F`, but this gets us going.

To implement this, it's easiest to start at the bottom. With `NoteLetter`, we also want to assign a numeric index but it's not as simple as with the intervals - these don't all have the same value. We will store an index:

```rust
use std::io;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum NoteLetter {
    C = 0,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl Default for NoteLetter {
    fn default() -> Self {
        NoteLetter::C
    }
}

impl FromStr for NoteLetter {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(NoteLetter::A),
            "B" => Ok(NoteLetter::B),
            "C" => Ok(NoteLetter::C),
            "D" => Ok(NoteLetter::D),
            "E" => Ok(NoteLetter::E),
            "F" => Ok(NoteLetter::F),
            "G" => Ok(NoteLetter::G),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not a valid note", s),
            )),
        }
    }
}
```

The notes are C-indexed, for better or for worse, so `NoteLetter::default()` should return that variant. We'll talk more about why it's C and not A after learning about Modes below. Don't worry, it's suitably disappointing.

Next up we have a `Note` which consists of a letter and optionally an accidental:

```rust
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Note {
    accidental: Option<Accidental>,
    letter: NoteLetter,
}
```

For this one, we only want to display a character for an accidental if there's anything there:

```rust
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let acc_str = if let Some(a) = self.accidental {
            format!("{}", a)
        } else {
            "".to_string()
        };
        write!(f, "{:?}{}", self.letter, acc_str)
    }
}
```

There's a little more logic to pull them out of strings:

```rust
impl FromStr for Note {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char_strs = char_strs(s);
        let mut char_strs = char_strs.iter();
        // note will be first
        if let Some(letter) = char_strs.next() {
            let letter = NoteLetter::from_str(letter)?;
            if let Some(accidental) = char_strs.next() {
                // check if it's valid
                let accidental = Accidental::from_str(accidental)?;
                return Ok(Self {
                    letter,
                    accidental: Some(accidental),
                });
            } else {
                return Ok(Self {
                    letter,
                    accidental: None,
                });
            }
        }
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} is not a valid note", s),
        ))
    }
}
```

This uses one helper function I defined:

```rust
#[test]
fn test_char_strs() {
        assert_eq!(char_strs("Hello"), ["H", "e", "l", "l", "o"])
}
```

If anyone has a cleaner solution I'm all ears:

```rust
fn char_strs<'a>(s: &'a str) -> Vec<&'a str> {
    s.split("")
        .skip(1)
        .take_while(|c| *c != "")
        .collect::<Vec<&str>>()
}
```

The missing piece is the `Accidental`. [Accidentals](<https://en.wikipedia.org/wiki/Accidental_(music)>) are represented in strings as `♭` for flat or `#` for sharp, which lower or raise the note by one semitone (or `Interval::Min2`) respectively. This does produce 14 possible values for 12 possible semitones - the exceptions are wherever there's no black key in between two white keys.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Accidental {
    Flat,
    Sharp,
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Accidental::*;
        let acc_str = match self {
            Flat => "♭",
            Sharp => "#",
        };
        write!(f, "{}", acc_str)
    }
}

impl FromStr for Accidental {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" | "♭" => Ok(Accidental::Flat),
            "#" => Ok(Accidental::Sharp),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not a valid accidental", s),
            )),
        }
    }
}
```

There is third accidental called "natural", `♮`, which cancels these out. To represent a pitch in data we don't need it - we can get each of the piano keys with just `Accidental::Sharp`. We really just include `Accidental::Flat` for a smooth user experience - people expect those to be valid notes, even though they represent the same pitch. The natural symbol is generally used for overriding a [key signature](https://en.wikipedia.org/wiki/Key_signature), which defines the default accidental for all the notes within a scale on [sheet music](<https://en.wikipedia.org/wiki/Staff_(music)>). There are a series of accidentals on the margin of the staff that apply to all notes, which is how we ensure we play notes within a single given scale, or [key](<https://en.wikipedia.org/wiki/Key_(music)>). However, you may choose to compose a melody that contains a note outside this key. If encounter the note `F#♮` on your sheet, you play an F. This program isn't (yet) smart enough to work with these.

Now we can finally define a specific tone on a full piano. A standard pitch, in our program a `PianoKey`, is composed of two components: a `Note` and a 0-indexed octave:

```rust
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct PianoKey {
    note: Note,
    octave: u8,
}
```

To show them, we just want to print them out next to each other:

```rust
impl fmt::Display for PianoKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.note, self.octave)
    }
}
```

This one also has a little more logic to pull out of a string, building from the constituent components:

```rust
impl FromStr for PianoKey {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // It makes sense to get the letter to Intervals
        if let Some(octave) = char_strs(s).last() {
            if let Ok(octave) = octave.parse::<u8>() {
                let note = Note::from_str(&s[0..s.len() - 1])?;
                if octave <= Self::max_octave() {
                    Ok(Self { note, octave })
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} is too high!", octave),
                    ))
                }
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} is too high for this keyboard", octave),
                ))
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not a valid note", s),
            ))
        }
    }
}
```

The octave just starts at 0 and won't ever realistically rise above 255, so a `u8` is fine. We can give ourselves a few convenience methods:

```rust
impl PianoKey {
    pub fn new(s: &str) -> Result<Self, io::Error> {
        Self::from_str(s)
    }
    fn max_octave() -> u8 {
        8
    }
}
```

Thanks to all the nested `Default` blocks, the `Default` implementation that the compiler derives for `PianoKey` corresponds to the official base pitch of this system, `C0`, specified in the first assertion of the test. Speaking of which, `test_new_pitch()` should now pass.

##### Intervals

The cyan key is Middle C, and A440 is highlighted in yellow. The octaves on an 88-key piano are numbered as shown, so often A440 is simply denoted "A4" especially when dealing with a keyboard. You may own a tuner that marks 440Hz/A4 specifically if you're a musician. This pitch is used for calibrating musical instruments and tuning a group, as well as a baseline constant for calculating frequencies.

Note how each octave starts at C, not A, so A4 is actually higher in pitch than C4. Octaves are "C-indexed" and base 8: `C D E F G A B C` is the base unmodified scale.

The smallest of interval between notes on a piano (and most of Western music) is called a [semitone](https://en.wikipedia.org/wiki/Semitone), also called a minor second or half step. We'll need to keep track of these as the basic unit of a keyboard interval:

```rust
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Semitones(i8);

impl From<i8> for Semitones {
    fn from(i: i8) -> Self {
        Self(i)
    }
}

impl From<Semitones> for i8 {
    fn from(s: Semitones) -> Self {
        s.0
    }
}
```

Take a look back at that piano diagram above - one semitone is the distance between two adjacent keys. A _whole_ step, or a [major second](https://en.wikipedia.org/wiki/Major_second), is equal to two semitones, or two adjacent white keys that pass over a black key. To play from C4 to C5, you'll use 12 keys (count all the white and black keys in a bracket), so octaves are divided into 12 equal semitones. There's a name for [each interval](<https://en.wikipedia.org/wiki/Interval_(music)#Main_intervals>):

```rust
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Interval {
    Unison = 0,
    Min2,
    Maj2,
    Min3,
    Maj3,
    Perfect4,
    Tritone,
    Perfect5,
    Min6,
    Maj6,
    Min7,
    Maj7,
    Octave,
}
```

By including a numeric index with `Unison = 0`, each variant also gets assigned the next successive ID. This way we can refer to each by name but also get an integer corresponding to the number of semitones when needed: `Interval::Maj2 as i8` returns `2_i8`.

Two identical notes are called a [unison](https://en.wikipedia.org/wiki/Unison), with 0 cents. These intervals are defined within a single octave, so any of them apply across octaves as well - A4 and A5 are in unison just like A4 and another A4, and C4 and A5 is still a major sixth. The terms "major", "minor", and "perfect" are not arbitrary, but that discussion is outside the scope of this post. I will note that the [tritone](https://en.wikipedia.org/wiki/Tritone), representing 3 whole tones or 6 semitones like `F-B`, is the only one that's none of the three.

If interested, I recommend [harmony](https://en.wikipedia.org/wiki/Harmony) for your next rabbit hole. The tritone takes a leading role in [dissonance](https://en.wikipedia.org/wiki/Consonance_and_dissonance), and to hear it in action you should check out what the [Locrian mode](https://en.wikipedia.org/wiki/Locrian_mode) we defined sounds like with this program. The C major scale has a perfect fifth, 5 semitones at the [dominant](<https://en.wikipedia.org/wiki/Dominant_(music)>) scale [degree](<https://en.wikipedia.org/wiki/Degree_(music)>) - and the Locrian mode has a tritone which is one extra semitone.

These all map to numbers, but we don't want to have to think about the rules when adding and subtracting. Let's do a little plumbing:

```rust
#[test]
fn test_add_interval() {
    use Interval::*;
    assert_eq!(Unison + Unison, Unison);
    assert_eq!(Unison + Maj3, Maj3);
    assert_eq!(Maj2 + Min3, Perfect4);
    assert_eq!(Octave + Octave, Unison);
    assert_eq!(Tritone + Tritone, Unison);
    assert_eq!(Maj7 + Min3, Maj2);
}

#[test]
fn test_sub_interval() {
    use Interval::*;
    assert_eq!(Unison - Unison, Unison);
    assert_eq!(Unison - Maj3, Min6);
    assert_eq!(Maj2 - Min3, Maj7);
    assert_eq!(Octave - Octave, Unison);
    assert_eq!(Tritone - Tritone, Unison);
    assert_eq!(Maj7 - Min3, Min6);
}
```

First, a little plumbing:

```rust
impl From<Interval> for i8 {
    fn from(i: Interval) -> Self {
        Semitones::from(i).into()
    }
}

impl From<Semitones> for Interval {
    fn from(s: Semitones) -> Self {
        use Interval::*;
        let int_semitones = i8::from(s);
        match int_semitones {
            0 => Unison,
            1 => Min2,
            2 => Maj2,
            3 => Min3,
            4 => Maj3,
            5 => Perfect4,
            6 => Tritone,
            7 => Perfect5,
            8 => Min6,
            9 => Maj6,
            10 => Min7,
            11 => Maj7,
            12 | _ => Interval::from(Semitones(int_semitones % Octave as i8)),
        }
    }
}

impl From<Interval> for Semitones {
    fn from(i: Interval) -> Self {
        Semitones(i as i8)
    }
}
```

Now we can define `Add` and `Sub`:

```rust
use std::ops::{Add, Sub};

impl Add for Interval {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Interval::from(Semitones(
            i8::from(self) + i8::from(rhs) % Interval::Octave as i8,
        ))
    }
}

impl Sub for Interval {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut delta = i8::from(self) - i8::from(rhs);
        if delta < 0 {
            delta += Interval::Octave as i8;
        };
        Interval::from(Semitones(delta))
    }
}
```

That gets us `+` and `-`, but we're going to want `+=` later too and that's really easy now:

```rust
use std::ops::AddAssign;

impl AddAssign for Interval {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
```

We can also relate Notes to Intervals pretty well:

```rust
#[test]
fn test_get_note_interval_from_c() {
    use Interval::*;
    assert_eq!(Note::from_str("A").unwrap().interval_from_c(), Maj6);
    assert_eq!(Note::from_str("A#").unwrap().interval_from_c(), Min7);
    assert_eq!(Note::from_str("Bb").unwrap().interval_from_c(), Min7);
    assert_eq!(Note::from_str("B").unwrap().interval_from_c(), Maj7);
    assert_eq!(Note::from_str("C").unwrap().interval_from_c(), Unison);
    assert_eq!(Note::from_str("C#").unwrap().interval_from_c(), Min2);
    assert_eq!(Note::from_str("D").unwrap().interval_from_c(), Maj2);
    assert_eq!(Note::from_str("D#").unwrap().interval_from_c(), Min3);
    assert_eq!(Note::from_str("E").unwrap().interval_from_c(), Maj3);
    assert_eq!(Note::from_str("F").unwrap().interval_from_c(), Perfect4);
    assert_eq!(Note::from_str("F#").unwrap().interval_from_c(), Tritone);
    assert_eq!(Note::from_str("G").unwrap().interval_from_c(), Perfect5);
    assert_eq!(Note::from_str("G#").unwrap().interval_from_c(), Min6);
}

#[test]
fn test_get_note_offset() {
    use Interval::*;
    let a = Note::from_str("A").unwrap();
    assert_eq!(Note::from_str("A").unwrap().get_offset(a), Unison);
    assert_eq!(Note::from_str("A#").unwrap().get_offset(a), Min2);
    assert_eq!(Note::from_str("B").unwrap().get_offset(a), Maj2);
    assert_eq!(Note::from_str("C").unwrap().get_offset(a), Min3);
    assert_eq!(Note::from_str("C#").unwrap().get_offset(a), Maj3);
    assert_eq!(Note::from_str("D").unwrap().get_offset(a), Perfect4);
    assert_eq!(Note::from_str("D#").unwrap().get_offset(a), Tritone);
    assert_eq!(Note::from_str("E").unwrap().get_offset(a), Perfect5);
    assert_eq!(Note::from_str("F").unwrap().get_offset(a), Min6);
    assert_eq!(Note::from_str("F#").unwrap().get_offset(a), Maj6);
    assert_eq!(Note::from_str("G").unwrap().get_offset(a), Min7);
    assert_eq!(Note::from_str("G#").unwrap().get_offset(a), Maj7);
}

#[test]
fn test_add_interval_to_note() {
    use Interval::*;
    let a = Note::from_str("A").unwrap();
    assert_eq!(a + Unison, a);
    assert_eq!(a + Min2, Note::from_str("A#").unwrap());
    assert_eq!(a + Maj2, Note::from_str("B").unwrap());
    assert_eq!(a + Min3, Note::from_str("C").unwrap());
    assert_eq!(a + Maj3, Note::from_str("C#").unwrap());
    assert_eq!(a + Perfect4, Note::from_str("D").unwrap());
    assert_eq!(a + Tritone, Note::from_str("D#").unwrap());
    assert_eq!(a + Perfect5, Note::from_str("E").unwrap());
    assert_eq!(a + Min6, Note::from_str("F").unwrap());
    assert_eq!(a + Maj6, Note::from_str("F#").unwrap());
    assert_eq!(a + Min7, Note::from_str("G").unwrap());
    assert_eq!(a + Maj7, Note::from_str("G#").unwrap());
}
```

This all works with the logic we've already modelled:

```rust
impl From<Interval> for Note {
    // Take an interval from C
    fn from(i: Interval) -> Self {
        use Interval::*;
        let mut offset = Unison;
        // That's a series of Min2
        let scale = Scale::Chromatic.get_intervals();
        scale.iter().take(i as usize).for_each(|i| offset += *i);
        Note::default() + offset
    }
}

impl Add<Interval> for Note {
    type Output = Self;

    fn add(self, rhs: Interval) -> Self {
        let semitones = Semitones::from(rhs);
        let mut ret = self;
        for _ in 0..i8::from(semitones) {
            ret.inc();
        }
        ret
    }
}

impl AddAssign<Interval> for Note {
    fn add_assign(&mut self, rhs: Interval) {
        *self = *self + rhs;
    }
}
```

For `Add<Interval> for Note` to work, we need to add some extra helper methods:

```rust
impl NoteLetter {
    // ..
    fn inc(self) -> Self {
        use NoteLetter::*;
        match self {
            C => D,
            D => E,
            E => F,
            F => G,
            G => A,
            A => B,
            B => C,
        }
    }
}

impl Note {
    fn interval_from_c(self) -> Interval {
        use Accidental::*;
        let ret = self.letter.interval_from_c();
        if let Some(acc) = self.accidental {
            match acc {
                Flat => return Interval::from(Semitones::from(i8::from(Semitones::from(ret)) - 1)),
                Sharp => return ret + Interval::Min2,
            }
        };
        ret
    }
    fn get_offset_from_interval(self, other: Interval) -> Interval {
        let self_interval_from_c = self.interval_from_c();
        self_interval_from_c - other
    }
    fn get_offset(self, other: Self) -> Interval {
        let self_interval_from_c = self.interval_from_c();
        let other_interval_from_c = other.interval_from_c();
        self_interval_from_c - other_interval_from_c
    }
    fn inc(&mut self) {
        use Accidental::*;
        use NoteLetter::*;
        if let Some(acc) = self.accidental {
            self.accidental = None;
            match acc {
                Sharp => {
                    self.letter = self.letter.inc();
                }
                Flat => {}
            }
        } else {
            // check for special cases
            if self.letter == B || self.letter == E {
                self.letter = self.letter.inc();
            } else {
                self.accidental = Some(Sharp);
            }
        }
    }
}
```

Hold on, though - this won't compile yet. The `impl From<Interval> for Note` block depends on the intervals for `Scale::Chromatic`, and we haven't talked about scales yet.

##### Scales

_[top](#table-of-contents)_

A [scale](<https://en.wikipedia.org/wiki/Scale_(music)>) is a series of notes (frequencies) defined in terms of successive intervals from a base note. We'll start with the [major scale](https://en.wikipedia.org/wiki/Major_scale):

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scale {
    Major,
}

impl Default for Scale {
    fn default() -> Self {
        Scale::Major
    }
}
```

Clearly, there isn't a black key between every white key - there must be a method to the madness. The piano is designed to play notes from a category of scales called [diatonic scales](https://en.wikipedia.org/wiki/Diatonic_scale), where the full range of an octave consists of five whole steps and two half steps. That's why our `NoteLetter` indices needed some extra logic - while each pair of adjacent keys is one semitone, that doesn't always mean a white key to a black key or vice versa - the note pairs B/C and E/F are both only separated by one semitone.

We can see this visually on the keyboard - it has the same 8-length whole/half step pattern all the way through. The distribution pattern begins on C, but the keyboard itself starts at A0 and ends at C8. A piano is thus designed because it can play music across the full range of diatonic scales. This is where we get those base 8 sequences - just start on a different note.

That base pattern is the C [major scale](https://en.wikipedia.org/wiki/Major_scale). Start at Middle C, the one highlighted in cyan above, and count up to the next C key, eight white keys to the left. Each time you skip a black key is a whole step and if the two white keys are adjacent it's a half step. These are the steps you get counting up to the next C, when the pattern repeats. This totals 12 semitones per octave:

```txt
whole, whole, half, whole, whole, whole, half
  2  +  2   +  1  +   2   +  2  +   2  +  1   =  12
C    D     E      F       G      A     B     C
```

We can hardcode this sequence in Rust as a `Vec<Interval>`:

```rust
impl Scale {
    fn get_intervals(self) -> Vec<Interval> {
        use Interval::*;
        use Scale::*;
        match self {
            Major => vec![Maj2, Maj2, Min2, Maj2, Maj2, Maj2, Min2],
        }
    }
}
```

We need a method to map to exact intervals:

```rust
#[test]
fn test_note_letter_to_interval() {
    use Interval::*;
    use NoteLetter::*;
    assert_eq!(C.interval_from_c(), Unison);
    assert_eq!(D.interval_from_c(), Maj2);
    assert_eq!(E.interval_from_c(), Maj3);
    assert_eq!(F.interval_from_c(), Perfect4);
    assert_eq!(G.interval_from_c(), Perfect5);
    assert_eq!(A.interval_from_c(), Maj6);
    assert_eq!(B.interval_from_c(), Maj7);
}
```

Check out that interval sequence - we've seen something like this somewhere before:

```bash
split("0,2,4,5,7,9,11,12",a,",");
```

Aha! It's was a major scale over one octave this whole time, as a series of semitone offsets:

```txt
whole, whole, half, whole, whole, whole, half
  2  +  2   +  1  +   2   +  2  +   2  +  1
0    2     4      5      7      9     11     12
Un. Min2  Maj2  Perf4  Perf5   Maj6 Maj7   Octave
A4    B4   C#5    D5     E5    F#5   G#5    A5
```

Luckily _we already told Rust about this_ when we defined the major scale. Now our modelling efforts are finally beginning to pay off:

```rust
impl NoteLetter {
    fn interval_from_c(self) -> Interval {
        use Interval::Unison;
        Scale::default()
            .get_intervals()
            .iter()
            .take(self as usize)
            .fold(Unison, |acc, i| acc + *i)
    }
}
```

We can work with scales using the Rust [iterator methods](https://doc.rust-lang.org/std/iter/trait.Iterator.html)! This function takes the first n intervals of a scale, and then uses the special `impl Add for Interval` logic we defined to total everything up. For instance, to calculate `F`, this function grabs the first 3 intervals, `[Maj2, Maj2, Min2]`, and then sums them up, using `Unison`, or 0, as the base. This calculates the sum of `[2,2,1]`, which is `5` semitones, or `Interval::Perfect4`.

Doing the same exercise with the same intervals starting on a different while key will also produce a major scale but you will start using the black keys to do so. C is the note that allows you to stick to only white keys with this interval pattern, or has no sharps or flats in the key signature. Before we start generating sequences of notes, though, we need a way to represent a note.

##### Key

_[top](#table-of-contents)_

For context, once again here's the original line we're dealing with:

```bash
split("0,2,4,5,7,9,11,12",a,",");
```

We've now discovered that that list represents the list of semitone offsets from A4 that represent an A major scale. The random notes that get produced will all be frequencies that correspond to these offsets from 440Hz.

We way, way overshot this in the process of modelling the domain. We can now automatically generate sequences of `PianoKey` structs that correspond to keys on an 88-key piano to select from: `[C4 D4 E4 F4 G4 A5 B5 C5]`. If we want a different scale, we can just ask.

We don't necessarily want to stick within a single octave, though. We want to make available the full 108 keys from C0 to B8 (even larger than the standard piano from the diagram), letting the user decide how many octaves to pick from, but only use notes in the key.

```rust
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Key {
    base_note: PianoKey,
    octaves: u8,
    scale: Scale,
}

impl Key {
    pub fn new(scale: Scale, base_note: PianoKey, octaves: u8) -> Self {
        let octaves = if base_note.octave + octaves > 8 {
            PianoKey::max_octave() - base_note.octave
        } else {
            octaves
        };
        Self {
            base_note,
            octaves,
            scale,
        }
    }
    fn all_keys(self) -> Vec<PianoKey> {
            let notes = self.get_notes();
            let mut ret = Vec::new();
            for i in 0..self.octaves {
                notes.iter().for_each(|n| {
                    ret.push(
                        PianoKey::from_str(&format!("{}{}", *n, i + self.base_note.octave)).unwrap_or_else(|_|
                            PianoKey::from_str(&format!("{}{}", *n, PianoKey::max_octave())).unwrap(),
                        ),
                    )
                });
            }
            ret
        }
}
```

These will be displayed as simply the octave-less notes in the scale:

```rust
#[test]
fn test_c_major() {
    assert_eq!(
        &Key::new(Scale::default(), PianoKey::default(), 1).to_string(),
        "[ C D E F G A B C ]"
    )
}

#[test]
fn test_a_major() {
    assert_eq!(
        &Key::new(Scale::default(), PianoKey::from_str("A4").unwrap(), 1).to_string(),
        "[ A B C# D E F# G# A ]"
    )
}

#[test]
fn test_g_major() {
    assert_eq!(
        &Key::new(Scale::default(), PianoKey::from_str("G4").unwrap(), 1).to_string(),
        "[ G A B C D E F# G ]"
    )
}
```

To produce all the notes in a given key, we need to calculate them from the scale and the base note:

```rust
impl Key {
    // ..
    pub fn get_notes(self) -> Vec<Note> {
        let mut ret = vec![self.base_note.note];
        let mut offset = Interval::Unison;
        self.scale.get_intervals().iter().for_each(|i| {
            offset += *i;
            ret.push(self.base_note.note + offset)
        });
        ret
    }
}
```

```rust
impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let notes = self.get_notes();
        let mut ret = String::from("[ ");
        notes.iter().for_each(|n| ret.push_str(&format!("{} ", *n)));
        ret.push_str("]");
        write!(f, "{}", ret)
    }
}
```

This uses the `impl Add for Interval` logic we'd previous defined to count up successive intervals across a scale, resulting in a more concrete set of notes. Now we can add the `Display` implementation used in the test code - this trait also provides the `to_string()` method:

```rust
impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let notes = self.get_notes();
        let mut ret = String::from("[ ");
        notes.iter().for_each(|n| ret.push_str(&n.to_string()));
        ret.push_str("]");
        write!(f, "{}", ret)
    }
}
```

##### Circle of Fifths

_[top](#table-of-contents)_

With `Key` defined, we can start talking about other scales.

Using the same intervals as the C major scale starting on a different note will also produce a major scale but you will start using the black keys. This is called the key signature, and there's one for each variant of the major scale starting from each black key. They're related by the [circle of fifths](https://en.wikipedia.org/wiki/Circle_of_fifths):

![circle](https://upload.wikimedia.org/wikipedia/commons/3/33/Circle_of_fifths_deluxe_4.svg)

The C major scale has all white keys. To find the version of the major scale that adds one single black key to augment a tone, you go up a perfect fifth, or 7 semitones: [`Interval::Perfect5`](https://en.wikipedia.org/wiki/Perfect_fifth). This has a ratio 3:2.

One perfect fifth up from `C` is `G`, so the next scale around the circle is [G major](https://en.wikipedia.org/wiki/G_major). It has one sharp: A. Go [back up](#a-little-music-theory) to the piano diagram and count up the major scale sequence from G, for example one note below the yellow A4. You'll need the `F#` black key at the last step right before G5, but all the other hops white stick to the white keys. [D major](https://en.wikipedia.org/wiki/D_major) will need two black keys, `F#` and `C#`. If you continue incrementing a fifth (remember, octave is irrelevant here), you'll hit all 12 possible patterns before getting back to C. To get through all the key signatures incrementally, one accidental at a time, you keep going up by perfect fifths. Once you come all the way back to C, you'll have hit all 12 keys, encompassing all possible key signatures.

This diagram also shows the [relative natural minor](https://en.wikipedia.org/wiki/Relative_key) for each. That's a sneak preview of the Aeolian mode in the next section!

It's true that, e.g. `D#` and `E♭` represent the same pitch - what's different is why we got there. After the midway point, it's easier to denote 5 flats than 7 sharps, even though they mean the same tones - there's only 5 black keys to choose from, after all.

To go counter-clockwise, go up by a perfect fourth every time, which is 5 semitones. This is known as "circle of fourths", and is more commonly associated with [jazz](https://en.wikipedia.org/wiki/Jazz) music whereas fifths are seen in more [classical](https://en.wikipedia.org/wiki/Classical_music) contexts.

This program doens't use it, but we can generate all of them by just passing each note into `Key::new()`:

```rust
impl Scale {
    pub fn circle_of_fifths() -> Vec<Key> {
        let mut ret = Vec::new();
        // Start with C
        let mut current_base = Note::default();
        // Increment by fifths and push to vector
        for _ in 0..ScaleLength::Dodecatonic as usize {
            ret.push(Key::new(Scale::default(), &current_base.to_string()));
            current_base += Interval::Perfect5;
        }
        ret
    }
}
```

That's twelve scales for free:

```txt
[ C D E F G A B C ]
[ G A B C D E F# G ]
[ D E F# G A B C# D ]
[ A B C# D E F# G# A ]
[ E F# G# A B C# D# E ]
[ B C# D# E F# G# A# B ]
[ F# G# A# B C# D# F F# ]
[ C# D# F F# G# A# C C# ]
[ G# A# C C# D# F G G# ]
[ D# F G G# A# C D D# ]
[ A# C D D# F G A A# ]
[ F G A A# C D E F ]
```

This implementation isn't smart enough to switch to flats halfway through to represent the black keys used - could be a fun mini-challenge! Maybe you could extend this to hop to different scales around the circle periodically.

##### Diatonic Modes

_[top](#table-of-contents)_

Now we can produce the 12 transpositions of major scale from C - just pick any note of the keyboard and count up the same intervals. However, this pattern of white and black repeats all the way up and down the whole length of the keyboard - what if we didn't start at C to set the base of the black-key/white-key pattern? Why not use `A B C D E F G A`?

If you start on any other white key and count up one octave skipping all the black keys, you will get a _different_ diatonic scale than a major scale. These scale variations are called [Modes](<https://en.wikipedia.org/wiki/Mode_(music)#Modern_modes>), and while high-school me was terrified of and terrible at whipping out arbitrary ones on a brass instrument from memory (mental math is _not_ one of my talents), they're easy to work with programmatically (and much less stressful).

The major scale is also known as the [Ionian mode](https://en.wikipedia.org/wiki/Ionian_mode). This is the base mode, each other is some offset from this scale. As we've seen, the key you need to start on to play this mode with no black keys (accidentals) is C.

The natural minor scale, is obtained by starting at A4 and counted up white keys, is called the [Aeolian mode](https://en.wikipedia.org/wiki/Aeolian_mode). Try it yourself on the diagram - march on up the white keys from A4 to A5:

```txt
whole, half, whole, whole, half, whole, whole
```

This should look like the C major scale, no sharps or flats, but with `A` at the beginning:

```rust
#[test]
fn test_a_minor() {
    use Mode::*;
    use Scale::*;
    assert_eq!(
        &Key::new(Diatonic(Aeolian), PianoKey::from_str("A4").unwrap(), 1).to_string(),
        "[ A B C D E F G A ]"
    )
}
```

It's the same pattern, just starting at a different offset. You can play a corresponding minor scale using only the white keys by simply starting at the sixth note of the C major scale (or incrementing a major sixth), which is A. Try counting it out yourself up from A4.

There's an absurdly fancy name for each offset:

```rust
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Mode {
    Ionian = 0,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}
```

We'll hardcode the C major sequence as the base:

```txt
whole, whole, half, whole, whole, whole, half
  2  +  2   +  1  +   2   +  2  +   2  +  1
```

```rust
impl Mode {
    fn base_intervals() -> Vec<Interval> {
        use Interval::*;
        vec![Maj2, Maj2, Min2, Maj2, Maj2, Maj2, Min2]
    }
}
```

Let's also hardcode the scale length:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScaleLength {
    Heptatonic = 7,
}
```

Now we can make our scales a little smarter:

```diff
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub enum Scale {
-     Major,
      Diatonic(Mode),
  }

  impl Default for Scale {
      fn default() -> Self {
-         Scale::Major
+         Scale::Diatonic(Mode)
      }
  }

  impl Scale {
    fn get_intervals(self) -> Vec<Interval> {
            use Interval::*;
            use Scale::*;
            match self {
-               Major => vec![Maj2, Maj2, Min2, Maj2, Maj2, Maj2, Min2],
+               Diatonic(mode) => Mode::base_intervals()
+                   .iter()
+                   .cycle()
+                   .skip(mode as usize)
+                   .take(ScaleLength::Heptatonic as usize)
+                   .copied()
+                   .collect::<Vec<Interval>>(),
+           }
        }
  }
```

Now we've got seven modes for each of twelve keys on the keyboard - that's 84 distinct key signatures. The `Ionian` and `Aeolian` modes have nicknames, the rest we'll just work with as the mode names:

```rust
impl FromStr for Scale {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Mode::*;
        use Scale::*;
        match s.to_uppercase().as_str() {
            "IONIAN" | "MAJOR" => Ok(Diatonic(Ionian)),
            "DORIAN" => Ok(Diatonic(Dorian)),
            "PHRYGIAN" => Ok(Diatonic(Phrygian)),
            "LYDIAN" => Ok(Diatonic(Lydian)),
            "MIXOLYDIAN" => Ok(Diatonic(Mixolydian)),
            "AEOLIAN" | "MINOR" => Ok(Diatonic(Aeolian)),
            "LOCRIAN" => Ok(Diatonic(Locrian)),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown scale")),
        }
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Scale::*;
        let s = match self {
            Diatonic(mode) => {
                use Mode::*;
                match mode {
                    Aeolian => "minor scale".into(),
                    Ionian => "major scale".into(),
                    _ => format!("{:?} mode", mode),
                }
            }
        };
        write!(f, "{}", s)
    }
}
```

The fact that Ionian Mode/C Major is Offset 0 is actually somewhat arbitrary - though definitely not completely. There's a reason C major is so commonly found in music - it sounds good.

I did a [bare-minimum](https://lmgtfy.com/?q=why+does+it+start+from+C+not+A) amount of research and found it's an ["unfortunate historical accident"](https://music.stackexchange.com/questions/893/why-is-c-the-base-note-of-standard-notation-and-keys). In a sentence, the concept of "mode" in an equally tempered system predates the modern scales and `C == 0` is a historical artifact. The letters were originally numbered from A, of course, but got mapped to frequencies well before the modern modes we use now were honed and refined from previous systems. The system eventually came to be based around the [C major scale](https://en.wikipedia.org/wiki/C_major), not A major. By then the fact that what's now Middle C was 261.626Hz was long done and over with.

##### Non Heptatonic Scales

_[top](#table-of-contents)_

Okay, Ben. Ben, okay. Okay, Ben. We've arrived at the version from the blog post, great. You also promised 86 in the introduction, not 84. This whole time, though, the line from the meme image has had something different:

```bash
split("4,5,7,11",a,",");
```

The diatonic scales we've been working with are a subset of the [heptatonic scales](https://en.wikipedia.org/wiki/Heptatonic_scale), with seven notes each. These tones are naturally further apart than we've been using. Let's add a couple others scale lengths to play with:

```diff
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub enum ScaleLength {
+     Tetratonic = 4,
      Heptatonic = 7,
+     Dodecatonic = 12,
  }
```

Interestingly, the scale shown is [tetratonic](https://en.wikipedia.org/wiki/Tetratonic_scale), given as octave-less notes, intervals from base, and offsets from A440:

```txt
[C#, D, E, G#]
[Min2, Maj2, Maj3]
[4, 5, 7, 11]
```

Oddly, this scale is primarily associated with pre-historic music and not often found since. Was `AWK` passed down from the before-times? I also don't understand how that snippet works, because it's still indexed with `a[$1 % 8]`, but I'm too lazy to find out why.

The only dodecatonic scale is the [chromatic scale](https://en.wikipedia.org/wiki/Chromatic_scale) is just all the notes:

```txt
[A, A#, B, C, C#, D, D#, E, F, F#, G, G#]
[Min2, Min2, Min2, Min2, Min2, Min2, Min2, Min2, Min2, Min2, Min2]
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
```

Who needs key signatures anyhow, that's a waste of all these other keys! This one throws 'em all in the mix.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scale {
    Chromatic,
    // ..
}
```

The chromatic scale is for people who don't have time to muck about with petty concerns like sounding good, and don't want to waste any piano keys - it's just 11 successive minor 2nds, giving you every note.

```txt
half, half, half, half, half, half, half, half, half, half, half
A    A#    B     C     C#    D     D#    E     F     F#    G    G#
```

Or, in Rust:

```rust
#[test]
fn test_chromatic_intervals() {
    use Interval::Min2;
    assert_eq!(
        Scale::Chromatic.get_intervals(),
        vec![Min2, Min2, Min2, Min2, Min2, Min2, Min2, Min2, Min2, Min2, Min2]
    );
}
```

```diff
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub enum Scale {
+     Chromatic,
      Diatonic(Mode),
+     Tetratonic,
  }

  impl Scale {
    // ..
    fn get_intervals(self) -> Vec<Interval> {
        use Interval::*;
        use Scale::*;
        match self {
+           Chromatic => [Min2]
+               .iter()
+               .cycle()
+               .take(ScaleLength::Dodecatonic as usize)
+               .copied()
+               .collect::<Vec<Interval>>(),
            Diatonic(mode) => Mode::base_intervals()
                .iter()
                .cycle()
                .skip(mode as usize)
                .take(ScaleLength::Heptatonic as usize)
                .copied()
                .collect::<Vec<Interval>>(),
+           Tetratonic => vec![Min2, Maj2, Maj3],
          }
      }
  }

  impl FromStr for Scale {
      type Err = io::Error;
      fn from_str(s: &str) -> Result<Self, Self::Err> {
          use Mode::*;
          use Scale::*;
          match s.to_uppercase().as_str() {
              "IONIAN" | "MAJOR" => Ok(Diatonic(Ionian)),
              "DORIAN" => Ok(Diatonic(Dorian)),
              "PHRYGIAN" => Ok(Diatonic(Phrygian)),
              "LYDIAN" => Ok(Diatonic(Lydian)),
              "MIXOLYDIAN" => Ok(Diatonic(Mixolydian)),
              "AEOLIAN" | "MINOR" => Ok(Diatonic(Aeolian)),
              "LOCRIAN" => Ok(Diatonic(Locrian)),
+             "CHROMATIC" => Ok(Chromatic),
+             "TETRATONIC" => Ok(Tetratonic),
              _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown scale")),
          }
      }
  }


  impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Scale::*;
        let s = match self {
+           Chromatic | Tetratonic => format!("{:?} scale", self).to_lowercase(),
            Diatonic(mode) => {
                use Mode::*;
                match mode {
                    Aeolian => "minor scale".into(),
                    Ionian => "major scale".into(),
                    _ => format!("{:?} mode", mode),
                }
            }
        };
        write!(f, "{}", s)
    }
}
```

This could be a potential natural application of [dependent types](https://en.wikipedia.org/wiki/Dependent_type), a programming language feature that Rust does not support. Few languages do. One example is [Idris](<https://en.wikipedia.org/wiki/Idris_(programming_language)#Dependent_types>), which is like [Haskell](<https://en.wikipedia.org/wiki/Haskell_(programming_language)>)++. A dependent type codifies a type restraint that's dependent on a _value_ of that type. The linked example describes a function that appends a list of `m` elements to a list `n` which specifies as part of the return type that the returned list has length `n + m`. A caller can then trust this fact implicitly, because the compiler won't build a binary if it's not true. I think this could be applied here to verify that a scale's intervals method returns an octave, regardless of length. That can be tested for in code with Rust, of course, but not encoded into the type signature directly.

Those are just two examples, I bet you could find some other interesting patterns on the keyboard diagram. For instance, what happens if you ignore the white keys and _only_ use the black keys?

#### Generating Music

_[top](#table-of-contents)_

It's finally time to make some music. We've now built ourselves a toolkit for working with piano keys and intervals, and a separate type for dealing with a frequency in Hertz, and they both know how to interact with the same `Interval` variants. Now we need to get from `PianoKey` obtects to `Pitch`es.

##### Cents

_[top](#table-of-contents)_

Discrete units like `Semitones` are useful for working with a keyboard, but as we know, sound is analog and continuous. We need to subdivide these intervals even more granularly, and because of equal temperament we're free to do so at any arbitrary level.

Beyond the twelve 12 semitones in an octave, each semitone is divided into 100 [cents](<https://en.wikipedia.org/wiki/Cent_(music)>). This means a full octave, representing a 2:1 ratio in frequency, spans 1200 cents, and each cent can be divided without losing the ratio as well if needed:

```rust
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Cents(f64);
```

We need to do a little plumbing to let ourselves work at this higher level of abstraction. We need to be able to translate our discrete `Semitones` into `Cents` ergonomically:

```rust
#[test]
fn test_semitones_to_cents() {
    assert_eq!(Cents::from(Semitones(1)), Cents(100.0));
    assert_eq!(Cents::from(Semitones(12)), Cents(1200.0));
}
```

We can give ourselves some conversions to the inner primitive:

```rust
impl From<f64> for Cents {
    fn from(f: f64) -> Self {
        Cents(f)
    }
}

impl From<Cents> for f64 {
    fn from(c: Cents) -> Self {
        c.0
    }
}
```

Now we can encode the conversion factor:

```rust
const SEMITONE_CENTS: Cents = Cents(100.0);

impl From<Semitones> for Cents {
    fn from(s: Semitones) -> Self {
        Cents(i8::from(s) as f64 * f64::from(SEMITONE_CENTS))
    }
}
```

With that in place, we're ready to start working with intervals directly and have Rust understand them in terms of cents:

```rust
#[test]
fn test_interval_to_cents() {
    use Interval::*;
    assert_eq!(Cents::from(Unison), Cents(0.0));
    assert_eq!(Cents::from(Min2), Cents(100.0));
    assert_eq!(Cents::from(Octave), Cents(1200.0));
}
```

We need `Interval` variants to map directly to `Semitones` instead of plain integers, to make sure they're always turned into `Cents` correctly:

With that, it's easy to map `Interval`s to `Cents`:

```rust
impl From<Interval> for Cents {
    fn from(i: Interval) -> Self {
        Semitones::from(i).into()
    }
}
```

Phew! Lots of code, but now we can operate directly in terms of `Interval` variants or anything in between and everything stays contextually tagged. Verify with `cargo test` that everything checks out.

There's one more step to get from our brand new floating point `Cents` to frequencies in `Hertz` though. Remember how Middle C was some crazy fraction, 261.626Hz? This is because cents are a [logarithmic](https://en.wikipedia.org/wiki/Logarithmic_scale) unit, standardized around the point 440.0. While a 2:1 ratio is nice and neat, we've been subdividing that arbitrarily wherever it makes sense to us. Now the arithmetic isn't always so clean. Doubling 440.0Hz will get 880.0Hz, but how would we add a semitone?

We know that to increase by one octave we double the frequency: `440 * 2`. We'd need to increase by a 12th of what doubling the number would do for a single semitone: `440 * 2^(1/12)`. Looks innocuous enough, but my calculator gives me 466.164, Rust gives me 466.1637615180899 - not enough to perceptually matter, but enough that it's important that the standard is the interval ratio and not the specific amount of Hertz to add or subtract. Those amounts will only be precise in floating point decimal representations at exact octaves from the base note, because that's integral factor after multiplying by 1 in either direction, 2 or 1/2.

Otherwise stated, the ratio between frequencies separated by a single cent is the 1200th root of 2, or 2^(1/1200). In decimal, it's about 1.0005777895. You wouldn't be able to hear a distinction between two tones a single cent apart. Using this math, it works out to just shy of 4 cents to cause an increase of 1Hz, more precisely around 3.9302 for a base frequency of 440.0.

Logarithmic units are helpful when the range of the y axis, in our case frequency, increases exponentially. We know the graph of frequency to pitch does because to jump by any single octave, we double what we have - we're multiplying at each step, not adding (which results in a linear graph). A4 is 440Hz, A5 is 880Hz, and by A6 we're already at 1,760Hz. The graph `f(x) = x^2` looks like this:

![x_squared](https://thepracticaldev.s3.amazonaws.com/i/mkh095mgcasg1soygrb7.png)

A [logarithm](https://en.wikipedia.org/wiki/Logarithm) is the inverse of an [exponent](https://en.wikipedia.org/wiki/Exponentiation). Our ratio had an exponent that was "1 divided by n", which is the inverse of raising something to the power of "n divided by 1", such as squaring it (n=2). This is otherwise written as an "nth root", in the case of a cent _n_ being 1,200. This counteracts the rapid growing curve we get by constantly squaring the frequency into a more linear scaled subdivision between octaves:

![cent graph](https://upload.wikimedia.org/wikipedia/commons/thumb/3/3f/Music_intervals_frequency_ratio_equal_tempered_pythagorean_comparison.svg/550px-Music_intervals_frequency_ratio_equal_tempered_pythagorean_comparison.svg.png)

Notice it's not a straight diagonal. We haven't removed the fact that frequencies are being multiplied, merely adjusted for it. We're taking a logarithm of something that has been squared, the frequency. This tames the steep increase but the line is still slightly curved.

Fractional cents and tones are a much better way to deal with intervals than by concrete frequency deltas. Knowing all this we can translate back to the frequency in Hertz of a desired pitch if we know both a base frequency and the number of cents to increase by:

![cents formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/920411bb22d357b13f69a76fa33557c707f7cb57)

Here, _a_ is the initial frequency in Hertz, _b_ is the target frequency, and _n_ is the number of cents by which to increase _a_.

Lets try to increase the standard pitch by single Hertz using the value above:

```rust
#[test]
fn test_add_cents_to_pitch() {
    let mut pitch = Pitch::default();
    pitch += Cents(3.9302);
    assert_eq!(pitch, Pitch::new(Hertz(441.0)));
}
```

It looks like we're going to need to divide some `Cents`, leveraging the `impl From<Cents> for f64` blocks we already defined:

```rust
use std::ops::Div;

impl Div for Cents {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Cents(f64::from(self) / f64::from(rhs))
    }
}
```

This is just performing floating point division on the inner value, but keeps it wrapped up in the `Cents` context for us so we can directly use `Cents(x) / Cents(y)`. We now know enough to manipulate a `Pitch` directly.

The [`AddAssign`](https://doc.rust-lang.org/std/ops/trait.AddAssign.html) trait gets us the `+=` operator, and can define it for any type we want on the right hand side:

```rust
use std::ops::AddAssign

impl AddAssign<Cents> for Pitch {
    #[allow(clippy::suspicious_op_assign_impl)] // needed to stop clippy from yelling
    fn add_assign(&mut self, rhs: Cents) {
        self.0 *= 2.0f64.powf((rhs / Cents::from(Interval::Octave)).into())
    }
}
```

Oops, we also need to `*=` an `f64` to a `Hertz`:

```rust
use std::ops::MulAssign;

impl MulAssign<f64> for Hertz {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}
```

Coincidentally, an `impl MulAssign<f64> for Frequency` in Hertz is the exact example on the official [`MulAssign`](https://doc.rust-lang.org/std/ops/trait.MulAssign.html) docs. Their style might be better. I don't know, you tell me?

If that's not quite clear, this is the exact equation shown above with a bit of extra noise. Dividing `cents` by `Cents::from(Interval::Octave)` leaves us with a `Cents` type, per the above `impl Div for Cents` block. However, we then want to pass the result to `2.0.powf(cents_ratio)`. The compiler knows it's an `f64` here because we explicitly specified it with `2.0_f64` to use as a base for [`powf()`](https://doc.rust-lang.org/std/primitive.f64.html#method.powf).

Sadly, though, `cargo test` tells us we have a problem:

![fail float](https://thepracticaldev.s3.amazonaws.com/i/bu70ahx1w5rfln6sa3jq.png)

Floating point arithmetic is not precise. However, a delta of as much as a whole Hertz, or almost 4 cents, isn't large enough for any human to perceive. The [just-noticeable difference](https://en.wikipedia.org/wiki/Just-noticeable_difference) is about 5 or 6 cents, or 5\*2^(1/1200). In this type we just care that it's "close enough". At a glance we can look at those results and understand that we got where we need to be. To convince Rust we're good to go, we can override the compiler-derived [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) behavior for this type:

```diff
- #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
+ #[derive(Debug, Clone, Copy, PartialOrd)]
  pub struct Pitch {
       frequency: Hertz,
  }
```

We can specify a tolerance for equality in code. I'm arbitrarily deciding that if two `Pitch` objects are within a tenth of a Hertz of each other, they're functionally equivalent:

```rust
impl Hertz {
    fn abs(self) -> Self {
        Self(self.0.abs())
    }
}

impl PartialEq for Pitch {
    fn eq(&self, other: &Pitch) -> bool {
        let tolerance = Hertz(0.1);
        let difference = (self.0 - other.0).abs();
        difference < tolerance
    }
}
```

There's no trait to define to get absolute values with `abs()`, but we can plop whatever method we want directly on `Hertz` too. Now the test we wrote will pass. Try it out!

Instead of adding single cents at a time, it's easier to work by semitone:

```rust
#[test]
fn test_add_semitones_to_pitch() {
    use Interval::Octave;
    let mut pitch = Pitch::default();
    pitch += Semitones::from(Octave);
    assert_eq!(pitch, Pitch::new(Hertz(880.0)))
}
```

That's pretty easy with the work we've already done:

```rust
impl AddAssign<Semitones> for Pitch {
    fn add_assign(&mut self, rhs: Semitones) {
        *self += Cents::from(rhs)
    }
}
```

In fact, why not just go straight for intervals:

```rust
#[test]
fn test_add_interval_to_pitch() {
    use Interval::Min2;
    let mut pitch = Pitch::default();
    pitch += Min2;
    assert_eq!(pitch, Pitch::new(Hertz(466.1)))
}
```

Naturally, this is also trivial:

```rust
impl AddAssign<Interval> for Pitch {
    fn add_assign(&mut self, rhs: Interval) {
        *self += Cents::from(rhs)
    }
}
```

Great - now we can add `Cents` to a `Pitch` and it automatically multiplies the `Hertz` correctly. However, we're not working with `Pitch` objects to generate key signatures. Our key signatures are sequences of `PianoKey`s. We need to convert to and from these two systems. Luckily, while they're based on different core unit, the both use the same `Interval` relationship, and we can use that as a go-between.

It's defined at a set frequency:

```rust
pub const C_ZERO: Hertz = Hertz(16.352);
```

This is super low - most humans bottom out around 20Hz. The 88-key piano's lowest note is up at A0, a 9-semitone [`major sixth`](https://en.wikipedia.org/wiki/Major_sixth) higher. Note how even though this is a different abstraction for working with pitches, the frequencies baked in to the standard are still pinned to the A440 scale.

We want to be able to convert from piano keys to pitches and have the frequencies work out for both standards:

```rust
#[test]
fn test_piano_key_to_pitch() {
    assert_eq!(Pitch::from(PianoKey::new("A4").unwrap()), Pitch::default());
    assert_eq!(Pitch::from(PianoKey::default()), Pitch::new(C_ZERO));
}
```

To get there, we can add octaves and smaller intervals up from `C0` to whatever note we need;

```rust
impl From<PianoKey> for Pitch {
    fn from(sp: PianoKey) -> Self {
        use Interval::*;
        let mut ret = Pitch::new(C_ZERO);
        // Add octaves
        for _ in 0..sp.octave {
            ret += Octave;
        }
        // Add note offset
        ret += sp.note.letter.interval_from_c();
        ret
    }
}
```

##### Random Notes

_[top](#table-of-contents)_

The only missing thing is picking what notes to play we just need to pick the notes to play.

Check out this section of the [source code](https://docs.rs/rodio/0.10.0/src/rodio/source/sine.rs.html#24) from the `rodio` crate for the `rodio::source::SineWave` we used above to check our A440 tone:

```rust
impl Iterator for SineWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let value = 2.0 * 3.14159265 * self.freq * self.num_sample as f32 / 48000.0;
        Some(value.sin())
    }
}
```

This `impl Iterator` block is handling the `for` loop in the cover image. It's calculating the exact amplitude of a sine wave at some fractional point between 0 and 1.

The math, in other words, is `440.0 * Pi * (current sample / total samples)`, multiplied by some value, in this case `2.0`. This code is calculating the sine wave at a given point within a cycle - for 0 to 1, there are 48,000 points to collect, so the current point is the sine wave of this frequency at whatever point we're at, stored as `self.num_sample`, between 0 and 1.

For some reason they've hardcoded [Pi](https://en.wikipedia.org/wiki/Pi), there are constants available like [`std::f32::consts::PI`](https://doc.rust-lang.org/std/f64/consts/constant.PI.html). I'd be interested to know if anyone would know why that's a good choice instead of relying on the language constant!

The `SineWave` struct reliably produces a single tone infinitely, but we want to change the pitch. The actual wave calculation is the same, though, we just need to add some extra logic to change up the pitch being produced.

We can actually use the linked source code as a template to provide our own `rodio::Source` implementation to append to the `Sink`.

Set up a data structure to hold on to some of the hardcoded values found in the above library snipper:

```rust
pub const SAMPLE_RATE: Hertz = Hertz(48_000.0);
pub type Sample = f32;

pub struct MusicMaker {
    key: Key,
    current_note: PianoKey,
    current_sample: usize,
    sample_rate: Hertz,
    volume: f32,
}

impl Default for MusicMaker {
    fn default() -> Self {
        Self {
            key: Key::default(),
            current_note: PianoKey::from_str("C4").unwrap(),
            current_sample: usize::default(),
            sample_rate: SAMPLE_RATE,
            volume: 2.0,
        }
    }
}

impl MusicMaker {
    pub fn new() -> Self {
        Self::default()
    }
    fn get_frequency(&mut self) -> Sample {
        let pitch = Pitch::from(self.current_note);
        pitch.into()
    }
}
```

To perform the wave sampling, we can actually pretty much copy-paste the `impl Iterator for SineWave` code, just using our struct's values:

```rust
use std::f32::consts::PI;

impl Iterator for MusicMaker {
    type Item = Sample; // Sampled amplitude
    fn next(&mut self) -> Option<Self::Item> {
        self.current_sample = self.current_sample.wrapping_add(1); // will cycle

        let value = self.volume
            * PI
            * self.get_frequency()
            * self.current_sample as Sample
            / f64::from(self.sample_rate) as Sample;

        if self.current_sample as f64 >= f64::from(self.sample_rate) {
            self.current_sample = 0;
            self.new_note();  // Hmm...
        }
        Some(value.sin())
    }
}
```

In order to use as a sound source we can pass to a `rodio::Sink`, we implement the `rodio::Source` trait, which can be implemented for any type that implements `Iterator`, so long as the `Item` associated type is valid as a sample:

```rust
use core::time::Duration;
use rodio::Source;

impl Source for MusicMaker {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        f64::from(self.sample_rate) as u32
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
```

The `current_frame_len()` and `total_duration()` bodies indicate that this source is infinite - there's no finite duration to return. You'll need to kill the process some other way. The `channels` method returns the number of frequencies in the signal, and we're just working with a single wave, so a single channel is all we need.

Now we're finally ready to call that `choose()` method on something. First, though, we need to select a random seed from the `rand` crate. We don't are about cryptographic soundness here, we just need random numbers, but speed is useful. The [`rand::rngs::SmallRng`](https://docs.rs/rand/0.7.2/rand/rngs/struct.SmallRng.html) random number generator is ideal for that. We can initialize it using `from_entropy()` to ultimately source it from the operating system - so, sorta in a roundabout way it actually is `dev/urandom`, or similar.

```diff
+  use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

   pub struct MusicMaker {
       key: Key,
+      seed: SmallRng,
       current_note: PianoKey,
       current_sample: usize,
       sample_rate: Hertz,
       volume: f32,
}

   impl Default for MusicMaker {
       fn default() -> Self {
           Self {
               key: Key::default(),
+              seed: SmallRng::from_entropy(),
               current_note: PianoKey::from_str("C4").unwrap(),
               current_sample: usize::default(),
               sample_rate: SAMPLE_RATE,
               volume: 2.0,
           }
       }
   }

   impl MusicMaker {
     pub fn new() -> Self {
         Self::default()
     }
     fn get_frequency(&mut self) -> Sample {
         let pitch = Pitch::from(self.current_note);
         println!("{:?}", pitch);
         pitch.into()
     }
+    fn new_note(&mut self) {
+        let keys = self.key.all_keys();
+        self.current_note = *keys.iter().choose(&mut self.seed).unwrap();  // There it is!  This whole time
+    }
  }
```

Now our `MusicMaker` can plug right into an audio output track. Replace your entry point `main()` function in `src/bin/music.rs` with this:

```rust
fn main() {
    println!("{}", GREETING);

    let device = default_output_device().unwrap();
    let sink = Sink::new(&device);
    let music = MusicMaker::new(PianoKey::new("A4").unwrap(), Scale::default(), 1);
    sink.append(music);
    sink.sleep_until_end();
}
```

Running this with `cargo run` will essentially match the output from the original `bash` one-liner.

![sad party](https://thepracticaldev.s3.amazonaws.com/i/82lipncvy6806zyjpg2r.gif)

##### User Parameters

_[top](#table-of-contents)_

There are several elements of this that are tweakable - the program that runs is a little lackluster given all the capability we've defined internally. Let's expose some options to the user at runtime.

Let's give a `base note`, a `scale` option, and a number of octaves to span upwards to define the valid notes, as well as a boolean to choose to instead just play a single tone:

```rust
// src/bin/music.rs
use structopt::StructOpt;

/// music is a procedural single-tone melody generator
#[derive(StructOpt, Debug)]
#[structopt(name = "music")]
struct Opt {
    /// Single-pitch mode
    #[structopt(short, long)]
    pitch_mode: bool,
    /// The base note to calculate the scale from
    #[structopt(short, long, default_value = "C4")]
    base_note: PianoKey,
    /// The series of intervals from the base note to use per octave
    #[structopt(short, long, default_value = "Ionian")]
    scale: Scale,
    /// Number of octaves over which to range, anything over 8 gets parsed as 8
    #[structopt(short, long, default_value = "1")]
    octaves: u8
}
```

```diff
// src/lib.rs
  impl MusicMaker {
-     pub fn new() -> Self
-         Self::default()
+     pub fn new(opt: Opt) -> Self {
+         Self::default().set_base_note(opt.base_note).set_scale(opt.scale).set_octaves(opt.octaves)
      }
      fn get_frequency(&mut self) -> Sample {
          let pitch = Pitch::from(self.current_note);
          pitch.into()
      }
      fn new_note(&mut self) {
          let keys = self.key.all_keys();
          self.current_note = *keys.iter().choose(&mut self.seed).unwrap();
      }
+     fn set_base_note(mut self, base_note: Note) -> Self {
+         self.key = Key::new(self.key.scale, &base_note.to_string());
+         self
+     }
+     fn set_scale(mut self, scale: Scale) -> Self {
+         self.key = Key::new(scale, &self.key.base_note.to_string());
+         self
+     }
  }
```

We have to dispatch a few different paths now - replace `main()` with the following:

```rust
fn main() {
    // Read arguments, greet user
    let opt = Opt::from_args();
    println!("{}", GREETING);

    // Set up audio playback
    let device = default_output_device().unwrap();
    let sink = Sink::new(&device);

    // Define music source from Opt
    if opt.pitch_mode {
        let wave = SineWave::from(Pitch::from(opt.base_note));
        println!("Playing single tone {}", opt.base_note);
        // Play sine wave
        sink.append(wave);
    } else {
        // Init procedural generator
        let music = MusicMaker::new(opt.base_note, opt.scale, opt.octaves);
        println!("{}", music);
        // Play random melody
        sink.append(music);
    };
    // Sleep thread to allow music to play infinitely
    sink.sleep_until_end();
}
```

Make sure the code generation worked as expected with `cargo run -- -h` - you use `--` to pass command line arguments through `cargo run`, but you'd pass them directly to a binary: `./music -h`:

```txt
$cargo run -- -h
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/music -h`
music 0.1.0
music is a procedural single-tone melody generator

USAGE:
    music [FLAGS] [OPTIONS]

FLAGS:
    -h, --help          Prints help information
    -p, --pitch-mode    Single-pitch mode
    -V, --version       Prints version information

OPTIONS:
    -b, --base-note <base-note>    The base note to calculate the scale from [default: C4]
    -o, --octaves <octaves>        Number of octaves over which to range, anything over 8 gets parsed as 8 [default: 1]
    -s, --scale <scale>            The series of intervals from the base note to use per octave [default: Ionian]

```

Structopt is great for quick prototyping. We should add an output line to the header to let the user know what's playing:

```rust
impl fmt::Display for MusicMaker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let key = self.key;
        write!(
            f,
            "Generating music from the {} {}\nOctaves: {} - {}\n{}",
            key.base_note.note,
            key.scale,
            key.base_note.octave,
            key.base_note.octave + key.octaves,
            key
        )
    }
}
```

Now we should see the current key at the top - both options are optional, and the default value will be used if not found:

```txt
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target\debug\music.exe`
.: Cool Tunes :.
Generating music from the C major scale
Octaves: 4 - 5
[ C D E F G A B C ]
```

```txt
$ cargo run -- -s chromatic
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target\debug\music.exe -s chromatic`
.: Cool Tunes :.
Generating music from the C chromatic scale
Octaves: 4 - 5
[ C C# D D# E F F# G G# A A# B C ]
```

```txt
$ cargo run -- -s locrian -b Eb3 -o 3
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target\debug\music.exe -s locrian -b Eb2 -o 3`
.: Cool Tunes :.
Generating music from the E♭ Locrian mode
Octaves: 3 - 6
[ E♭ E F# G# A B C# E♭ ]
```

```txt
$ cargo run -- -p -b C3
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target\debug\music.exe -p -b C3`
Cool Tunes (tm)
.: Cool Tunes :.
Playing single tone C3
```

Check out C0 and A0, and be careful with headphones when getting to the upper octaves!

![human music](https://thepracticaldev.s3.amazonaws.com/i/92xyu0xcenfmpvrf6kbq.gif)

## Challenges

_[top](#table-of-contents)_

I wanted to keep this post to around an hour, but there are a number of ways this code could be extended:

- I can't even hear `C0` - can you? Restrict the 108-key keyboard to the standard 88-key from the diagram, that only includes the top three notes of Octave 0 and the top note of Octave 8 (12 x 7 + 3 + 1).
- Support even more types of key signatures like the [harmonic minor](https://en.wikipedia.org/wiki/Minor_scale#Harmonic_minor_scale), which is the Aeolian mode with the seventh note one semitone higher, or [pentatonic scales](https://en.wikipedia.org/wiki/Pentatonic_scale), which were hinted at above as using solely the black keys. Those have modes too...
- Generate those extended key signatures from strings like `"Cmaj"` or `"Amin7"`.
- Let the user decide how long each note should sound. Which part of `MusicMaker` do you think is responsible for that?
- Support [Helmholtz pitch notation](https://en.wikipedia.org/wiki/Helmholtz_pitch_notation).
- Instead of picking notes at random, support different kinds of seeds. For instance, every file on your computer is a stream of bytes. Maybe you could accept an `impl Iterator<Item = u8>`?
- Support other types of wave forms than sines, such as square waves or sawtooth waves.
- We can already read piano keys from strings like `"D#4"`. Parse and play back predefined sequences of notes from text files. This will involve some work: stacked accidentals, naturals, represent durations, etc.
- A [`WAV`](https://en.wikipedia.org/wiki/WAV) file is an uncompressed audio stream, like the one we've built. Write audio files containing your music with with [`hound`](https://github.com/ruuda/hound).
- Implement and play a `Chord`.
- Port this program to another language.

This has been a Public Service Announcement on the dangers of online encyclopedias. Thank you for your time.

_Cover image: [reddit](https://www.reddit.com/r/linuxmasterrace/comments/dyqri7/like_god_would_have_wanted/)_
