## Lint summary
{
  "fmt_ok": true,
  "fmt_diff_lines": 0,
  "clippy_errors": 0,
  "clippy_warnings": 1,
  "panic_lints": 153,
  "arithmetic_side_effects": 491
}
Showing 2 of 2 lints, at most 3 examples each.

## clippy::indexing_slicing (153 sites)

```text
warning: indexing may panic
   --> src/expat.rs:500:17
    |
500 |     let idx = ((pages[hi as usize] as usize) << 3) + ((lo as usize) >> 5);
    |                 ^^^^^^^^^^^^^^^^^^
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#indexing_slicing
    = note: requested on the command line with `-W clippy::indexing-slicing`
```

```text
warning: indexing may panic
   --> src/expat.rs:501:6
    |
501 |     (NAMING_BITMAP[idx] & (1u32 << (lo & 0x1F))) != 0
    |      ^^^^^^^^^^^^^^^^^^
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#indexing_slicing
```

```text
warning: indexing may panic
   --> src/expat.rs:506:14
    |
506 |     let b0 = byte[0] as usize;
    |              ^^^^^^^
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#indexing_slicing
```

## clippy::too_many_arguments (1 sites)

```text
warning: this function has too many arguments (8/7)
    --> src/expat.rs:3158:1
     |
3158 | / fn parse_pseudo_attribute(
3159 | |     enc: Encoding,
3160 | |     buf: &[u8],
3161 | |     mut ptr: usize,
...    |
3166 | |     next_tok_ptr: &mut usize,
3167 | | ) -> bool {
     | |_________^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
     = note: `#[warn(clippy::too_many_arguments)]` on by default
```