use std::ops::Div;

use crate::*;
use crate::unit::length::*;
use crate::unit::time::*;


//********** UNIT **********
pub enum VelocityUnit<'a>{
    Simple{ _name: &'a str, _symbol: &'a str, _interval: f64 },
    Quotient{ _numerator: &'a LengthUnit<'a>, _denominator: &'a TimeUnit<'a> }
}

impl<'a> VelocityUnit<'a>{

    pub fn new_quotient(n: &'a LengthUnit, d: &'a TimeUnit) -> VelocityUnit<'a>{
        return VelocityUnit::Quotient { _numerator: n, _denominator: d };
    }

    // pub fn of(&'a self, value: f64) -> Box<dyn Velocity<'a> + 'a>{
    //     return Box::new(SimpleLinearQuantity{ _value: value, _unit: self });
    // }
}

impl<'a> LinearUnit<'a> for VelocityUnit<'a>{
    
    fn name(&self) ->  String {
        match self {
            VelocityUnit::Simple { _name, _symbol, _interval } => (*_name).to_string(),
            VelocityUnit::Quotient { _numerator, _denominator } =>
                format!("{} per {}", _numerator.name(), _denominator.name()),
        }
    }

    fn symbol(&self) ->  String {
        match self {
            VelocityUnit::Simple { _name, _symbol, _interval } => (*_symbol).to_string(),
            VelocityUnit::Quotient { _numerator, _denominator } => 
                format!("{}/{}", _numerator.symbol(), _denominator.symbol()),
        }
    }

    fn interval(&self) -> f64 {
        match self {
            VelocityUnit::Simple { _name, _symbol, _interval } => *_interval,
            VelocityUnit::Quotient { _numerator, _denominator } => 
                _numerator.interval() / _denominator.interval(),
        }
    }
}

impl<'a> Div<&'a TimeUnit<'a>> for &'a LengthUnit<'a>{

    type Output = VelocityUnit<'a>;

    fn div(self, rhs: &'a TimeUnit<'a>) -> Self::Output {
        return VelocityUnit::new_quotient(self, rhs);
    }
}

#[allow(non_upper_case_globals)]
pub const c: &VelocityUnit = &VelocityUnit::Simple{ _name: "speed of light", _symbol: "c", _interval: SPEED_OF_LIGHT };



//********** QUANTITY **********
pub enum Velocity<'a>{
    Ref{ _value: f64, _unit: &'a VelocityUnit<'a> },
    Val{ _value: f64, _unit: VelocityUnit<'a> }
}
// pub struct Velocity<'a>{
//     _value: f64,
//     _unit: &'a VelocityUnit<'a>
// }

impl<'a> Velocity<'a>{

    pub fn value(&self) -> f64 {
        match self {
            Velocity::Ref { _value, _unit } => *_value,
            Velocity::Val { _value, _unit } => *_value
        }
    }
    
    pub fn unit(&'a self) -> &'a VelocityUnit<'a> {
        match self {
            Velocity::Ref { _value, _unit } => *_unit,
            Velocity::Val { _value, _unit } => _unit
        }
    }

    pub fn value_in<U: AsRef<VelocityUnit<'a>>>(&self, unit: U) -> f64 {
        return self.value() * self.unit().interval() / unit.as_ref().interval();
    }
}

impl<'a> AsRef<VelocityUnit<'a>> for VelocityUnit<'a>{
    fn as_ref(&self) -> &VelocityUnit<'a> {
        return self;
    }
}

impl<'a> Into<Velocity<'a>> for (f64, &'a VelocityUnit<'a>){

    fn into(self) -> Velocity<'a>{
        return Velocity::Ref { _value: self.0, _unit: self.1 };
    }
}

impl<'a> Into<Velocity<'a>> for (f64, VelocityUnit<'a>){

    fn into(self) -> Velocity<'a>{
        return Velocity::Val { _value: self.0, _unit: self.1 };
    }
}


#[cfg(test)]
use crate::test_util::f_eq;

#[test]
fn test_unit_conversion(){
    let three_kmh: Velocity = (3., km/h).into();
    // let three_kmh = (km/h).of(3.);

    assert!(f_eq(three_kmh.value_in(km/h), 3.));
    assert!(f_eq(three_kmh.value_in(m/s), 3000./3600.));
    assert!(f_eq(three_kmh.value_in(c), 3000./3600./SPEED_OF_LIGHT));

    let half_c: Velocity = (0.5, c).into();
    assert!(f_eq(half_c.value_in(m/s), SPEED_OF_LIGHT / 2.))
}