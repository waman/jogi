pub mod unit;

#[cfg(test)]
pub mod test_util;



pub const SPEED_OF_LIGHT: f64 = 299_792_458.;



pub trait LinearUnit<'a>: Sized{
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn interval(&self) -> f64;
}


pub trait LinearQuantity<'a, U: LinearUnit<'a> + 'a>: Sized{
    
    fn value(&self) -> f64;
    fn unit(&self) -> &'a U;

    // fn value_in<V: AsRef<U>>(&self, unit: V) -> f64 {
    //     return self.value() * self.unit().interval() / unit.as_ref().interval();
    // }
}

pub struct SimpleLinearQuantity<'a, U: LinearUnit<'a> + 'a>{
    _value: f64,
    _unit: &'a U
}

impl<'a, U: LinearUnit<'a>> LinearQuantity<'a, U> for SimpleLinearQuantity<'a, U>{

    fn value(&self) -> f64 { return self._value; }
    
    fn unit(&self) -> &'a U { return self._unit; }
}