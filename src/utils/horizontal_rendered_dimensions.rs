use core::cmp;

/// Similar to [`RenderedDimensions`], but only in the horizontal axis.
/// Saves a lot of memory in [`args::ArgsLineDimensionsIterator`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HorizontalRenderedDimensions {
    /// The advance in x direction
    pub advance: i32,
    /// The width of the bounding box.
    /// 0 in the case of no bounding box.
    pub bounding_box_width: u32,
    /// The horizontal offset of the bounding box.
    /// 0 in the case of no bounding box.
    pub bounding_box_offset: i32,
}

impl HorizontalRenderedDimensions {
    pub fn empty() -> Self {
        Self {
            advance: 0,
            bounding_box_width: 0,
            bounding_box_offset: 0,
        }
    }

    pub fn add(&mut self, other: HorizontalRenderedDimensions) {
        self.advance += other.advance;

        if self.bounding_box_width == 0 {
            self.bounding_box_width = other.bounding_box_width;
            self.bounding_box_offset = other.bounding_box_offset;
        } else if other.bounding_box_offset != 0 {
            debug_assert!(self.bounding_box_width <= i32::MAX as u32);
            debug_assert!(other.bounding_box_width <= i32::MAX as u32);

            let self_right = self.bounding_box_offset + self.bounding_box_width as i32;
            let other_right = other.bounding_box_offset + other.bounding_box_width as i32;
            let right = cmp::max(self_right, other_right);
            let left = cmp::min(self.bounding_box_offset, other.bounding_box_offset);

            debug_assert!(right >= left);

            self.bounding_box_offset = left;
            self.bounding_box_width = (right - left) as u32;
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn Add_SumsAdvance() {
        let mut a = HorizontalRenderedDimensions::empty();
        let mut b = HorizontalRenderedDimensions::empty();

        a.advance = 5;
        b.advance = 7;

        a.add(b);

        assert_eq!(a.advance, 12);
    }

    #[test]
    fn Add_BothNone_ReturnsNone() {
        let mut a = HorizontalRenderedDimensions::empty();
        let mut b = HorizontalRenderedDimensions::empty();

        a.bounding_box_offset = 0;
        a.bounding_box_width = 0;
        b.bounding_box_offset = 0;
        b.bounding_box_width = 0;

        a.add(b);

        assert_eq!(a.bounding_box_offset, 0);
        assert_eq!(a.bounding_box_width, 0);
    }

    #[test]
    fn Add_SelfNone_ReturnsOther() {
        let mut a = HorizontalRenderedDimensions::empty();
        let mut b = HorizontalRenderedDimensions::empty();

        a.bounding_box_offset = 0;
        a.bounding_box_width = 0;
        b.bounding_box_offset = 42;
        b.bounding_box_width = 69;

        a.add(b);

        assert_eq!(a.bounding_box_offset, 42);
        assert_eq!(a.bounding_box_width, 69);
    }

    #[test]
    fn Add_OtherNone_ReturnsSelf() {
        let mut a = HorizontalRenderedDimensions::empty();
        let mut b = HorizontalRenderedDimensions::empty();

        a.bounding_box_offset = 42;
        a.bounding_box_width = 69;
        b.bounding_box_offset = 0;
        b.bounding_box_width = 0;

        a.add(b);

        assert_eq!(a.bounding_box_offset, 42);
        assert_eq!(a.bounding_box_width, 69);
    }

    #[test]
    fn Add_NonOverlapping_ReturnsJoinedBox() {
        let mut a = HorizontalRenderedDimensions::empty();
        let mut b = HorizontalRenderedDimensions::empty();

        a.bounding_box_offset = 42;
        a.bounding_box_width = 3;
        b.bounding_box_offset = 12;
        b.bounding_box_width = 5;

        a.add(b);

        assert_eq!(a.bounding_box_offset, 12);
        assert_eq!(a.bounding_box_width, 33);
    }

    #[test]
    fn Add_Overlapping_ReturnsJoinedBox() {
        let mut a = HorizontalRenderedDimensions::empty();
        let mut b = HorizontalRenderedDimensions::empty();

        a.bounding_box_offset = 5;
        a.bounding_box_width = 7;
        b.bounding_box_offset = 9;
        b.bounding_box_width = 6;

        a.add(b);

        assert_eq!(a.bounding_box_offset, 5);
        assert_eq!(a.bounding_box_width, 10);
    }
}
