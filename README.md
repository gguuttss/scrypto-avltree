# scrypto-avltree

# Why
Scrypto currently does not provide a scalable `BtreeMap` since the current implementation loads the full `BtreeMap` stored in component state into memory. Meaning the amount of items you can store in the Scrypto `BtreeMap` is fairly limited, because if the `Btreemap` grows past a certain threshold the component can not be loaded anymore to execute a transaction.  That also opens attack vectors because people can flood your component trying to lock it by letting the `BtreeMap` state grow too large.

# Features
To solve that issue we have implemented an `AVL tree` as a Scrypto library, a balanced binary search tree, based on the scalable `KeyValueStore`.
Other options were using a `red black tree`. However, compared to that the `AVL tree` is optimised for lookup/query performance instead of insert/write performance which made the `AVL tree` the more appropriate fit for being used in Ociswap.

To further optimise lookups we have combined our `AVL tree` implemention with a linked list - allowing us to traverse the next
left and right nodes directly in `O(1)` without needing to traverse the tree up and down which would be `O(log n)`.

# Usage
## Example
Checkout the example folder, that provides some basic usage examples.
### Dependencies
Add avl tree to your toml config:
```toml
[dependencies]
avl_tree = { git = "https://github.com/ociswap/scrypto-avltree", version = "0.1.0" }
```
### Instantiation 
Instantiation is rather simple:
```rust
use scrypto::prelude::*;
use avl_tree::AvlTree;
let mut tree: AVLTree<Decimal, String> = AVLTree::new();
```
### Insert and get
Inserting a new key value pair is also straight forward:
```rust
tree.insert(Decimal::from(1), "value");
```
If the key is already present in the tree, the value will be overwritten and the old value will be returned.
```rust
let old_value = tree.insert(Decimal::from(1), "new value");
assert_eq!(old_value, Some("value"));
```

### Get and get_mut
The tree can be queried by key:
```rust
let value = tree.get(&Decimal::from(1));
```
Or to get a mutable reference to the value:
```rust
let value = tree.get_mut(&Decimal::from(1));
```
### Range
To iterate over the tree you can use the `range`, `range_back` methods.
It accepts a range of keys and returns an iterator over the key value pairs:
The range is default in rust and can have inclusive or exclusive bounds.
```rust
for value in tree.range(Decimal::from(1)..Decimal::from(10)) {
    println!("value: {}", value);
}
```
gives you all values for the keys between 1 and 10 ascending and excluding 10.
```rust
for value in tree.range_back(Excluded(Decimal::from(1)),Included(Decimal::from(10))) {
    println!("value: {}", value);
}
```
gives you all values for the keys between 1 and 10 descending and excluding 1.

### Mutable Range
To iterate over the tree and mutate the values you can use the `range_mut`, `range_back_mut` methods.
It accepts a range of keys and returns an iterator that can be used with the for_each callback
```rust
tree.range_mut(Decimal::from(1)..Decimal::from(10)).for_each(|value| {
    *value=String::from("mutated");
}
for value in tree.range(Decimal::from(1)..Decimal::from(10)) {
    println!("value: {}", value);
}
```
gives 10 times "mutated" as output.
Analogue to the `range` method the `range_back_mut` method gives you a descending iterator.

### Delete
To delete a key value pair from the tree you can use the `delete` method:
```rust
let value = tree.delete(&Decimal::from(1));
print(value);
```
The method returns the value that was deleted from the tree. 
None is returned, if the key is not present in the tree.


# Contribute
The AVL tree itself is implemented in `avl_tree.rs`. The other modules and files contain helpers for testing.