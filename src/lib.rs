pub mod unit;

#[cfg(test)]
pub mod test_util;

pub trait LinearUnit<'a>{
    fn interval(&self) -> f64;
}

pub trait LinearQuantity<'a, U> where U: LinearUnit<'a>{

    fn value(&self) -> f64;
    fn unit(&self) -> &U;

    fn value_in(&self, unit: U) -> f64 {
        return self.value() * self.unit().interval() / unit.interval();
    }
}