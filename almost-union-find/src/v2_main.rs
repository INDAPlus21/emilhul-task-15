use std::io::{self, BufRead};

fn main() {
    // Get input
    let input = io::stdin();

    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.unwrap());
    
    // Get first line
    let values = lines.next().unwrap()
        .split_whitespace()
        .map(|_value| _value.parse::<usize>().ok().unwrap())
        .collect::<Vec<usize>>();

    if values.len() == 0 { return; }

    let (n, m) = (values[0], values[1]);

    if n == 0 || m == 0 { return; }

    // Create Almost Union Find datastructure of size n
    let mut auf = AlmostUnionFind::new(n);

    // For remaining lines in input
    for _ in 0..m {
        // Split up line at spaces
        let line = lines.next().unwrap()
        .split_whitespace()
        .map(|_value| _value.parse::<usize>().ok().unwrap())
        .collect::<Vec<usize>>();
    
        // Match first number to function
        match line[0] {
            1 => auf.union(line[1], line[2]),
            2 => auf._move(line[1], line[2]),
            3 => {
                let (size, sum) = auf._return(line[1]);
                println!("{} {}", size, sum);
            },
            _ => ()
        }
    }
}

#[derive(Debug)]
/// ## AlmostUnionFind
/// A datastructure that's almost union finde :)
struct AlmostUnionFind {
    /// Number of sets in the Almost Union-Find
    num_sets: usize,
    /// The size of each set in the Almost Union-Find
    set_size: Vec<usize>,
    /// The id of each element in the Almost Union-Find.
    /// element_id.0 is index in outer vector and element_id.1 is index in outer vector.
    element_id: Vec<(usize, usize)>,
    /// Representation of the Almost Union-Find as a vector of vectors
    sets: Vec<Vec<usize>>
}

impl AlmostUnionFind {
    /// ## new
    /// Creates a new AlmostUnionFind of size n
    fn new(n: usize) -> AlmostUnionFind {
        let num_sets = n;
        let set_size = vec![1;n];
        let mut element_id = Vec::with_capacity(n);
        let mut sets = Vec::with_capacity(n);

        for i in 0..n {
            element_id.push((i, 0));
            sets.push(vec![i+1])
        }

        AlmostUnionFind { num_sets, set_size, element_id, sets }
    }

    /// ## union
    /// Union the sets containing p and q
    fn union(&mut self, p: usize, q: usize) {
        let _p = self.element_id[p-1];
        let _q = self.element_id[q-1]; 

        // If they're not already in the same set
        if _p.0 != _q.0 {
            // Move the smaler set into the larger one.
            if self.set_size[_p.0] < self.set_size[_q.0] {
                // Switch id for all elements in the set
                for i in &self.sets[_p.0] {
                    self.element_id[*i-1] = (_q.0, self.set_size[_q.0] + self.element_id[*i-1].1);
                }
                // Change set sizes
                self.set_size[_q.0] += self.set_size[_p.0];
                self.set_size[_p.0] = 0;

                // Move append elements to other set. Then clear first set
                let mut _temp = self.sets[_q.0].clone();
                self.sets[_p.0].append(&mut _temp);
                self.sets[_q.0] = vec![];
            } else {
                // Same as above but with _p and _q switched
                for i in &self.sets[_q.0] {
                    self.element_id[*i-1] = (_p.0, self.set_size[_p.0] + self.element_id[*i-1].1);
                }
                self.set_size[_p.0] += self.set_size[_q.0];
                self.set_size[_q.0] = 0;
                let mut _temp = self.sets[_q.0].clone();
                self.sets[_p.0].append(&mut _temp);
                self.sets[_q.0] = vec![];
            }

            self.num_sets -= 1;
        }
    }

    /// ## move
    /// Moves element p into the set containing q
    fn _move(&mut self, p: usize, q: usize) {
        let _p = self.element_id[p-1];
        let _q = self.element_id[q-1]; 

        // If they're not already in the same set
        if _p.0 != _q.0 {
            // Swap remove is fast, and oreder doesn't matter
            let _temp = self.sets[_p.0].swap_remove(_p.1);
            self.sets[_q.0].push(_temp);
            // Update element id for p
            self.element_id[p-1] = (_q.0, self.set_size[_q.0]);


            // Update set_sizes
            self.set_size[_q.0] += 1;
            self.set_size[_p.0] -= 1;

            // Cleanup
            if self.set_size[_p.0] == 0 {
                // If empty that set is effectively removed
                self.num_sets -= 1;
            } else {
                // If not empty update the index of the previously last element in the set. 
                let last_elem = self.sets[_p.0][self.set_size[_p.0]-1];
                // Since swap remove was used switch to p's old index
                self.element_id[last_elem-1] = (_p.0, _p.1);
            }
        }
    }

    /// ## return
    /// Returns the size of the set containing p as well as the sum of all elements in that set.
    fn _return(&mut self, p: usize) -> (usize, usize) {
        let _p = self.element_id[p-1];
        let size = self.set_size[_p.0];
        let sum = self.sets[_p.0].iter().sum();

        (size, sum)
    }
}