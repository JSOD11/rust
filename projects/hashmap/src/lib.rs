const STARTING_BUCKETS: usize = 5;

#[derive(Debug)]
struct HashMap {
    buckets: Vec<Vec<Node>>,
    num_values: u64,
}

#[derive(Debug)]
struct Node {
    key: u64,
    val: u64,
}

impl HashMap {
    pub fn new() -> Self {
        let mut hm = HashMap {
            buckets: Vec::new(),
            num_values: 0,
        };

        for _ in 0..STARTING_BUCKETS {
            let v = Vec::new();
            hm.buckets.push(v);
        }

        hm
    }

    pub fn put(&mut self, key: u64, val: u64) {
        let bucket_index = key % self.buckets.len() as u64;
        let mut bucket: &mut Vec<Node> = self
            .buckets
            .get(bucket_index as usize)
            .expect("bucket must not be empty");
        bucket.push(Node { key, val });
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_basic() {
        let mut hm = HashMap::new();
        println!("{:#?}", hm);
        println!("{:#?}, {:#?}", hm.buckets.len(), hm.buckets.capacity());

        hm.put(5, 5);
    }
}
