use crate::*;

//********** UNIT **********
pub enum LengthUnit<'a>{
    Simple{ name: &'a str, symbol: &'a str, interval: f64 }
}

impl<'a> LinearUnit<'a> for LengthUnit<'a>{
    fn get_name(&self) -> String {
        match self {
            LengthUnit::Simple { name, symbol:_, interval:_ } => (*name).to_string(),
        }
    }

    fn get_symbol(&self) -> String {
        match self {
            LengthUnit::Simple { name:_, symbol, interval:_ } => (*symbol).to_string(),
        }
    }

    fn get_interval(&self) -> f64 {
        match self {
            LengthUnit::Simple { name:_, symbol:_, interval } => *interval,
        }
    }
}

#[allow(non_upper_case_globals)]
pub const m: &LengthUnit = &LengthUnit::Simple{ name: "metre", symbol: "m", interval: 1. };

#[allow(non_upper_case_globals)]
pub const cm: &LengthUnit = &LengthUnit::Simple{ name: "centimetre", symbol: "cm", interval: 0.01 };
#[allow(non_upper_case_globals)]
pub const mm: &LengthUnit = &LengthUnit::Simple{ name: "millimetre", symbol: "mm", interval: 0.001 };

#[allow(non_upper_case_globals)]
pub const km: &LengthUnit = &LengthUnit::Simple{ name: "kilometre", symbol: "km", interval: 1000. };



//********** QUANTITY **********
pub struct Length<'a>{
    value: f64,
    unit: UnitContainer<'a, LengthUnit<'a>>
}

impl<'a> Length<'a>{

    pub fn get_value_in<U: AsRef<LengthUnit<'a>>>(&self, unit: U) -> f64 {
        self.value * self.unit.get().get_interval() / unit.as_ref().get_interval()
    }
}

impl<'a> AsRef<LengthUnit<'a>> for LengthUnit<'a>{
    fn as_ref(&self) -> &LengthUnit<'a> { self }
}

impl<'a> LinearQuantity<'a, LengthUnit<'a>> for Length<'a>{

    fn get_value(&self) -> f64 { self.value }

    fn get_unit(&'a self) -> &'a LengthUnit<'a> { self.unit.get() }
}

impl<'a> Into<Length<'a>> for (f64, &'a LengthUnit<'a>){

    fn into(self) -> Length<'a>{
        Length { value: self.0, unit: UnitContainer::Ref { unit: self.1 } }
    }
}

#[cfg(test)]
use crate::test_util::f_eq;

#[test]
fn test_unit_conversion(){
    let x3cm: Length = (3., cm).into();

    assert!(f_eq(x3cm.get_value_in(cm), 3.));
    assert!(f_eq(x3cm.get_value_in(m), 0.03));
    assert!(f_eq(x3cm.get_value_in(mm), 30.));
    assert!(f_eq(x3cm.get_value_in(km), 0.00003));
}