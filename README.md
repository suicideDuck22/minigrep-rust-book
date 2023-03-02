## Minigrep
This is a simple project written on Rust following the [Rust Programming Language Book](https://doc.rust-lang.org/book/) but with a little improvements.

---

### Improvement
#### Search Case-insensitive usgin Arg Flag or Environment Variable

On the Rust book, is provided to us a way to control the search by case sensitive or insensitive using a Environment Variable. Example:
```bash
$ IGNORE_CASE=1 cargo run -- to poem.txt
```

Or on powershell:
```POWERshell
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

But with this changes, the option persist for all section, so we provided a way to pass a flag to this.
It can be used, calling:
```bash
$ cargo run -- --ignore-case to poem.txt
```

We parsed the args, presuming that the args are divided by, [cargo call, flags, params].
So, the program assume that the last two arguments are the Query to search and the File path repectively.
This make possible, insert more flags and treat them correctly.

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