use embedded_graphics_core::{prelude::*, primitives::Rectangle};

pub fn combine_bounding_boxes(bb1: Option<Rectangle>, bb2: Option<Rectangle>) -> Option<Rectangle> {
    if let Some(bb1) = bb1 {
        if let Some(bb2) = bb2 {
            let top_left = bb1.top_left.component_min(bb2.top_left);

            let bottomright1 = bb1.top_left + bb1.size;
            let bottomright2 = bb2.top_left + bb2.size;

            let bottomright = bottomright1.component_max(bottomright2);

            let size = Size::new(
                (bottomright.x - top_left.x).try_into().unwrap(),
                (bottomright.y - top_left.y).try_into().unwrap(),
            );

            Some(Rectangle::new(top_left, size))
        } else {
            Some(bb1)
        }
    } else {
        bb2
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn CombineBoundingBoxes_BothNone_ReturnsNone() {
        assert_eq!(combine_bounding_boxes(None, None), None);
    }

    #[test]
    fn CombineBoundingBoxes_OneNone_ReturnsOther() {
        let bb = Rectangle::new(Point::new(42, 69), Size::new(3, 2));
        assert_eq!(combine_bounding_boxes(Some(bb.clone()), None), Some(bb));
        assert_eq!(combine_bounding_boxes(None, Some(bb.clone())), Some(bb));
    }

    #[test]
    fn CombineBoundingBoxes_NonOverlapping_ReturnsJoinedBox() {
        let bb1 = Rectangle::new(Point::new(42, 69), Size::new(3, 2));
        let bb2 = Rectangle::new(Point::new(12, 15), Size::new(5, 4));
        let bbo = Rectangle::new(Point::new(12, 15), Size::new(33, 56));
        assert_eq!(combine_bounding_boxes(Some(bb1), Some(bb2)), Some(bbo));
    }

    #[test]
    fn CombineBoundingBoxes_Overlapping_ReturnsJoinedBox() {
        let bb1 = Rectangle::new(Point::new(5, 15), Size::new(7, 5));
        let bb2 = Rectangle::new(Point::new(9, 13), Size::new(6, 4));
        let bbo = Rectangle::new(Point::new(5, 13), Size::new(10, 7));
        assert_eq!(combine_bounding_boxes(Some(bb1), Some(bb2)), Some(bbo));
    }
}
