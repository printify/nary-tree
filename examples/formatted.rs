extern crate nary_tree;

use nary_tree::*;

fn main() {
    let mut tree = TreeBuilder::new().with_root(0).build();
    let root = tree.root_mut().unwrap();
    let root = {
        let one = root.append(1);
        let mut two = one.append(2);
        two.append2(3);
        two.append2(4);
        two.parent().unwrap()
    };
    let mut root = {
        let five = root.append(5);
        let mut five = five.append(6).append(7).parent().unwrap().parent().unwrap();
        five.append2(8);
        five.parent().unwrap()
    };
    root.append2(9);

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
