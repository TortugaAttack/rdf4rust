use crate::rdf::graph::{Graph, GraphType, IndexedGraph, FullIndexedGraph, SimpleGraph, Statement};
use std::collections::{HashMap, HashSet};
use crate::rdf::node_factory::{IRIResource, RDFNode};

pub struct Database{
    graph_type: GraphType,
    default_graph: Box<dyn Graph>,
    named_graphs: HashMap<String, Box<dyn Graph>>
}

impl Database{

    pub fn count(&self) -> usize{
        let mut count = self.default_graph.count();
        for (key, val) in &self.named_graphs{
            count+=val.count();
        }
        count
    }

    pub fn get_named_graphs(&self) -> &HashMap<String, Box<dyn Graph>>{
        &self.named_graphs
    }

    fn add_new_graph(&mut self, graph: &str){
        match self.graph_type {
            GraphType::SimpleGraph => {
                self.named_graphs.insert(String::from(graph), Box::new(SimpleGraph::new()));
            }
            GraphType::IndexedGraph => {
                self.named_graphs.insert(String::from(graph), Box::new(IndexedGraph::new()));
            }
            GraphType::FullIndexedGraph => {
                self.named_graphs.insert(String::from(graph), Box::new(SimpleGraph::new()));
            }
        }
    }

    pub fn add_statement(&mut self, graph: Option<IRIResource>, stmt: Statement){
        if let Some(graph_iri) = graph{
            let graph_str = graph_iri.as_string(false);
            match self.named_graphs.get_mut(&graph_str) {
                None => {
                    self.add_new_graph(graph_str.as_str());
                    match self.named_graphs.get_mut(&graph_str){
                        None => {}
                        Some(mut g) => {g.add_statement(stmt);}
                    }

                }
                Some(mut graph) => { graph.add_statement(stmt);}
            }
        }else{
            self.default_graph.add_statement(stmt);
        }
    }

    pub fn new(graph_type: GraphType)-> Self{
        match graph_type{
            GraphType::SimpleGraph => {
                Database{
                    graph_type: GraphType::SimpleGraph,
                    default_graph: Box::new(SimpleGraph::new()),
                    named_graphs: HashMap::new()
                }
            }
            GraphType::IndexedGraph => {
                Database{
                    graph_type: GraphType::IndexedGraph,
                    default_graph: Box::new(IndexedGraph::new()),
                    named_graphs: HashMap::new()
                }
            }
            _ => Database{
                graph_type: GraphType::SimpleGraph,
                default_graph: Box::new(SimpleGraph::new()),
                named_graphs: HashMap::new()
            }
        }

    }

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