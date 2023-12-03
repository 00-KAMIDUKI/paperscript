use crate::{expr::Expr, Value, Type};

pub type BinaryOp = fn(&dyn Expr, &dyn Expr) -> Result<Box<dyn Value>, ()>;

pub fn add(lhs: &dyn Expr, rhs: &dyn Expr) -> Result<Box<dyn Value>, ()> {
    let value1 = lhs.evaluate()?;
    let value2 = rhs.evaluate()?;
    if value1.type_() != value2.type_() { Err(()) } else { 
        match value1.type_() {
            Type::Int64 => Ok(Box::new(value1.as_i64().unwrap() + value2.as_i64().unwrap())),
            Type::Float64 =>  Ok(Box::new(value1.as_f64().unwrap() + value2.as_f64().unwrap())), 
            _ => Err(())
        } 
    }
}

pub fn sub(lhs: &dyn Expr, rhs: &dyn Expr) -> Result<Box<dyn Value>, ()> {
   let value1 = lhs.evaluate()?;
   let value2 = rhs.evaluate()?;
   if value1.type_() != value2.type_() { Err(()) } else { 
       match value1.type_() {
           Type::Int64 => Ok(Box::new(value1.as_i64().unwrap() - value2.as_i64().unwrap())),
           Type::Float64 =>  Ok(Box::new(value1.as_f64().unwrap() - value2.as_f64().unwrap())), 
           _ => Err(())
       } 
   }
}

pub fn mul(lhs: &dyn Expr, rhs: &dyn Expr) -> Result<Box<dyn Value>, ()> {
   let value1 = lhs.evaluate()?;
   let value2 = rhs.evaluate()?;
   if value1.type_() != value2.type_() { Err(()) } else { 
       match value1.type_() {
           Type::Int64 => Ok(Box::new(value1.as_i64().unwrap() * value2.as_i64().unwrap())),
           Type::Float64 =>  Ok(Box::new(value1.as_f64().unwrap() * value2.as_f64().unwrap())), 
           _ => Err(())
       } 
   }
}

pub fn div(lhs: &dyn Expr, rhs: &dyn Expr) -> Result<Box<dyn Value>, ()> {
   let value1 = lhs.evaluate()?;
   let value2 = rhs.evaluate()?;
   if value1.type_() != value2.type_() { Err(()) } else { 
       match value1.type_() {
           Type::Int64 => Ok(Box::new(value1.as_i64().unwrap() / value2.as_i64().unwrap())),
           Type::Float64 =>  Ok(Box::new(value1.as_f64().unwrap() / value2.as_f64().unwrap())), 
           _ => Err(())
       } 
   }
}

pub fn mod_(lhs: &dyn Expr, rhs: &dyn Expr) -> Result<Box<dyn Value>, ()> {
   let value1 = lhs.evaluate()?;
   let value2 = rhs.evaluate()?;
   if value1.type_() != value2.type_() { Err(()) } else { 
       match value1.type_() {
           Type::Int64 => Ok(Box::new(value1.as_i64().unwrap() % value2.as_i64().unwrap())),
           Type::Float64 =>  Ok(Box::new(value1.as_f64().unwrap() % value2.as_f64().unwrap())), 
           _ => Err(())
       } 
   }
}

pub fn gt(lhs: &dyn Expr, rhs: &dyn Expr) -> Result<Box<dyn Value>, ()> {
   let value1 = lhs.evaluate()?;
   let value2 = rhs.evaluate()?;
   if value1.type_() != value2.type_() { Err(()) } else { 
       match value1.type_() {
           Type::Int64 => Ok(Box::new(value1.as_i64().unwrap() > value2.as_i64().unwrap())),
           Type::Float64 =>  Ok(Box::new(value1.as_f64().unwrap() > value2.as_f64().unwrap())), 
           _ => Err(())
       } 
   }
}

pub fn lt(lhs: &dyn Expr, rhs: &dyn Expr) -> Result<Box<dyn Value>, ()> {
   let value1 = lhs.evaluate()?;
   let value2 = rhs.evaluate()?;
   if value1.type_() != value2.type_() { Err(()) } else { 
       match value1.type_() {
           Type::Int64 => Ok(Box::new(value1.as_i64().unwrap() < value2.as_i64().unwrap())),
           Type::Float64 =>  Ok(Box::new(value1.as_f64().unwrap() < value2.as_f64().unwrap())), 
           _ => Err(())
       } 
   }
}

pub fn ge(lhs: &dyn Expr, rhs: &dyn Expr) -> Result<Box<dyn Value>, ()> {
   let value1 = lhs.evaluate()?;
   let value2 = rhs.evaluate()?;
   if value1.type_() != value2.type_() { Err(()) } else { 
       match value1.type_() {
           Type::Int64 => Ok(Box::new(value1.as_i64().unwrap() >= value2.as_i64().unwrap())),
           Type::Float64 =>  Ok(Box::new(value1.as_f64().unwrap() >= value2.as_f64().unwrap())), 
           _ => Err(())
       } 
   }
}

pub fn le(lhs: &dyn Expr, rhs: &dyn Expr) -> Result<Box<dyn Value>, ()> {
   let value1 = lhs.evaluate()?;
   let value2 = rhs.evaluate()?;
   if value1.type_() != value2.type_() { Err(()) } else { 
       match value1.type_() {
           Type::Int64 => Ok(Box::new(value1.as_i64().unwrap() <= value2.as_i64().unwrap())),
           Type::Float64 =>  Ok(Box::new(value1.as_f64().unwrap() <= value2.as_f64().unwrap())), 
           _ => Err(())
       } 
   }
}

pub fn eq(lhs: &dyn Expr, rhs: &dyn Expr) -> Result<Box<dyn Value>, ()> {
    let value1 = lhs.evaluate()?;
    let value2 = rhs.evaluate()?;
    if value1.type_() != value2.type_() { Err(()) } 
    else {
        match value1.type_() {
            Type::Int64 => Ok(Box::new(value1.as_i64().unwrap() == value2.as_i64().unwrap())),
            Type::Float64 => Ok(Box::new(value1.as_f64().unwrap() == value2.as_f64().unwrap())),
            Type::Bool => Ok(Box::new(value1.as_bool().unwrap() == value2.as_bool().unwrap())),
            Type::Function(..) => unimplemented!(),
        }
    }
}

pub fn ne(lhs: &dyn Expr, rhs: &dyn Expr) -> Result<Box<dyn Value>, ()> {
   let value1 = lhs.evaluate()?;
   let value2 = rhs.evaluate()?;
   if value1.type_() != value2.type_() { Err(()) } 
   else {
       match value1.type_() {
           Type::Int64 => Ok(Box::new(value1.as_i64().unwrap() != value2.as_i64().unwrap())),
           Type::Float64 => Ok(Box::new(value1.as_f64().unwrap() != value2.as_f64().unwrap())),
           Type::Bool => Ok(Box::new(value1.as_bool().unwrap() != value2.as_bool().unwrap())),
           Type::Function(..) => unimplemented!(),
       }
   }
}

