use crate::rdf::node_factory::{RDFNode, BlankNode, IRIResource, Literal, InvalidLiteralError};
use crate::rdf::graph::Node::{BNode, IRINode, LiteralNode};
use crate::util::iri::{IRI, IRIInvalidError};
use std::error::Error;
use std::collections::HashMap;
use std::fmt;
use std::ops::Add;
use crate::rdf::graph::{Node, Graph, SimpleGraph, Statement, ResourceNode, IndexedGraph};

use std::fs;
use std::path::Path;
use std::io::{Read, Seek, SeekFrom};
use std::fs::File;
use crate::io::reader::Lang::NTRIPLE;
use std::borrow::Cow;
use std::string::FromUtf8Error;
use crate::io::buffered_reader::BufferedReader;


pub enum Lang{
    TTL,
    TURTLE,
    NTRIPLE,
    NT,
    NQ,
    NQUADS,
    RdfXml
}



pub trait Parser {

    fn read_from_line(&self, line: &str, line_no: usize, graph: &mut Box<&mut impl Graph>) -> Result<bool, ParserError>;

    fn parse_iri(&self, token: &str)-> Result<IRIResource, ParserError> {
        let mut trimmed_token = token.trim();
        if !trimmed_token.starts_with("<") { return Err(ParserError::new(String::from("IRI has to start with <"))) }
        if !trimmed_token.ends_with(">") { return Err(ParserError::new(String::from("IRI has to end with >"))) }
        trimmed_token = &trimmed_token[1..trimmed_token.len()-1];
        let iri = match IRI::create_iri(&String::from(trimmed_token)){
            Ok(iri) => IRIResource::create_resource(iri),
            Err(err) => return Err(ParserError::new(err.msg))
        };
        Ok(iri)
    }
}

pub struct Reader{

}

impl Reader {

    fn lang_to_parser(lang: &Lang) -> impl Parser{
        match lang{
            Lang::NTRIPLE => {NTripleReader{}}
            Lang::NT => {NTripleReader{}}
            _ => NTripleReader{}
        }
    }

    pub fn read_indexed_graph(file: &str, lang: Lang)-> Result<IndexedGraph, ParserError>{
        let mut graph = IndexedGraph::new();
        match Reader::read_graph(&mut Box::new(&mut graph), file, lang){
            Ok(_) => {Ok(graph)}
            Err(err) => {Err(err)}
        }
    }

    pub fn read_simple_graph(file: &str, lang: Lang)-> Result<SimpleGraph, ParserError>{
        let mut graph = SimpleGraph::new();
        match Reader::read_graph(&mut Box::new(&mut graph), file, lang){
            Ok(_) => {Ok(graph)}
            Err(err) => {Err(err)}
        }
    }

    pub fn read_graph(graph: &mut Box<&mut impl Graph>, file: &str, lang: Lang) -> Result<usize, ParserError> {
        let reader = Reader::lang_to_parser(&lang);
        let mut buffered_reader = BufferedReader::new(file).expect("");
        let mut line_count = 1;
        while let Some(line) = buffered_reader.read_line(){
            if !line.is_empty() {
                match reader.read_from_line(line.as_str(), line_count, graph){
                    Ok(_) => {},
                    Err(err) => {return Err(err)}
                }
            }
            line_count += 1;
        }
        Ok(graph.count())
    }

}

struct TurtleReader{

}

struct NTripleReader{

}

struct NQuadsReader{

}

struct RDFXMLReader{

}

enum Token{
    IRI{node: Node, pos: usize, len: usize},
    BNode{node: Node, pos: usize, len: usize},
    Literal{node: Node, pos: usize, len: usize},
    DOT{pos: usize},
    COMMENT{pos: usize},
    ERROR
}


