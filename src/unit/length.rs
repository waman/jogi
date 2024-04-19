use crate::*;

pub fn length<'a>(value: f64, unit: &'a dyn LengthUnit<'a>) -> Box<dyn Length<'a> + 'a > {
    return Box::new(SimpleLinearQuantity{ _value: value, _unit: unit });
}



pub trait LengthUnit<'a>: LinearUnit<'a>{}

impl<'a> dyn LengthUnit<'a> + 'a{

    pub fn of(&'a self, value: f64) -> Box<dyn Length<'a> + 'a> {
        return length(value, self);
    }
}

impl<'a> LengthUnit<'a> for SimpleLinearUnit<'a>{}

#[allow(non_upper_case_globals)]
pub const m: &dyn LengthUnit = &SimpleLinearUnit{_name: "metre", _symbol: "m", _interval: 1.};

#[allow(non_upper_case_globals)]
pub const cm: &dyn LengthUnit = &SimpleLinearUnit{_name: "centimetre", _symbol: "cm", _interval: 0.01};
#[allow(non_upper_case_globals)]
pub const mm: &dyn LengthUnit = &SimpleLinearUnit{_name: "millimetre", _symbol: "mm", _interval: 0.001};

#[allow(non_upper_case_globals)]
pub const km: &dyn LengthUnit = &SimpleLinearUnit{_name: "kilometre", _symbol: "km", _interval: 1000.};



pub trait Length<'a>: LinearQuantity<'a, dyn LengthUnit<'a> + 'a>{}

impl<'a> Length<'a> for SimpleLinearQuantity<'a, dyn LengthUnit<'a> + 'a>{}



#[cfg(test)]
use crate::test_util::f_eq;

#[test]
fn test_unit_conversion(){
    // let three_cm = length(3., cm);
    let three_cm = cm.of(3.);

    assert!(f_eq(three_cm.value_in(cm), 3.));
    assert!(f_eq(three_cm.value_in(m), 0.03));
    assert!(f_eq(three_cm.value_in(mm), 30.));
    assert!(f_eq(three_cm.value_in(km), 0.00003));
}