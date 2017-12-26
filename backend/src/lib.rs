extern crate futures;

pub use self::buffer::Buffer;

/// Contains the `Backend` trait and related types.
pub mod backend;

pub mod buffer;

/// Contains the `Capability` enum.
pub mod capability;
pub mod event;
pub mod stream;

#[macro_use] pub mod compliance;

pub mod prelude {
    pub use super::backend::{
        Backend as NotifyBackend,
        Error as BackendError,
        Result as BackendResult,
    };

    pub use super::capability::Capability;

    pub use super::event::{
        AccessKind,
        AccessMode,
        CreateKind,
        DataChange,
        Event,
        EventKind,
        MetadataKind,
        ModifyKind,
        RemoveKind,
        RenameMode,
    };

    pub use super::stream::{
        Error as StreamError,
        Item as StreamItem,
        EmptyResult as EmptyStreamResult,
    };
}