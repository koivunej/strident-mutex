# ~~noisy-~~ ~~screaming-~~ strident-mutex

A wrapper around `std::sync::{Mutex, RwLock}` that panicks if used within tokio
async context.
Not useful as a general crate, but it might be useful if you are hunting down
or trying to get an understanding of blocking mutex usage in async context.

## Features

- `hide_async_test_panic`, enabled by default

`hide_async_test_panic` uses an crate internal mutex to remove the current
global panic hook, and restores it after the test.

## Naming

`strident` sorts after `std`, so you might end up with nice diffs:

```patch
-use std::sync::Mutex;
+use strident_mutex::sync::Mutex;
```
