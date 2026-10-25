## Lint summary
{
  "fmt_ok": true,
  "fmt_diff_lines": 0,
  "clippy_errors": 0,
  "clippy_warnings": 2,
  "panic_lints": 0,
  "arithmetic_side_effects": 40
}
Showing 1 of 1 lints, at most 3 examples each.

## clippy::same_item_push (2 sites)

```text
warning: it looks like the same item is being pushed into this `Vec`
   --> src/cjson.rs:573:13
    |
573 |             out.push(b'0');
    |             ^^^
    |
    = help: consider using `vec![b'0';SIZE]`
    = help: or `out.extend(std::iter::repeat_n(b'0', SIZE))`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#same_item_push
    = note: `#[warn(clippy::same_item_push)]` on by default
```

```text
warning: it looks like the same item is being pushed into this `Vec`
   --> src/cjson.rs:582:17
    |
582 |                 out.push(b'0');
    |                 ^^^
    |
    = help: consider using `vec![b'0';SIZE]`
    = help: or `out.extend(std::iter::repeat_n(b'0', SIZE))`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#same_item_push
```