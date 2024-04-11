pub struct LengthUnit<'a>{
    name: &'a str,
    symbol: &'a str,
    interval: f64
}

impl<'a> LengthUnit<'a> {

    pub fn of(&'a self, value: f64) -> Length<'a> {
        return Length{ value, unit: &self };
    }
}

#[allow(non_upper_case_globals)]
pub static metre: &LengthUnit = &LengthUnit{name: "metre", symbol: "m", interval: 1.};
#[allow(non_upper_case_globals)]
pub static centi_metre: &LengthUnit = &LengthUnit{name: "centi metre", symbol: "cm", interval: 0.01};
#[allow(non_upper_case_globals)]
pub static milli_metre: &LengthUnit = &LengthUnit{name: "milli metre", symbol: "mm", interval: 0.001};

pub struct Length<'a>{
    value: f64,
    unit: &'a LengthUnit<'a>
}

impl<'a> Length<'a> {

    pub fn value<'b>(&self, unit: &'b LengthUnit<'b>) -> f64 {
        return self.value * self.unit.interval / unit.interval;
    }
}

#[test]
fn test_unit_conversion(){
    let three_cm = centi_metre.of(3.0);

    let f_eq = |x: f64, y: f64| (x - y).abs() < 1e-10;

    assert!(f_eq(three_cm.value(metre), 0.03));
    assert!(f_eq(three_cm.value(milli_metre), 30.));
    assert!((three_cm.value(milli_metre) - 30.0).abs() < 1e-10);
}