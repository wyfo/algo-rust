use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::ops::Index;
use std::rc::Rc;

#[derive(Clone)]
pub enum List<T, N> {
    Cons(T, Rc<List<T, N>>),
    Nil(N),
}

impl<T: Debug + Clone, N> Debug for List<T, N> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self.iter().collect::<Vec<_>>())
    }
}

impl<T: Clone, N> Index<usize> for List<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            List::Nil(..) => panic!("empty list"),
            List::Cons(ref elt, _) if index == 0 => &elt,
            List::Cons(_, list) => &list[index - 1],
        }
    }
}

pub struct ListIterator<'a, T: 'a, N: 'a>(&'a List<T, N>);

impl<'a, T: Clone, N> Iterator for ListIterator<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.0 {
            List::Nil(..) => None,
            List::Cons(elt, next) => {
                self.0 = next;
                Some(elt)
            }
        }
    }
}

impl<T: Clone, N> List<T, N> {
    pub fn iter(&self) -> ListIterator<T, N> {
        ListIterator(self)
    }
}

impl<'a, T: Clone, N> IntoIterator for &'a List<T, N> {
    type Item = &'a T;
    type IntoIter = ListIterator<'a, T, N>;

    fn into_iter(self) -> ListIterator<'a, T, N> {
        self.iter()
    }
}

pub trait Stack<T: Clone>: Sized {
    fn push(&self, elt: T) -> Self;
    fn pop(&self) -> (Self, T);
    fn peek(&self) -> &T;
}

impl<T: Clone, N> Stack<T> for Rc<List<T, N>> {
    fn push(&self, elt: T) -> Rc<List<T, N>> {
        Rc::new(List::Cons(elt.clone(), self.clone()))
    }
    fn pop(&self) -> (Rc<List<T, N>>, T) {
        match **self {
            List::Nil(..) => panic!("empty list"),
            List::Cons(ref elt, ref list) => (list.clone(), elt.clone()),
        }
    }
    fn peek(&self) -> &T {
        match **self {
            List::Nil(..) => panic!("empty list"),
            List::Cons(ref elt, _) => elt,
        }
    }
}



