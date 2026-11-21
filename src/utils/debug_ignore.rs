use core::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    extern crate std;
    use std::format;

    use super::*;

    #[test]
    fn ignores_content_on_debug() {
        assert_eq!(format!("{:?}", DebugIgnore(42)), "...");
    }

    #[test]
    fn ignores_content_on_pretty_debug() {
        assert_eq!(format!("{:#?}", DebugIgnore(42)), "...");
    }

    #[test]
    fn works_for_types_without_debug() {
        struct NotDebug;
        assert_eq!(format!("{:?}", DebugIgnore(NotDebug)), "...");
    }

    #[test]
    fn hides_content_inside_derived_debug() {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct Outer {
            visible: u8,
            hidden: DebugIgnore<u8>,
        }
        let outer = Outer {
            visible: 1,
            hidden: DebugIgnore(2),
        };
        assert_eq!(format!("{:?}", outer), "Outer { visible: 1, hidden: ... }");
    }

    #[test]
    fn deref_gives_access_to_inner_value() {
        let value = DebugIgnore(42);
        assert_eq!(*value, 42);
    }

    #[test]
    fn deref_forwards_method_calls() {
        let value = DebugIgnore([1u8, 2, 3]);
        assert_eq!(value.len(), 3);
    }

    #[test]
    fn deref_mut_modifies_inner_value() {
        let mut value = DebugIgnore(42);
        *value += 1;
        assert_eq!(value.0, 43);
    }

    #[test]
    fn is_copy_and_clone() {
        let a = DebugIgnore(42);
        let b = a;
        #[allow(clippy::clone_on_copy)]
        let c = a.clone();
        assert_eq!(a.0, 42);
        assert_eq!(b.0, 42);
        assert_eq!(c.0, 42);
    }

    #[test]
    fn compares_by_inner_value() {
        assert_eq!(DebugIgnore(1), DebugIgnore(1));
        assert_ne!(DebugIgnore(1), DebugIgnore(2));
    }
}
