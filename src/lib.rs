pub mod unit;

#[cfg(test)]
pub mod test_util;



pub const SPEED_OF_LIGHT: f64 = 299_792_458.;



pub trait LinearUnit<'a>{
    fn get_name(&self) -> String;
    fn get_symbol(&self) -> String;
    fn get_interval(&self) -> f64;
}


pub trait LinearQuantity<'a, U: LinearUnit<'a> + 'a>{
    
    fn get_value(&self) -> f64;
    fn get_unit(&'a self) -> &'a U;

    fn get_value_in<V: AsRef<U>>(&'a self, unit: V) -> f64 {
        self.get_value() * self.get_unit().get_interval() / unit.as_ref().get_interval()
    }
}

pub(crate) enum UnitContainer<'a, U: LinearUnit<'a>>{
    Ref{ unit: &'a U },
    Val{ unit: U }
}

impl<'a, U: LinearUnit<'a> + 'a> UnitContainer<'a, U>{

    pub fn get(&'a self) -> &'a U {
        match self {
            UnitContainer::Ref { unit } => *unit,
            UnitContainer::Val { unit } => unit
        }
    }
}