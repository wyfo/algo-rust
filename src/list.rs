use std::any::Any;
use std::ops::Index;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

impl<T: Clone> Index<usize> for List<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            List::Nil => panic!("empty list"),
            List::Cons(ref elt, _) if index == 0 => &elt,
            List::Cons(_, list) => &list[index - 1],
        }
    }
}

pub struct ListIterator<'a, T: 'a>(&'a List<T>);

impl<'a, T: Clone> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.0 {
            List::Nil => None,
            List::Cons(elt, next) => {
                self.0 = next;
                Some(elt)
            }
        }
    }
}

impl<T: Clone> List<T> {
    pub fn iter(&self) -> ListIterator<T> {
        ListIterator(self)
    }
}

impl<'a, T: Clone> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = ListIterator<'a, T>;

    fn into_iter(self) -> ListIterator<'a, T> {
        self.iter()
    }
}

pub trait Stack<T: Clone>: Sized {
    fn push(&self, elt: T) -> Self;
    fn pop(&self) -> (Self, T);
    fn peek(&self) -> &T;
}

impl<T: Clone> Stack<T> for Rc<List<T>> {
    fn push(&self, elt: T) -> Rc<List<T>> {
        Rc::new(List::Cons(elt.clone(), self.clone()))
    }
    fn pop(&self) -> (Rc<List<T>>, T) {
        match **self {
            List::Nil => panic!("empty list"),
            List::Cons(ref elt, ref list) => (list.clone(), elt.clone()),
        }
    }
    fn peek(&self) -> &T {
        match **self {
            List::Nil => panic!("empty list"),
            List::Cons(ref elt, _) => elt,
        }
    }
}



