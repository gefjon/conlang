# conlang

A _sentence_ is a _value_ followed by a period.

A _value_ is either:
- a _complement_
- a _sequence_
- a _number_
- a _word_

a _word_ is a series of characters other than `"'.:,` or whitespace

a _number_ is a decimal with optional leading sign, fractional part, and
exponent

a _sequence_ is any number of values each preceded by either `,` or `;` (the
delimiter must be consistent and may be surrounded on either side by
whitespace, as seems appropriate)

a _complement_ consists of:
- a _prefixed_ value, called the _head_
- any number of values with the same _prefix_, called the "tail" (tails of more
  than one value are a shorthand for a sequence as the tail)

a _prefixed_ value consists of:
- a "word", called the _prefix_
- a `:` (the "declension operator", which may not be surrounded by whitespace)
- a value
