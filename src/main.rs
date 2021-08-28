#![feature(generic_associated_types)]
#![allow(incomplete_features)]

// Hack to get around #20041.
// See https://github.com/rust-lang/rust/issues/20041#issuecomment-820911297
// This does not make type equality completely work, but it does constrain the
// two types to be equal.
trait TyEq {}
impl<T> TyEq for (T, T) {}

trait Functorish<A>
where
    // We constrain that Functor<A> and Self must be equal.
    // This way we can actually use functors somewhat ergonomically.
    (Self::Functor<A>, Self): TyEq,
{
    type Functor<T>;
    // I didn't name this map, as to not conflict with e.g. [`Option::map`]
    // Very similar to Haskell's fmap, just switched around, and obviously
    // not curried.
    fn fmap<B>(self, _: impl FnMut(A) -> B) -> Self::Functor<B>;
}

impl<A> Functorish<A> for Vec<A> {
    type Functor<T> = Vec<T>;
    fn fmap<B>(self: Self::Functor<A>, f: impl FnMut(A) -> B) -> Self::Functor<B> {
        self.into_iter().map(f).collect()
    }
}

impl<A> Functorish<A> for Option<A> {
    type Functor<T> = Option<T>;
    fn fmap<B>(self: Self::Functor<A>, f: impl FnMut(A) -> B) -> Self::Functor<B> {
        self.map(f)
    }
}

fn main() {
    println!("It works with lists!\n");
    let v1 = vec![1, 2, 3];
    let v2 = v1.clone().fmap(|x| x * 2);
    println!("{:?}.fmap(|x| x * 2) => {:?}\n", v1, v2);
    assert_eq!(v2, [2, 4, 6]);

    println!("It even works with different types in lists!\n");
    let v3 = vec!["4", "5", "6"];
    let v4 = v3.clone().fmap(|x| x.parse::<i32>().unwrap());
    println!("{:?}.fmap(|x| x.parse::<i32>().unwrap()) => {:?}\n", v3, v4);
    assert_eq!(v4, vec![4, 5, 6]);

    println!("Even Options!\n");
    let o1: Option<i32> = None;
    let o2 = o1.fmap(|x| x.to_string());
    println!("{:?}.fmap(|x| x.to_string()) => {:?}\n", o1, o2);
    assert_eq!(o2, None);

    let o3 = Some(1);
    let o4 = o3.fmap(|x| x + 10);
    println!("{:?}.fmap(|x| x + 10) => {:?}\n", o3, o4);
    assert_eq!(o4, Some(11));

    println!("Buy a ~~Functor~~ Funkter today at your local Rust emporium!");
}
