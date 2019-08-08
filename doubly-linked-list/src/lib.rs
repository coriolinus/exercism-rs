// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

use std::fmt;
use std::ptr::NonNull;
type NNMut<T> = Option<NonNull<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    item: T,

    // the pointers below are const because we expect it to be relatively rare
    // for them to change. It doesn't really matter either way; it's possible
    // to cast freely between them.
    /// next steps toward the back of the list
    next: NNMut<T>,

    /// prev steps toward the front of the list
    prev: NNMut<T>,
}

impl<T> Node<T> {
    fn new(item: T) -> Node<T> {
        Node {
            item,
            next: None,
            prev: None,
        }
    }

    unsafe fn into_ptr(self) -> NNMut<T> {
        let heaped = Box::new(self);
        let ptr = Box::into_raw(heaped);
        debug_assert!(ptr != std::ptr::null_mut());
        NonNull::new(ptr)
    }

    fn len(&self) -> usize {
        1 + self.next.map_or(0, |next| unsafe { next.as_ref().len() })
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    // these pointers are mut because we expect them to change relatively frequently
    front: NNMut<T>,
    back: NNMut<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            front: None,
            back: None,
        }
    }

    pub fn len(&self) -> usize {
        self.front.map_or(0, |node| unsafe { node.as_ref().len() })
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<T> {
        Cursor::new(self, self.front)
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<T> {
        Cursor::new(self, self.back)
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<T> {
        Iter::new(self, self.front)
    }
}

impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut first = true;
        write!(f, "[")?;
        for item in self.iter() {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // basically the same as in the stdlib implementation of drop
        while let Some(_) = self.pop_front() {}
    }
}

// Send is safe: just moving the list between threads breaks no invariants;
// the pointers are still all valid.
unsafe impl<T: Send> Send for LinkedList<T> {}

// I'm _pretty sure_ Sync is safe: in the event of a mutable ref, only the ref
// can mutate the struct via the provided API, which is safe. In the event of
// many immutable refs, none of them mutate it anyway, which is safe.
//
// I do wish I were more confident of this, though.
unsafe impl<T: Sync> Sync for LinkedList<T> {}

#[derive(Debug)]
pub struct Cursor<'a, T> {
    ll: &'a mut LinkedList<T>,
    ptr: NNMut<T>,
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    fn new(ll: &mut LinkedList<T>, ptr: NNMut<T>) -> Cursor<T> {
        Cursor { ll, ptr }
    }

    pub fn peek<'a>(&'a self) -> Option<&'a T> {
        self.ptr
            .map(|raw_ptr| &Box::leak(unsafe { Box::from_raw(raw_ptr.as_ptr()) }).item)
    }

    /// Take a mutable reference to the current element
    pub fn peek_mut<'a>(&'a mut self) -> Option<&'a mut T> {
        self.ptr
            .map(|raw_ptr| &mut Box::leak(unsafe { Box::from_raw(raw_ptr.as_ptr()) }).item)
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    pub fn next<'a>(&'a mut self) -> Option<&'a mut T> {
        self.ptr = match self.ptr {
            None => None,
            Some(raw_ptr) => unsafe { (*raw_ptr.as_ptr()).next },
        };
        self.peek_mut()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        self.ptr = match self.ptr {
            None => None,
            Some(raw_ptr) => unsafe { (*raw_ptr.as_ptr()).prev },
        };
        self.peek_mut()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        let mut rv = None;
        let mut next = None;
        if let Some(node) = self.ptr {
            unsafe {
                let node = *Box::from_raw(node.as_ptr());
                // select next and update pointers
                if node.next.is_some() {
                    next = node.next;
                } else if node.prev.is_some() {
                    next = node.prev;
                }
                // update external pointers
                if let Some(nnext) = node.next {
                    (*nnext.as_ptr()).prev = node.prev;
                } else {
                    self.ll.back = node.prev;
                }
                if let Some(nprev) = node.prev {
                    (*nprev.as_ptr()).next = node.next;
                } else {
                    self.ll.front = node.next;
                }

                // get return value
                rv = Some(node.item);
            }
            // update self
            self.ptr = next;
        }
        rv
    }

    pub fn insert_after(&mut self, element: T) {
        let new_node_ptr = unsafe { Node::new(element).into_ptr() };
        debug_assert!(new_node_ptr.is_some());
        self.ptr = match self.ptr {
            None => {
                self.ll.front = new_node_ptr;
                self.ll.back = new_node_ptr;
                new_node_ptr
            }
            Some(cur_ptr) => {
                unsafe {
                    let cur_node = cur_ptr.as_ptr();
                    // update both node pointers
                    (*new_node_ptr.unwrap().as_ptr()).prev = Some(cur_ptr);
                    (*new_node_ptr.unwrap().as_ptr()).next = (*cur_node).next;
                    // update external pointers
                    if let Some(next) = (*cur_node).next {
                        (*next.as_ptr()).prev = new_node_ptr;
                    } else {
                        self.ll.back = new_node_ptr;
                    }
                    // update self pointer
                    (*cur_node).next = new_node_ptr;
                }

                Some(cur_ptr)
            }
        };
        debug_assert!(self.ll.front.is_some());
        debug_assert!(self.ll.back.is_some());
        debug_assert!(self.ptr.is_some());
    }

    pub fn insert_before(&mut self, element: T) {
        let new_node_ptr = unsafe { Node::new(element).into_ptr() };
        debug_assert!(new_node_ptr.is_some());
        self.ptr = match self.ptr {
            None => {
                self.ll.front = new_node_ptr;
                self.ll.back = new_node_ptr;
                new_node_ptr
            }
            Some(cur_ptr) => {
                unsafe {
                    let cur_node = cur_ptr.as_ptr();
                    // update both node pointers
                    (*new_node_ptr.unwrap().as_ptr()).next = Some(cur_ptr);
                    (*new_node_ptr.unwrap().as_ptr()).prev = (*cur_node).prev;
                    // update external pointers
                    if let Some(prev) = (*cur_node).prev {
                        (*prev.as_ptr()).next = new_node_ptr;
                    } else {
                        self.ll.front = new_node_ptr;
                    }
                    // update self pointer
                    (*cur_node).prev = new_node_ptr;
                }

                Some(cur_ptr)
            }
        };
        debug_assert!(self.ll.front.is_some());
        debug_assert!(self.ll.back.is_some());
        debug_assert!(self.ptr.is_some());
    }
}

impl<T: fmt::Display> fmt::Display for Cursor<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.ll.fmt(f)
    }
}

pub struct Iter<'a, T> {
    lifetime: std::marker::PhantomData<&'a T>,
    ptr: NNMut<T>,
}

impl<'a, T> Iter<'a, T> {
    fn new(_: &'a LinkedList<T>, ptr: NNMut<T>) -> Iter<'a, T> {
        Iter {
            lifetime: std::marker::PhantomData,
            ptr,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let mut rv = None;
        if let Some(ptr) = self.ptr {
            let t = Box::leak(unsafe { Box::from_raw(ptr.as_ptr()) });
            rv = Some(&t.item);
            self.ptr = t.next;
        }
        rv
    }
}
