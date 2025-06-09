# Nessie Parse

<p align="center">
    <img src="./icon.png" alt="Nessie Parse Logo" width="200"/>
    <br/>
    <em>What if parsing was easy?</em>
</p>

This library let's you define a parser for your language by combining smaller
parsers together in a simple and familiar syntax to `Iterator` and `Option`
types. This way of combining parsers together is called parser combinators.

## Non Goals
What does this library _not_ try to achieve?
- Good error reporting
- Warnings
- Good Performance
- Non-UTF-8 text parsing

## Inspired By
The reason I believe in parser combinators is because of Elm. Elm's compiler has
great error messages and one of the best white-space-sensitive syntax I've seen
implemented. That compiler is implemented in Haskell and uses parsers-as-monads.
You can't have do notation for monads in Rust, but you can get close to it.
- (The Elm Parser)[https://github.com/elm/compiler/tree/master/compiler/src/Parse]
