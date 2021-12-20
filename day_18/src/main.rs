use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
    path::Path,
    str::FromStr,
};

#[derive(Debug)]
struct ParseErr(String);

impl From<ParseIntError> for ParseErr {
    fn from(e: ParseIntError) -> Self {
        ParseErr(e.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SFNumType<T> {
    Pair,
    Num(T),
    None,
}

impl<T> SFNumType<T> {
    fn num(&self) -> &T {
        match self {
            SFNumType::Num(t) => t,
            SFNumType::Pair => panic!("tried to get value of Pair"),
            SFNumType::None => panic!("tried to get value of None"),
        }
    }

    fn num_mut(&mut self) -> &mut T {
        match self {
            SFNumType::Num(t) => t,
            SFNumType::Pair => panic!("tried to get value of Pair"),
            SFNumType::None => panic!("tried to get value of None"),
        }
    }
}

// snailfish numbers can be represented as binary trees
#[derive(Debug, PartialEq, Eq, Clone)]
struct SFNumTree {
    data: Vec<SFNumType<u32>>,
    num_order: Vec<usize>,
}

impl SFNumTree {
    fn parent_idx(idx: usize) -> usize {
        match idx {
            0 => 0,
            _ => (idx - 1) / 2,
        }
    }

    fn left_idx(idx: usize) -> usize {
        idx * 2 + 1
    }

    fn left(&self, idx: usize) -> (usize, Option<&SFNumType<u32>>) {
        let left_idx = SFNumTree::left_idx(idx);
        (left_idx, self.data.get(left_idx))
    }

    fn right_idx(idx: usize) -> usize {
        idx * 2 + 2
    }

    fn right(&self, idx: usize) -> (usize, Option<&SFNumType<u32>>) {
        let right_idx = SFNumTree::right_idx(idx);
        (right_idx, self.data.get(right_idx))
    }

    fn clear_trailing_none(&mut self) {
        for (idx, sf_num) in self.data.iter().enumerate().rev() {
            match sf_num {
                SFNumType::None => (),
                _ => {
                    self.data.truncate(idx + 1); // keep everything up to and including this element
                    return;
                }
            }
        }
    }

    fn explode(&mut self, idx: usize) {
        let (left_idx, left_val) = self.left(idx);
        let left_val = *left_val.unwrap().num();
        let (right_idx, right_val) = self.right(idx);
        let right_val = *right_val.unwrap().num();

        for (i, order_idx) in self.num_order.iter().enumerate() {
            if *order_idx == left_idx {
                if i > 0 {
                    *self.data[self.num_order[i - 1]].num_mut() += left_val;
                }
                if i < self.num_order.len() - 2 {
                    *self.data[self.num_order[i + 2]].num_mut() += right_val;
                }
                self.num_order[i] = SFNumTree::parent_idx(*order_idx);
                self.num_order.remove(i + 1);
                break;
            }
        }

        self.data[idx] = SFNumType::Num(0);
        self.data[left_idx] = SFNumType::None;
        self.data[right_idx] = SFNumType::None;
        self.clear_trailing_none();
    }

    fn split(&mut self, idx: usize) {
        let val = *self.data[idx].num();
        let left_val = val / 2;
        let right_val = (val + 1) / 2;

        let left_idx = SFNumTree::left_idx(idx);
        let right_idx = left_idx + 1;
        if self.data.len() <= right_idx {
            self.data.resize(right_idx + 1, SFNumType::None);
        }

        self.data[idx] = SFNumType::Pair;
        self.data[left_idx] = SFNumType::Num(left_val);
        self.data[right_idx] = SFNumType::Num(right_val);

        for (i, order_idx) in self.num_order.iter().enumerate() {
            if *order_idx == idx {
                self.num_order[i] = left_idx;
                self.num_order.insert(i + 1, right_idx);
                break;
            }
        }
    }

    fn to_string(&self, idx: usize) -> String {
        match self.data[idx] {
            SFNumType::Pair => format!(
                "[{},{}]",
                self.to_string(SFNumTree::left_idx(idx)),
                self.to_string(SFNumTree::right_idx(idx))
            ),
            SFNumType::Num(num) => num.to_string(),
            SFNumType::None => String::new(),
        }
    }

    fn reduce(&mut self) {
        'outer: loop {
            for idx in 15..=30 {
                if let Some(SFNumType::Pair) = self.data.get(idx) {
                    self.explode(idx);
                    continue 'outer;
                }
            }
            for i in 0..self.num_order.len() {
                let idx = self.num_order[i];
                if *self.data[idx].num() >= 10 {
                    self.split(idx);
                    continue 'outer;
                }
            }
            break;
        }
    }

    fn add(&self, tree: &SFNumTree) -> SFNumTree {
        // i originally tried to be smart about this, but this was easier
        let mut new_tree: SFNumTree = format!("[{},{}]", self.to_string(0), tree.to_string(0))
            .parse()
            .unwrap();
        new_tree.reduce();
        new_tree
    }

    fn magnitude(&self, idx: usize) -> u32 {
        match self.data[idx] {
            SFNumType::Pair => {
                3 * self.magnitude(SFNumTree::left_idx(idx))
                    + 2 * self.magnitude(SFNumTree::right_idx(idx))
            }
            SFNumType::Num(num) => num,
            SFNumType::None => 0,
        }
    }
}

impl FromStr for SFNumTree {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::new();
        let mut num_order = Vec::new();
        let mut idx = 0;
        let mut current_num = String::new();
        for c in s.chars() {
            if data.len() <= idx {
                data.resize(idx + 1, SFNumType::None);
            }
            match c {
                '[' => {
                    data[idx] = SFNumType::Pair;
                    idx = SFNumTree::left_idx(idx);
                }
                ',' | ']' => {
                    if !current_num.is_empty() {
                        data[idx] = SFNumType::Num(current_num.parse()?);
                        current_num.clear();
                        num_order.push(idx);
                    }
                    match c {
                        ',' => idx += 1,
                        ']' => idx = SFNumTree::parent_idx(idx),
                        _ => unreachable!(),
                    }
                }
                _ => current_num.push(c),
            }
        }
        Ok(SFNumTree { data, num_order })
    }
}

fn read_file(filename: impl AsRef<Path>) -> Vec<SFNumTree> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .filter_map(|line| line.unwrap().parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use self::TreeSide::*;
    use super::*;

    #[derive(Clone, Copy)]
    enum TreeSide {
        Left,
        Right,
    }

    impl SFNumTree {
        fn chain(
            &self,
            start_idx: usize,
            chain: Vec<TreeSide>,
        ) -> (usize, Option<&SFNumType<u32>>) {
            let mut idx = start_idx;
            for side in &chain {
                idx = idx * 2
                    + match side {
                        TreeSide::Left => 1,
                        TreeSide::Right => 2,
                    };
            }
            (idx, self.data.get(idx))
        }
    }

    fn test_explode(s1: &str, s2: &str, chain: Vec<TreeSide>) {
        let mut tree: SFNumTree = s1.parse().unwrap();
        let (idx, _) = tree.chain(0, chain);
        tree.explode(idx);
        assert_eq!(tree, s2.parse().unwrap());
    }

    fn test_split(num: u32, s: &str) {
        let mut tree: SFNumTree = format!("[{},0]", num).parse().unwrap();
        tree.split(1);
        assert_eq!(tree, format!("[{},0]", s).parse().unwrap());
    }

    fn test_add(s1: &str, s2: &str, result: &str) {
        let mut tree: SFNumTree = s1.parse().unwrap();
        tree = tree.add(&s2.parse().unwrap());
        assert_eq!(tree, result.parse().unwrap())
    }

    #[test]
    fn parse_num() {
        let tree: SFNumTree = "[[[10,[3,8]],[[0,9],6]],[[[13,7],[4,9]],3]]"
            .parse()
            .unwrap();
        assert_eq!(*tree.chain(0, vec![Left, Left, Left]).1.unwrap().num(), 10);
        assert_eq!(
            *tree
                .chain(0, vec![Left, Left, Right, Right])
                .1
                .unwrap()
                .num(),
            8
        );
        assert_eq!(
            *tree
                .chain(0, vec![Right, Left, Left, Left])
                .1
                .unwrap()
                .num(),
            13
        );
        assert_eq!(*tree.chain(0, vec![Right, Right]).1.unwrap().num(), 3);
        assert_eq!(
            tree.num_order
                .iter()
                .filter_map(|i| match tree.data[*i] {
                    SFNumType::Num(num) => Some(num),
                    _ => None,
                })
                .collect::<Vec<_>>(),
            vec![10, 3, 8, 0, 9, 6, 13, 7, 4, 9, 3]
        )
    }

    #[test]
    fn to_string() {
        let s = "[[[10,[3,8]],[[0,9],6]],[[[13,7],[4,9]],3]]";
        assert_eq!(s, s.parse::<SFNumTree>().unwrap().to_string(0));
    }

    #[test]
    fn explode_with_no_next_left() {
        test_explode(
            "[[[[[9,8],1],2],3],4]",
            "[[[[0,9],2],3],4]",
            vec![Left, Left, Left, Left],
        );
    }

    #[test]
    fn explode_with_no_next_right() {
        test_explode(
            "[7,[6,[5,[4,[3,2]]]]]",
            "[7,[6,[5,[7,0]]]]",
            vec![Right, Right, Right, Right],
        );
    }

    #[test]
    fn explode_with_next_left_and_right() {
        test_explode(
            "[[6,[5,[4,[3,2]]]],1]",
            "[[6,[5,[7,0]]],3]",
            vec![Left, Right, Right, Right],
        );
    }

    #[test]
    fn explode_with_multiple_pairs_1() {
        test_explode(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            vec![Left, Right, Right, Right],
        );
    }

    #[test]
    fn explode_with_multiple_pairs_2() {
        test_explode(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            vec![Right, Right, Right, Right],
        );
    }

    #[test]
    fn split_10() {
        test_split(10, "[5,5]");
    }

    #[test]
    fn split_11() {
        test_split(11, "[5,6]");
    }

    #[test]
    fn split_12() {
        test_split(12, "[6,6]");
    }

    #[test]
    fn add_without_reduce() {
        test_add("[1,2]", "[[3,4],5]", "[[1,2],[[3,4],5]]");
    }

    #[test]
    fn add_with_reduce() {
        test_add(
            "[[[[4,3],4],4],[7,[[8,4],9]]]",
            "[1,1]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        );
    }

    #[test]
    fn magnitude_143() {
        assert_eq!(
            "[[1,2],[[3,4],5]]"
                .parse::<SFNumTree>()
                .unwrap()
                .magnitude(0),
            143
        );
    }

    #[test]
    fn magnitude_1384() {
        assert_eq!(
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
                .parse::<SFNumTree>()
                .unwrap()
                .magnitude(0),
            1384
        );
    }

    #[test]
    fn magnitude_29() {
        assert_eq!("[9,1]".parse::<SFNumTree>().unwrap().magnitude(0), 29);
    }
}

fn main() {
    let input = read_file("data/input.txt");

    // part 1
    let mut tree = input[0].clone();
    for rhs in input.iter().skip(1) {
        tree = tree.add(rhs);
    }
    println!("Part 1: magnitude = {}", tree.magnitude(0));

    // part 2
    let mut max_magnitude = 0;
    for lhs in &input {
        for rhs in &input {
            let tree = lhs.add(rhs);
            max_magnitude = max_magnitude.max(tree.magnitude(0));
        }
    }
    println!("Part 2: max_magnitude = {}", max_magnitude);
}
