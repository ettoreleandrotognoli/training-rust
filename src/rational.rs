use std::ops::Add; 
use std::ops::Sub; 
use std::ops::Mul; 
use std::ops::Div; 
use std::marker::Copy;
use std::cmp::Eq;
use std::cmp::PartialEq;

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

impl <T : Mul<Output=T> + PartialEq + Clone> PartialEq for Rational<T> {
  fn eq(&self,other: &Rational<T>) -> bool {
    self.nominator.clone() * other.denominator.clone() == self.denominator.clone() * other.nominator.clone()
  }
}

impl<T : Eq + Mul<Output=T> + Clone> Eq for Rational<T> {}

impl<T : Mul<Output=T>> Mul for Rational<T> {
  type Output = Rational<T>;

  fn mul(self,other: Rational<T>) -> Self {
    Rational{
      nominator: self.nominator * other.nominator,
      denominator: self.denominator * other.denominator
    }
  }
}

impl<T : Mul<Output=T>> Div for Rational<T> {
  type Output = Rational<T>;

  fn div(self,other: Rational<T>) -> Self {
    Rational{
      nominator: self.nominator * other.denominator,
      denominator: self.denominator * other.nominator
    }
  }
}

impl<T: Add<Output=T> + Mul<Output=T> + Copy> Add for Rational<T> {
  type Output = Rational<T>;

  fn add(self, other : Rational<T>) -> Self{
    let new_denominator = self.denominator * other.denominator;
    Rational{
      nominator: self.nominator * other.denominator + other.nominator * self.denominator,
      denominator: new_denominator
    }
  }
}

impl<T : Sub<Output=T> + Mul<Output=T> + Copy> Sub for Rational<T> {
  type Output = Rational<T>;

  fn sub(self,other: Rational<T>) -> Self{
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
