use crate::*;

#[cfg(test)]
use crate::test_util::f_eq;

pub struct LengthUnit<'a> {
    _name: &'a str,
    _symbol: &'a str,
    _interval: f64
}

impl<'a> LengthUnit<'a> {

    pub fn of(&'a self, value: f64) -> Length<'a> {
        return Length{ _value: value, _unit: &self };
    }



}

#[allow(non_upper_case_globals)]
pub const m: LengthUnit<'static> = LengthUnit{_name: "metre", _symbol: "m", _interval: 1.};

#[allow(non_upper_case_globals)]
pub const cm: LengthUnit<'static> = LengthUnit{_name: "centimetre", _symbol: "cm", _interval: 0.01};
#[allow(non_upper_case_globals)]
pub const mm: LengthUnit<'static> = LengthUnit{_name: "millimetre", _symbol: "mm", _interval: 0.001};

#[allow(non_upper_case_globals)]
pub const km: LengthUnit<'static> = LengthUnit{_name: "kilometre", _symbol: "km", _interval: 1000.};

impl<'a> LinearUnit<'a> for LengthUnit<'a>{
    fn interval(&self) -> f64 { return self._interval; }
}

pub struct Length<'a>{
    _value: f64,
    _unit: &'a LengthUnit<'a>
}

impl<'a> LinearQuantity<'a, LengthUnit<'a>> for Length<'a> {
    fn value(&self) -> f64 { return self._value; }
    fn unit(&self) -> &LengthUnit<'a> { return self._unit; }
}

#[test]
fn test_unit_conversion(){
    let three_cm = cm.of(3.0);

    assert!(f_eq(three_cm.value_in(cm), 3.));
    assert!(f_eq(three_cm.value_in(m), 0.03));
    assert!(f_eq(three_cm.value_in(mm), 30.));
    assert!(f_eq(three_cm.value_in(km), 0.00003));
}