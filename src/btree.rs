//! Binary Search Tree implementation
use std::mem;
use std::iter::IntoIterator;
use std::iter::FromIterator;
use std::cmp::max;

/// The Binary Search Tree, possibly empty
#[derive(Debug)]
pub struct BTree<K,V> {
    root: Option<Box<BNode<K,V>>>
}

/// A node in the tree
#[derive(Debug)]
struct BNode<K,V> {
    key: K,
    value: V,
    left: Option<Box<BNode<K,V>>>,
    right: Option<Box<BNode<K,V>>>,
} 

impl <'a, K: Eq + PartialOrd,V> BTree<K,V> {
    /// new empty tree
    pub fn new() -> BTree<K,V> {
        BTree{root: None}
    }

    /// insert key/value
    pub fn insert(&mut self, key: K, value: V){
        let oroot = mem::replace(&mut self.root, None);
        let newroot = insert_into_node(oroot, key, value);
        self.root = Some(Box::new(newroot));
    } 

    /// is the tree empty
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// get a reference to a value for the given key, if present
    pub fn get(&'a self, key: &K) -> Option<&V> {
        search_node(&self.root, key)
    }

    /// delete a key and return the value if it was present
    pub fn delete(&'a mut self, key: &K) -> Option<V> {
        let r = mem::replace(&mut self.root,None);
        let (r,v) = delete_node(r, key);
        self.root = r;
        v
    }

    /// number of elements in the tree
    pub fn len(&self) -> usize {
        self.into_iter().count()
    }

    /// depth of the tree
    pub fn depth(&self) -> usize {
        depth(&self.root)
    }
}

/// Turn into an in-order iterator
impl <'a, K, V> IntoIterator for &'a BTree<K,V> {
    type Item = (&'a K,&'a V);
    type IntoIter = BTIterator<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        let mut v = vec!();
        let mut on = &self.root;
        while let Some(ro) = on {
            on = &ro.left;
            v.push(ro);
        }
       
        BTIterator{stack: v}
    }
}

/// iterator with a stack of nodes to visit
pub struct BTIterator<'a, K, V> {
    stack: Vec<&'a Box<BNode<K,V>>>
}

/// iterator implementation
impl <'a, K, V> Iterator for BTIterator<'a, K, V> {
    type Item = (&'a K,&'a V);

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some(n) => {
                let mut on = &n.right;
                while let Some(ro) = on {
                    on = &ro.left;
                    self.stack.push(ro);
                }
                Some ((&n.key,&n.value))
            }
        }
        
    }
}

/// convert from an iterator
impl <K: Eq + PartialOrd,V> FromIterator<(K,V)> for BTree<K,V> {
    
    fn from_iter<I: IntoIterator<Item=(K,V)>>(iter: I) -> Self {
        let mut t = BTree::new();
        for i in iter {
            t.insert(i.0,i.1);
        }
        t
    }
}

/// insert key/value in the right node
fn insert_into_node<K: Eq + PartialOrd,V>(onode : Option<Box<BNode<K,V>>>, key: K, value: V) -> BNode<K,V>{
    match onode {
        None => BNode {key: key, value: value, left: None, right: None},
        Some (mut node) => {
            if node.key == key {
                node.value = value;
            } else if key < node.key {
                let left = mem::replace(&mut node.left,None);
                node.left = Some(Box::new(insert_into_node(left, key, value)));
            } else {
                let right = mem::replace(&mut node.right,None);
                node.right = Some(Box::new(insert_into_node(right, key, value)));
            };
            *node
        }
    }
    
}

/// search for the given key and return a reference to the value if found
fn search_node<'a, K: Eq + PartialOrd,V>(onode : &'a Option<Box<BNode<K,V>>>, key: &K) -> Option<&'a V> {
    match onode {
        None => None,
        Some (node) => {
            if node.key == *key {
                Some(&node.value)
            } else if *key < node.key {
                search_node(&node.left, key)
            } else {
                search_node(&node.right, key)
            }
        }
    }
}

/// delete the node for the given key
fn delete_node<K: Eq + PartialOrd,V>(onode : Option<Box<BNode<K,V>>>, key: &K) -> (Option<Box<BNode<K,V>>>,Option<V>) {
    match onode {
        None => (None,None),
        Some (mut node) => {
            if node.key == *key {
                let v=Some(node.value);
                let n = match (node.left, node.right){
                    (None,None) => None,
                    (Some(n), None) => Some(n),
                    (None, Some(n)) => Some(n),
                    (Some(l), Some(r)) => {
                        let (nr, k, v) = find_next(r);
                        Some(Box::new(BNode {key: k, value: v, left: Some(l), right: nr}))
                    },
                };
                (n,v)
            } else if *key < node.key {
                let left = mem::replace(&mut node.left,None);
                let (n,v) = delete_node(left, key);
                node.left = n;
                (Some(node),v)
            } else {
                let right = mem::replace(&mut node.right,None);
                let (n,v) = delete_node(right, key);
                node.right = n;
                (Some(node),v)
            }
        }
    }
}

