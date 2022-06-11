use std::time::Instant;
pub trait Hash: Eq + Clone {
    fn hash(&self) -> i64;
}

// Use linked list to handle the hash conflicts for the same bucket.
pub struct HashMap<K: Hash, V: Clone> {
    size: usize,
    buckets: Vec<Ptr<K, V>>,
}

type Ptr<K, V> = Option<Box<Node<K, V>>>;
pub struct Node<K: Hash, V> {
    key: K,
    val: V,
    next: Ptr<K, V>,
}

pub struct Iter<'a, K: Hash, V> {
    index: usize,
    cur: &'a Ptr<K, V>,
}

fn hash<K: Hash>(bucket_num: usize, key: &K) -> usize {
    let m = bucket_num as i64;
    ((key.hash() % m + m) % m) as usize
}

impl<K, V> HashMap<K, V>
where
    K: Hash,
    V: Clone,
{
    pub fn new() -> Self {
        HashMap {
            size: 0,
            buckets: (0..4).map(|_| None).collect(),
        }
    }

    pub fn bucket_num(&self) -> usize {
        return self.buckets.capacity();
    }

    fn rehash(&mut self) {
        let mut buckets: Vec<Ptr<K, V>> = (0..self.bucket_num() * 2).map(|_| None).collect();
        for h in 0..self.bucket_num() {
            let mut ptr = &mut self.buckets[h];
            while let Some(node) = ptr.as_deref_mut() {
                let h = hash(buckets.capacity(), &node.key);

                // TODO Don't clone the key value for better performance.
                buckets[h] = Some(Box::new(Node {
                    key: node.key.clone(),
                    val: node.val.clone(),
                    next: buckets[h].take(),
                }));

                ptr = &mut node.next;
            }
        }

        self.buckets = buckets;
    }

    pub fn put(&mut self, key: K, val: V) {
        let h = hash(self.bucket_num(), &key);

        let mut cur = &mut self.buckets[h];
        loop {
            match cur {
                None => {
                    *cur = Some(Box::new(Node {
                        key,
                        val,
                        next: None,
                    }));
                    self.size += 1;
                    break;
                }
                Some(node) if node.key == key => {
                    *node = Box::new(Node {
                        key,
                        val,
                        next: node.next.take(),
                    });
                    self.size += 1;
                    break;
                }
                Some(node) => {
                    cur = &mut node.next;
                }
            }
        }

        if self.size >= self.bucket_num() * 2 {
            self.rehash();
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let h = hash(self.bucket_num(), &key);
        let mut ptr = &self.buckets[h];
        while let Some(node) = ptr {
            if node.key == key {
                return Some(&node.val);
            }
            ptr = &node.next;
        }
        None
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        // Accessing the immutable ref before borrowing mutable ref is OK
        let h = hash(self.buckets.capacity(), &key);
        let mut cur = &mut self.buckets[h];
        loop {
            match cur {
                None => return None,
                Some(node) if node.key == key => {
                    // TODO don't use the clone here.
                    let val = node.val.clone();
                    *cur = node.next.take();
                    self.size -= 1;

                    return Some(val);
                }
                Some(node) => {
                    cur = &mut node.next;
                }
            }
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        Iter {
            index: 0,
            cur: &None,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Hash for String {
    fn hash(&self) -> i64 {
        let mut ret = 0 as i64;
        for c in self.bytes() {
            ret = (ret << 5) + ret + (c as i64);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basics() {
        let mut map = HashMap::new();
        map.put("ABC".to_string(), 1);
        map.put("DEF".to_string(), 2);
        map.put("HIG".to_string(), 3);

        assert_eq!(map.size(), 3);
        assert_eq!(map.get("ABC".to_string()), Some(&1));
        assert_eq!(map.get("DEF".to_string()), Some(&2));
        assert_eq!(map.get("HIG".to_string()), Some(&3));

        assert_eq!(map.remove("ABC".to_string()), Some(1));
        assert_eq!(map.size(), 2);

        assert_eq!(map.remove("DEF".to_string()), Some(2));
        assert_eq!(map.size(), 1);

        assert_eq!(map.remove("HIG".to_string()), Some(3));
        assert_eq!(map.size(), 0);
    }

    #[test]
    pub fn insert_many_strings() {
        let mut map = HashMap::new();

        let now = Instant::now();
        let n = 1000_000;

        for i in 0..n {
            map.put(i.to_string(), i.to_string());
            assert_eq!(map.get(i.to_string()), Some(&i.to_string()));
            assert_eq!(map.size(), (i + 1) as usize);
        }

        for i in 0..n {
            assert_eq!(map.remove(i.to_string()), Some(i.to_string()));
            assert_eq!(map.size(), n - 1 - i);
        }

        println!("Time elapsed: {} millis", now.elapsed().as_millis());
    }
}
