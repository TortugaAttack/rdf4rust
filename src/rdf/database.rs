use crate::rdf::graph::Graph;
use std::collections::HashMap;

pub struct Database{
    default_graph: Box<dyn Graph>,
    named_graphs: HashMap<String, Box<dyn Graph>>
}