//! This crate contains every types used to interact with BRUTAL.
//! BRUTAL (<https://github.com/brutal-org/brutal>) is an operating system made from scratch, which is based on a microkernel.

/// BRUTAL handler type.
pub type BrHandle = u64;

pub type BrRight = u32;

pub type BrTaskFlags = u64;

/// BRUTAL handler value if null.
pub const BR_HANDLE_NIL: BrHandle = 0;
/// BRUTAL handler value if error.
pub const BR_HANDLE_ERR: BrHandle = unsafe { core::mem::transmute(-1i64) };
/// BRUTAL handler value if self.
pub const BR_HANDLE_SELF: BrHandle = unsafe { core::mem::transmute(-2i64) };

/// BRUTAL memory flags.
/// TODO: List bits.
pub type BrMemoryFlags = u64;
