use deepsize::DeepSizeOf;
use serde::{Serialize, Deserialize};

use crate::{initial_context::{Type, Context}, ContextSize, compact_temp_mapping::{TempMapping, self}};

// Compressed version of Context
pub type PackedContext = Box<[ContextDelta]>;

// The fields of each variant should only use 2 bytes, which makes this enum 3 bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, DeepSizeOf)]
pub enum ContextDelta {
    None,
    // stack_size, sp_offset: Small case
    SmallStack(u8, i8),
    // stack_size: Large case
    //StackSize(u8, u8), // TODO: implement this (using u16 here takes more than 2 bytes)
    // sp_offset: Large case
    //SpOffset(i8, i8),  // TODO: implement this (using i16 here takes more than 2 bytes)
    // chain_depth
    ChainDepth(u8),
    // local_types
    LocalType(u8, Type),
    // temp_types
    TempType(u8, Type),
    // self_type
    SelfType(Type),
    // temp_mapping: Not using (u8, TempMapping) to save 1 byte.
    TempMapping0(TempMapping),
    TempMapping1(TempMapping),
    TempMapping2(TempMapping),
    TempMapping3(TempMapping),
    TempMapping4(TempMapping),
    TempMapping5(TempMapping),
    TempMapping6(TempMapping),
    TempMapping7(TempMapping),
}

// Deflate Context
pub fn pack_context(ctx: &Context) -> PackedContext {

    let ctx : compact_temp_mapping::Context = ctx.clone().into();
    let mut packed = vec![];

    if ctx.self_type != Type::Unknown {
        packed.push(ContextDelta::SelfType(ctx.self_type));
    }


    if ctx.chain_depth > 0 {
        packed.push(ContextDelta::ChainDepth(ctx.chain_depth));
    }

    for (i, &local_type) in ctx.local_types.iter().enumerate() {
        if local_type != Type::Unknown {
            packed.push(ContextDelta::LocalType(i.try_into().unwrap(), local_type));
        }
    }

    for (i, &temp_type) in ctx.temp_types.iter().enumerate() {
        if temp_type != Type::Unknown {
            packed.push(ContextDelta::TempType(i.try_into().unwrap(), temp_type));
        }
    }


    for (i, &temp_mapping) in ctx.temp_mapping.iter().enumerate() {
        if temp_mapping != TempMapping::MapToStack {
            match i {
                0 => packed.push(ContextDelta::TempMapping0(temp_mapping)),
                1 => packed.push(ContextDelta::TempMapping1(temp_mapping)),
                2 => packed.push(ContextDelta::TempMapping2(temp_mapping)),
                3 => packed.push(ContextDelta::TempMapping3(temp_mapping)),
                4 => packed.push(ContextDelta::TempMapping4(temp_mapping)),
                5 => packed.push(ContextDelta::TempMapping5(temp_mapping)),
                6 => packed.push(ContextDelta::TempMapping6(temp_mapping)),
                7 => packed.push(ContextDelta::TempMapping7(temp_mapping)),
                _ => unreachable!(),
            }
        }
    }

    if ctx.stack_size != 0 || ctx.sp_offset != 0 {
        match (u8::try_from(ctx.stack_size), i8::try_from(ctx.sp_offset)) {
            (Ok(stack_size), Ok(sp_offset)) => packed.push(
                ContextDelta::SmallStack(stack_size, sp_offset)
            ),
            _ => {
                unreachable!("not implemented yet")
                //if ctx.stack_size != 0 {
                //    packed.push(ContextDelta::StackSize(ctx.stack_size));
                //}
                //if ctx.sp_offset != 0 {
                //    packed.push(ContextDelta::SpOffset(ctx.sp_offset));
                //}
            },
        }
    }


    packed.into_boxed_slice()
}


impl From<Context> for PackedContext {
    fn from(ctx: Context) -> Self {
        pack_context(&ctx)
    }
}

impl ContextSize for PackedContext {
    type Context = PackedContext;
    type Pointer = PackedContext;
    type Storage = ();

    fn get_pointer(&self) -> Self::Pointer {
        self.clone()
    }

    fn get_storage(&self) -> Option<Self::Storage> {
        None
    }
}