impl NTripleReader{
    fn peek_next_token(&self, rest_line: &str) -> Token{

        let pos= match rest_line.find(|c: char| !c.is_whitespace()){
            None => {return Token::ERROR}
            Some(position) => {position}
        };

        let line = &rest_line[pos..];
        //end is either end of string or depends
        let len= if line.starts_with("<") || line.starts_with("_:"){
            match line.find(|c: char| c.is_whitespace()) {
                None => { rest_line.len() }
                Some(position) => { position+pos }
            }
        }else{

            //find end of literal... start_token -> next unescaped end token -> next whitespace or eol
            let mut ret =rest_line.len();
            let start_token = if line.starts_with("\""){
                "\""
            }else if line.starts_with("'''"){
                "'''"
            }else if line.starts_with("'"){
                "'"
            }else{
                ""
            };
            if !start_token.is_empty() {
                let mut current_start = start_token.len();
                // check every encountered rof start_token until one is found which is not escaped

                while let Some(position) = line[current_start..].find(start_token) {
                    if !line[position - 1..].starts_with("\\") {
                        ret = match line[position..].find(|c: char| c.is_whitespace()) {
                            None => { rest_line.len() }
                            Some(position2) => { position2 + position }
                        };
                        break;
                    }
                    current_start += position + 1;
                }
            }else{
                ret = match line[pos..].find(|c: char| c.is_whitespace()) {
                    None => { rest_line.len() }
                    Some(position2) => { position2 + pos }
                };
            }
            ret
        };

        if line.starts_with("#"){
            return Token::COMMENT {pos}
        }
        if line.starts_with("."){
            return Token::DOT {pos}
        }
        let node = match parse_resolved_object(&rest_line[pos..len]){
            Ok(node) => {node}
            Err(err) => {
                println!("{}", err);
                return Token::ERROR}
        };
        match node{
            Node::IRINode { .. } => { Token::IRI {node, pos, len}}
            Node::LiteralNode { .. } => {Token::Literal {node, pos, len}}
            Node::BNode { .. } => {Token::BNode {node, pos, len}}
        }
    }
}

impl Parser for NTripleReader {

    fn read_from_line(&self, line: &str, line_no: usize, graph: &mut Box<&mut impl Graph>) -> Result<bool, ParserError> {
        if line.is_empty(){
            return Ok(false)
        }
        //peek_next_token() -> expect _: or < expect < -> literal -> expect .
        let mut current_pos =0;
        let subject =  match self.peek_next_token(line){
            Token::IRI{node, pos, len} => {current_pos = pos+len; node}
            Token::BNode{node, pos, len} => {current_pos =pos+len; node}
            Token::Literal{node, pos, len} => {return Err(ParserError::new(format!("Line [#line: {}, pos: {}] Literal is not allowed as subject.", line_no, current_pos+pos)))}
            Token::DOT{pos} => {return Err(ParserError::new(format!("Line [#line: {}, pos: {}] starts with dot, not allowed in NTriple syntax.", line_no,current_pos+ pos)))}
            Token::COMMENT{pos} => {return Ok(false)}
            _  => {return Err(ParserError::new(format!("Line [#line: {}] unexpected error.", line_no)))}
        };

        let predicate =  match self.peek_next_token(&line[current_pos..]){
            Token::IRI{node, pos, len} => {current_pos += pos+len; node}
            Token::BNode{node, pos, len} => {return Err(ParserError::new(format!("Line [#line: {}, pos: {}] BNode not allowed as predicate. Only IRI.", line_no, current_pos+pos)))}
            Token::Literal{node, pos, len} => {return Err(ParserError::new(format!("Line [#line: {}, pos: {}] Literal is not allowed as predicate. Only IRI.", line_no, current_pos+pos)))}
            Token::DOT{pos} => {return Err(ParserError::new(format!("Line [#line: {}, pos: {}] Found dot, but not allowed here.", line_no, current_pos+pos)))}
            Token::COMMENT{pos} => {return Err(ParserError::new(format!("Line [#line: {}, pos: {}] Comment starts within triple, which is not allowed in NTriple Syntax.", line_no, current_pos+pos)))}
            _  => {return Err(ParserError::new(format!("Line [#line: {}] unexpected error.", line_no)))}

        };

        let object =  match self.peek_next_token(&line[current_pos..]){
            Token::IRI{node, pos, len} => {current_pos += pos+len; node}
            Token::BNode{node, pos, len} => {current_pos += pos+len; node}
            Token::Literal{node, pos, len} => {current_pos += pos+len; node}
            Token::DOT{pos} => {return Err(ParserError::new(format!("Line [#line: {}, pos: {}] Found dot, but not allowed here.", line_no, current_pos+pos)))}
            Token::COMMENT{pos} => {return Err(ParserError::new(format!("Line [#line: {}, pos: {}] Comment starts within triple, which is not allowed in NTriple Syntax.", line_no, current_pos+pos)))}
            _  => {return Err(ParserError::new(format!("Line [#line: {}] unexpected error.", line_no)))}
        };


        let subject = match subject{
            Node::IRINode { iri } => {ResourceNode::IRINode {iri}}
            Node::BNode { bnode } => {ResourceNode::BNode {bnode}}
            _ => {return Err(ParserError::new(format!("WTF")));}

        };

        let predicate = IRIResource::create_resource(IRI::create_iri(&predicate.as_uri_resource().expect("").as_string(false)).expect(""));

        match self.peek_next_token(&line[current_pos..]){
            Token::DOT { pos } => {
                graph.add_statement(Statement::create(
                    subject,
                    predicate,
                    object
                ));
                Ok(true)
            },
            _ => {return Err(ParserError::new(format!("Line [#line: {}] doesn't end on dot.", line_no)));}
        }
    }
}


