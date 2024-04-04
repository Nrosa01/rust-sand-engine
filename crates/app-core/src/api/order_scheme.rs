
use crate::api::CustomRange;

pub(super) struct OrderScheme {
    pub(super) order_x: CustomRange,
    pub(super) order_y: CustomRange,
}

impl OrderScheme {
    pub fn new(order_x: CustomRange, order_y: CustomRange) -> OrderScheme {
        OrderScheme {
            order_x: order_x,
            order_y: order_y,
        }
    }
}

pub struct OrderSchemes {
    ltr_ttb: OrderScheme,
    ltr_btt: OrderScheme,
    rtl_ttb: OrderScheme,
    rtl_btt: OrderScheme,
    current: usize,
}

impl OrderSchemes {
    pub fn new(width: usize, height: usize) -> OrderSchemes {
        OrderSchemes {
            ltr_ttb: OrderScheme::new(
                CustomRange::new(0, width as isize, 1),
                CustomRange::new(0, height as isize, 1),
            ),
            ltr_btt: OrderScheme::new(
                CustomRange::new(0, width as isize, 1),
                CustomRange::new((height - 1) as isize, -1, -1),
            ),
            rtl_ttb: OrderScheme::new(
                CustomRange::new((width - 1) as isize, -1, -1),
                CustomRange::new(0, height as isize, 1),
            ),
            rtl_btt: OrderScheme::new(
                CustomRange::new((width - 1) as isize, -1, -1),
                CustomRange::new((height - 1) as isize, -1, -1),
            ),
            current: 0,
        }
    }

    pub(super) fn get_ciclying(&mut self) -> &OrderScheme {
        let scheme = match self.current {
            0 => &self.ltr_ttb,
            1 => &self.rtl_ttb,
            2 => &self.rtl_btt,
            3 => &self.ltr_btt,
            _ => &self.ltr_ttb,
        };

        self.current = (self.current + 1) % 4;
        scheme
    }
}