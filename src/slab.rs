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

    pub(crate) fn insert(&mut self, data: T) -> usize {
        self.slab.insert(data)
    }

    pub(crate) fn try_remove(&mut self, index: usize) -> Option<T> {
        self.slab.try_remove(index)
    }

    pub(crate) fn get(&self, index: usize) -> Option<&T> {
        self.slab.get(index)
    }

    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.slab.get_mut(index)
    }
}
