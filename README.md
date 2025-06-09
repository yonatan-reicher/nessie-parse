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
