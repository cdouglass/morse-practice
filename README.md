# Principles

Teaches decoding Morse code. Ideally it does this using a rough version of the Koch method and Farnsworth timing as described in [http://www.justlearnmorsecode.com/](http://www.justlearnmorsecode.com/) (which also provides Windows software with many more features). In practice there isn't an interface for either of these yet, but for Farnsworth timing you can edit the constants `CHAR_SPACE` and `WORD_SPACE` in `src/audio.rs` and rebuild, and for Koch method you can comment out unwanted characters in `src/encoding.rs`.

# Setup + requirements

Project is written in Rust and built using `cargo`. Installation link and documentation can be found at [https://www.rust-lang.org/en-US/install.html](https://www.rust-lang.org/en-US/install.html).

Uses the `beep` utility to make sounds, so probably needs Linux.

Beep volume is set separately from normal volume, and you will probably want to lower it before using this program. Run `alsamixer`, press F6, and select a sound card to access fine-grained volume settings. There should be a `Beep` setting shown.

On my machine at least, whether `beep` will produce a sound is not very consistent but it seems to depend on several things:

* Laptop should not be running on battery.
* If laptop is running on battery and failing to beep, running `speaker-test` in another terminal re-enables beep at least for the duration of the test. This could be superstition on my part though.

If the program is stopped with ctrl+C while it is playing a tone, the current tone may continue sounding. This is annoying, but running `beep` in a terminal when this happens should make it stop.

```
$ sudo apt-get install beep
$ sudo apt-get install rlwrap # Optional but recommended
$ cargo build
```

# Operation

Basic operation: play groups of random characters, with upper and lower bounds on word length and count.
```
$ ./target/debug/morse          # Default: 5 words, each 5 characters long
$ ./target/debug/morse 3 2      # 2 words, each 3 characters long
$ ./target/debug/morse 3 5 2 10 # 2 to 10 words, each 3 to 5 characters long
```

Use a dictionary file. Here, play exactly one word of exactly three letters. This is a good way to exercise auditory memory: don't start typing until the entire word has played.
```
./target/debug/morse -d 3 3 1 1
```
Note: The location of this file is currently hard-coded. To change it, you can edit the constant `DICT_FILENAME` in `src/main.rs` and rebuild.

Use a text file. Here, play this README ten words at a time, starting 30 words into the file.
```
./target/debug/morse -t "README.md" -o 30 10
```

## Misc other options:

* `-p` Set pitch in Hz. Default pitch is 440, ie A above middle C.
* `-l` Set dot length in ms. Default dot length is 80, corresponding to about 15 words per minute.
* `-c` Filter words to use only ones containing all listed characters. Can be useful for focused practice.
* `-m` Choose character set, if not reading through a text. Default is letters only, `all` is all characters, and `num` is digits only.

Since I did not use `readline`, I recommend preceding all commands with `rlwrap` to make it easy to correct your copy (eg fill in missing letters if you can make a good guess) before entering it.
```
$ rlwrap ./target/debug/morse
```

# TODO

* Flags for Farnsworth timing
* Some usable way to do Koch method
* Support other OSes. Pull requests welcome if anyone wants to make this happen.
* Misc cleanup, eg don't hardcode dictionary location
