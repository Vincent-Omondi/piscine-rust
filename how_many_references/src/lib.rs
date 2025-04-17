pub use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list }
    }

    pub fn add_element(&mut self, v: Rc<String>) {
        self.ref_list.push(v);
    }

    pub fn rm_all_ref(&mut self, v: Rc<String>) {
        self.ref_list.retain(|x| is_same_allocate(x, &v));
    }
}

fn is_same_allocate(x: &Rc<String>, v: &Rc<String>) -> bool {
    if is_eq(x, v) {
        return v != x;
    }
    return true;
}

pub fn how_many_references(value: &Rc<String>) -> usize {
    Rc::strong_count(value)
}

fn is_eq(value: &Rc<String>, cmp: &Rc<String>) -> bool {
    Rc::ptr_eq(value, cmp)
}