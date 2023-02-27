use deepsize::DeepSizeOf;
use serde::{Serialize, Deserialize};

use crate::ContextSize;

// Maximum number of temp value types we keep track of
pub const MAX_TEMP_TYPES: usize = 8;

// Maximum number of local variable types we keep track of
pub const MAX_LOCAL_TYPES: usize = 8;

// Represent the type of a value (local/stack/self) in YJIT
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize, DeepSizeOf)]
pub enum Type {
    Unknown,
    UnknownImm,
    UnknownHeap,
    Nil,
    True,
    False,
    Fixnum,
    Flonum,
    Hash,
    ImmSymbol,

    #[allow(unused)]
    HeapSymbol,

    TString, // An object with the T_STRING flag set, possibly an rb_cString
    CString, // An un-subclassed string of type rb_cString (can have instance vars in some cases)
    TArray,  // An object with the T_ARRAY flag set, possibly an rb_cArray
    CArray,  // An un-subclassed string of type rb_cArray (can have instance vars in some cases)

    BlockParamProxy, // A special sentinel value indicating the block parameter should be read from
                     // the current surrounding cfp
}

// Default initialization
impl Default for Type {
    fn default() -> Self {
        Type::Unknown
    }
}


// Potential mapping of a value on the temporary stack to
// self, a local variable or constant so that we can track its type
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize, DeepSizeOf)]
#[allow(clippy::enum_variant_names)]
pub enum TempMapping {
    MapToStack,     // Normal stack value
    MapToSelf,      // Temp maps to the self operand
    MapToLocal(u8), // Temp maps to a local variable with index
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


#[derive(Debug, Serialize, Deserialize)]
pub struct ContextWithCount {
    pub context: Context,
    pub count: u64,
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
