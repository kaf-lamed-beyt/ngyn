use crate::{RustleController, RustleInjectable};

/// `RustleModule` is a trait that defines the basic structure of a module in Rustle.
pub trait RustleModule {
    /// Creates a new instance of the module.
    /// This is for internal use only.
    fn new() -> Self;

    /// Returns the controllers of the module.
    fn get_controllers(&self) -> Vec<std::sync::Arc<dyn RustleController>>;

    /// Returns the providers of the module.
    fn get_providers(&self) -> Vec<std::sync::Arc<dyn RustleInjectable>>;
}