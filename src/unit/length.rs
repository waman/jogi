use crate::*;

//********** UNIT **********
pub enum LengthUnit<'a>{
    Simple{ _name: &'a str, _symbol: &'a str, _interval: f64 }
}

// impl<'a> LengthUnit<'a>{

//     pub fn of(&'a self, value: f64) -> Box<dyn Length<'a> + 'a>{
//         return Box::new(SimpleLinearQuantity{ _value: value, _unit: self });
//     }
// }

impl<'a> LinearUnit<'a> for LengthUnit<'a>{
    fn name(&self) -> String {
        match self {
            LengthUnit::Simple { _name, _symbol, _interval } => (*_name).to_string(),
        }
    }

    fn symbol(&self) -> String {
        match self {
            LengthUnit::Simple { _name, _symbol, _interval } => (*_symbol).to_string(),
        }
    }

    fn interval(&self) -> f64 {
        match self {
            LengthUnit::Simple { _name, _symbol, _interval } => *_interval,
        }
    }
}

#[allow(non_upper_case_globals)]
pub const m: &LengthUnit = &LengthUnit::Simple{ _name: "metre", _symbol: "m", _interval: 1. };

#[allow(non_upper_case_globals)]
pub const cm: &LengthUnit = &LengthUnit::Simple{ _name: "centimetre", _symbol: "cm", _interval: 0.01 };
#[allow(non_upper_case_globals)]
pub const mm: &LengthUnit = &LengthUnit::Simple{ _name: "millimetre", _symbol: "mm", _interval: 0.001 };

#[allow(non_upper_case_globals)]
pub const km: &LengthUnit = &LengthUnit::Simple{ _name: "kilometre", _symbol: "km", _interval: 1000. };



//********** QUANTITY **********
pub struct Length<'a>{
    _value: f64,
    _unit: &'a LengthUnit<'a>
}

impl<'a> Length<'a>{

    pub fn value(&self) -> f64 { return self._value; }
    
    pub fn unit(&self) -> &'a LengthUnit<'a> { return self._unit; }

    pub fn value_in<U: AsRef<LengthUnit<'a>>>(&self, unit: U) -> f64 {
        return self.value() * self.unit().interval() / unit.as_ref().interval();
    }
}

impl<'a> AsRef<LengthUnit<'a>> for LengthUnit<'a>{
    fn as_ref(&self) -> &LengthUnit<'a> {
        return self;
    }
}

impl<'a> Into<Length<'a>> for (f64, &'a LengthUnit<'a>){

    fn into(self) -> Length<'a>{
        return Length { _value: self.0, _unit: self.1 };
    }
}

#[cfg(test)]
use crate::test_util::f_eq;

#[test]
fn test_unit_conversion(){
    let three_cm: Length = (3., cm).into();

    assert!(f_eq(three_cm.value_in(cm), 3.));
    assert!(f_eq(three_cm.value_in(m), 0.03));
    assert!(f_eq(three_cm.value_in(mm), 30.));
    assert!(f_eq(three_cm.value_in(km), 0.00003));
}