use std::ops::Add; 
use std::ops::Sub; 
use std::ops::Mul; 
use std::ops::Div; 
use std::marker::Copy;
use std::cmp::Eq;
use std::cmp::PartialEq;

pub use num::{One, one};

#[derive(Debug)]
struct Rational<T> {
  nominator : T,
  denominator: T
}

impl<T> Rational<T> {
  
  fn new(nominator:T,denominator:T) -> Self{
    Rational{
      nominator : nominator,
      denominator: denominator
    }
  }

}


impl <T : One + Div<Output=T>> From<T> for Rational<T> {
  fn from(value : T) -> Rational<T>{
    Rational{ nominator : value, denominator : one() }
  }
}

impl <T : Mul<Output=T> + PartialEq + Copy> PartialEq for Rational<T> {
  fn eq(&self,other: &Rational<T>) -> bool {
    self.nominator * other.denominator == self.denominator * other.nominator
  }
}

impl<T : Eq + Mul<Output=T> + Copy> Eq for Rational<T> {}

impl<A : Mul<Output=B>,B> Mul for Rational<A> {
  type Output = Rational<B>;

  fn mul(self,other: Rational<A>) -> Self::Output {
    Rational{
      nominator: self.nominator * other.nominator,
      denominator: self.denominator * other.denominator
    }
  }
}

impl<A : Mul<Output=B>,B> Div for Rational<A> {
  type Output = Rational<B>;

  fn div(self,other: Rational<A>) -> Self::Output {
    Rational{
      nominator: self.nominator * other.denominator,
      denominator: self.denominator * other.nominator
    }
  }
}

impl<T: Add<Output=T> + Mul<Output=T> + Copy> Add for Rational<T> {
  type Output = Rational<T>;

  fn add(self, other : Rational<T>) -> Self::Output{
    let new_denominator = self.denominator * other.denominator;
    Rational{
      nominator: self.nominator * other.denominator + other.nominator * self.denominator,
      denominator: new_denominator
    }
  }
}

impl<T : Sub<Output=T> + Mul<Output=T> + Copy> Sub for Rational<T> {
  type Output = Rational<T>;

  fn sub(self,other: Rational<T>) -> Self::Output{
    let new_denominator = self.denominator * other.denominator;
    Rational{
      nominator: self.nominator * other.denominator - other.nominator * self.denominator,
      denominator: new_denominator
    }
  }
}


#[cfg(test)]
mod tests{

  use rational::Rational;


  #[test]
  fn from() {
    assert_eq!(Rational::new(2,2),Rational::from(1));
    assert_eq!(Rational::new(2.0,2.0),Rational::from(1.0));
  }

  #[test]
  fn add() {
   assert_eq!(Rational::new(2,1),Rational::new(1,1) + Rational::new(2,2));
   assert_eq!(Rational::new(4,2),Rational::new(1,1) + Rational::new(2,2));
   assert_eq!(Rational::new(3,1),Rational::new(4,2) + Rational::new(1,1));
  }

  #[test]
  fn sub() {
   assert_eq!(Rational::new(0,1),Rational::new(1,1) - Rational::new(2,2));
   assert_eq!(Rational::new(1,1),Rational::new(4,2) - Rational::new(1,1));
  }

  #[test]
  fn mul(){
    assert_eq!(Rational::new(1,1),Rational::new(1,1) * Rational::new(2,2));
    assert_eq!(Rational::new(3,1),Rational::new(3,2) * Rational::new(8,4));
  }

  #[test]
  fn div(){
    assert_eq!(Rational::new(2,1),Rational::new(4,1) / Rational::new(2,1));
    assert_eq!(Rational::new(9,1),Rational::new(9,2) / Rational::new(1,2));
  }

}
