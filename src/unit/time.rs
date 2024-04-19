use crate::*;

pub fn time<'a>(value: f64, unit: &'a dyn TimeUnit<'a>) -> Box<dyn Time<'a> + 'a > {
    return Box::new(SimpleLinearQuantity{ _value: value, _unit: unit });
}



pub trait TimeUnit<'a>: LinearUnit<'a>{}

impl<'a> dyn TimeUnit<'a> + 'a{

    pub fn of(&'a self, value: f64) -> Box<dyn Time<'a> + 'a> {
        return time(value, self);
    }
}

impl<'a> TimeUnit<'a> for SimpleLinearUnit<'a>{}

#[allow(non_upper_case_globals)]
pub const s: &dyn TimeUnit = &SimpleLinearUnit{_name: "second", _symbol: "s", _interval: 1.};

#[allow(non_upper_case_globals)]
pub const h: &dyn TimeUnit = &SimpleLinearUnit{_name: "hour", _symbol: "h", _interval: 3600.};

#[allow(non_upper_case_globals)]
pub const ms: &dyn TimeUnit = &SimpleLinearUnit{_name: "millisecond", _symbol: "ms", _interval: 0.001};



pub trait Time<'a>: LinearQuantity<'a, dyn TimeUnit<'a> + 'a>{}

impl<'a> Time<'a> for SimpleLinearQuantity<'a, dyn TimeUnit<'a> + 'a>{}




#[cfg(test)]
use crate::test_util::f_eq;

#[test]
fn test_unit_conversion(){
    // let three_ms = time(3., ms);
    let three_ms = ms.of(3.);

    assert!(f_eq(three_ms.value_in(ms), 3.));
    assert!(f_eq(three_ms.value_in(s), 0.003));
    assert!(f_eq(three_ms.value_in(h), 0.003 / 3600.));
}