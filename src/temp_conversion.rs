//! Temperature conversion vec and lookup function

/// The lookup table maps temperature values to resistance values. The
/// temperature values are the range [min, min + step, ..., min + step *
/// (len(data) - 1)]. The resistance values are stored in the array data,
pub struct LookupTable<'a, D> {
    min: i16,
    step: i16,
    data: &'a [D],
}

fn interpolate(ohm_100: i32, first: (i32, i32), second: (i32, i32)) -> i32 {
    let numerator = (second.0 - first.0) * (ohm_100 - first.1);
    let denominator = second.1 - first.1;

    numerator / denominator + first.0
}

pub trait LookupToI32 {
    fn lookup(&self, ind: usize) -> i32;
    fn binary_search(&self, val: i32) -> Result<usize, usize>;
}

impl<'a> LookupToI32 for LookupTable<'a, u16> {
    fn lookup(&self, ind: usize) -> i32 {
        self.data[ind] as i32
    }

    fn binary_search(&self, val: i32) -> Result<usize, usize> {
        let val = val as u16;
        self.data.binary_search(&val)
    }
}

impl<'a> LookupToI32 for LookupTable<'a, u32> {
    fn lookup(&self, ind: usize) -> i32 {
        self.data[ind] as i32
    }

    fn binary_search(&self, val: i32) -> Result<usize, usize> {
        let val = val as u32;
        self.data.binary_search(&val)
    }
}

impl<'a, D> LookupTable<'a, D>
where
    LookupTable<'a, D>: LookupToI32,
{
    fn reverse_index(&self, index: usize) -> i32 {
        (self.min as i32 + (index * self.step as usize) as i32) * 100
    }

    /// The value from which lower bound interpolation should occur
    fn ohm_lower_bound(&self) -> i32 {
        self.lookup(1)
    }

    /// The value from which upper bound interpolation should occur
    fn ohm_upper_bound(&self) -> i32 {
        self.lookup(self.data.len() - 2)
    }

    fn interpolate_index(&self, ohm_100: i32, index: usize) -> i32 {
        let first = (self.reverse_index(index) as i32, self.lookup(index));
        let second = (self.reverse_index(index + 1) as i32, self.lookup(index + 1));
        interpolate(ohm_100, first, second)
    }

    /// Convert the specified resistance value into a temperature using the provided lookup table.
    ///
    /// # Arguments
    ///
    /// * `val` - A 16 bit unsigned integer specifying the resistance in Ohms
    /// multiplied by 100, e.g. 13851 would indicate 138.51 Ohms and convert to 100
    /// degrees Celsius.
    ///
    /// # Remarks
    ///
    /// The output temperature will be in degrees Celsius multiplied by 100, e.g.
    /// 10000 would signify 100.00 degrees Celsius.
    ///
    /// *Note*: This interpolates from the bottom or top values if the resistance
    /// value is out of range.
    pub fn lookup_temperature(&self, ohm_100: i32) -> i32 {
        if ohm_100 < self.ohm_lower_bound() {
            self.interpolate_index(ohm_100, 0)
        } else if ohm_100 > self.ohm_upper_bound() {
            self.interpolate_index(ohm_100, self.data.len() - 2)
        } else {
            let index = match self.binary_search(ohm_100) {
                Ok(val) => val,
                Err(val) => val - 1,
            };
            self.interpolate_index(ohm_100 as i32, index)
        }
    }
}

/// This lookup table contains the resistance values for a PT100 RTD ranging
/// from 0 C° up to 130 C° in steps of 10 C°, corresponding to a range from
/// 100.0 Ohms to 149.83 Ohms.
pub const LOOKUP_TABLE_PT100_SHORT: LookupTable<'static, u16> = LookupTable {
    min: 0,
    step: 20,
    data: &[
        10000, 10390, 10779, 11167, 11554, 11940, 12324, 12708, 13090, 13471, 13851, 14229, 14607,
        14983,
    ],
};

/// This lookup table contains the resistance values for a PT100 RTD ranging
/// from -200 C° up to 800 C°, corresponding to a range from 18.52 Ohms to
/// 369.71 Ohms. Calculated using `fn make_lookup()` below.
pub const LOOKUP_VEC_PT100: LookupTable<'static, u32> = LookupTable {
    min: -200,
    step: 20,
    data: &[
        1852, 2710, 3554, 4388, 5211, 6026, 6833, 7633, 8427, 9216, 10000, 10779, 11554, 12324,
        13090, 13851, 14607, 15358, 16105, 16848, 17586, 18319, 19047, 19771, 20490, 21205, 21915,
        22621, 23321, 24018, 24709, 25396, 26078, 26756, 27429, 28098, 28762, 29421, 30075, 30725,
        31371, 32012, 32648, 33279, 33906, 34528, 35146, 35759, 36367, 36971,
    ],
};

