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

    // fn value_in<V: AsRef<U>>(&self, unit: V) -> f64 {
    //     return self.value() * self.unit().interval() / unit.as_ref().interval();
    // }
}

pub(crate) enum UnitContainer<'a, U: LinearUnit<'a>>{
    Ref{ unit: &'a U },
    Val{ unit: U }
}

impl<'a, U: LinearUnit<'a>> UnitContainer<'a, U>{

    pub fn get(&'a self) -> &'a U {
        match self {
            UnitContainer::Ref { unit } => *unit,
            UnitContainer::Val { unit } => unit
        }
    }
}