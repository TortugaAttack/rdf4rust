use crate::rdf::graph::Graph;
use std::collections::{HashMap, HashSet};

pub struct Database{
    default_graph: Box<dyn Graph>,
    named_graphs: HashMap<String, Box<dyn Graph>>
}

impl Database{

    pub fn get_default_graph(&self) -> &Box<dyn Graph> {
        &self.default_graph
    }

    pub fn get_named_graph_names(&self) -> HashSet<&String>{
        let mut ret = HashSet::new();
        for key in self.named_graphs.keys(){
            ret.insert(key);
        }
        ret
    }

    pub fn get_named_graph(&self, name: &str) -> Option<&Box<dyn Graph>>{
        self.named_graphs.get(name)
    }

}