///
/// Parses an `RDFNode` object from a String.
///
/// This doesn't allow IRIs in prefixed form and will result into an error if one occurs.
///
/// # Parameters
///
/// * `object` - The object represented as a string
///
/// # Returns
///
/// Result object containing either the parsed `RDFNode` or the specific `Error`
///
/// # Examples
///
/// ```
/// use rdf4rust::rdf::graph::Node;
/// use rdf4rust::io::reader::{parse_object, parse_resolved_object};
/// use std::collections::HashMap;
/// use rdf4rust::rdf::{node_factory::RDFNode, xsd};
/// use rdf4rust::rdf::node_factory::BlankNode;
///
/// if let Some(node) = parse_resolved_object("true").ok(){
///     match node{
///         Node::LiteralNode { literal } => {
///             assert_eq!(xsd::XSD_BOOLEAN(), literal.get_datatype());
///             assert_eq!("true", literal.get_value());
///         }
///         _ => {}
///     };
/// }
///
/// // Uncommenting the next line will thrown an error
/// //parse_object("example:abcdef", prefixes).expect("Will Throw error");
///
/// // Blank Node
///
/// if let Some(node) = parse_resolved_object("_:abc").ok(){
///     match node{
///         Node::BNode { bnode } => {
///             //Note that we generate an ID here
///             let expected = BlankNode::generate_from_string("abc").get_value();
///             assert_eq!(expected, bnode.get_value());
///         }
///         _ => {}
///     };
/// }
///
/// // Plain IRI
///
/// if let Some(node) = parse_resolved_object("<http://example.com/abcdef>").ok(){
///     match node{
///         Node::IRINode { iri } => {
///             assert_eq!("http://example.com/abcdef", iri.get_value());
///         }
///         _ => {}
///     };
/// }
///
/// ```
pub fn parse_resolved_object(object: &str) -> Result<Node, ObjectParserError> {
    parse_object(object, &HashMap::new())
}

