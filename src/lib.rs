pub mod unit;

#[cfg(test)]
pub mod test_util;

pub trait LinearUnit<'a>{
    fn name(&self) -> &'a str;
    fn symbol(&self) -> &'a str;
    fn interval(&self) -> f64;
}

pub struct SimpleLinearUnit<'a> {
    _name: &'a str,
    _symbol: &'a str,
    _interval: f64
}

impl<'a> LinearUnit<'a> for SimpleLinearUnit<'a>{
    
    fn name(&self) -> &'a str { return self._name;}
    fn symbol(&self) -> &'a str { return self._symbol; }
    fn interval(&self) -> f64 { return self._interval; }
}

pub trait LinearQuantity<'a, U: LinearUnit<'a> + 'a + ?Sized>{
    
    fn value(&self) -> f64;
    fn unit(&self) -> &'a U;

    fn value_in(&self, unit: &'_ U) -> f64 {
        return self.value() * self.unit().interval() / unit.interval();
    }
}

pub struct SimpleLinearQuantity<'a, U: LinearUnit<'a> + ?Sized>{
    _value: f64,
    _unit: &'a U
}

impl<'a, U: LinearUnit<'a> + ?Sized> LinearQuantity<'a, U> for SimpleLinearQuantity<'a, U>{

    fn value(&self) -> f64 { return self._value; }
    
    fn unit(&self) -> &'a U { return self._unit; }
}