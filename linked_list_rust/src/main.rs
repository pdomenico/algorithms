struct Node {
    value: i32,
    next: Link,
}

type Link = Option<Box<Node>>;

struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: None
        }
    }

    pub fn push(&mut self, v: i32) {
        let new_head = Box::new(Node {
            value: v,
            next: self.head.take(),
        });

        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }
}

fn main() {
    let mut list = List::new();

    for i in 0..5 {
        list.push(i);
    }

    while let Some(v) = list.pop() {
        println!("{}", v);
    }
}
