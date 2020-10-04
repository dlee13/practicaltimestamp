#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weekday(u64);

impl Weekday {
    pub const MONDAY: Weekday = Self::new(0);
    pub const TUESDAY: Weekday = Self::new(1);
    pub const WEDNESDAY: Weekday = Self::new(2);
    pub const THURSDAY: Weekday = Self::new(3);
    pub const FRIDAY: Weekday = Self::new(4);
    pub const SATURDAY: Weekday = Self::new(5);
    pub const SUNDAY: Weekday = Self::new(6);

    pub(super) const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn previous(self) -> Self {
        if self.0 > 0 {
            Self::new(self.0.wrapping_sub(1))
        } else {
            Self::new(6)
        }
    }

    pub const fn next(self) -> Self {
        if self.0 < 6 {
            Self::new(self.0.wrapping_add(1))
        } else {
            Self::new(0)
        }
    }

    pub const fn days_since(self, earlier: Self) -> u64 {
        let difference = self.0.wrapping_sub(earlier.0);
        if difference > 6 {
            difference.wrapping_add(7)
        } else {
            difference
        }
    }

    pub const fn days_until(self, later: Self) -> u64 {
        later.days_since(self)
    }
}
