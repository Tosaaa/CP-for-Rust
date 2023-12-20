use std::ops::{Deref, DerefMut};

struct ValueHolder<T> {
    value: T
}

impl<T> Deref for ValueHolder<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

struct Selector<T> {
    elements: Vec<T>,
    current: usize
}

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current] 
        // 여기서 deref 강제 변환이 어떻게 됨? 재귀적이지 않나?
        // (*self).current에서 self.current로 어떤 과정으로 넘어감?
        // https://doc.rust-lang.org/nomicon/dot-operator.html
        // 미쳤다.. 궁금증 해결
        // deref 강제 변환이 아니라 러스트 자체의 dot operator 기능에 의해 암묵적으로 변환된 것
    }
}


impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}


fn main() {
    // 명시적으로 deref 함수를 호출하는 경우에는 그대로 레퍼런스를 반환하지만,
    // *s처럼 * 연산자를 이용하면 암묵적으로 반환된 레퍼런스를 한번 더 역참조함 (syntactic sugar)
    // https://stackoverflow.com/a/31627018

    // let s: String = "Hello".to_string();
    // let _: () = *s;
    // let _: () = s.deref();
    // let _: () = Deref::deref(&s);
    // let _: () = <String as Deref>::deref(&s);
    

    // let x = ValueHolder { value: 10 };
    // let _: () = *x;
    // let _: () = x.deref();
    // let _: () = Deref::deref(&x);

    let s = Selector { elements: vec!["good", "bad", "ugly"], current: 2 };

    fn show_it(thing: &str) { println!("{}", thing); }
    show_it(&s);

    use std::fmt::Display;
    fn show_it_generic<T: Display>(thing: T) { println!("{}", thing); }
    // show_it_generic(&s);
    show_it_generic(&s as &str);
    show_it_generic(&*s);
}