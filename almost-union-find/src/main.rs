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
    /// The size of each set in the Almost Union-Find
    set_size: Vec<usize>,
    /// The id of each set in the Almost Union-Find.
    /// The id is the parent of i, if set_id[i] = i, i is a root node
    set_id: Vec<usize>,

    set_sum: Vec<usize>,
}

impl AlmostUnionFind {
    /// ## new
    /// Creates a new AlmostUnionFind of size n
    fn new(n: usize) -> AlmostUnionFind {
        let set_size = vec![1;2*(n+1)];
        let mut set_id = vec![0;2*(n+1)];
        let mut set_sum = vec![0;2*(n+1)];

        let mut j = n+1;

        for i in 1..=n {
            set_id[i] = j;
            set_id[j] = j;
            set_sum[j] = i;
            j += 1;
        }

        AlmostUnionFind {  set_size, set_id, set_sum }
    }

    /// ## find
    /// Helper function that finds the root for a set. Also compresses the path there.
    fn find(&mut self, mut p: usize) -> usize {
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
            self.set_size[root_q] += self.set_size[root_p];
            self.set_sum[root_q] += self.set_sum[root_p];
            self.set_id[root_p] = root_q;
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
            self.set_sum[root_q] += p;
            self.set_sum[root_p] -= p;

            self.set_id[p] = root_q;
        }
    }

    /// ## return
    /// Returns the size of the set containing p as well as the sum of all elements in that set.
    fn _return(&mut self, p: usize) -> (usize, usize) {
        let root_p = self.find(p);
        let size = self.set_size[root_p];
        let sum = self.set_sum[root_p];
        (size, sum)
    }
}