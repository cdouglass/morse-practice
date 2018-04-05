# Principles

Teaches decoding Morse code. Ideally it does this using a rough version of the Koch method and Farnsworth timing as described in [http://www.justlearnmorsecode.com/](http://www.justlearnmorsecode.com/) (which also provides Windows software with many more features). In practice there isn't an interface for either of these yet, but for Farnsworth timing you can edit the constants `CHAR_SPACE` and `WORD_SPACE` in `src/audio.rs` and rebuild, and for Koch method you can comment out unwanted characters in `src/encoding.rs`.

# Setup

Uses the `beep` utility to make sounds, so probably needs Linux.

Beep volume is set separately from normal volume, and you will probably want to lower it before using this program. Run `alsamixer`, press F6, and select a sound card to access fine-grained volume settings. There should be a `Beep` setting shown. Speaker setting should not be muted, but 0 volume is OK.

On my machine at least, whether `beep` will produce a sound is not very consistent but it seems to depend on several things:
* Laptop should not be running on battery.
* If laptop is running on battery and failing to beep, running `speaker-test` in another terminal re-enables beep at least for the duration of the test. This could be superstition on my part though.

If the program is stopped with ctrl+C while it is playing a tone, the current tone may continue sounding. This is annoying, but running `beep` in a terminal when this happens should make it stop.

```
$ sudo apt-get install beep
$ cargo build
```

# Operation

I recommend running with `rlwrap`, which makes it easy to correct your copy (eg fill in missing letters if you can make a good guess) before entering it.

Basic operation: play groups of random characters, with upper and lower bounds on "word" length and count. Here, play groups of 2 to 10 "words" of 3 to 5 random characters.
```
$ rlwrap ./target/debug/morse 3 5 2 10
```

Use a dictionary file. Here, play exactly one word of exactly three letters. This is a good way to exercise auditory memory: don't start typing until the entire word has played.
```
rlwrap ./target/debug/morse -d 3 3 1 1
```
Note: The location of this file is currently hard-coded. To change it, you can edit the constant `DICT_FILENAME` in `src/main.rs` and rebuild.


Use a text file. Here, play this README five words at a time.
```
rlwrap ./target/debug/morse -t "README.md" 5 5
```

Optional flags not shown in examples: `-o` to start at an offset into a text, `-p` to set pitch, and `-l` to set dot length.

# TODO

* Support other OSes. Pull requests welcome if anyone wants to make this happen.
* Default argument values so you don't have to type so much
