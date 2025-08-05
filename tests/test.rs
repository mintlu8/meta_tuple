use core::fmt::Display;

use meta_tuple::{meta_tuple, meta_tuple_type, IntoMetaTuple, MetaBox, MetaItem, MetaTuple};

#[derive(Debug, PartialEq, Eq, MetaItem)]
struct MyType;

#[derive(Debug, PartialEq, Eq, MetaItem)]
struct MyTypeGeneric<T>(T);

#[derive(MetaItem)]
struct MyTypeGeneric2<T: Display>(T);

#[derive(MetaItem)]
struct MyTypeGeneric3<T: Display, const N: usize>(T);

#[derive(MetaItem)]
struct MyTypeGeneric4<T: Display, const N: usize>(T);

#[test]
pub fn test() {
    let _: () = meta_tuple!();
    let mut a = meta_tuple!(1i32);
    assert_eq!(a.get::<i32>(), Some(&1));
    assert_eq!(a.get_mut::<i32>(), Some(&mut 1));
    assert_eq!(a.get::<u32>(), None);

    let b = meta_tuple!(1i32, "hello", vec![1, 2, 3]);
    assert_eq!(b.get::<i32>(), Some(&1));
    assert_eq!(b.get::<&str>(), Some(&"hello"));
    assert_eq!(b.get::<Vec<i32>>(), Some(&vec![1, 2, 3]));
    assert_eq!(b.get::<u32>(), None);

    let s = String::from("hello");
    let mut v = vec![1, 2, 3];
    let mut c = meta_tuple!(1i32, &s, &mut v);
    assert_eq!(c.get::<i32>(), Some(&1));
    assert_eq!(c.get::<String>(), Some(&s));
    assert_eq!(c.get_mut::<String>(), None);
    assert_eq!(c.get::<Vec<i32>>(), Some(&vec![1, 2, 3]));
    assert_eq!(c.get_mut::<Vec<i32>>(), Some(&mut vec![1, 2, 3]));
    assert_eq!(c.get::<&str>(), None);

    let d = meta_tuple!(4.0f32, vec![1,2], #MyTypeGeneric("Hi"), #&MyType, #c);
    assert_eq!(d.get::<i32>(), Some(&1));
    assert_eq!(d.get::<f32>(), Some(&4.0));
    assert_eq!(d.get::<String>(), Some(&s));
    assert_eq!(d.get::<MyTypeGeneric<&str>>(), Some(&MyTypeGeneric("Hi")));
    assert_eq!(d.get::<MyType>(), Some(&MyType));
    assert_eq!(d.get::<Vec<i32>>(), Some(&vec![1, 2]));

    let e: Box<dyn MetaBox> = Box::new(d);
    assert_eq!(e.get::<i32>(), Some(&1));
    assert_eq!(e.get::<f32>(), Some(&4.0));
    assert_eq!(e.get::<String>(), Some(&s));
    assert_eq!(e.get::<MyTypeGeneric<&str>>(), Some(&MyTypeGeneric("Hi")));
    assert_eq!(e.get::<MyType>(), Some(&MyType));
    assert_eq!(e.get::<Vec<i32>>(), Some(&vec![1, 2]));

    let _t0: meta_tuple_type!(i32) = meta_tuple!(1i32);
    let _t1: meta_tuple_type!(i32, f32, u32) = meta_tuple!(1i32, 1f32, 1u32);
    let _t2: meta_tuple_type!(&i32) = meta_tuple!(&1i32);
    let _t3: meta_tuple_type!(&mut i32) = meta_tuple!(&mut 1i32);
    let _t4: meta_tuple_type!(i32, &u32) = meta_tuple!(1i32, &2u32);

    let f = (1, 4.5f32, "hi").into_meta_tuple();
    assert_eq!(f.get::<i32>(), Some(&1));
    assert_eq!(f.get::<f32>(), Some(&4.5));
    assert_eq!(f.get::<&str>(), Some(&"hi"));
}
