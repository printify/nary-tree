extern crate nary_tree;

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
    let hello = tree.get_mut(root_id).unwrap();

    hello.append("world");
    let hello = tree.get_mut(root_id).unwrap();
    hello.append("trees").append("are").append("cool");
}
