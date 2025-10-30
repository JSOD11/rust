pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn largest<T: PartialOrd>(a: &[T]) -> &T {
    let mut largest_num = &a[0];
    for num in a {
        if num > &largest_num {
            largest_num = num;
        }
    }
    largest_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = &[1, 2, 3, 4, 5];
        println!("{:#?}", largest(a));
    }
}
