# `nitric`(ABANDONED)

[![Build Status](https://img.shields.io/travis-ci/torkleyy/nitric.svg?style=flat-square)](https://travis-ci.org/torkleyy/nitric)
[![Code coverage](https://img.shields.io/codecov/c/gitlab/nitric/nitric/master.svg?style=flat-square)](https://codecov.io/gl/nitric/nitric/branch/master)
[![Crates.io](https://img.shields.io/crates/v/nitric.svg?style=flat-square)](https://crates.io/crates/nitric)
[![API Docs](https://img.shields.io/badge/API-on%20docs.rs-blue.svg?style=flat-square)](https://docs.rs/nitric)

General-purpose data processing library.

> Status notes: highly experimental, unfinished, work in progress, not recommended for use

This library is meant as a successor for [Specs], a parallel ECS library. However, `nitric` aims to be more general
and composable than Specs. In fact, Specs can be implemented as a frontend for `nitric` once this library is complete.

[Specs]: https://github.com/slide-rs/specs

* [nitric on GitLab](https://gitlab.com/nitric/nitric)
* [nitric on GitHub (mirror)](https://github.com/torkleyy/nitric)

## Motivation

Specs has many problems, both big and small. The library grew big without much planning and as it is now this will
continue and make it very hard to maintain. That's what made me think about what I want to create. This is what I
ended up with:

### Vision

The vision for `nitric` is to provide a set of crates that evolve as the standard way to solve data processing
problems. There were already very interesting use cases for Specs, including using it for a compiler and performing
larger simulations, both outside of Specs' original scope (ECS for game development). This is what I intend to make
`nitric` suitable for. So to list a few examples of what `nitric` could be used for in the future:

* game development
* game physics
* simulations
* compilers
* data validation
* Graphical User Interfaces

The question of how to structure your library/application is a very common one, everywhere in programming. The
plan for `nitric` is not to force any of them, but to provide useful and modular facilities that allow for specific
patterns (e.g. [Entity Component System] (1)), and to provide "recipes", similar to the [Rust cookbook] that show how
common tasks can be solved. Nice side effects of that are that we can work on one implementation, that is efficient
and can allow for neat extra functionality (debugging facilities, profiling, easy multi-threading, etc.).

(1) for ECS, also see [this great presentation by Catherine West][gpcw]

[gpcw]: https://kyren.github.io/rustconf_2018_slides/index.html
[Entity Component System]: https://en.wikipedia.org/wiki/Entity%E2%80%93component%E2%80%93system
[Rust cookbook]: https://rust-lang-nursery.github.io/rust-cookbook/

### Philosophy

`nitric` will be a collection of crates all about processing data. All of its crates follow this philosophy:

* Only solve a single problem, in a reasonably composable way
* Expose a general, composable and robust API (also refer to the [API design document](docs/API.md))
    * APIs should either be designed to not produce any error cases or return a `Result` with only the possible error
      conditions
    * Do not assume how the API is being used (-> composability)
    * Expose internals in a `*-internals` crate for stability by default, with the option to opt into more unstable
      facilities
* Impose minimal friction to use `nitric`
    * The goal is for `nitric` to be cheap and easy to use in one place of your project for solving a particular problem
    * `nitric` is meant to be compatible with other data structure / ECS / [CGS] libraries, e.g. [Specs], [froggy], etc.
      instead of competing with them

[froggy]: https://github.com/kvark/froggy
[CGS]: https://github.com/kvark/froggy/wiki/Component-Graph-System

### Using `nitric` as ECS

How will this allow you to use `nitric` instead of Specs? Here's the tentative design for ECS:

* `nitric-entity`: Provides entity allocators, storages, and with that mappings between entities and components

This would already be enough to have an ECS. Systems can simply be functions that accept references to the storages
and eventually the allocator. In fact, that is the recommended way for libraries to expose their API; libraries should
not assume how the code is executed. For example:

(pseudo code for now)

```rust
pub fn process_local_transforms(
    local: &Storage<LocalTransform>,
    global: &mut Storage<GlobalTransform>,
    parents: &Storage<Parent>)
{
    // compute global transforms    
}
```

If those component storages come from a dynamically typed, string mapped `HashMap`, fine. If they are stored in a 
`struct` - works, too. How systems are run also doesn't matter.

Now, there surely are other things Specs users would miss, so the next crate will be...

* `nitric-world`

As you might have guessed, this provides a map that can store arbitrary component storages. In contrast to Specs,
I also plan to include support for multiple values of the same type by allowing an additional key (e.g. a string).

* `nitric-graph`

This will be a re-worked version of `shred`'s `Dispatcher`, allowing to parallelize data processing (execution of
systems). A "system" will simply be a `FnMut(T) -> R`, which means it's up to the user how the data is fetched
(`nitric` will provide solutions for this, but it doesn't force you to use any of them).

## Structure

The main crate, `nitric` will simply serve as a top-level crate for re-exporting all `nitric-*` crates.
However, since everything is optional, `nitric` is controlled with Cargo features, only exporting the crates for which
you enable the flag.

Current crates:

* [`nitric-component`] - Component storages with custom id spaces
* [`nitric-lock`] - Locks with deadlock prevention & lock ordering

[`nitric-component`]: crates/nitric-component/
[`nitric-lock`]: crates/nitric-lock/

## FAQ

### What does this mean for Specs?

For the immediate future, this has no effect on Specs. It will not be deprecated. The biggest change for now is that
I won't spend much time on it (just merge PRs and fix critical bugs).

As for when nitric is in a usable state, that has yet to be seen. In any case, it should be possible to make Specs a
thin wrapper over `nitric` crates (if that's necessary). All that depends on how well `nitric` will be adopted.

### What does this mean for Amethyst?

[Amethyst] (a game engine that makes heavy use of Specs) will continue to use Specs. Whether it will use `nitric` in
the future will be decided by all members, through the usual RFC process. 

[Amethyst]: https://github.com/amethyst/amethyst

## Contribution

`nitric` can only exist with lively contributions and every help is very much appreciated!

Please note that in its current state, however, the project might not be very friendly for contributions. If you're
still interested in helping out, please contact me (@torkleyy) so we can make sure there's no duplicated effort.

## License

All `nitric` projects are, except stated otherwise, dual-licensed under Apache-2.0 / MIT. You're free to choose one of
both licenses.

Every contribution made to this project is assumed to be licensed according to these terms.

See [LICENSE](LICENSE), [docs/LICENSE-MIT](docs/LICENSE-MIT) and [docs/LICENSE-APACHE](docs/LICENSE-APACHE) for more
information.
