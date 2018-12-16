# `nitric`

General-purpose data processing library.

> Status notes: highly experimental, unfinished, work in progress, not recommended for use

This library is meant as a successor for [Specs], a parallel ECS library. However, `nitric` aims to be more general
and composable than Specs. In fact, Specs can be implemented as a frontend for `nitric` once this library is complete.

[Specs]: https://github.com/slide-rs/specs

## Motivation

Specs has many problems, both big and small. The library grew big without much planning and as it is now this will
continue and make it very hard to maintain.

### Philosophy

`nitric` will be a collection of crates all about processing data. All of its crates follow this philosophy:

* Only solve a single problem, in a reasonably composable way
* Expose a general, composable and robust API (also refer to the [API design document](docs/API.md))
    * APIs should either be designed to not produce any error cases or return a `Result` with only the possible error
      conditions
    * Do not assume how the API is being used (-> composability)
    * Expose internals in an `*-internals` crate for stability by default, with the option to opt into more unstable
      facilities
* Impose minimal friction to use `nitric`
    * The goal is for `nitric` to be cheap and easy to use in one place of your project for solving a particular
    * `nitric` is meant to be compatible with other data structure / ECS / CGS libraries, e.g. [Specs], [froggy], etc. 
      instead of competing with them

[froggy]: https://github.com/kvark/froggy

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
    parents: Storage<Parent>)
{
    // compute global transforms    
}
```

If those component storages come from a dynamically typed, string mapped `HashMap`, fine. If they are stored in a 
`struct` - works, too. How systems are ran also doesn't matter.

Now, there surely are other things Specs users would miss, so the next crate will be...

* `nitric-world`

As you might have guessed, this provides a map that can store arbitrary component storages. In contrast to Specs,
I also plan to include support for multiple values of the same type by allowing an additional key (e.g. a string).

* `nitric-graph`

This will be a re-worked version of `shred`'s `Dispatcher`, allowing to parallelize data processing (execution of
systems). A "system" will simply be a `FnMut(T) -> R`, which means it's up to the user how the data is fetched
(`nitric` will provide solutions for this, but it doesn't force you to use any of them).

## Vision

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

## Structure

The main crate, `nitric` will simply serve as a top-level crate for re-exporting all `nitric-*` crates.
However, since everything is optional, `nitric` is controlled with Cargo features, only exporting the crates for which
you enable the flag.

Current crates:

* [`nitric-lock`] - Locks with deadlock prevention & lock ordering

[`nitric-lock`]: crates/nitric-lock/

## Contribution

`nitric` can only exist with lively contributions and every help is very much appreciated!

Please note that in its current state however, the project might not be very friendly for contributions. If you're
still interested in helping out, please contact me (@torkleyy) so we can make sure there's no duplicated effort.

## License

All `nitric` projects are, except states otherwise, dual-licensed under Apache-2.0 / MIT. You're free to choose on of
both licenses.

Every contribution made to this project is assumed to be licensed according to these terms.
