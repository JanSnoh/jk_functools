

impl<T: Clone> Semigroup for Vec<T>{
    fn add(self, other:Self) -> Self{
        [self, other].concat()
    }
}

///A Semigroup is a Type with an addition Operator
pub trait Semigroup{ fn add(self, other:Self) -> Self; }
///A Function that maps the In type to the Out Type
/// ```
/// let int_identity: Map<i32,i32> = Box::new(|x| x);
/// assert_eq!(5, int_identity(5));
/// ```
pub type Map<In,Out> = Box<dyn Fn(In) -> Out>;
pub type AddFn<In,Out> = fn( Map<In,Out>, Map<In, Out>) -> Map<In, Out>;

/// A way to additively compose(?) two Maps F and G
/// such that
/// 
/// f(x) + g(x) = f+g(x)
/// 
/// 
pub fn add_functions<'a, In: Clone + 'static, Out:Semigroup + 'static>(f: Map<In, Out>, g:Map<In, Out>) -> Map<In, Out> {
    Box::new(move |x: In| (f)(x.clone()).add((g)(x.clone())))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn additive_functions() {
        fn identity<T>(x: T) -> T {x}
        type Endomorphism<T> = Map<T,T>;
        let x: Endomorphism<Vec<i32>> = add_functions(Box::new(identity),Box::new(identity)); 
        assert_eq!(x(vec![1,2,3]), vec![1,2,3, 1,2,3])
    }

}
