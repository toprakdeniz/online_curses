use std::ops::{Add, AddAssign, Neg};

#[derive(Debug, PartialEq)]
struct Complex<T> {
    re: T,
    im: T,
}


impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self {
        Complex::<T> { re, im } // :: prevent the following < character as a less than operator
    }
}


impl<T> Add for Complex<T>
where 
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::<T> {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}


impl<T> Neg for Complex<T>
where 
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Complex::<T> {
            re: -self.re,
            im: -self.im,
        }
    }
}


impl<T> AddAssign for Complex<T>
where 
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}



// pastial eq
// full eq: x = x
// NAN = not a number 0/0 inf/inf
// NAN == NAN is false this violates the full eq



fn main(){
    let a = Complex::new(1, 2);
    let b = Complex::new(3, 4);
    let c = Complex::new(4, 6);
    assert_eq!(a + b, c);

    let mut a = Complex::new(1, 2);
    let b = Complex::new(3, 4);
    a+=b;
    assert_eq!(a, c);

    let a = Complex::new(1, 2);
    let b = Complex::new(-1, -2);
    assert_eq!(-a, b);
}