use crate::*;

//********** UNIT **********
pub enum TimeUnit<'a>{
    Simple{ name: &'a str, symbol: &'a str, interval: f64 }
}

impl<'a> LinearUnit<'a> for TimeUnit<'a>{
    fn get_name(&self) ->  String {
        match self {
            TimeUnit::Simple { name, symbol:_, interval:_ } => (*name).to_string(),
        }
    }

    fn get_symbol(&self) ->  String {
        match self {
            TimeUnit::Simple { name:_, symbol, interval:_ } => (*symbol).to_string(),
        }
    }

    fn get_interval(&self) -> f64 {
        match self {
            TimeUnit::Simple { name:_, symbol:_, interval } => *interval,
        }
    }
}

#[allow(non_upper_case_globals)]
pub const s: &TimeUnit = &TimeUnit::Simple{ name: "second", symbol: "s", interval: 1. };

#[allow(non_upper_case_globals)]
pub const ms: &TimeUnit = &TimeUnit::Simple{ name: "millisecond", symbol: "ms", interval: 0.001 };

#[allow(non_upper_case_globals)]
pub const h: &TimeUnit = &TimeUnit::Simple{ name: "hour", symbol: "h", interval: 3600. };



//********** QUANTITY **********
pub struct Time<'a>{
    value: f64,
    unit: UnitContainer<'a, TimeUnit<'a>>
}

impl<'a> Time<'a>{

    pub fn value_in<U: AsRef<TimeUnit<'a>>>(&self, unit: U) -> f64 {
        self.value * self.unit.get().get_interval() / unit.as_ref().get_interval()
    }
}

impl<'a> AsRef<TimeUnit<'a>> for TimeUnit<'a>{
    fn as_ref(&self) -> &TimeUnit<'a> { self }
}

impl<'a> LinearQuantity<'a, TimeUnit<'a>> for Time<'a>{

    fn get_value(&self) -> f64 { self.value }

    fn get_unit(&'a self) -> &'a TimeUnit<'a> { self.unit.get() }
}

impl<'a> Into<Time<'a>> for (f64, &'a TimeUnit<'a>){

    fn into(self) -> Time<'a>{ 
        Time { value: self.0, unit: UnitContainer::Ref { unit: self.1 } }
     }
}


#[cfg(test)]
use crate::test_util::f_eq;

#[test]
fn test_unit_conversion(){
    let t3ms: Time = (3., ms).into();
    // let three_ms = ms.of(3.);

    assert!(f_eq(t3ms.value_in(ms), 3.));
    assert!(f_eq(t3ms.value_in(s), 0.003));
    assert!(f_eq(t3ms.value_in(h), 0.003/3600.));
}