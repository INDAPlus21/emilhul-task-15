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

        eprintln!("{:?}\n", auf);
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
    /// The id of each set in the Almost Union-Find.
    /// The id is the parent of i, if set_id[i] = i, i is a root node
    set_id: Vec<usize>,
}

impl AlmostUnionFind {
    /// ## new
    /// Creates a new AlmostUnionFind of size n
    fn new(n: usize) -> AlmostUnionFind {
        let num_sets = n;
        let set_size = vec![1;n];
        let mut set_id = Vec::with_capacity(n);

        for i in 0..n {
            set_id.push(i);
        }

        AlmostUnionFind { num_sets, set_size, set_id }
    }

    /// ## find
    /// Helper function that finds the root for a set. Also compresses the path there.
    fn find(&mut self, mut p: usize) -> usize {
        // Vectors are zero indexed while the AlmostUnionFind is one indexed.
        // Therefore subtract one from input.
        p -= 1;

        let mut root: usize = p;

        // Follow chain until root. Root is node with set_id equal to itself.
        while root != self.set_id[root] {
            root = self.set_id[root];
        }

        // Go back through chain compressing path
        while p != root {
            let next: usize = self.set_id[p];
            self.set_id[p] = root;
            p = next;
        }
        root
    }

    /// ## union
    /// Union the sets containing p and q
    fn union(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q); 

        // If they're not already in the same set
        if root_p != root_q {
            // Move the smaler set into the larger one.
            if self.set_size[root_p] < self.set_size[root_q] {
                self.set_size[root_q] += self.set_size[root_p];
                self.set_size[root_p] = 0;
                self.set_id[root_p] = root_q;
            } else {
                self.set_size[root_p] += self.set_size[root_q];
                self.set_size[root_q] = 0;
                self.set_id[root_q] = root_p;
            }

            self.num_sets -= 1;
        }
    }

    /// ## move
    /// Moves element p into the set containing q
    fn _move(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q);

        // If they're not already in the same set
        if root_p != root_q {
            self.set_size[root_q] += 1;
            self.set_size[root_p] -= 1;
            self.set_id[p-1] = root_q;

            // If p is root of a set containing other elements, some clean up will be necessary
            if p-1 == root_p && self.set_size[root_p] != 0 {
                let mut new_root: Option<usize> = None;
                for i in 0..self.set_id.len() {
                    // The first element that belonged to p's set becomes the new root
                    // Others update their set_id to match
                    if self.set_id[i] == root_p {
                        match new_root {
                            Some(_root) => self.set_id[i] = _root,
                            None => {
                                new_root = Some(i);
                                self.set_id[i] = i;
                                self.set_size[i] = self.set_size[root_p];
                                self.set_size[root_p] = 0;
                            }
                        }
                    }
                }
            }
        }
    }

    /// ## return
    /// Returns the size of the set containing p as well as the sum of all elements in that set.
    fn _return(&mut self, p: usize) -> (usize, usize) {
        let root_p = self.find(p);
        let size = self.set_size[root_p];
        let mut sum= 0;

        for i in 0..self.set_id.len() {
            if self.set_id[i] == root_p {
                sum += i + 1;
            }
        }
        (size, sum)
    }
}