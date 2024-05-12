use crate::*;

//********** UNIT **********
pub enum TimeUnit<'a>{
    Simple{ _name: &'a str, _symbol: &'a str, _interval: f64 }
}

impl<'a> LinearUnit<'a> for TimeUnit<'a>{
    fn name(&self) ->  String {
        match self {
            TimeUnit::Simple { _name, _symbol, _interval } => (*_name).to_string(),
        }
    }

    fn symbol(&self) ->  String {
        match self {
            TimeUnit::Simple { _name, _symbol, _interval } => (*_symbol).to_string(),
        }
    }

    fn interval(&self) -> f64 {
        match self {
            TimeUnit::Simple { _name, _symbol, _interval } => *_interval,
        }
    }
}

#[allow(non_upper_case_globals)]
pub const s: &TimeUnit = &TimeUnit::Simple{ _name: "second", _symbol: "s", _interval: 1. };

#[allow(non_upper_case_globals)]
pub const ms: &TimeUnit = &TimeUnit::Simple{ _name: "millisecond", _symbol: "ms", _interval: 0.001 };

#[allow(non_upper_case_globals)]
pub const h: &TimeUnit = &TimeUnit::Simple{ _name: "hour", _symbol: "h", _interval: 3600. };



//********** QUANTITY **********
pub struct Time<'a>{
    _value: f64,
    _unit: &'a TimeUnit<'a>
}

impl<'a> Time<'a>{

    pub fn value(&self) -> f64 { return self._value; }
    
    pub fn unit(&self) -> &'a TimeUnit<'a> { return self._unit; }

    pub fn value_in<U: AsRef<TimeUnit<'a>>>(&self, unit: U) -> f64 {
        return self.value() * self.unit().interval() / unit.as_ref().interval();
    }
}

impl<'a> AsRef<TimeUnit<'a>> for TimeUnit<'a>{
    fn as_ref(&self) -> &TimeUnit<'a> {
        return self;
    }
}

impl<'a> Into<Time<'a>> for (f64, &'a TimeUnit<'a>){

    fn into(self) -> Time<'a>{
        return Time { _value: self.0, _unit: self.1 };
    }
}


#[cfg(test)]
use crate::test_util::f_eq;

#[test]
fn test_unit_conversion(){
    let three_ms: Time = (3., ms).into();
    // let three_ms = ms.of(3.);

    assert!(f_eq(three_ms.value_in(ms), 3.));
    assert!(f_eq(three_ms.value_in(s), 0.003));
    assert!(f_eq(three_ms.value_in(h), 0.003/3600.));
}