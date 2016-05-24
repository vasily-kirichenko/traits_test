trait Monoid {
    fn zero() -> Self;
    fn append(&self, other: &Self) -> Self;
}

impl Monoid for i32 {
    fn zero() -> i32 {
        0_i32
    }

    fn append(&self, other: &i32) -> i32 {
        self + other
    }
}

impl<T: Clone> Monoid for Vec<T> {
    fn zero() -> Vec<T> {
        vec![]
    }
    
    fn append(&self, other: &Vec<T>) -> Vec<T> {
        let mut res = self.clone();
        res.extend_from_slice(other.as_slice());
        res
    }
}

fn foo<T: Monoid>(xs: &[T]) -> T {
    let mut res: T = Monoid::zero();
    for x in xs {
        res = res.append(x);
    }
    res
}

fn main() {
    let i32arr = &[1, 2, 3];
    println!("foo({:?}) = {}", i32arr, foo(i32arr));
    
    let vecs = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("foo({:?}) = {:?}", vecs, foo(vecs))
}
