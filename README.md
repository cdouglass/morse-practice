# Principles

Teaches decoding Morse code, using a rough version of the Koch method and Farnsworth timing as described in [http://www.justlearnmorsecode.com/](http://www.justlearnmorsecode.com/) (which also provides Windows software with many more features).

# Setup

Uses the `beep` utility to make sounds, so probably needs Linux.

Beep volume is set separately from normal volume in Alsamixer. You will probably want to lower it before using this program.

On my machine at least, whether `beep` will produce a sound seems to depend on several things:
* Laptop should not be running on battery
* If laptop is running on battery and failing to beep, running `speaker-test` in another terminal re-enables beep at least for the duration of the test

```
$ sudo apt-get install beep
```

# Operation

To use the entire alphabet:

```
$ cargo run
```

To use only characters whose encoding uses a certain number of tones, give the number of tones as a CLI argument. For example, to use only the letters {a, i, m, n}:

```
$ cargo run 2
```

# TODO (maybe)

* CLI options for word generation
  * real words
  * all dashes
  * all dots
  * maybe difficulty level too, eg average word and phrase length?
