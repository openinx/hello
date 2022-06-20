pub trait Hash: Eq {
    fn hash(&self) -> i64;
}

// Use linked list to handle the hash conflicts for the same bucket.
pub struct HashMap<K: Hash, V> {
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
    buckets: Vec<&'a Ptr<K, V>>,
    h: usize,           // Bucket index.
    cur: &'a Ptr<K, V>, // The current iterator.
}

fn hash<K: Hash>(bucket_num: usize, key: &K) -> usize {
    let m = bucket_num as i64;
    ((key.hash() % m + m) % m) as usize
}

impl<K, V> HashMap<K, V>
where
    K: Hash,
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
        let mut buckets: Vec<Ptr<K, V>> = (0..self.bucket_num() << 1).map(|_| None).collect();
        for h in 0..self.bucket_num() {
            let ptr = &mut self.buckets[h];

            // Great! Because I've estimated the key and vlaue clone in this body successfully.
            while let Some(node) = ptr.take() {
                let h = hash(buckets.capacity(), &node.key);

                buckets[h] = Some(Box::new(Node {
                    key: node.key,
                    val: node.val,
                    next: buckets[h].take(),
                }));

                *ptr = node.next;
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

        if self.size >= self.bucket_num() << 1 {
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

    pub fn get_mut_ptr(&mut self, key: K) -> Option<&mut Ptr<K, V>> {
        let h = hash(self.buckets.capacity(), &key);
        let mut cur = &mut self.buckets[h];
        loop {
            if cur.is_none() {
                return None;
            } else if cur.as_deref().unwrap().key == key {
                return Some(cur);
            }
            cur = &mut cur.as_deref_mut().unwrap().next;
        }
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let mut ret: Option<V> = None;
        let ptr = self.get_mut_ptr(key);
        ptr.map(|cur| {
            cur.take().map(|node| {
                *cur = node.next;
                ret = Some(node.val);
            })
        });

        ret.map(|v| {
            self.size -= 1;
            v
        })
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        Iter {
            buckets: (0..self.bucket_num()).map(|i| &self.buckets[i]).collect(),
            h: 0,
            cur: &self.buckets[0],
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<'a, K, V> Iter<'a, K, V>
where
    K: Hash,
{
    pub fn next(&mut self) -> Option<&'a Node<K, V>> {
        loop {
            // Exhaust the current bucket.
            if let Some(node) = self.cur {
                self.cur = &node.next;
                return Some(node);
            }

            // Switch to the next bucket.
            self.h += 1;
            if self.h >= self.buckets.capacity() {
                return None;
            } else {
                self.cur = self.buckets[self.h];
            }
        }
    }
}

impl Hash for i32 {
    fn hash(&self) -> i64 {
        *self as i64
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
    use std::time::Instant;

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
    pub fn test_iter() {
        let mut map = HashMap::new();
        let n = 100_000;
        for i in 0..n {
            map.put(i, i);
            assert_eq!(map.get(i), Some(&i));
            assert_eq!(map.size(), (i + 1) as usize);
        }

        {
            let mut iter = map.iter();
            let mut hit = vec![false; n as usize];
            while let Some(node) = iter.next() {
                let k = node.key as usize;
                let v = node.val as usize;

                assert_eq!(k, v);
                assert_eq!(hit[k], false);
                assert_eq!(hit[v], false);
                hit[k] = true;
            }

            assert_eq!(hit, vec![true; n as usize]);
        }

        for i in 100..n {
            assert_eq!(map.remove(i), Some(i));
        }

        {
            let mut iter = map.iter();
            let mut hit = vec![false; 100];
            while let Some(node) = iter.next() {
                let k = node.key as usize;
                let v = node.val as usize;

                assert_eq!(k, v);
                assert_eq!(hit[k], false);
                assert_eq!(hit[v], false);
                hit[k] = true;
            }

            assert_eq!(hit, vec![true; 100 as usize]);
        }
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
