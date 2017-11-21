pub struct MyRange {
    start: u8,
    end: u8,
    cur: u8,
    done: bool,
}

pub fn range_inclusive(start: u8, end: u8) -> MyRange {
    MyRange {
        done: false,
        start,
        end,
        cur: start,
    }
}

impl Iterator for MyRange {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.done {
            None
        } else {
            let val = self.cur;
            if self.cur == self.end {
                self.done = true;
            } else {
                self.cur += 1;
            }
            Some(val)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::range_inclusive;

    #[test]
    fn test_range() {
        let v: Vec<_> = range_inclusive(0, 5).collect();
        assert_eq!(&v[..], [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_range_full() {
        let v: Vec<u8> = range_inclusive(0, 255).collect();
        assert_eq!(v.len(), 256);
        assert_eq!(*v.first().unwrap(), 0);
        assert_eq!(*v.last().unwrap(), 255);
    }
}
