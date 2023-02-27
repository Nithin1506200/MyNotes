# Hashmap

- [Hashmap](#hashmap)
  - [import](#import)
  - [methods](#methods)

## import

```rust
use std::collections::HashMap;
```

## methods

| Methods         | Params                                             | Description |
| --------------- | -------------------------------------------------- | ----------- |
| `new`           | ()                                                 |             |
| `insert`        | (&mut self, k: K, v:V)-> `Option<V>`               |             |
| `get`           | (&self , k : &Q)-> `Option<&V>`                    |             |
| `remove`        | (&mut self, k:&Q) -> `Option<V>`                   |
| `get_key_value` | (&self , k:&Q)-> `Option<(&K,&V)>`                 |             |
| `get_many_mut`  | ( &mut self, ks: [&Q; N]) ->` Option<[&mut V; N]>` |             |
