use crate::*;

#[cfg(test)]
use crate::test_util::f_eq;

pub struct TimeUnit<'a>{
    _name: &'a str,
    _symbol: &'a str,
    _interval: f64
}

impl<'a> TimeUnit<'a> {

    pub fn of(&'a self, value: f64) -> Time<'a> {
        return Time{ _value: value, _unit: &self };
    }
}

impl<'a> LinearUnit<'a> for TimeUnit<'a>{
    fn interval(&self) -> f64 { return self._interval; }
}

#[allow(non_upper_case_globals)]
pub const s: TimeUnit<'static> = TimeUnit{_name: "second", _symbol: "s", _interval: 1.};

#[allow(non_upper_case_globals)]
pub const h: TimeUnit<'static> = TimeUnit{_name: "hour", _symbol: "h", _interval: 3600.};

#[allow(non_upper_case_globals)]
pub const ms: TimeUnit<'static> = TimeUnit{_name: "millisecond", _symbol: "ms", _interval: 0.001};

pub struct Time<'a>{
    _value: f64,
    _unit: &'a TimeUnit<'a>
}

impl<'a> LinearQuantity<'a, TimeUnit<'a>> for Time<'a> {
    fn value(&self) -> f64 { return self._value; }
    fn unit(&self) -> &TimeUnit<'a> { return self._unit; }
}

#[test]
fn test_unit_conversion(){
    let three_ms = ms.of(3.0);

    assert!(f_eq(three_ms.value_in(ms), 3.));
    assert!(f_eq(three_ms.value_in(s), 0.003));
    assert!(f_eq(three_ms.value_in(h), 0.003 / 3600.));
}