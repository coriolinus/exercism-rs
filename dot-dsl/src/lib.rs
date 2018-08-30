use std::collections::HashMap;
type Attrs = HashMap<String, String>;
type AttrsIn<'a, 'b, 'c> = &'a [(&'b str, &'c str)];

fn to_attrs(input: AttrsIn) -> Attrs {
    input
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

pub mod graph {
    pub mod graph_items {
        pub mod edge {
            use {to_attrs, Attrs, AttrsIn};

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Edge {
                vertices: (String, String),
                attrs: Attrs,
            }

            impl Edge {
                pub fn new(v1: &str, v2: &str) -> Edge {
                    Edge {
                        vertices: (v1.to_string(), v2.to_string()),
                        attrs: Attrs::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: AttrsIn) -> Edge {
                    self.attrs = to_attrs(attrs);
                    self
                }
            }
        }
        pub mod node {
            use {to_attrs, Attrs, AttrsIn};

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Node {
                name: String,
                attrs: Attrs,
            }

            impl Node {
                pub fn new(name: &str) -> Node {
                    Node {
                        name: name.to_string(),
                        attrs: Attrs::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: AttrsIn) -> Node {
                    self.attrs = to_attrs(attrs);
                    self
                }
            }
        }
    }

    use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;
    use {to_attrs, Attrs, AttrsIn};

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: Attrs,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: Attrs::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Graph {
            self.nodes = nodes.iter().cloned().collect();
            self
        }

        pub fn with_attrs(mut self, attrs: AttrsIn) -> Graph {
            self.attrs = to_attrs(attrs);
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Graph {
            self.edges = edges.iter().cloned().collect();
            self
        }
    }

}
