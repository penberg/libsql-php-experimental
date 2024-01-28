# libSQL API for PHP

libSQL is an open source, open contribution fork of SQLite. This source repository contains libSQL API bindings for PHP, which aims to be compatible with the [SQLite3 database extension](https://www.php.net/manual/en/book.sqlite3.php).

**Please note that this is work-in-progress. If you are looking for a libSQL PHP SDK to us in production, this is not it.**

## Getting Started

```
cargo build
```

```
php -d extension=./target/debug/liblibsql_php.dylib test.php
```

## License

This project is licensed under the [MIT license].

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in libSQL by you, shall be licensed as MIT, without any additional
terms or conditions.

[MIT license]: https://github.com/penberg/libsql-php/blob/main/LICENSE
