# ts

## Summary

*A small command pipeline tool to prepend timestamps.*

`ts` is a drop-in replacement for `ts` from [moreutils](http://joeyh.name/code/moreutils/), written in [Rust](https://www.rust-lang.org/).

After moving to a system without Perl, I needed a replacement for the original `ts`.

To change the format, either specify a new format on the command line or set the TS_FORMAT environment variables

## Examples

```
$ (echo hi ;sleep 1; echo mom) | ts
2020-05-07 02:43:57 hi
2020-05-07 02:43:58 mom
$
```

```
$ (echo hi ;sleep 1; echo mom) | ts '[%s]'
[1588819477] hi
[1588819478] mom
$
```

```
$ (echo hi ;sleep 1; echo mom) | TS_FORMAT='%F %T :' ./target/release/ts
2020-05-07 02:45:52 : hi
2020-05-07 02:45:53 : mom
$
```