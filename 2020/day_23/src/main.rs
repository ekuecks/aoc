use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Node {
    val: u64,
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let mut nodes: HashMap<u64, Rc<RefCell<Node>>> = HashMap::new();
    let mut cur = Rc::new(RefCell::new(Node { val: 1, next: None }));
    nodes.insert(1, Rc::clone(&cur));
    for val in &[6, 7, 2, 4, 8, 3, 5, 9] {
        let next = Rc::new(RefCell::new(Node {
            val: *val,
            next: None,
        }));
        cur.borrow_mut().next = Some(Rc::clone(&next));
        cur = next;
        nodes.insert(*val, Rc::clone(&cur));
    }
    for val in 10..=1000000 {
        let next = Rc::new(RefCell::new(Node {
            val: val,
            next: None,
        }));
        cur.borrow_mut().next = Some(Rc::clone(&next));
        cur = next;
        nodes.insert(val, Rc::clone(&cur));
    }
    let head_ptr = nodes.get(&1).unwrap();
    cur.borrow_mut().next = Some(Rc::clone(head_ptr));

    let mut cur_node = Rc::clone(nodes.get(&1).unwrap());
    for j in 0..10_000_000 {
        if j % 100_000 == 0 {
            println!("{}", j);
        }
        let value = cur_node.borrow().val.clone();
        let mut to_remove = Vec::new();
        for _ in 0..3 {
            let to_insert = Rc::clone(cur_node.borrow().next.as_ref().unwrap());
            cur_node.borrow_mut().next = Some(Rc::clone(to_insert.borrow().next.as_ref().unwrap()));
            to_remove.push(Rc::clone(&to_insert));
        }
        let mut target_value = value - 1;
        loop {
            if target_value == 0 {
                target_value = 1_000_000;
            }
            let mut found = false;
            for removed in to_remove.iter() {
                if removed.borrow().val == target_value {
                    target_value -= 1;
                    found = true;
                    break;
                }
            }
            if !found {
                break;
            }
        }
        let mut insert_node = nodes.get(&target_value).unwrap();
        let stored = Rc::clone(insert_node.borrow().next.as_ref().unwrap());
        for removed in to_remove.iter() {
            insert_node.borrow_mut().next = Some(Rc::clone(removed));
            insert_node = removed;
        }
        insert_node.borrow_mut().next = Some(stored);
        let next_node = Rc::clone(cur_node.borrow_mut().next.as_ref().unwrap());
        cur_node = next_node;
    }
    let cur_node = nodes.get(&1).unwrap();
    let n_v = cur_node.borrow().next.as_ref().unwrap().borrow().val;
    let nn_v = cur_node
        .borrow()
        .next
        .as_ref()
        .unwrap()
        .borrow()
        .next
        .as_ref()
        .unwrap()
        .borrow()
        .val;
    println!("{}, {}", n_v, nn_v);
    println!("Answer: {}", n_v * nn_v);
}
