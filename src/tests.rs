use crate as dynamic;
use dynamic::*;

#[derive(TypeInfo, Debug, Clone, PartialEq, Eq)]
struct A(String);

#[derive(TypeInfo, Debug, Clone, PartialEq, Eq)]
struct B;

#[derive(TypeInfo, Debug, Clone, PartialEq, Eq)]
struct C<T>(T);

#[derive(TypeInfo, Debug, Clone, PartialEq, Eq)]
struct X;

#[derive(TypeInfo, Debug, Clone, PartialEq, Eq)]
struct Y;

#[test]
fn test_cast() {
    let a = A("Hi".to_string());
    let dynamic = Dynamic::new(a.clone());
    assert_eq!(dynamic.cast::<A>(), &a);
    assert!(dynamic.try_cast::<B>().is_none());
}

#[test]
fn test_into_cast() {
    let a = A("Hi".to_string());
    let dynamic = Dynamic::new(a.clone());
    assert_eq!(dynamic.into_cast::<A>(), a);
}

#[test]
fn test_cast_mut() {
    let a = A("Hi".to_string());
    let mut dynamic = Dynamic::new(a);
    dynamic.cast_mut::<A>().0 = "Hello".to_string();
    assert_eq!(dynamic.into_cast::<A>(), A("Hello".to_string()));
}

#[test]
fn test_clone() {
    let a = A("Hi".to_string());
    let dynamic1 = Dynamic::new(a);
    let dynamic2 = dynamic1.clone();
    assert_eq!(dynamic1.into_cast::<A>(), dynamic2.into_cast::<A>());
}

#[test]
fn test_cast_generic() {
    let cx = C(X);
    let cy = C(Y);
    let dynamic_cx = Dynamic::new(cx.clone());
    let dynamic_cy = Dynamic::new(cy.clone());
    assert_eq!(dynamic_cx.try_cast::<C<X>>(), Some(&cx));
    assert_eq!(dynamic_cy.try_cast::<C<Y>>(), Some(&cy));
    assert_eq!(dynamic_cx.try_cast::<C<Y>>(), None);
    assert_eq!(dynamic_cy.try_cast::<C<X>>(), None);
}
