use deepsize::DeepSizeOf;
use serde::{Serialize, Deserialize};

use crate::{initial_context::{self, Type, MAX_TEMP_TYPES, MAX_LOCAL_TYPES}, ContextSize};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize, DeepSizeOf)]
pub enum LocalIndex {
    Local0,
    Local1,
    Local2,
    Local3,
    Local4,
    Local5,
    Local6,
    Local7,
}


// Potential mapping of a value on the temporary stack to
// self, a local variable or constant so that we can track its type
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, DeepSizeOf)]
#[allow(clippy::enum_variant_names)]
pub enum TempMapping {
    MapToStack,     // Normal stack value
    MapToSelf,      // Temp maps to the self operand
    MapToLocal(LocalIndex), // Temp maps to a local variable with index
}

impl Default for TempMapping {
    fn default() -> Self {
        TempMapping::MapToStack
    }
}


/// Code generation context
/// Contains information we can use to specialize/optimize code
/// There are a lot of context objects so we try to keep the size small.
#[derive(Clone, Default, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, DeepSizeOf)]
pub struct Context {
    // Number of values currently on the temporary stack
    pub stack_size: u16,

    // Offset of the JIT SP relative to the interpreter SP
    // This represents how far the JIT's SP is from the "real" SP
    pub sp_offset: i16,

    // Depth of this block in the sidechain (eg: inline-cache chain)
    pub chain_depth: u8,

    // Local variable types we keep track of
    pub local_types: [Type; MAX_LOCAL_TYPES],

    // Temporary variable types we keep track of
    pub temp_types: [Type; MAX_TEMP_TYPES],

    // Type we track for self
    pub self_type: Type,

    // Mapping of temp stack entries to types we track
    pub temp_mapping: [TempMapping; MAX_TEMP_TYPES],
}



impl From<initial_context::Context> for Context {
    fn from(value: initial_context::Context) -> Self {
        let mut context = Context::default();

        // Copy local types
        for (i, local_type) in value.local_types.iter().enumerate() {
            context.local_types[i] = *local_type;
        }

        // Copy temp types
        for (i, temp_type) in value.temp_types.iter().enumerate() {
            context.temp_types[i] = *temp_type;
        }

        // Copy self type
        context.self_type = value.self_type;

        // Copy temp mapping
        for (i, temp_mapping) in value.temp_mapping.iter().enumerate() {
            match temp_mapping {
                initial_context::TempMapping::MapToStack => {
                    context.temp_mapping[i] = TempMapping::MapToStack;
                }
                initial_context::TempMapping::MapToSelf => {
                    context.temp_mapping[i] = TempMapping::MapToSelf;
                }
                initial_context::TempMapping::MapToLocal(local_index) => {
                    let local_index = match local_index {
                        0 => LocalIndex::Local0,
                        1 => LocalIndex::Local1,
                        2 => LocalIndex::Local2,
                        3 => LocalIndex::Local3,
                        4 => LocalIndex::Local4,
                        5 => LocalIndex::Local5,
                        6 => LocalIndex::Local6,
                        7 => LocalIndex::Local7,
                        _ => unreachable!(),
                    };
                    context.temp_mapping[i] = TempMapping::MapToLocal(local_index);
                }
            }
        }

        context
    }
}


impl ContextSize for Context {
    type Context = Context;
    type Pointer = Context;
    type Storage = ();
    fn get_pointer(&self) -> Context {
        self.clone()
    }

    fn get_storage(&self) -> Option<()> {
        None
    }
}