///
/// Parses an `RDFNode` object from a String.
///
/// # Parameters
///
/// * `object` - The object represented as a string
/// * `prefix_mapping` - The prefix mapping to use if the object is an IRI of type `prefix:suffix`
///
/// # Returns
///
/// Result object containing either the parsed `RDFNode` or the `Error`
///
/// # Examples
///
/// You can use the `as_XYZ` functions to convert the node to an RDFNode
/// ```
/// use rdf4rust::io::reader::parse_object;
/// use std::collections::HashMap;
/// use rdf4rust::rdf::node_factory::RDFNode;
///
/// if let Some(node) = parse_object("true", &HashMap::new()).ok(){
///
///     let literal = node.as_literal().expect("Should be a Literal");
///     assert_eq!("\"true\"^^<http://www.w3.org/2001/XMLSchema#boolean>", literal.as_string());
/// }
/// ```
///
/// or use a match if you're not sure what the `Node` might be
/// ```
/// use rdf4rust::rdf::graph::Node;
/// use rdf4rust::io::reader::parse_object;
/// use std::collections::HashMap;
/// use rdf4rust::rdf::{node_factory::RDFNode, xsd};
/// use rdf4rust::rdf::node_factory::BlankNode;
///
/// if let Some(node) = parse_object("true", &HashMap::new()).ok(){
///     match node{
///         Node::LiteralNode { literal } => {
///             assert_eq!(xsd::XSD_BOOLEAN(), literal.get_datatype());
///             assert_eq!("true", literal.get_value());
///         }
///         _ => {}
///     };
/// }
///
/// // Use Prefix Mapping
/// // IRI http://example.com/abcdef as example:abcdef
/// let mut  prefixes: HashMap<String, String> = HashMap::new();
/// prefixes.insert(String::from("example"), String::from("http://example.com/"));
///
/// if let Some(node) = parse_object("example:abcdef", &prefixes).ok(){
///     match node{
///         Node::IRINode { iri } => {
///             assert_eq!("http://example.com/abcdef", iri.get_value());
///         }
///         _ => {}
///     };
/// }
///
/// // Blank Node
///
/// if let Some(node) = parse_object("_:abc", &HashMap::new()).ok(){
///     match node{
///         Node::BNode { bnode } => {
///             //Note that we generate an ID here
///             let expected = BlankNode::generate_from_string("abc").get_value();
///             assert_eq!(expected, bnode.get_value());
///         }
///         _ => {}
///     };
/// }
///
/// // Plain IRI
///
/// if let Some(node) = parse_object("<http://example.com/abcdef>", &HashMap::new()).ok(){
///     match node{
///         Node::IRINode { iri } => {
///             assert_eq!("http://example.com/abcdef", iri.get_value());
///         }
///         _ => {}
///     };
/// }
///
/// ```
pub fn parse_object(object: &str, prefix_mapping: &HashMap<String, String>) -> Result<Node, ObjectParserError> {
    if object.starts_with("<"){
        let iri = match IRI::create_iri(&String::from(&object[1..object.len()-1])){
            Ok(val) => val,
            Err(err) => return Err(ObjectParserError::new(err.msg))
        };
        Ok(IRINode { iri: IRIResource::create_resource(iri)})
    }
    else if object.starts_with("_:"){
        Ok(BNode {bnode: BlankNode::generate_from_string(&object[2..])})
    }
    else if !(object.starts_with("\"") || object.starts_with("'")) && object.contains(":") {
        //prefixed uri
        if let Some((prefix, suffix)) = object.split_once(":"){
            if let Some(uri) = prefix_mapping.get(prefix){
                let iri = match IRI::create_iri(&String::from(uri).add(suffix)){
                    Ok(val) => val,
                    Err(err) => return Err(ObjectParserError::new(err.msg))
                };
                Ok(IRINode{ iri: IRIResource::create_resource(iri)})
            }
            else{
                Err(ObjectParserError::new(String::from("Unresolved prefix mapping for prefix: ").add(prefix)))
            }
        }
        else{
            Err(ObjectParserError::new(String::from("Unknown Error occurred")))
        }
    }
    else{
        return match Literal::parse_literal(object, &prefix_mapping){
            Ok(literal) => Ok(LiteralNode {literal}),
            Err(err) => Err(ObjectParserError::new(err.msg))
        }
    }
}



#[derive(Debug)]
pub struct ParserError {
    msg: String
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.msg)
    }
}

impl ParserError {
    pub fn new(msg: String) -> ParserError {
        ParserError {msg}
    }
}

#[derive(Debug)]
pub struct ObjectParserError {
    msg: String
}

impl Error for ObjectParserError {}

impl fmt::Display for ObjectParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.msg)
    }
}

impl ObjectParserError {
    pub fn new(msg: String) -> ObjectParserError {
        ObjectParserError {msg}
    }
}