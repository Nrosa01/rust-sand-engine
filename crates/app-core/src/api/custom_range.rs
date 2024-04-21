#[derive(Copy, Clone)]
pub struct CustomRange {
    current: isize,
    end: isize,
    step: isize,
}

impl CustomRange {
    pub fn new(start: isize, end: isize, step: isize) -> CustomRange {
        CustomRange {
            current: start,
            end,
            step,
        }
    }
}

impl Iterator for CustomRange {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.step > 0 && self.current < self.end) || (self.step < 0 && self.current > self.end)
        {
            let result = self.current;
            self.current += self.step;
            Some(result)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end - self.current).abs() as usize;
        (len, Some(len))
    }
}