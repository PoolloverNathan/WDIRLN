use std::mem::replace;

#[derive(Clone, PartialEq, Eq, Debug, Default, Hash)]
struct ListNode<T>(T, ListOf<T>);

type ListOf<T> = Option<Box<ListNode<T>>>;

trait List<T>: Sized {
    /// Helper method to remove a list's internal representation.
    fn into_list(self) -> impl List<T>;
    /// Helper method to remove a list's internal representation.
    fn list(&self) -> &impl List<T>;
    /// Helper method to remove a list's internal representation.
    fn list_mut(&mut self) -> &mut impl List<T>;
    /// Returns a reference to this node's value.
    fn value(&self) -> Option<&T>;
    /// Returns a reference to this node
    fn value_mut(&mut self) -> Option<&mut T>;
    fn tail(&self) -> &Self;
    fn tail_mut(&mut self) -> &mut Self;
    fn get(&self, index: usize) -> Option<&T>;
    /// Replaces the list with a new list in-place, with the original list as the tail as the new node.
    fn push(&mut self, value: T);
    /// Replaces the list with its tail in-place, returning the old value.
    fn pop(&mut self) -> Option<T>;
    /// Converts a linked list into an iterator of its elements, deallocating each element as it is consumed.
    fn into_iter(self) -> impl Iterator<Item = T>;
    /// Returns an iterator over references to the list's elements. (The type of the iterator cannot be named and is different from [into_iter].)
    /// 
    /// [into_iter]: #into_iter
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a;
    /// Returns an iterator over mutable references to the list's elements.  (The type of the iterator cannot be named and is different than [into_iter] and [iter].)
    /// 
    /// [into_iter]: #into_iter
    /// [iter]: #iter
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a;
}

impl<T> List<T> for ListOf<T> {
    fn into_list(self) -> impl List<T> {
        self
    }
    fn list(&self) -> &impl List<T> {
        self
    }
    fn list_mut(&mut self) -> &mut impl List<T> {
        self
    }
    fn value(&self) -> Option<&T> {
        self.as_ref().map(|node| &node.0)
    }
    fn value_mut(&mut self) -> Option<&mut T> {
        self.as_mut().map(|node| &mut node.0)
    }
    fn tail(&self) -> &Self {
        match self {
            r@&None => r,
            Some(ref node) => &node.1
        }
    }
    fn tail_mut(&mut self) -> &mut Self {
        match self {
            r@&mut None => r,
            Some(ref node) => &mut node.1
        }
    }
    fn get(&self, index: usize) -> Option<&T> {
        if let None = self {
            None
        } else if let 0 = index {
            self.value()
        } else {
            self.tail().get(index)
        }
    }
    fn push(&mut self, value: T) {
        // make new node with given value and no tail
        let new_node = Some(Box::new(ListNode(value, None)));
        // replace this node with the new node
        let old_node = replace(self, new_node);
        // put the old node as the new node's tail
        if let Some(tail) = old_node {
            // potential UAF, investigate later
            // let a = **self.as_mut().unwrap();
            self.as_mut().unwrap().as_mut().1.insert(tail);
        }
    }
    fn pop(&mut self) -> Option<T> {
        if self.is_none() {
            None
        } else {
            // first, &mut the real box
            let node = self.as_mut().unwrap();
            // replace ourself with our tail
            let old_self = replace(self, node.1);
            // wait why am I doing this?
            // anyway, give away our only remaining field and therefore die
            Some(node.0)
        }
    }
    fn into_iter(self) -> impl Iterator<Item = T> {
        struct Iter<T>(ListOf<T>);
        impl<T> Iterator for Iter<T> {
            type Item = T;
            fn next(&mut self) -> Option<Self::Item> {
                self.0.map(|list| {
                    let ListNode(value, tail) = *list;
                    self.0 = tail; // pleasantly surprised this works
                    value
                })
            }
        }
        Iter(self)
    }
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a {
        struct Iter<'a, T>(&'a ListOf<T>);
        impl<'a, T> Iterator for Iter<'a, T> {
            type Item = &'a T;
            fn next(&mut self) -> Option<Self::Item> {
                self.0.map(|list| {
                    let ListNode(ref value, ref tail) = *list;
                    self.0 = tail; // pleasantly surprised this works
                    value
                })
            }
        }
        Iter(self)
    }
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a {
        struct Iter<'a, T>(&'a mut ListOf<T>);
        impl<'a, T> Iterator for Iter<'a, T> {
            type Item = &'a mut T;
            fn next(&mut self) -> Option<Self::Item> {
                self.0.map(|list| {
                    let ListNode(ref mut value, ref mut tail) = *list;
                    self.0 = tail; // pleasantly surprised this works
                    value
                })
            }
        }
        Iter(self)
    }
}