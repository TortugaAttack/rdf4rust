use std::collections::{HashMap, HashSet};
use crate::rdf::node_factory::{RDFNode, URIResource, Literal, BlankNode};
use std::convert::TryInto;

///
/// Graph trait to define all functions a graph implementation should be able to execute
///
pub trait Graph{
    fn count(&self)-> u64;
    fn list_all_statements(&self) -> &Vec<Statement>;
    fn add_statement(&self,stmt: Statement);
    fn add_all_statements(&self,stmts: &Vec<Statement>);
    fn remove_statement(&self,stmt: Statement);
    fn remove_all_statements(&self,stmts: &Vec<Statement>);
    fn list_statements_for_subject(&self,subject: ResourceNode) -> Vec<&Statement>;
    fn list_statements_for_object(&self,object: ResourceNode) -> Vec<&Statement>;
    fn list_statements_for_predicate(&self,predicate: ResourceNode) -> Vec<&Statement>;
    fn list_statements(&self,subject: Option<ResourceNode>, predicate: Option<URIResource>, object: Option<Node>) -> Vec<&Statement>;
    fn get_name(&self) -> Option<String>;
    fn load(&self) -> dyn Graph;
    fn store(&self);
}

pub enum ResourceNode{
    URINode{
        uri: URIResource
    },
    BNode{
        bnode:BlankNode
    }
}

impl ResourceNode {
    pub fn is_bnode(&self)->bool{
        match &self{
            ResourceNode::URINode { .. } => {false}
            ResourceNode::BNode { .. } => {true}
        }
    }
    pub fn is_uri(&self)->bool{
        match &self{
            ResourceNode::URINode { .. } => {true}
            ResourceNode::BNode { .. } => {false}
        }
    }

    pub fn as_uri_resource(&self) -> Result<&URIResource,&str> {
        match self {
            ResourceNode::URINode {  uri} => { Ok(uri)}
            ResourceNode::BNode { .. } => { Err("Blank Node cannot be converted to URI resource.")}
        }
    }

    pub fn as_blank_node(&self) -> Result<&BlankNode,&str> {
        match self {
            ResourceNode::URINode { .. } => { Err("URI Node cannot be converted to Blank Node.")}
            ResourceNode::BNode { bnode } => { Ok(bnode)}
        }
    }
}

pub enum Node{
    URINode{
        uri: URIResource
    },
    LiteralNode {
        literal: Literal
    },
    BNode{
        bnode:BlankNode
    }
}

impl Node{
    pub fn is_literal(&self)->bool{
        match &self{
            Node::URINode { .. } => {false}
            Node::LiteralNode { .. } => {true}
            Node::BNode { .. } => {false}
        }
    }
    pub fn is_bnode(&self)->bool{
        match &self{
            Node::URINode { .. } => {false}
            Node::LiteralNode { .. } => {false}
            Node::BNode { .. } => {true}
        }
    }
    pub fn is_uri(&self)->bool{
        match &self{
            Node::URINode { .. } => {true}
            Node::LiteralNode { .. } => {false}
            Node::BNode { .. } => {false}
        }
    }

    pub fn as_uri_resource(&self) -> Result<&URIResource,&str> {
        match self {
            Node::URINode { uri } => { Ok(uri)}
            Node::LiteralNode { .. } => {Err("Cannot convert Literal to URI Resource")}
            Node::BNode { .. } => {Err("Cannot convert Blank node to URI Resource")}
        }
    }

    pub fn as_blank_node(&self) -> Result<&BlankNode,&str> {
        match self {
            Node::BNode { bnode } => {Ok(bnode)}
            Node::URINode { .. } => { Err("Cannot convert URI Node to Blank Node")}
            Node::LiteralNode { .. } => {Err("Cannot convert Literal to Blank Node")}
        }
    }

    pub fn as_literal(&self) -> Result<&Literal,&str> {
        match self {
            Node::LiteralNode { literal } => {Ok(literal)}
            Node::URINode { .. } => { Err("Cannot convert URI Node to Literal")}
            Node::BNode { .. } => {Err("Cannot convert Blank node to Literal")}
        }
    }
}



///
/// Basic Statement Structure
///
/// A Statement is a Triple of subject, predicate and object
///
/// * `subject` - `URIResource` or `BlankNode`
/// * `predicate` - `URIResource`
/// * `object` - `URIResource` or `BlankNode` or `Literal`
///
/// Hence Variables are not allowed, which makes the distinction to a plain `Triple` where all can be a `Variable` as well.
pub struct Statement{
    subject: ResourceNode,
    predicate: URIResource,
    object: Node
}

impl Statement{
    pub fn get_subject(&self)-> &ResourceNode{
        &self.subject
    }

    pub fn get_predicate(&self)-> &URIResource{
        &self.predicate
    }

    pub fn get_object(&self)-> &Node{
        &self.object
    }
}

///
/// Simple Graph storing all Statements in a Set.
///
/// Thus it is fast to build, but has a bad performance.
/// The bigger the graph the worse the performance gets.
///
/// # Query Evaluation
///
/// Evaluates queries of any type by iterating over all statements.
///
/// Thus complexity is always O(N)
///
pub struct SimpleGraph{
    statements: HashSet<Statement>
}


///
/// Uses SPO and OPS indexes to assure a faster performance in trait for a higher Memory consumption.
///
/// However this Graph still has a bad performance for queries like `?s <p> ?o`
///
/// # Query Evaluation
///
/// Evaluates queries of type `<s> ?p ?o` (resp. `?s ?p <o>`) as follows
///
/// 1. Return all Statements from SPO index for `<s>` (-> O(1))
///
/// Evaluates queries of type `<s> <p> ?o` (resp. `?s <p> <o>`) as follows
///
/// 1. Get `<s> ?p ?p` from SPO index
/// 2. for all results, filter for `<p>`
///
/// Thus complexity is O(M), whereas `M` is the size of Statements concerning `<s>`.
///
///
/// Evaluates queries of type `?s <p> ?o` by iterating over all values in the map.
/// If you need such queries use either a `SimpleGraph` or a `FullIndexedGraph`
pub struct IndexedGraph{
    spo: HashMap<ResourceNode, HashSet<Statement>>,
    ops: HashMap<Node, HashSet<Statement>>,
}

///
/// Uses SPO, PSO and OPS indexes to assure a faster performance in trait for a higher Memory consumption.
///
/// Be aware that this means all Statements will be stored three times in a `HashMap`.
///
/// # Query Evaluation
///
/// Evaluates queries of type `<s> ?p ?o` (resp. `?s ?p <o>` and `?s <p> ?o`) as follows
///
/// 1. Return all Statements from SPO index for `<s>` (-> O(1))
///
/// Evaluates queries of type `<s> <p> ?o` (resp. `?s <p> <o>`) as follows
///
/// 1. Get `<s> ?p ?p` from SPO index
/// 2. for all results, filter for `<p>`
///
/// Thus complexity is O(M), whereas `M` is the size of Statements concerning `<s>`.
pub struct FullIndexedGraph{
    spo: HashMap<ResourceNode, HashSet<Statement>>,
    ops: HashMap<URIResource, HashSet<Statement>>,
    pso: HashMap<Node, HashSet<Statement>>
}