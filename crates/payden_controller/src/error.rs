pub(crate) type ResultDyn<T> = core::result::Result<T, alloc::boxed::Box<dyn core::error::Error>>;

#[derive(Debug)]
pub(crate) struct WebStoreInitError;

impl core::fmt::Display for WebStoreInitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Failed to initialize webstore")
    }
}

impl core::error::Error for WebStoreInitError {}
