# Requirements

The following is a list of requirements that have been collected, and with
which crates for `nitric` are built. Feel free to request more by creating a
Pull Request! Be aware that this list does not explicitly mention features
Specs had. It mostly contains the ones it lacks, but I'm not opposed to
adding some explicitly.

Please note though that a requirement does not necessarily mean it will be
implemented by a `nitric` crate. It only means that it should _compatible_ with
them. Some requirements could very well serve as a good way to validate that
`nitric` is as composable as it should be.

> Warning: many of these reference [Specs] or problems of it. I try to make
  these understandable without knowledge of it, but might not always be
  possible. Still, the requirements will be mainly structured according to
  needs of an ECS library.

[Specs]: https://github.com/slide-rs/specs

## Systems

As mentioned in the readme, systems are no longer a concept that's controlled
by the library. A system (which is the ECS term for a function that operates
on data) can simply be created by writing a Rust function, therefore the
solution to the many original requirements will often just be that there's no
longer a restriction. Therefore, I will not list all of them here. Since
`nitric` will provide a graph that works similar to Specs' dispatcher (a
facility to execute systems in parallel, with dependencies on each other),
those requirements will still be listed. Thus, this section is basically
specific to `nitric-graph`.

### System dependencies

* allow to specify dependencies in a type-safe manner (not just using a `&str`)
* do not restrict to systems that were inserted already
    * detect cycles
    * allow fallback behavior (error handling)



## Components

Requirements for components, which is the term for a data-point in an ECS.

* Provide a solution that allows component dependencies
    * mutually exclusive components
* Provide a solution to track component modifications

## World



