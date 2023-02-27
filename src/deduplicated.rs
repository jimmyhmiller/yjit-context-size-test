use std::{collections::HashSet, rc::Rc, mem::size_of};

use deepsize::DeepSizeOf;

use crate::{ContextSize};


pub struct ContextStorage<Context>  {
    hash_set: HashSet<Context>,
}

impl<Context> Default for ContextStorage<Rc<Context>> where Context : DeepSizeOf + Eq + std::hash::Hash + Clone + Default {
    fn default() -> Self {
        Self::new()
    }
}

impl<Context> ContextStorage<Context> where Context : DeepSizeOf + Eq + std::hash::Hash + Clone {
    fn new() -> Self {
        Self {
            hash_set: HashSet::new(),
        }
    }

    fn insert<T : Into<Context>>(&mut self, context: T) {
        self.hash_set.insert(context.into());
    }

}


impl<Context> ContextSize for ContextStorage<Rc<Context>> where Context : DeepSizeOf + Eq + std::hash::Hash + Clone + Default {
    type Context = Context;
    type Pointer = Rc<Context>;
    type Storage = HashSet<Rc<Context>>;

    fn get_pointer(&self) -> Rc<Context> {
        self.hash_set.iter().next().unwrap().clone()
    }

    fn get_storage(&self) -> Option<HashSet<Rc<Context>>> {
        Some(self.hash_set.clone())
    }

    fn get_pointer_size(&self, count: usize) -> usize {
        // Don't do deepsize here because we are just referencing
        let pointer_size = size_of::<Self::Pointer>();
        pointer_size * count
    }

    fn store_context(&mut self, context: Self::Context) {
        self.insert(context);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct RcLite<T>(rclite::Rc<T>);

impl<T> DeepSizeOf for RcLite<T> where T : DeepSizeOf {
    fn deep_size_of_children(&self, context: &mut deepsize::Context) -> usize {
        self.0.deep_size_of_children(context)
    }
}

impl<T> From<T> for RcLite<T> {
    fn from(t: T) -> Self {
        Self(rclite::Rc::new(t))
    }
}

impl<Context> Default for ContextStorage<RcLite<Context>> where Context : DeepSizeOf + Eq + std::hash::Hash + Clone + Default {
    fn default() -> Self {
        Self::new()
    }
}



impl<Context> ContextSize for ContextStorage<RcLite<Context>> where Context : DeepSizeOf + Eq + std::hash::Hash + Clone + Default {
    type Context = Context;
    type Pointer = RcLite<Context>;
    type Storage = HashSet<RcLite<Context>>;

    fn get_pointer(&self) -> RcLite<Context> {
        self.hash_set.iter().next().unwrap().clone()
    }

    fn get_storage(&self) -> Option<HashSet<RcLite<Context>>> {
        Some(self.hash_set.clone())
    }

    fn get_pointer_size(&self, count: usize) -> usize {
        // Don't do deepsize here because we are just referencing
        let pointer_size = size_of::<Self::Pointer>();
        pointer_size * count
    }

    fn store_context(&mut self, context: Self::Context) {
        self.insert(context);
    }
}


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RawPointer<T>(*const T);

impl<T> DeepSizeOf for RawPointer<T> where T : DeepSizeOf {
    // We aren't going to take the deepsize of the pointer, so it is fine to have a dummy implementation
    fn deep_size_of_children(&self, _context: &mut deepsize::Context) -> usize {
        0
    }
}


impl<Context> Default for ContextStorage<RawPointer<Context>> where Context : DeepSizeOf + Eq + std::hash::Hash + Clone + Default {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<T> for RawPointer<T> {
    fn from(t: T) -> Self {
        Self(&t as *const T)
    }
}


impl<Context> ContextSize for ContextStorage<RawPointer<Context>> where Context : DeepSizeOf + Eq + std::hash::Hash + Clone + Default {
    type Context = Context;
    type Pointer = RawPointer<Context>;
    type Storage = HashSet<RawPointer<Context>>;

    fn get_pointer(&self) -> RawPointer<Context> {
        self.hash_set.iter().next().unwrap().clone()
    }

    fn get_storage(&self) -> Option<HashSet<RawPointer<Context>>> {
        Some(self.hash_set.clone())
    }

    fn get_pointer_size(&self, count: usize) -> usize {
        // Don't do deepsize here because we are just referencing
        let pointer_size = size_of::<Self::Pointer>();
        pointer_size * count
    }

    fn store_context(&mut self, context: Self::Context) {
        self.insert(context);
    }
}
