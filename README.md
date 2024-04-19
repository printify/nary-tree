[![](https://tokei.rs/b1/github/osimarr/nary-tree)](https://github.com/osimarr/nary-tree)

# nary_tree

A vec-backed tree structure with tree-specific generational indexes.

This is a fork from the slab_tree crate which is no longer maintained.
At this initial stage, the main difference (aside bug fix) is that the slab layer is now using the
slab crate from the tokio-rs project.

There is currently a new version being developed which will push changes to the crate's interface.
This will be released under version v0.5, while version v0.4.x will be maintained as LTS for compatibility.
The new version is currently on branch `new_interface`.

## Overview

This library provides a `Tree<T>` struct which allows the creation and manipulation of in-memory trees.
The tree itself is backed by a vector and the tree's node relationships are managed by tree-specific
generational indexes called `NodeId`s (more below). In addition, "views" of tree nodes are handed out
which are either immutable (`NodeRef`) or mutable (`NodeMut`) instead of handing out references
directly. Most tree mutations are achieved by modifying `NodeMut`s instead of talking to the tree
itself.

The `Tree`s in this crate are "just" trees. They do not allow cycles, and they do not allow arbitrary
graph structures to be created. Each node in the tree can have an arbitrary number of children, and
there is no weight associated with edges between the nodes in the tree.

**Please Note:** This crate does not support comparison-based data insertion. In other words, this is
not a binary search tree (or any other kind of search tree) crate. It is purely a crate for hierarchically storing data.
The caller must know the structure that they wish to build and then use
this crate to do so; this library will not make those structural decisions for you.

## Safety
This crate uses `#![forbid(unsafe_code)]` to prevent any `unsafe` code usage.

## Example Usage
```rust
use nary_tree::*;

fn main() {

    //      "hello"
    //        / \
    // "world"   "trees"
    //              |
    //            "are"
    //              |
    //            "cool"

    let mut tree = TreeBuilder::new().with_root("hello").build();
    let root_id = tree.root_id().expect("root doesn't exist?");
    let mut hello = tree.get_mut(root_id).unwrap();

    hello.append2("world");
    hello
        .append("trees")
        .append("are")
        .append("cool");
}
```

## `NodeId`s
`NodeId`s are tree-specific generational indexes. Using generational indexes means that we can re-use
space inside the tree (after nodes have been removed) without also having to re-use the same tree
indexes which could potentially cause confusion and bugs. The "tree-specific" part means that indexes
from one tree cannot be confused with indexes for another tree. This is because each index contains a
process-unique-id which is shared by the tree from which that index originated.

## Project Goals
* Allow caller control of as many allocations as possible (through pre-allocation)
* Fast and Ergonomic Node insertion and removal
* Arbitrary Tree structure creation and manipulation

## Non-Goals
* Arbitrary _Graph_ structure creation and manipulation
* Comparison-based node insertion of any kind
