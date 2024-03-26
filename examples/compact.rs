use slab_tree::{RemoveBehavior, TreeBuilder};

fn main() {
    #[cfg(feature = "experimental")]
    {
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

        println!("{}", tree);

        // 0
        // ├── 1
        // │   └── 2
        // │       ├── 3
        // │       └── 4
        // ├── 5
        // │   ├── 6
        // │   │   └── 7
        // │   └── 8

        let three_id = tree.find(&3).unwrap()[0];
        let five_id = tree.find(&5).unwrap()[0];

        tree.remove(three_id, RemoveBehavior::DropChildren);
        tree.remove(five_id, RemoveBehavior::DropChildren);

        println!("{}", tree);

        // 0
        // └── 1
        //     └── 2
        //         └── 4

        let two = tree.get(tree.find(&2).unwrap()[0]).unwrap();
        assert_eq!(two.first_child().unwrap().data(), &4);
        let four = tree.get(tree.find(&4).unwrap()[0]).unwrap();
        assert!(four.prev_sibling().is_none());
        assert_eq!(tree.root().unwrap().last_child().unwrap().data(), &1);

        println!("capacity before compact: {}", tree.capacity());
        assert!(tree.capacity() >= 9);

        tree.compact();

        println!("{}", tree);

        let two = tree.get(tree.find(&2).unwrap()[0]).unwrap();
        assert_eq!(two.first_child().unwrap().data(), &4);
        let four = tree.get(tree.find(&4).unwrap()[0]).unwrap();
        assert!(four.prev_sibling().is_none());
        assert_eq!(tree.root().unwrap().last_child().unwrap().data(), &1);

        println!("capacity after compact: {}", tree.capacity());
        assert!(tree.capacity() == 4);
    }
}
