## Minigrep
This is a simple project written on Rust following the [Rust Programming Language Book](https://doc.rust-lang.org/book/) but with a little improvements.

---

### Improvement
#### Search Case-insensitive
Now you can search for PROGRAmming and find a line with programming, for example.

The others improvements is on the level of coding, but only syntax sugar.

### Params received
\$1 = Query pattern
\$2 = File path to search

### Usage
Clone the project:
```bash
$ git clone https://github.com/suicideDuck22/minigrep-rust-book.githttps://github.com/suicideDuck22/minigrep-rust-book.git
```

Run the tests:
```bash
$ cargo test
```

Make a search:
```
$ cargo run -- ell poem.txt
```