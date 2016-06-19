use std::cmp;

struct LevenshteinMatrix {
    size_a: usize,
    size_b: usize,
    elements: Vec<u32>
}

impl LevenshteinMatrix {
    fn element(&self, i: usize, j: usize) -> u32 {
        if self.size_a <= i || self.size_b <= j {
            panic!("invalid element")
        }

        self.elements[self.size_a * j + i]
    }

    fn set(&mut self, i: usize, j: usize, value: u32) {
        if self.size_a <= i || self.size_b <= j {
            panic!("invalid element")
        }

        self.elements[self.size_a * j + i] = value;
    }

    fn new(length_a: usize, length_b: usize) -> LevenshteinMatrix {
        let size_a = length_a + 1;
        let size_b = length_b + 1;

        let mut v : Vec<u32> = vec![0; size_a * size_b];

        // initialize first row and column
        if 0 < size_a && 0 < size_a {
            for i in 0..size_a {
                v[i] = i as u32;
            }
            for j in 0..size_b {
                v[size_a * j] = j as u32;
            }
        }

        LevenshteinMatrix { size_a: size_a, size_b: size_b, elements: v }
    }
}

fn main() {
    let a = "kitten";
    let b = "sitting";

    println!("distance between 'kitten' and 'sitting' is {}", levenshtein_distance(&a, &b));
}

// based on https://en.wikipedia.org/wiki/Levenshtein_distance#Iterative_with_full_matrix
fn levenshtein_distance(a: &str, b: &str) -> u32 {
    let mut m = LevenshteinMatrix::new(a.len(), b.len());

    // iterate left-to-right, top-to-bottom over the matrix
    for j in 1..m.size_b {
        for i in 1..m.size_a {
            // are last characters of a[0..i] and b[0..j] the same
            let last_character_same = if a[0..i].to_string().pop() == b[0..j].to_string().pop() { true } else { false };

            // now let's compute the distance between a[0..i] and b[0..j]

            let distance_by_substitution = m.element(i-1, j-1) + if last_character_same { 0 } else { 1 };
            let distance_by_deletion = m.element(i-1, j) + 1;
            let distance_by_insertion = m.element(i, j-1) + 1;

            let best_distance = cmp::min(distance_by_substitution,
                                            cmp::min(distance_by_deletion, distance_by_insertion));

            m.set(i, j, best_distance);
        }
    }

    m.element(a.len(), b.len())
}

#[test]
fn test_distances() {
    assert_eq!(0, levenshtein_distance("", ""));
    assert_eq!(1, levenshtein_distance("a", ""));
    assert_eq!(1, levenshtein_distance("cat", "hat"));
    assert_eq!(3, levenshtein_distance("kitten", "sitting"));
}
