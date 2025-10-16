use std::collections::HashSet;

const STARTING_BUCKETS: usize = 5;

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
    pub fn new() -> Self {
        let mut hm = HashMap {
            buckets: Vec::new(),
            present_keys: HashSet::new(),
        };

        for _ in 0..STARTING_BUCKETS {
            let v = Vec::new();
            hm.buckets.push(v);
        }

        hm
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
    }

    pub fn get(&self, key: u64) -> Option<u64> {
        // print!("GET: {:#?} -> ", key);
        if !self.present_keys.contains(&key) {
            // println!("not found");
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
                // println!("{:#?}", node.val);
                return Some(node.val);
            }
        }
        None
    }

    pub fn remove(&mut self, key: u64) {
        let bucket_index: usize = key as usize % self.buckets.len();
        let bucket = self
            .buckets
            .get_mut(bucket_index)
            .expect("bucket must not be empty");
        for i in 0..bucket.len() {
            let node = bucket.get(i).expect("node must not be empty");
            if node.key == key {
                bucket.swap_remove(i);
                self.present_keys.remove(&key);
                return;
            }
        }
    }

    pub fn print(&self) {
        print!("\nHashMap {{\n\n");
        for i in 0..self.buckets.len() {
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
        println!(" }}, size = {:#?}", self.present_keys.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hm_init() -> HashMap {
        let mut hm = HashMap::new();
        hm.put(5, 5);
        hm.put(2, 10);
        hm.put(8, 13);
        hm.put(21, 6);
        hm.put(21, 100);
        hm.put(4, 6);
        hm.put(12, 14);
        hm.put(18, 1);
        hm.put(17, 4);
        hm
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
        hm.remove(12);
        hm.remove(18);
        hm.remove(100);
        assert_eq!(hm.get(12), None);
        assert_eq!(hm.get(18), None);
    }
}

