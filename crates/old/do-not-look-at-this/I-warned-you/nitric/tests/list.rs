use nitric::list::*;
use std::fmt::Debug;

#[test]
fn test_append() {
    let list = Leaf.append(5).append(3).append(1);

    assert_eq!(list, Node {
        elem: 5,
        next: Node {
            elem: 3,
            next: Node {
                elem: 1,
                next: Leaf,
            }
        }
    });
}

#[test]
fn test_visit() {
    let list = Leaf.append(5).append(false).append("This is a\nnewline");

    list.visit(DebugVisitor);
}

struct DebugVisitor;

impl<T: Debug> Visitor<T> for DebugVisitor {
    fn visit(&mut self, elem: T) {
        println!("{:?}", elem);
    }
}
