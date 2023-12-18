use std::ops::{Add, Mul, Neg, AddAssign};
use std::cmp::{Ordering, PartialOrd, Eq, PartialEq};

impl<T> Add for Complex<T> 
    where T: Add<Output=T>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

/// 일반성을 최대로 끌어올린 제네릭 구현, 
/// 왼쪽과 오른쪽의 피연산자를 서로 다르게 가져갈 수 있으면서, 
/// 구성 요소 간의 덧셈이 어떤 타입의 값을 만들어 내든지 Complex가 수용 가능
/// 하지만 L이 Add<R>을 구현해야 한다는 점에서 보통 L과 R은 같은 타입이 될 가능성이 높음.
// impl<L: Add<R>, R> Add<Complex<R>> for Complex<L> {
//     type Output = Complex<L::Output>; // L::Output?? 이해가 잘 안됨
//     fn add(self, rhs: Complex<R>) -> Self::Output {
//         Complex {
//             re: self.re + rhs.re,
//             im: self.im + rhs.im
//         }
//     }
// }

impl<T> Mul for Complex<T> 
    where T: Mul<Output=T>
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Complex {
            re: self.re * rhs.re,
            im: self.im * rhs.im
        }
    }
}

impl<T> Neg for Complex<T> 
    where T: Neg<Output=T>
{
    type Output = Self;
    fn neg(self) -> Self {
        Complex {
            re: -self.re,
            im: -self.im
        }
    }
}

/// PartialEq는 간단해서 굳이 수작업으로 구현하지 않고 Complex<T> 타입 정의의
/// derive 어트리뷰트에 PartialEq를 추가하기만 하면 됨.
impl<T> PartialEq for Complex<T> 
    where T: PartialEq
{
    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

/// Eq도 derive 어트리뷰트에 Eq를 추가하기만 하면 됨.
impl<T: Eq> Eq for Complex<T> {}

impl<T> AddAssign for Complex<T>
    where T: AddAssign
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

// #[derive(Copy, Clone, Debug, Eq, PartialEq)] << 이렇게 하면 Eq, PartialEq 자동으로 구현해 줌
#[derive(Copy, Clone, Debug)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> PartialOrd for Interval<T>
    where T: PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.upper <= other.lower {
            Some(Ordering::Less)
        } else if self.lower >= other.upper {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

// ParitalOrd를 위한 반개구간 구현
#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T, // 포함
    upper: T, // 미포함
}

// Index, IndexMut를 위한 이미지 구현
struct Image<P> {
    width: usize,
    pixels: Vec<P>
}

impl<P: Default + Copy> Image<P> {
    fn new(width: usize, height: usize) -> Image<P> {
        Image {
            width,
            pixels: vec![P::default(); width * height]
        }
    }
}

impl<P> std::ops::Index<usize> for Image<P> {
    type Output = [P];
    fn index(&self, row: usize) -> &[P] {
        let start = row * self.width;
        &self.pixels[start..start+self.width]
    }
}

impl<P> std::ops::IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row: usize) -> &mut [P] {
        let start = row * self.width;
        &mut self.pixels[start..start+self.width]
    }
}

fn main() {
    let mut x = Complex {re: 1.0, im: 2.0};
    let y = Complex {re: 1.5, im: 2.5};

    assert_eq!(x + y, Complex {re: 2.5, im: 4.5});
    assert_eq!(x * y, Complex {re: 1.5, im: 5.0});
    assert_eq!(-x, Complex {re: -1.0, im: -2.0});
    x += Complex {re: 1.0, im: 2.0};
    assert_eq!(x, Complex {re: 2.0, im: 4.0});
////////////////////////////////////////////////////
    assert!(Interval {lower: 10, upper: 20} < Interval {lower: 20, upper: 40});
    assert!(Interval {lower: 7, upper: 8} >= Interval {lower: 0, upper: 1});
    assert!(Interval {lower: 7, upper: 8} <= Interval {lower: 7, upper: 8});

    let left = Interval {lower: 10, upper: 30};
    let right = Interval {lower: 20, upper: 40};
    assert!(!(left < right));
    assert!(!(left >= right));

}