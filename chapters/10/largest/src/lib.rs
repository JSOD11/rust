use std::fmt::Debug;

pub fn largest<T>(a: &[T]) -> &T
where
    T: PartialOrd + Debug,
{
    let mut largest_num = &a[0];
    for num in a {
        if num > &largest_num {
            largest_num = num;
        }
    }
    println!("{:#?}", largest_num);
    largest_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(5, *largest(&[1, 2, 3, 4, 5]));
        assert_eq!(20f64, *largest(&[1f64, 20f64, 3f64, 4f64, 5f64]));
        assert_eq!('z', *largest(&['a', 'f', 'z', 'g', 'y']));
    }
}
