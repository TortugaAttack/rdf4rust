use std::collections::{HashMap, HashSet};
use crate::rdf::node_factory::{RDFNode, IRIResource, Literal, BlankNode};
use crate::io::reader::{parse_resolved_object, ParserError};
use crate::util::iri::IRI;
use crate::rdf::graph::ResourceNode::{BNode, IRINode};
use std::hash::Hash;
use std::fmt::Display;
use std::fmt;

///
/// Graph trait to define all functions a graph implementation should be able to execute
///
pub trait Graph{
    fn count(&self)-> usize;
    fn list_all_statements(&self) -> &Vec<Statement>;
    fn add_statement(&mut self,stmt: Statement);
    fn add_all_statements(&self,stmts: &Vec<Statement>);
    fn remove_statement(&self,stmt: Statement);
    fn remove_all_statements(&self,stmts: &Vec<Statement>);
    fn list_statements_for_subject(&self,subject: ResourceNode) -> Vec<&Statement>;
    fn list_statements_for_object(&self,object: ResourceNode) -> Vec<&Statement>;
    fn list_statements_for_predicate(&self,predicate: ResourceNode) -> Vec<&Statement>;
    fn list_statements(&self, subject: Option<ResourceNode>, predicate: Option<IRIResource>, object: Option<Node>) -> Vec<&Statement>;
    fn get_name(&self) -> Option<String>;
    fn load(&self);
    fn store(&self);
    fn print(&self);
}

pub enum GraphType{
    SimpleGraph,
    IndexedGraph,
    FullIndexedGraph
}

#[derive(PartialOrd, PartialEq, Eq, Hash)]
pub enum ResourceNode{
    IRINode {
        iri: IRIResource
    },
    BNode{
        bnode:BlankNode
    }
}



impl ResourceNode {

    pub fn as_string(&self) -> String {
        match &self{
            ResourceNode::IRINode { iri } => {iri.as_string(false)}
            ResourceNode::BNode { bnode } => {bnode.as_string(true)}
        }
    }

    pub fn is_bnode(&self)->bool{
        match &self{
            ResourceNode::IRINode { .. } => {false}
            ResourceNode::BNode { .. } => {true}
        }
    }
    pub fn is_uri(&self)->bool{
        match &self{
            ResourceNode::IRINode { .. } => {true}
            ResourceNode::BNode { .. } => {false}
        }
    }

    pub fn as_uri_resource(&self) -> Result<&IRIResource,&str> {
        match self {
            ResourceNode::IRINode { iri: uri } => { Ok(uri)}
            ResourceNode::BNode { .. } => { Err("Blank Node cannot be converted to URI resource.")}
        }
    }

    pub fn as_blank_node(&self) -> Result<&BlankNode,&str> {
        match self {
            ResourceNode::IRINode { .. } => { Err("URI Node cannot be converted to Blank Node.")}
            ResourceNode::BNode { bnode } => { Ok(bnode)}
        }
    }
}

#[derive(PartialOrd, PartialEq, Eq, Hash)]
pub enum Node{
    IRINode {
        iri: IRIResource
    },
    LiteralNode {
        literal: Literal
    },
    BNode{
        bnode:BlankNode
    }
}

impl Node{

    pub fn as_string(&self) -> String{
        match &self{
            Node::IRINode { iri } => {iri.as_string(false)}
            Node::LiteralNode { literal } => {literal.as_string(true)}
            Node::BNode { bnode } => {bnode.as_string(true)}
        }
    }

    pub fn is_literal(&self)->bool{
        match &self{
            Node::IRINode { .. } => {false}
            Node::LiteralNode { .. } => {true}
            Node::BNode { .. } => {false}
        }
    }
    pub fn is_bnode(&self)->bool{
        match &self{
            Node::IRINode { .. } => {false}
            Node::LiteralNode { .. } => {false}
            Node::BNode { .. } => {true}
        }
    }
    pub fn is_uri(&self)->bool{
        match &self{
            Node::IRINode { .. } => {true}
            Node::LiteralNode { .. } => {false}
            Node::BNode { .. } => {false}
        }
    }

    pub fn as_uri_resource(&self) -> Result<&IRIResource,&str> {
        match self {
            Node::IRINode { iri: uri } => { Ok(uri)}
            Node::LiteralNode { .. } => {Err("Cannot convert Literal to URI Resource")}
            Node::BNode { .. } => {Err("Cannot convert Blank node to URI Resource")}
        }
    }

    pub fn as_blank_node(&self) -> Result<&BlankNode,&str> {
        match self {
            Node::BNode { bnode } => {Ok(bnode)}
            Node::IRINode { .. } => { Err("Cannot convert URI Node to Blank Node")}
            Node::LiteralNode { .. } => {Err("Cannot convert Literal to Blank Node")}
        }
    }

