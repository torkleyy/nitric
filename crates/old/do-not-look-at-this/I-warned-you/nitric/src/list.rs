pub trait Append<T> {
    type Output;

    fn append(self, elem: T) -> Self::Output;
}

#[derive(Debug, PartialEq)]
pub struct Leaf;

impl<V> Visitable<V> for Leaf {
    fn visit(self, _: V) {}
}

impl<T> Append<T> for Leaf {
    type Output = Node<T, Leaf>;

    fn append(self, elem: T) -> Self::Output {
        Node { elem, next: Leaf }
    }
}

#[derive(Debug, PartialEq)]
pub struct Node<T, N> {
    pub elem: T,
    pub next: N,
}

impl<T, N, I> Append<I> for Node<T, N>
where
    N: Append<I>,
{
    type Output = Node<T, <N as Append<I>>::Output>;

    fn append(self, elem: I) -> Self::Output {
        Node {
            elem: self.elem,
            next: self.next.append(elem),
        }
    }
}

impl<T, N, V> Visitable<V> for Node<T, N>
where
    N: Visitable<V>,
    V: Visitor<T>,
{
    fn visit(self, mut visitor: V) {
        visitor.visit(self.elem);
        self.next.visit(visitor);
    }
}

pub trait Visitable<V> {
    fn visit(self, visitor: V);
}

pub trait Visitor<T> {
    fn visit(&mut self, elem: T);
}
