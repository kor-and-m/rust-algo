use std::{cmp::Ordering::{Equal, Greater, Less}, rc::Rc, cell::RefCell};

pub type Node<T> = Rc<RefCell<UnionFind<T>>>;
pub type UnionValLink<T> = Rc<RefCell<T>>;

#[derive(Debug)]
pub struct UnionFind<T> {
    leader: Option<Node<T>>,
    rank: usize,
    val: UnionValLink<T>
}

impl<T: PartialEq> UnionFind<T> {
    pub fn create_root(val: T) -> Node<T> {
        let union_struct = UnionFind {val: Rc::new(RefCell::new(val)), leader: None, rank: 0};
        Rc::new(RefCell::new(union_struct))
    }

    pub fn find_val(node: &Node<T>) -> UnionValLink<T> {
        let leader = Self::find(Rc::clone(node));
        let unionfind = leader.borrow();
        Rc::clone(&unionfind.val)
    }

    pub fn find(node: Node<T>) -> Node<T> {
        if node.borrow().leader.is_none() {
            node
        } else {
            let current_leader = Rc::clone(&node.borrow().leader.as_ref().unwrap());
            let leader: Node<T> = Self::find(Rc::clone(&current_leader));
            if !Rc::ptr_eq(&current_leader, &leader) {
                current_leader.borrow_mut().leader = Some(leader.clone());
            }
            leader
        }
    }

    pub fn union(self_node: &Node<T>, other_node: &Node<T>) {
        let self_leader = Self::find(Rc::clone(self_node));
        let other_leader = Self::find(Rc::clone(other_node));

        let cmp_result = self_leader.borrow().rank.cmp(&other_leader.borrow().rank);

        match cmp_result {
            Less => self_leader.borrow_mut().leader = Some(other_leader),
            Equal => {
                self_leader.borrow_mut().rank += 1;
                other_leader.borrow_mut().leader = Some(self_leader);
            },
            Greater => other_leader.borrow_mut().leader = Some(self_leader)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_find_works() {
        let union1 = UnionFind::create_root(1);
        let union2 = UnionFind::create_root(2);
        let union3 = UnionFind::create_root(3);
        let union4 = UnionFind::create_root(4);
        let union5 = UnionFind::create_root(5);
        let union6 = UnionFind::create_root(6);

        assert_eq!(1, *UnionFind::find_val(&union1).borrow());
        assert_eq!(2, *UnionFind::find_val(&union2).borrow());
        
        UnionFind::union(&union1, &union2);
        UnionFind::union(&union1, &union3);

        let mut union2_leader = *UnionFind::find_val(&union2).borrow();
        let mut union3_leader = *UnionFind::find_val(&union2).borrow();

        assert_eq!(union2_leader, union3_leader);

        UnionFind::union(&union4, &union3);

        let union4_leader = *UnionFind::find_val(&union4).borrow();
        let union1_leader = *UnionFind::find_val(&union1).borrow();
        union3_leader = *UnionFind::find_val(&union2).borrow();

        assert_eq!(union4_leader, union1_leader);
        assert_eq!(union3_leader, union1_leader);

        UnionFind::union(&union5, &union6);

        let mut union5_leader = *UnionFind::find_val(&union5).borrow();
        let mut union6_leader = *UnionFind::find_val(&union6).borrow();
        union2_leader = *UnionFind::find_val(&union2).borrow();

        assert_eq!(union5_leader, union6_leader);
        assert_ne!(union6_leader, union2_leader);

        UnionFind::union(&union3, &union6);

        union6_leader = *UnionFind::find_val(&union6).borrow();
        union2_leader = *UnionFind::find_val(&union2).borrow();

        assert_eq!(union6_leader, union2_leader);

        union5_leader = *UnionFind::find_val(&union5).borrow();

        assert_eq!(union5_leader, union2_leader);
    }
}
