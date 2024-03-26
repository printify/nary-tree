extern crate slab_tree;

use slab_tree::*;

fn main() {
    let mut tree = TreeBuilder::new().with_root(0).build();
    let mut root = tree.root_mut().unwrap();
    {
        let mut one = root.append(1);
        let mut two = one.append(2);
        two.append(3);
        two.append(4);
    }
    {
        let mut five = root.append(5);
        five.append(6).append(7);
        five.append(8);
    }
    root.append(9);

    // Find the first node that contains 5 via traverse level order
    let found = root
        .as_ref()
        .traverse_level_order()
        .find(|n| n.data() == &5);
    assert!(found.is_some());
    assert_eq!(found.unwrap().data(), &5);

    // 0
    // ├── 1
    // │   └── 2
    // │       ├── 3
    // │       └── 4
    // ├── 5
    // │   ├── 6
    // │   │   └── 7
    // │   └── 8
    // └── 9

    print!("{}", tree);
}
