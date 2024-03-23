#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Index {
    index: usize,
}

impl Index {
    fn new(index: usize) -> Self {
        Self { index }
    }
}

#[derive(Debug)]
pub(crate) struct Slab<T> {
    slab: slab::Slab<T>,
}

impl<T> Slab<T> {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            slab: slab::Slab::with_capacity(capacity),
        }
    }

    pub(crate) fn capacity(&self) -> usize {
        self.slab.capacity()
    }

    pub(crate) fn insert(&mut self, data: T) -> Index {
        Index::new(self.slab.insert(data))
    }

    pub(crate) fn try_remove(&mut self, index: Index) -> Option<T> {
        self.slab.try_remove(index.index)
    }

    pub(crate) fn get(&self, index: Index) -> Option<&T> {
        self.slab.get(index.index)
    }

    pub(crate) fn get_mut(&mut self, index: Index) -> Option<&mut T> {
        self.slab.get_mut(index.index)
    }
}
