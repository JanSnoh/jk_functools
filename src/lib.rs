use std::ops::Deref;

impl<T: Clone> Semigroup for Vec<T> {
    fn add(self, other: Self) -> Self {
        [self, other].concat()
    }
}

///A Semigroup is a Type with an addition Operator
pub trait Semigroup {
    fn add(self, other: Self) -> Self;
}
///A Function that maps the In type to the Out Type
/// ```
/// let int_identity: Map<i32,i32> = Box::new(|x| x);
/// assert_eq!(5, int_identity(5));
/// ```
pub type Map<In, Out> = Box<dyn Fn(In) -> Out>;

pub struct SavedMap<In, Out>(Map<In, Out>);
impl<In, Out> Deref for SavedMap<In, Out>{
    type Target = Map<In,Out>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub type AddFn<In, Out> = fn(Map<In, Out>, Map<In, Out>) -> Map<In, Out>;

/// A way to additively compose(?) two Maps F and G
/// such that
///
/// f(x) + g(x) = f+g(x)
///
///
pub fn add_functions<'a, In: Clone + 'static, Out: Semigroup + 'static>(
    f: Map<In, Out>,
    g: Map<In, Out>,
) -> Map<In, Out> {
    Box::new(move |x: In| (f)(x.clone()).add((g)(x.clone())))
}

impl<'a, T: Semigroup + Clone + 'static> std::ops::Add for SavedMap<T, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        SavedMap(add_functions::<T,T>(self.0, rhs.0))
    }
}
mod GenericMapEach {
    ///This module is useless.
    ///I did this because I got lost in the generics and wanted to see how far I could push it
    ///The "goal" was to create a function .map_each() that is implemented for every Iterable container:
    /// e.g. [T], [T;n], Vec<T> etc.
    /// which maps each Element using a Map f (T->U) provided as an argument
    /// and returns a [U], [U;n], Vec<U> etc.
    /// this is not that much more useful than just calling .intoIter().map(f).collect() manually.
    /// But yknow..
    use super::Map;

    trait MappableSolution<T>: IntoIterator<Item = T> {
        fn map<U, F, O>(self, f: F) -> O
        where
            O: FromIterator<U>,
            F: FnMut(T) -> U;
    }
    impl<I, T> MappableSolution<T> for I
    where
        I: IntoIterator<Item = T>,
    {
        fn map<U, F, O>(self, f: F) -> O
        where
            O: FromIterator<U>,
            F: FnMut(T) -> U,
        {
            Iterator::map(self.into_iter(), f).collect()
        }
    }
    


    pub(crate) trait MappableToSelf<T>{
        fn map_each(self, x: Map<T,T>) -> Self;
    }

    pub(crate) trait Mappable2<In, Out>{
        fn map_each(self, x: Map<In, Out>) -> Vec<Out>;
    }

    pub(crate) trait MappableContainerFamily {
        type In<T>: IntoIterator<Item=T>;
        type Out<U> : FromIterator<U>;

        //fn map_each(self, x: Map<Self::In<T>::T, Self::U>) -> Self::Out::<U>;
    }

//There doesn't seem to be a way to make this work yet.

    impl<T, Container, U> Mappable2<T, U> for Container
        where Container: IntoIterator<Item=T> + FromIterator<U>{
        fn map_each<>(self, f: Box<dyn Fn(T) -> U>) -> Vec<U> {
            self.into_iter()
            .map(|x| f(x))
            .collect()
        }
    }
}


///mostly for tests
pub(crate) mod concretes{
    pub fn identity<T>(x: T) -> T {
        x
    }
    pub fn double<G: std::ops::Mul<i32, Output = G>>(x: G) -> G {
        x*2
    }
    pub fn negate_unsigned(unsigned:u32) -> i64{
        -(unsigned as i64)
    }

} 

#[cfg(test)]
mod tests {
    use super::*;
    use super::concretes::*;

    #[test]
    fn additive_functions() {
        type Endomorphism<T> = Map<T, T>;
        let x: Endomorphism<Vec<i32>> = add_functions(Box::new(identity), Box::new(identity));
        assert_eq!(x(vec![1, 2, 3]), vec![1, 2, 3, 1, 2, 3])
    }

    #[test]
    fn additive_functions2() {

        assert_eq!(10, double(5));

        let iden_map :Map<i32,i32> = Box::new(identity);
        let double_map :Map<i32,i32> = Box::new(double);
        //let x = SavedMap(iden_map) + SavedMap(double_map);
        assert_eq!((vec![1, 2, 3].map_each(double_map)), vec![2, 4, 6]);
    }


    #[test]
    fn cross_map(){
        let neg_map:Map<u32,i64> = Box::new(negate_unsigned);
        assert_eq!(vec![1,2,3].map_each(Box::new(negate_unsigned)), vec![-1,-2,-3]);
        assert_eq!([1,2,3].map_each(Box::new(negate_unsigned)), vec![-1,-2,-3]);
    }
}
