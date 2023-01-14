use std::cmp::Ordering;

pub struct UnionFind {
    pub parents: Vec<usize>,
    pub ranks: Vec<usize>,
}

impl UnionFind {
    pub fn new(elements: usize) -> Self {
        UnionFind {
            parents: (0..elements).collect(),
            ranks: vec![0; elements],
        }
    }

    pub fn find(&mut self, element: usize) -> Option<usize> {
        if element >= self.parents.len() {
            return None;
        }

        let real_parent = match self.parents[element].cmp(&element) {
            Ordering::Equal => element,
            _ => self.find(self.parents[element]).unwrap(),
        };

        self.parents[element] = real_parent;
        Some(real_parent)
    }

    pub fn union(&mut self, elem1: usize, elem2: usize) -> Result<(), &'static str> {
        let (parent1, parent2) = match (self.find(elem1), self.find(elem2)) {
            (Some(p1), Some(p2)) => (p1, p2),
            _ => return Err("Invalid element"),
        };

        match self.ranks[parent1].cmp(&self.ranks[parent2]) {
            Ordering::Less => self.parents[parent1] = parent2,
            Ordering::Greater => self.parents[parent2] = parent1,
            Ordering::Equal => {
                self.parents[parent1] = parent2;
                self.ranks[parent2] += 1;
            }
        };

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.parents.len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let mut uf = super::UnionFind::new(10);

        for i in 0..10 {
            assert_eq!(uf.find(i), Some(i));
        }

        uf.union(1, 0).unwrap();
        uf.union(2, 0).unwrap();
        uf.union(3, 0).unwrap();
        uf.union(4, 0).unwrap();
        uf.union(5, 9).unwrap();
        uf.union(6, 9).unwrap();

        println!("rank of 0: {}", uf.ranks[0]);
        println!("rank of 9: {}", uf.ranks[9]);

        println!("Direct parent of 0: {}", uf.parents[0]);
        println!("Direct parent of 1: {}", uf.parents[1]);
        println!("Direct parent of 2: {}", uf.parents[2]);
        println!("Direct parent of 3: {}", uf.parents[3]);
        println!("Direct parent of 4: {}", uf.parents[4]);
        println!("Direct parent of 5: {}", uf.parents[5]);
        println!("Direct parent of 6: {}", uf.parents[6]);
        println!();
        uf.union(9, 0).unwrap();
        println!("Direct parent of 5: {}", uf.parents[5]);
        println!("Direct parent of 6: {}", uf.parents[6]);
        println!();
        uf.find(5);
        uf.find(6);
        println!("Direct parent of 5: {}", uf.parents[5]);
        println!("Direct parent of 6: {}", uf.parents[6]);

        assert_eq!(uf.find(0), uf.find(2));
        assert_eq!(uf.find(1), uf.find(2));
        assert_eq!(uf.find(0), uf.find(1));

        println!("Direct parent of 0: {}", uf.parents[0]);
        println!("Direct parent of 1: {}", uf.parents[1]);
        println!("Direct parent of 2: {}", uf.parents[2]);
        println!("Direct parent of 3: {}", uf.parents[3]);
        println!("Direct parent of 4: {}", uf.parents[4]);
    }
}
