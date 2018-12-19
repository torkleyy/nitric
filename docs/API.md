# API Design

(Sorry, this file still has to be written properly)

## Error handling

The best error handling is the one that isn't required. Ideally, an API cannot be used in a way
that can error.

TODO: describe how to expose errors, etc

## Design

* Impose minimal friction to use `nitric`
* `nitric` is meant to be compatible with other data structure / ECS / CGS libraries, e.g.
  [Specs], [froggy], etc. instead of competing with them

## Structure

* Only solve a single problem, in a reasonably composable way
* Expose internals in an `*-internals` crate for stability by default, with the option to opt
into more unstable facilities
