# vec

- [vec](#vec)
  - [https://doc.rust-lang.org/std/vec/struct.Vec.html#](#httpsdocrust-langorgstdvecstructvechtml)
  - [methods](#methods)

## https://doc.rust-lang.org/std/vec/struct.Vec.html#

## methods

| method     | params                                   | description |
| ---------- | ---------------------------------------- | ----------- |
| `new`      | creates                                  |             |
| `truncate` | (&mut self, len: usize)                  |             |
| `insert`   | (&mut self, index: usize, element: T)    |             |
| `remove`   | (&mut self, index: usize) -> T           |             |
| `retain`   | (&mut self, f: F)                        |             |
| `push`     | (&mut self, value: T)                    |             |
| `append`   | (&mut self, other: &mut Vec<T, A>)       |             |
| `pop`      | (&mut self) -> Option<T>                 |             |
| `len`      | (&self) -> usize                         |             |
| `is_empty` | (&self) -> bool                          |             |
| `dedup`    | (&mut self)                              |             |
| `concat`   | (&self) -> <[T] as Concat<Item>>::Output |             |
| `join`     | (&self, sep: Separator) ->               |             |
| `contains` | (&self, x: &T) -> bool                   |             |
| `fill`     | (&mut self, value: T)                    |             |
| `reverse`  | (&mut self)                              |             |
| `iter`     | (&self) -> Iter<'\_, T>                  |             |
| `iter_mut` | (&mut self) -> IterMut<'\_, T>           |             |
| `sort`     | (&mut self)                              |             |
| `sort_by`  | (&mut self, compare: F)                  |             |
| `to_vec`   | (&self)                                  |             |
| `repeat`   | (&self, n: usize)                        |             |
