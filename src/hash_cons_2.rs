use std::{rc::Rc, cell::RefCell};

use deepsize::DeepSizeOf;
use once_cell::sync::Lazy;

use crate::{packed_context::{ContextDelta, pack_context}, initial_context::Context, ContextSize};



static mut CONTEXT_ROOT : Lazy<Rc<ContextNode>> =  Lazy::new(|| Rc::new(ContextNode::default()));

#[derive(PartialEq, Debug, DeepSizeOf, Clone)]
pub struct ContextNode {
    delta: ContextDelta,
    parent: Option<Rc<ContextNode>>,
    children: RefCell<Vec<Rc<ContextNode>>>,
}

impl Default for ContextNode {
    fn default() -> Self {
        ContextNode {
            delta: ContextDelta::None,
            parent: None,
            children: RefCell::new(vec![]),
        }
    }
}

impl ContextNode {


    pub fn _get_node_count(ctx: &Rc<ContextNode>) -> usize {
        let mut count = 1;
        for child in ctx.children.borrow().iter() {
            count += ContextNode::_get_node_count(child);
        }
        count
    }

    pub fn _dump_nodes(ctx: &Rc<ContextNode>, indent: usize) {
        for _ in 0..indent {
            print!("  ");
        }
        println!("{:?}", ctx.delta);
        for child in ctx.children.borrow().iter() {
            ContextNode::_dump_nodes(child, indent + 1);
        }
    }



    pub fn compress(ctx: &Context) -> Rc<ContextNode> {
        let mut parent = unsafe { CONTEXT_ROOT.clone() };
        let mut node = parent.clone();

        let deltas = pack_context(ctx);
        for delta in deltas.iter() {
            let mut found = node.clone();
            for child in parent.children.borrow().iter() {
                if child.delta == *delta {
                    found = child.clone();
                    break;
                }
            }
            if parent.delta == found.delta {
            //if parent == found {
                let new_node = Rc::new(ContextNode {
                    delta: *delta,
                    parent: Some(parent.clone()),
                    children: RefCell::new(vec![]),
                });
                parent.children.borrow_mut().push(new_node.clone());
                found = new_node;
            }
            node = found.clone();
            parent = found.clone();
        }

        node
    }
}


impl ContextSize for ContextNode {
    type Context = Context;

    type Pointer = Rc<ContextNode>;

    type Storage = Rc<ContextNode>;

    fn get_pointer(&self) -> Self::Pointer {
        Rc::new(self.clone())
    }

    fn get_storage(&self) -> Option<Self::Storage> {
        Some(unsafe { CONTEXT_ROOT.clone() })
    }

    fn store_context(&mut self, context: Self::Context) {
        Self::compress(&context);
    }

    fn get_pointer_size(&self, count: usize) -> usize {
        // Don't do deepsize here because we are just referencing
        let pointer_size = std::mem::size_of::<Self::Pointer>();
        pointer_size * count
    }


}