/// find the next in order node
fn find_next<K: Eq + PartialOrd,V>(mut onode : Box<BNode<K,V>>) -> (Option<Box<BNode<K,V>>>, K,V) {
    match onode.left {
        Some(n) => {
            let (nl, k, v) = find_next(n);
            onode.left = nl;
            (Some(onode),k,v)   
        },
        None =>(onode.right, onode.key, onode.value),
    }
}

/// depth of a node
fn depth<K,V>(onode: &Option<Box<BNode<K,V>>>) -> usize {
    match onode {
        None => 0,
        Some(node) => max(depth(&node.left),depth(&node.right))+1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic(){
        let mut tree = BTree::new();
        assert!(tree.is_empty());
        assert_eq!(0,tree.len());
        assert_eq!(0,tree.depth());
        tree.insert(1, "01");
        assert_eq!(false, tree.is_empty());
        assert_eq!(1,tree.len());
        assert_eq!(1,tree.depth());
        tree.insert(2, "10");
        assert_eq!(false, tree.is_empty());
        assert_eq!(2,tree.len());
        assert_eq!(2,tree.depth());
        tree.insert(0, "00");
        assert_eq!(false, tree.is_empty());
        assert_eq!(3,tree.len());
        assert_eq!(2,tree.depth());

        assert_eq!(Some(&"01"),tree.get(&1));
        assert_eq!(Some(&"10"),tree.get(&2));
        assert_eq!(Some(&"00"),tree.get(&0));
        assert_eq!(None,tree.get(&3));

        assert_eq!(Some("01"),tree.delete(&1));
        assert_eq!(false, tree.is_empty());
        assert_eq!(None,tree.get(&1));
        assert_eq!(Some(&"10"),tree.get(&2));
        assert_eq!(Some(&"00"),tree.get(&0));
        assert_eq!(2,tree.len());
        assert_eq!(2,tree.depth());

        assert_eq!(Some("10"),tree.delete(&2));
        assert_eq!(false, tree.is_empty());
        assert_eq!(1,tree.len());
        assert_eq!(1,tree.depth());
        assert_eq!(Some("00"),tree.delete(&0));
        assert!(tree.is_empty());
        assert_eq!(0,tree.len());
        assert_eq!(0,tree.depth());
    }

    #[test]
    fn test_in_order(){
        let mut tree = BTree::new();
        tree.insert(1, "01");
        tree.insert(2, "10");
        tree.insert(0, "00");
        tree.insert(3, "11");
        let mut it = tree.into_iter();
        assert_eq!(Some((&0,&"00")),it.next());
        assert_eq!(Some((&1,&"01")),it.next());
        assert_eq!(Some((&2,&"10")),it.next());
        assert_eq!(Some((&3,&"11")),it.next());
    }

    #[test]
    fn test_from_it(){
        let iter = (0..3).into_iter().map(|i| (i,i.to_string()));
        let tree = BTree::from_iter(iter);
        assert_eq!(Some(&String::from("1")),tree.get(&1));
        assert_eq!(Some(&String::from("2")),tree.get(&2));
        assert_eq!(Some(&String::from("0")),tree.get(&0));
        assert_eq!(None,tree.get(&3));
    }

    #[test]
    fn test_wikipedia() {
        // <https://en.wikipedia.org/wiki/Binary_search_tree>
        let mut tree = BTree::new();
        tree.insert(8,"8");
        tree.insert(3,"3");
        tree.insert(1,"1");
        tree.insert(6,"6");
        tree.insert(4,"4");
        tree.insert(7,"7");
        tree.insert(10,"10");
        tree.insert(14,"14");
        tree.insert(13,"13");
        assert_eq!(9,tree.len());
        assert_eq!(4,tree.depth());

        let mut it = tree.into_iter();
        assert_eq!(Some((&1,&"1")),it.next());
        assert_eq!(Some((&3,&"3")),it.next());
        assert_eq!(Some((&4,&"4")),it.next());
        assert_eq!(Some((&6,&"6")),it.next());
        assert_eq!(Some((&7,&"7")),it.next());
        assert_eq!(Some((&8,&"8")),it.next());
        assert_eq!(Some((&10,&"10")),it.next());
        assert_eq!(Some((&13,&"13")),it.next());
        assert_eq!(Some((&14,&"14")),it.next());
    }
}