/// This lookup table contains the resistance values for a PT1000 RTD ranging
/// from -200 C° up to 800 C°, corresponding to a range from 185.20 Ohms to
/// 3697.12 Ohms. Calculated using `fn make_lookup()` below.
pub const LOOKUP_VEC_PT1000: LookupTable<'static, u32> = LookupTable {
    min: -200,
    step: 20,
    data: &[
        18520, 27096, 35543, 43876, 52110, 60256, 68325, 76328, 84271, 92160, 100000, 107794,
        115541, 123242, 130897, 138505, 146068, 153584, 161054, 168478, 175856, 183188, 190473,
        197712, 204905, 212052, 219152, 226206, 233214, 240176, 247092, 253961, 260785, 267562,
        274293, 280978, 287616, 294208, 300754, 307254, 313708, 320116, 326477, 332792, 339061,
        345284, 351460, 357590, 363674, 369712,
    ],
};

#[cfg(test)]
mod test {
    use super::{
        index, lookup_temperature, lookup_temperature_pt1000, reverse_index, MAX, MIN, STEP,
    };

    const A: f64 = 3.9083e-3;
    const B: f64 = -5.775e-7;
    const C: f64 = -4.18301e-12;

    #[test]
    fn make_lookup_pt100() {
        make_lookup(100);
    }

    #[test]
    fn make_lookup_pt1000() {
        make_lookup(1000);
    }

    fn make_lookup(r0: u16) {
        // use Callendar–Van Dusen equation

        /*
        R(T) = R0(1 + aT + bT2 + c(T - 100)T3)
        where:
        T = temperature (NC)
        R(T) = resistance at T
        R0 = resistance at T = 0NC
        IEC 751 specifies α = 0.00385055 and the following
        Callendar-Van Dusen coefficient values:
        a = 3.90830 x 10-3
        b = -5.77500 x 10-7
        c = -4.18301
        */

        // according to wikipedia there are more accurate formula
        let mut arr = [0u32; 50];

        for t in (MIN..MAX).step_by(STEP) {
            let c = if t < 0 { C } else { 0.0 };
            let t1 = t as f64;
            let t2 = t1 * t1;
            let t3 = t2 * t1;
            //R_0*(1+a_*A4+b_*B4+D4*(A4-100)*C4)
            let r = r0 as f64 * (1.0 + A * t1 + B * t2 + c * (t1 - 100.0) * t3);

            arr[index(t)] = (r * 100.0).round() as u32;
        }

        if r0 == 100 {
            // value taken from https://datasheets.maximintegrated.com/en/ds/MAX31865.pdf TABLE 9
            assert_eq!(arr[index(-200i16)], 1_852);
            assert_eq!(arr[index(-100i16)], 6_026);
            assert_eq!(arr[index(0i16)], 10_000);
            assert_eq!(arr[index(100i16)], 13_851);
        } else if r0 == 1000 {
            assert_eq!(arr[index(0i16)], 100_000);
        }

        //println!("{:?}", arr);
    }

    #[test]
    fn test_index() {
        assert_eq!(index(-1), 9);
        assert_eq!(index(0), 10);
        assert_eq!(index(5), 10);
        assert_eq!(index(20), 11);
    }

    #[test]
    fn test_reverse_index() {
        assert_eq!(reverse_index(0), -20_000); // -200 C°
        assert_eq!(reverse_index(1), -18_000); // -180 C°
        assert_eq!(reverse_index(10), 0);
        assert_eq!(reverse_index(20), 20_000); // 20 C°
    }

    #[test]
    fn test_lookup() {
        assert!(lookup_temperature(0).is_none());

        assert_eq!(lookup_temperature(10_000).unwrap(), 0);
        assert_eq!(lookup_temperature(10_390).unwrap(), 1_001);
        assert_eq!(lookup_temperature(20_000).unwrap(), 26_636);
        assert_eq!(lookup_temperature(2_000).unwrap(), -19_656);

        assert_eq!(lookup_temperature_pt1000(100_000).unwrap(), 0);
        assert_eq!(lookup_temperature_pt1000(103_900).unwrap(), 1_000);
    }
}
