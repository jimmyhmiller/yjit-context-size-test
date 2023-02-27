mod compact_temp_mapping;
mod deduplicated;
mod hash_cons;
mod initial_context;
mod packed_context;
mod hash_cons_2;
use std::rc::Rc;

use crate::{deduplicated::{RcLite, RawPointer}, initial_context::ContextWithCount};
use deepsize::DeepSizeOf;
use itertools::Itertools;


trait ContextSize {
    type Context: DeepSizeOf + Clone;
    type Pointer: DeepSizeOf;
    type Storage: DeepSizeOf;
    fn get_pointer(&self) -> Self::Pointer;
    fn get_storage(&self) -> Option<Self::Storage>;
    fn store_context(&mut self, _context: Self::Context) {}

    fn get_pointer_size(&self, count: usize) -> usize {
        let pointer_size = self.get_pointer().deep_size_of();
        let total_size: usize = pointer_size * count;
        total_size
    }

    fn get_storage_size(&self) -> usize {
        let storage = self.get_storage();
        storage.map(|x| x.deep_size_of()).unwrap_or(0)
    }
}


fn total_size<T>(contexts: &[initial_context::ContextWithCount]) -> usize
where
    T: ContextSize + Default,
    <T as ContextSize>::Context: From<initial_context::Context>,
{
    let mut t = T::default();
    let mut total_size = 0;
    for context in contexts.iter() {
        let new_context: T::Context = Into::into(context.context.clone());
        for _ in 0..context.count {
            t.store_context(new_context.clone());
        }
        let size = t.get_pointer_size(context.count as usize);
        total_size += size;
    }
    total_size += t.get_storage_size();

    total_size
}


macro_rules! print_size {
    ($type:ty, $contexts:expr) => {
        println!("Total size {0: <80} {1: <10}", stringify!($type), total_size::<$type>(&$contexts))
    };
}

fn main() {
    let path = "/Users/jimmyhmiller/Downloads/railsbench_ctx_duplications.json";

    let contexts: Vec<ContextWithCount> = serde_jsonlines::json_lines(path)
        .unwrap()
        .map(|x| x.unwrap())
        .collect_vec();

    print_size!(initial_context::Context, &contexts);
    print_size!(compact_temp_mapping::Context, &contexts);
    print_size!(deduplicated::ContextStorage<Rc<initial_context::Context>>, &contexts);
    print_size!(hash_cons_2::ContextNode, &contexts);
    print_size!(packed_context::PackedContext, &contexts);
    print_size!(hash_cons::ContextHash, &contexts);
    print_size!(deduplicated::ContextStorage<Rc<compact_temp_mapping::Context>>, &contexts);
    print_size!(deduplicated::ContextStorage<Rc<packed_context::PackedContext>>, &contexts);
    print_size!(deduplicated::ContextStorage<RcLite<packed_context::PackedContext>>, &contexts);
    print_size!(deduplicated::ContextStorage<RcLite<compact_temp_mapping::Context>>, &contexts);
    print_size!(deduplicated::ContextStorage<RawPointer<compact_temp_mapping::Context>>, &contexts);
}
