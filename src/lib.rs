mod graph {
    #[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
    // Struct lifetime example
    // http://www.charlesetc.com/rust/2015/10/29
    pub struct Node<'c> {
        pub value: String,
        pub children: Vec<&'c Node<'c>>
    }

    impl<'c> Node<'c> {
        // translation of Ruby
        // def dfs(n)
        //   p "node: #{n.value}"
        //   [n] + n.children.flat_map(&method(:dfs))
        // end
        pub fn dfs(&self) -> Vec<&Node> {
            println!("node: {:?}", self.value);

            let mut v = vec![self];
            v.extend(
                self.children.
                    iter().
                    flat_map (|n| n.dfs() ).
                    collect::<Vec<_>>()
            );

            v
        }
    }
}

#[cfg(test)]
mod tests {
    use super::graph::Node;

    #[test]
    fn it_works() {
        let c = Node { value: String::from("child"), children: vec![] };
        let p = Node { value: String::from("parent"), children: vec![&c] };
        let gp = Node { value: String::from("grandparent"), children: vec![&p] };

        assert_eq!(String::from("grandparent"), gp.value);
        assert_eq!(vec![&p], gp.children);

        assert_eq!(vec![&gp, &p, &c], gp.dfs());
    }
}
