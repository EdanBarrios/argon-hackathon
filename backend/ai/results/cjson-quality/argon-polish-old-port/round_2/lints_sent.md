## Lint summary
{
  "fmt_ok": true,
  "fmt_diff_lines": 0,
  "clippy_errors": 0,
  "clippy_warnings": 3,
  "panic_lints": 41,
  "arithmetic_side_effects": 62
}
Showing 3 of 3 lints, at most 3 examples each.

## clippy::indexing_slicing (41 sites)

```text
warning: indexing may panic
  --> src/cjson.rs:57:9
   |
57 |         self.content[self.offset + index]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: consider using `.get(n)` or `.get_mut(n)` instead
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#indexing_slicing
   = note: requested on the command line with `-W clippy::indexing-slicing`
```

```text
warning: slicing may panic
  --> src/cjson.rs:62:10
   |
62 |         &self.content[self.offset..self.length]
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: consider using `.get(n..m)` or `.get_mut(n..m)` instead
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#indexing_slicing
```

```text
warning: indexing may panic
  --> src/cjson.rs:96:18
   |
96 |     if i < n && (s[i] == b'+' || s[i] == b'-') {
   |                  ^^^^
   |
   = help: consider using `.get(n)` or `.get_mut(n)` instead
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#indexing_slicing
```

## clippy::same_item_push (2 sites)

```text
warning: it looks like the same item is being pushed into this `Vec`
   --> src/cjson.rs:555:13
    |
555 |             out.push(b'0');
    |             ^^^
    |
    = help: consider using `vec![b'0';SIZE]`
    = help: or `out.extend(std::iter::repeat_n(b'0', SIZE))`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#same_item_push
    = note: `#[warn(clippy::same_item_push)]` on by default
```

```text
warning: it looks like the same item is being pushed into this `Vec`
   --> src/cjson.rs:563:17
    |
563 |                 out.push(b'0');
    |                 ^^^
    |
    = help: consider using `vec![b'0';SIZE]`
    = help: or `out.extend(std::iter::repeat_n(b'0', SIZE))`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#same_item_push
```

## clippy::needless_range_loop (1 sites)

```text
warning: the loop variable `i` is used to index `input`
   --> src/cjson.rs:167:14
    |
167 |     for i in 0..4 {
    |              ^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
    = note: `#[warn(clippy::needless_range_loop)]` on by default
help: consider using an iterator and enumerate()
    |
167 -     for i in 0..4 {
167 +     for (i, <item>) in input.iter().enumerate().take(4) {
    |
```