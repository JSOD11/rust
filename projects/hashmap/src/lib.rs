use std::collections::HashSet;

#[derive(Debug)]
struct HashMap {
    buckets: Vec<Vec<Node>>,
    present_keys: std::collections::HashSet<u64>,
}

impl Node {
    pub fn new(key: u64, val: u64) -> Self {
        Self { key, val }
    }
}

#[derive(Debug)]
struct Node {
    key: u64,
    val: u64,
}

#[allow(dead_code)]
impl HashMap {
    pub fn new(num_buckets: u64) -> Self {
        let mut hm = HashMap {
            buckets: Vec::new(),
            present_keys: HashSet::new(),
        };
        hm.push_buckets(num_buckets);
        hm
    }

    fn push_buckets(&mut self, n: u64) {
        for _ in 0..n {
            self.buckets.push(Vec::new());
        }
    }

    pub fn num_buckets(&self) -> u64 {
        self.buckets.len() as u64
    }

    pub fn num_present_keys(&self) -> u64 {
        self.present_keys.len() as u64
    }

    pub fn put(&mut self, key: u64, val: u64) {
        let bucket_index: usize = key as usize % self.buckets.len();
        let bucket: &mut Vec<Node> = self
            .buckets
            .get_mut(bucket_index)
            .expect("bucket must not be empty");

        if self.present_keys.contains(&key) {
            for i in 0..bucket.len() {
                let node = bucket.get_mut(i).expect("node must not be empty");
                if node.key == key {
                    node.val = val;
                    return;
                }
            }
        } else {
            bucket.push(Node::new(key, val));
            self.present_keys.insert(key);
        }

        self.double_table();
    }

    pub fn get(&self, key: u64) -> Option<u64> {
        if !self.present_keys.contains(&key) {
            return None;
        }

        let bucket_index: usize = key as usize % self.buckets.len();
        let bucket = self
            .buckets
            .get(bucket_index)
            .expect("bucket must not be empty");
        for i in 0..bucket.len() {
            let node = bucket.get(i).expect("node must not be empty");
            if node.key == key {
                return Some(node.val);
            }
        }
        None
    }

    pub fn remove(&mut self, key: u64) -> Option<u64> {
        let bucket_index: usize = key as usize % self.buckets.len();
        let bucket = self
            .buckets
            .get_mut(bucket_index)
            .expect("bucket must not be empty");
        for i in 0..bucket.len() {
            let node = bucket.get(i).expect("node must not be empty");
            let k = node.key;
            let v = node.val;
            if k == key {
                bucket.swap_remove(i);
                self.present_keys.remove(&k);
                return Some(v);
            }
        }
        None
    }

    pub fn double_table(&mut self) {
        if self.num_present_keys() >= self.num_buckets() {
            let mut doubled_hm = HashMap::new(2 * self.num_buckets());
            for bucket in &self.buckets {
                for node in bucket {
                    doubled_hm.put(node.key, node.val);
                }
            }
            self.buckets = doubled_hm.buckets;
        }
    }

    pub fn print(&self, verbose: bool) {
        if verbose {
            print!("\nHashMap {{\n\n");
            for i in 0..self.num_buckets() as usize {
                let bucket = self.buckets.get(i).expect("bucket should exist");
                print!("    [");
                for j in 0..bucket.len() {
                    let node = bucket.get(j).expect("node should exist");
                    print!(" ({:#?}, {:#?})", node.key, node.val);
                }
                println!(" ]");
            }
            print!("\n}}\n");
            print!("Present keys: {{");
            for key in self.present_keys.clone() {
                print!(" {:#?}", key);
            }
            println!(" }}");
        }
        println!(
            "size = {:#?}, capacity = {:#?}",
            self.num_present_keys(),
            self.num_buckets()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    const STARTING_BUCKETS: u64 = 10;

    fn hm_init() -> HashMap {
        let mut hm = HashMap::new(STARTING_BUCKETS);
        assert_eq!(hm.num_buckets(), STARTING_BUCKETS);
        hm.put(5, 5);
        hm.put(2, 10);
        hm.put(8, 13);
        hm.put(21, 6);
        hm.put(21, 100);
        hm.put(4, 6);
        hm.put(12, 14);
        hm.put(18, 1);
        hm.put(17, 4);
        assert_eq!(hm.num_present_keys(), 8);
        hm
    }

    fn insert_n_random_pairs(hm: &mut HashMap, num_pairs: u64, range: u64) {
        for _ in 0..num_pairs {
            hm.put(
                rand::rng().random_range(1..=range),
                rand::rng().random_range(1..=range),
            );
        }
    }

    #[test]
    fn test_get() {
        let hm = hm_init();
        assert_eq!(hm.get(5), Some(5));
        assert_eq!(hm.get(8), Some(13));
        assert_eq!(hm.get(1), None);
        assert_eq!(hm.get(18), Some(1));
        assert_eq!(hm.get(100), None);
        assert_eq!(hm.get(1), None);
        assert_eq!(hm.get(17), Some(4));
        assert_eq!(hm.get(25), None);
    }

    #[test]
    fn test_remove() {
        let mut hm = hm_init();
        assert_eq!(hm.get(12), Some(14));
        assert_eq!(hm.get(18), Some(1));
        assert_eq!(hm.num_present_keys(), 8);
        hm.print(false);
        hm.remove(12);
        hm.remove(18);
        hm.remove(100);
        hm.print(false);
        assert_eq!(
            hm.num_present_keys(),
            6,
            "testing number of keys present after remove()"
        );
        assert_eq!(hm.get(12), None);
        assert_eq!(hm.get(18), None);
    }

    #[test]
    fn test_bucket_doubling() {
        let mut hm = hm_init();
        assert_eq!(hm.num_present_keys(), 8);

        insert_n_random_pairs(&mut hm, 10, 100);

        if hm.num_present_keys() > STARTING_BUCKETS {
            assert_eq!(
                hm.num_buckets(),
                STARTING_BUCKETS * 2,
                "number of buckets should double after we reach 1 key per bucket on average"
            );
        }
    }

    #[test]
    fn test_larger_volume() {
        let mut hm = HashMap::new(1);
        insert_n_random_pairs(&mut hm, 1000, 10000);
        hm.print(false);
        assert!(hm.num_buckets() > hm.num_present_keys());

        let mut hm = HashMap::new(1);
        insert_n_random_pairs(&mut hm, 100000, 100000);
        hm.print(false);
        assert!(hm.num_buckets() > hm.num_present_keys());
    }
}
