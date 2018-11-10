use symbols::Tag;
use std::rc::Rc;
use traces::Trace;
use list::*;
use std::iter::empty;
use std::iter::once;

#[derive(Debug)]
pub enum Tree<Tk> {
    Nil,
    Leaf(Tk, Tag),
    Node(Vec<Tree<Tk>>, Tag),
}

use trees::Tree::*;
use std::fmt::Debug;

impl<Tk: 'static> Tree<Tk> {
    pub fn tag(&self) -> Tag {
        match *self {
            Nil => None,
            Leaf(_, tag) => tag,
            Node(_, tag) => tag,
        }
    }

    fn iter_on_children<'a>(&'a self, iter: impl FnMut(&'a Tree<Tk>) -> Box<dyn Iterator<Item=&'a Tree<Tk>> + 'a> + 'a) -> Box<dyn Iterator<Item=&'a Tree<Tk>> + 'a> {
        match *self {
            Node(ref children, _) => Box::new(children.iter().flat_map(iter)),
            _ => Box::new(empty()),
        }
    }

    pub fn leaves<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Tree<Tk>> + 'a> {
        self.iter_on_children(|tree| match tree {
            Nil => Box::new(empty()),
            leaf @ Leaf(..) => Box::new(once(leaf)),
            node @ Node(..) => node.leaves(),
        })
    }

    pub fn tagged<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Tree<Tk>> + 'a> {
        self.iter_on_children(|tree| if tree.tag().is_some() {
            Box::new(once(tree))
        } else {
            tree.tagged()
        })
    }

    pub fn tagged_and_leaves<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Tree<Tk>> + 'a> {
        self.iter_on_children(|tree| match tree {
            Nil => Box::new(empty()),
            leaf @ Leaf(..) => Box::new(once(leaf)),
            node @ Node(..) => if node.tag().is_some() {
                Box::new(once(node))
            } else {
                node.tagged_and_leaves()
            },
        })
    }
}

pub type VolatileBuilder<'a> = Option<(&'a dyn TreeBuilder, Tag)>;

pub enum LeafBuilder {
    Epsilon(Tag),
    // LoopReader can be epsilon and have tag
    Token(Tag),
}

pub enum SwitchBuilder<'a> {
    Case(&'a dyn TreeBuilder, Tag),
    Loop,
}

pub type NodeBuilder<'a> = (Box<dyn Iterator<Item=&'a dyn TreeBuilder> + 'a>, Tag);

pub trait TreeBuilder: AsTreeBuilder {
    fn tag(&self) -> Tag;
    fn is_volatile(&self) -> VolatileBuilder {
        None
    }
    fn leaf_builder(&self) -> LeafBuilder;
    fn switch_builder(&self, case: usize) -> SwitchBuilder;
    fn node_builder(&self) -> NodeBuilder; // impl Iterator doesn't compile
}

pub trait AsTreeBuilder {
    fn as_tree_builder(&self) -> &TreeBuilder;
}

impl<T: TreeBuilder> AsTreeBuilder for T {
    fn as_tree_builder(&self) -> &TreeBuilder {
        self
    }
}

fn build_node<'a, 'b, Tk: Clone + Debug>(elts_with_traces: impl Iterator<Item=(&'a dyn TreeBuilder, &'a List<Trace>)>, tokens: &'b [Tk], tag: Tag) -> (Tree<Tk>, &'b [Tk]) {
    let (children, tokens) = elts_with_traces.fold((Vec::<Tree<Tk>>::new(), tokens),
                                                   |(mut children, tokens), (builder, traces)| {
                                                       let (tree, tokens) = build_rec(builder, traces, tokens);
                                                       children.push(tree);
                                                       (children, tokens)
                                                   });
    (Node(children, tag), tokens)
}

fn as_rec_trace(trace: &Trace) -> &List<Trace> {
    if let Trace::Rec(traces) = trace {
        traces.as_ref()
    } else {
        unimplemented!()
    }
}

fn build_rec<'a, 'b, 'c, Tk: Clone + Debug>(builder: &'a dyn TreeBuilder, traces: &'b List<Trace>, tokens: &'c [Tk]) -> (Tree<Tk>, &'c [Tk]) {
    let add_branch = |next: &dyn TreeBuilder, traces: &List<Trace>, tag: Tag| if tag.is_some() {
        let (tree, tokens) = build_rec(next, traces, tokens);
        (Tree::Node(vec![tree], tag), tokens)
    } else {
        build_rec(next, traces, tokens)
    };
    let volatile = builder.is_volatile();
    if let Some((next, tag)) = volatile {
        return add_branch(next, traces, tag);
    }
    match traces {
        List::Nil => match builder.leaf_builder() {
            LeafBuilder::Epsilon(tag) => (if tag.is_some() {
                Tree::Node(vec![Tree::Nil], tag)
            } else {
                Tree::Nil
            }, tokens),
            LeafBuilder::Token(tag) => (Leaf(tokens[0].clone(), tag), &tokens[1..])
        },
        List::Cons(trace, tail) => match trace {
            Trace::Switch(index, _) => match builder.switch_builder(*index) {
                SwitchBuilder::Case(next, tag) => add_branch(next, tail, tag),
                SwitchBuilder::Loop => build_rec(builder, tail, tokens),
            },
            Trace::Rec(..) => {
                let (elts, tag) = builder.node_builder();
                build_node(elts.zip(traces.iter().map(as_rec_trace)), tokens, tag)
            }
            Trace::Tmp(tmp) => {
                let (elts, tag) = builder.node_builder();
                let rev_traces: Vec<&List<Trace>> = tmp.iter().map(|t| t.as_ref()).collect();
                build_node(elts.zip(rev_traces.iter().rev().map(|t| *t)), tokens, tag)
            }
        },
    }
}

pub fn tree_from_trace<Tk: Clone + Debug>(builder: &dyn TreeBuilder, traces: &Rc<List<Trace>>, tokens: &[Tk]) -> Tree<Tk> {
    build_rec(builder, &traces, tokens).0
}