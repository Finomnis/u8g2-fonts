use core::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct DebugIgnore<T>(pub T);

impl<T> core::fmt::Debug for DebugIgnore<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("...")
    }
}

impl<T> Deref for DebugIgnore<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for DebugIgnore<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
