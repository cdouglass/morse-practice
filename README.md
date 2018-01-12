# Principles

Teaches decoding Morse code, using a rough version of the Koch method and Farnsworth timing as described in [http://www.justlearnmorsecode.com/](http://www.justlearnmorsecode.com/) (which also provides Windows software with many more features).

# Setup

Uses the `beep` utility to make sounds, so probably needs Linux.

Beep volume is set separately from normal volume in Alsamixer. You will probably want to lower it before using this program.

On my machine at least, whether `beep` will produce a sound seems to depend on several things. Plugging in the laptop seems to help but this may be superstition.

```
$ sudo apt-get install beep
```

# Operation

```
$ cargo run
```

# TODO (maybe)

* CLI options for word generation
  * real words
  * chars all same number of elements
  * all dashes
  * all dots
  * maybe difficulty level too, eg average word and phrase length?
