use std::ops::Div;

use crate::*;
use crate::unit::length::*;
use crate::unit::time::*;


//********** UNIT **********
pub enum VelocityUnit<'a>{
    Simple{ name: &'a str, symbol: &'a str, interval: f64 },
    Quotient{ numerator: &'a LengthUnit<'a>, denominator: &'a TimeUnit<'a> }
}

impl<'a> LinearUnit<'a> for VelocityUnit<'a>{
    
    fn get_name(&self) ->  String {
        match self {
            VelocityUnit::Simple { name, symbol: _, interval: _ } => (*name).to_string(),
            VelocityUnit::Quotient { numerator, denominator } =>
                format!("{} per {}", numerator.get_name(), denominator.get_name()),
        }
    }

    fn get_symbol(&self) ->  String {
        match self {
            VelocityUnit::Simple { name: _, symbol, interval: _ } => (*symbol).to_string(),
            VelocityUnit::Quotient { numerator, denominator } => 
                format!("{}/{}", numerator.get_symbol(), denominator.get_symbol()),
        }
    }

    fn get_interval(&self) -> f64 {
        match self {
            VelocityUnit::Simple { name:_, symbol:_, interval } => *interval,
            VelocityUnit::Quotient { numerator, denominator } => 
                numerator.get_interval() / denominator.get_interval(),
        }
    }
}

impl<'a> Div<&'a TimeUnit<'a>> for &'a LengthUnit<'a>{

    type Output = VelocityUnit<'a>;

    fn div(self, rhs: &'a TimeUnit<'a>) -> Self::Output {
        VelocityUnit::Quotient { numerator: self, denominator: rhs }
    }
}

#[allow(non_upper_case_globals)]
pub const c: &VelocityUnit = &VelocityUnit::Simple{ name: "speed of light", symbol: "c", interval: SPEED_OF_LIGHT };



//********** QUANTITY **********
pub struct Velocity<'a>{
    value: f64,
    unit: UnitContainer<'a, VelocityUnit<'a>>
}

impl<'a> Velocity<'a>{

    pub fn value_in<U: AsRef<VelocityUnit<'a>>>(&self, unit: U) -> f64 {
        return self.value * self.unit.get().get_interval() / unit.as_ref().get_interval();
    }
}

impl<'a> AsRef<VelocityUnit<'a>> for VelocityUnit<'a>{
    fn as_ref(&self) -> &VelocityUnit<'a> { self }
}

impl<'a> Into<Velocity<'a>> for (f64, &'a VelocityUnit<'a>){

    fn into(self) -> Velocity<'a>{
        Velocity{ value: self.0, unit: UnitContainer::Ref { unit: self.1 } }
    }
}

impl<'a> Into<Velocity<'a>> for (f64, VelocityUnit<'a>){

    fn into(self) -> Velocity<'a>{
        Velocity{ value: self.0, unit: UnitContainer::Val { unit: self.1 } }
    }
}

impl<'a> Div<&'a Time<'a>> for &'a Length<'a>{

    type Output = Velocity<'a>;

    fn div(self, rhs: &'a Time<'a>) -> Self::Output {
        Velocity{ 
            value: self.get_value() / rhs.get_value(),
            unit: UnitContainer::Val { unit: self.get_unit() / rhs.get_unit() }
        }
    }
}

pub fn velocity<'a>(value: f64, unit: &'a VelocityUnit<'a>) -> Velocity {
    (value, unit).into()
}


#[cfg(test)]
use crate::test_util::f_eq;

#[test]
fn test_unit_conversion(){
    let v3km_h: Velocity = (3., km/h).into();
    // let three_kmh = (km/h).of(3.);

    assert!(f_eq(v3km_h.value_in(km/h), 3.));
    assert!(f_eq(v3km_h.value_in(m/s), 3000. / 3600.));
    assert!(f_eq(v3km_h.value_in(c), 3000. / 3600. / SPEED_OF_LIGHT));

    let half_c: Velocity = (0.5, c).into();
    assert!(f_eq(half_c.value_in(m/s), SPEED_OF_LIGHT / 2.))
}

#[test]
fn test_quantity_division(){
    let x3km: Length = (3., km).into();
    let t2h: Time = (2., h).into();

    let v: Velocity = &x3km / &t2h;

    assert!(f_eq(v.value_in(km/h), 1.5));
    assert!(f_eq(v.value_in(m/s), 1.5 * 1000. / 3600.));
}