    pub fn as_literal(&self) -> Result<&Literal,&str> {
        match self {
            Node::LiteralNode { literal } => {Ok(literal)}
            Node::IRINode { .. } => { Err("Cannot convert URI Node to Literal")}
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
#[derive(PartialOrd, PartialEq, Eq, Hash)]
pub struct Statement{
    subject: ResourceNode,
    predicate: IRIResource,
    object: Node
}

impl Display for Statement{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let subj = match &self.subject{
            ResourceNode::IRINode { iri } => {iri.as_string(true)}
            ResourceNode::BNode { bnode } => {bnode.as_string(true)}
        };
        let str = match &self.object{
            Node::IRINode { iri } => {
                format!("{} <{}> <{}> .", subj, self.predicate.get_iri(), iri.get_iri())}
            Node::LiteralNode { literal } => {
                format!("{} <{}> {} .", subj, self.predicate.get_iri(), literal.as_string(true))
            }
            Node::BNode { bnode } => {
                format!("{} <{}> {} .", subj, self.predicate.get_iri(), bnode.as_string(true))
            }
        };
        write!(f,"{}",str)
    }

}

impl Statement{

    pub fn create(subject: ResourceNode,
                  predicate: IRIResource,
                  object: Node) -> Statement{
        Statement{
            subject,
            predicate,
            object
        }
    }


    pub fn get_subject(&self)-> &ResourceNode{
        &self.subject
    }

    pub fn get_predicate(&self)-> &IRIResource {
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

impl Graph for SimpleGraph{
    fn count(&self) -> usize {
        self.statements.len()
    }

    fn list_all_statements(&self) -> &Vec<Statement> {
        todo!()
    }

    fn add_statement(&mut self, stmt: Statement) {
       &self.statements.insert(stmt);
    }

    fn add_all_statements(&self, stmts: &Vec<Statement>) {
        todo!()
    }

    fn remove_statement(&self, stmt: Statement) {
        todo!()
    }

    fn remove_all_statements(&self, stmts: &Vec<Statement>) {
        todo!()
    }

    fn list_statements_for_subject(&self, subject: ResourceNode) -> Vec<&Statement> {
        todo!()
    }

    fn list_statements_for_object(&self, object: ResourceNode) -> Vec<&Statement> {
        todo!()
    }

    fn list_statements_for_predicate(&self, predicate: ResourceNode) -> Vec<&Statement> {
        todo!()
    }

    fn list_statements(&self, subject: Option<ResourceNode>, predicate: Option<IRIResource>, object: Option<Node>) -> Vec<&Statement> {
        todo!()
    }

    fn get_name(&self) -> Option<String> {
        todo!()
    }

    fn load(&self) {
        todo!()
    }

    fn store(&self) {
        todo!()
    }

    fn print(&self) {
        {
            for stmt in &self.statements{
                println!("{}", stmt);
            }
        }
    }
}

impl SimpleGraph{
    pub fn new()-> SimpleGraph{
        SimpleGraph{
            statements: HashSet::new()
        }
    }

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
/// 1. Return all Statements from SPO index for `<s>` (-> O(M))
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
    statements: Vec<Statement>,
    spo: HashMap<String, HashSet<usize>>,
    ops: HashMap<String, HashSet<usize>>
}

impl IndexedGraph{
    pub fn new()-> Self{
        IndexedGraph{
            statements: Vec::new(),
            spo: HashMap::new(),
            ops: HashMap::new()
        }
    }
}

impl Graph for IndexedGraph{
    fn count(&self) -> usize {
        let mut count: usize = 0;
        self.spo.values().for_each(|x| {count+=x.len();});
        count
    }

    fn list_all_statements(&self) -> &Vec<Statement> {
        todo!()
    }

    fn add_statement(&mut self, stmt: Statement) {
        let mut index = self.statements.len();

        let subj_str = (&stmt).subject.as_string();
        if !self.spo.contains_key(&subj_str){
            self.spo.insert(subj_str, HashSet::new());
        }
        let obj_str = (&stmt).object.as_string();
        if !self.ops.contains_key(&obj_str){
            self.ops.insert(obj_str, HashSet::new());
        }
        let subj_str = (&stmt).subject.as_string();
        let obj_str = (&stmt).object.as_string();
        self.spo.get_mut(&subj_str).expect("").insert(index);
        self.ops.get_mut(&obj_str).expect("").insert(index);
        self.statements.push(stmt);

    }

    fn add_all_statements(&self, stmts: &Vec<Statement>) {
        todo!()
    }

    fn remove_statement(&self, stmt: Statement) {
        todo!()
    }

    fn remove_all_statements(&self, stmts: &Vec<Statement>) {
        todo!()
    }

    fn list_statements_for_subject(&self, subject: ResourceNode) -> Vec<&Statement> {
        let mut ret:Vec<&Statement> = Vec::new();
        for &x in self.spo.get(&subject.as_string()).expect("").iter(){
            ret.push(&self.statements[x]);
        }
        ret
    }

    fn list_statements_for_object(&self, object: ResourceNode) -> Vec<&Statement> {
        let mut ret:Vec<&Statement> = Vec::new();
        for &x in self.ops.get(&object.as_string()).expect("").iter(){
            ret.push(&self.statements[x]);
        }
        ret
    }

    fn list_statements_for_predicate(&self, predicate: ResourceNode) -> Vec<&Statement> {
        todo!()
    }

    fn list_statements(&self, subject: Option<ResourceNode>, predicate: Option<IRIResource>, object: Option<Node>) -> Vec<&Statement> {
        todo!()
    }

    fn get_name(&self) -> Option<String> {
        todo!()
    }

    fn load(&self) {
        todo!()
    }

    fn store(&self) {
        todo!()
    }

    fn print(&self) {
        for stmt in &self.statements {
            println!("{}", stmt);
        }
    }
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
/// 1. Return all Statements from SPO index for `<s>` (-> O(M))
///
/// Evaluates queries of type `<s> <p> ?o` (resp. `?s <p> <o>`) as follows
///
/// 1. Get `<s> ?p ?p` from SPO index
/// 2. for all results, filter for `<p>`
///
/// Thus complexity is O(M), whereas `M` is the size of Statements concerning `<s>`.
pub struct FullIndexedGraph{
    statements: Vec<Statement>,
    spo: HashMap<ResourceNode, HashSet<usize>>,
    ops: HashMap<IRIResource, HashSet<usize>>,
    pso: HashMap<Node, HashSet<usize>>
}