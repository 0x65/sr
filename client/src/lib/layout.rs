use tui::layout::Rect;

pub fn center_rect(width: u16, height: u16, area: Rect) -> Rect {
    Rect {
        x: area.x + ((area.width - width) / 2),
        y: area.y + ((area.height - height) / 2),
        width: width,
        height: height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_rect() {
        /*
        ...      1x1 rect in 3x3 area
        .o.
        ...
        */
        let area = Rect {
            x: 0,
            y: 0,
            width: 3,
            height: 3,
        };
        let expected = Rect {
            x: 1,
            y: 1,
            width: 1,
            height: 1,
        };
        assert_eq!(center_rect(1, 1, area), expected);
        /*
        ....     2x2 rect in 4x4 area
        .oo.
        .oo.
        ....
        */
        let area = Rect {
            x: 0,
            y: 0,
            width: 4,
            height: 4,
        };
        let expected = Rect {
            x: 1,
            y: 1,
            width: 2,
            height: 2,
        };
        assert_eq!(center_rect(2, 2, area), expected);
        /*
        ....     2x2 rect in 4x4 area with (x, y) offset
        .oo.
        .oo.
        ....
        */
        let area = Rect {
            x: 1,
            y: 1,
            width: 4,
            height: 4,
        };
        let expected = Rect {
            x: 2,
            y: 2,
            width: 2,
            height: 2,
        };
        assert_eq!(center_rect(2, 2, area), expected);
        /*
        oo.     2x2 rect in 3x3 area (default to upper-left)
        oo.
        ...
        */
        let area = Rect {
            x: 0,
            y: 0,
            width: 3,
            height: 3,
        };
        let expected = Rect {
            x: 0,
            y: 0,
            width: 2,
            height: 2,
        };
        assert_eq!(center_rect(2, 2, area), expected);
    }
}
