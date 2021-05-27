use crate::rdf::node_factory::{RDFNode, BlankNode, IRIResource, Literal};
use crate::rdf::graph::Node::{BNode, IRINode, LiteralNode};
use crate::util::iri::IRI;
use std::error::Error;
use std::collections::HashMap;
use std::fmt;
use std::ops::Add;
use crate::rdf::graph::{Node, Graph, Statement, ResourceNode};

use std::fs;
use std::path::Path;
use std::io::{Read, Seek, SeekFrom};
use std::fs::File;
use std::borrow::Cow;
use std::string::FromUtf8Error;
use crate::io::buffered_reader::BufferedReader;
use crate::rdf::database::Database;


pub enum Lang{
    TTL,
    TURTLE,
    NTRIPLE,
    NT,
    NQ,
    NQUADS,
    RdfXml,
    TRIG,
    TRIX
}

impl Lang{
    pub fn guess_lang(file: &str) -> Option<Self>{
        if file.to_lowercase().ends_with(".nt"){
            return Some(Lang::NTRIPLE)
        }
        else if file.to_lowercase().ends_with(".nq"){
            return Some(Lang::NQUADS)
        }
        else if file.to_lowercase().ends_with(".ttl"){
            return Some(Lang::TTL)
        }
        else if file.to_lowercase().ends_with(".rdf") || file.ends_with(".xml"){
            return Some(Lang::RdfXml)
        }
        else if file.to_lowercase().ends_with(".trig"){
            return Some(Lang::TRIG)
        }
        else if file.to_lowercase().ends_with(".trix"){
            return Some(Lang::TRIG)
        }
        None
    }
}

pub trait Parser {

    fn read_from_line(&mut self, line: &str, line_no: usize, database: &mut Database) -> Result<bool, ParserError>;

}

pub struct Reader{

}

impl Reader {

    pub fn read_to_rdf_guess_lang(database: &mut Database, file: &str) -> Result<usize, ParserError>{
        let lang = match Lang::guess_lang(file){
            None => {return Err(ParserError::new(format!("Cannot guess language for file {}", file)))}
            Some(val) => {val}
        };
        Reader::read_to_rdf(database, file, lang)
    }

    fn get_parser(lang: &Lang) -> Box<dyn Parser>{
        match lang{
            Lang::TTL | Lang::TURTLE => {return Box::new(TurtleReader::new())}
            Lang::NTRIPLE | Lang::NT => {return Box::new(NTripleReader{ tokenizer: SimpleTokenizer {} })}
            Lang::NQ | Lang::NQUADS => {return Box::new(NQuadsReader{ tokenizer: SimpleTokenizer {} })}
            Lang::RdfXml => {}
            Lang::TRIG => {}
            Lang::TRIX => {}
        }
        return Box::new( NTripleReader{ tokenizer: SimpleTokenizer {} })
    }

    pub fn read_to_rdf(database: &mut Database, file: &str, lang: Lang) -> Result<usize, ParserError>{
        //nt and nq won't need mut, but ttl will
        let mut reader = Reader::get_parser(&lang);

        let mut buffered_reader = BufferedReader::new(file).expect("");
        let mut line_count = 1;
        while let Some(line) = buffered_reader.read_line(){
            if !line.is_empty() {
                reader.read_from_line(line.as_str(), line_count, database);


            }
            line_count += 1;
        }
        Ok(database.count())
    }

}

struct TurtleReader{
    //next token has to be one of -> next token -> next token has to be one of
    next_token_type: Vec<String>
}

struct NTripleReader{
    tokenizer: SimpleTokenizer
}

struct NQuadsReader{
    tokenizer: SimpleTokenizer
}

impl NQuadsReader{
    fn check_end(&mut self, line: &str,) -> bool{
        return match self.tokenizer.peek_next_token(&line) {
            Token::DOT { pos } => true,
            _ => false
        }
    }
}

struct RDFXMLReader{

}

enum Token{
    IRI{node: Node, pos: usize, len: usize},
    BNode{node: Node, pos: usize, len: usize},
    Literal{node: Node, pos: usize, len: usize},
    DOT{pos: usize},
    COMMENT{pos: usize},
    COMMA{pos: usize},
    SEMICOLON{pos: usize},
    BNODE_OPEN{pos: usize},
    BNODE_CLOSE{pos:usize},
    BASE{pos: usize, base: String},
    PREFIX{pos: usize, prefix: String, suffix: String},
    GRAPH_OPEN{pos: usize},
    GRAPH_CLOSE{pos: usize},
    ///
    /// Marks tokens which can be ignored, as they are optional
    ///
    OPTIONAL{pos: usize},
    ERROR
}

trait Tokenizer {

    fn peek_next_token(&self, rest_line: &str) -> Token;

    fn subject(&self, line: &str, line_no: usize, mut current_pos: usize) -> Result<Option<(ResourceNode, usize)>, ParserError> {
        let subject = match self.peek_next_token(&line[current_pos..]) {
            Token::IRI { node, pos, len } => {
                current_pos =  len;
                node
            }
            Token::BNode { node, pos, len } => {
                current_pos =  len;
                node
            }
            Token::Literal { node, pos, len } => { return Err(ParserError::new(format!("Line [#line: {}, pos: {}] Literal is not allowed as subject.", line_no, current_pos + pos))) }
            Token::DOT { pos } => { return Err(ParserError::new(format!("Line [#line: {}, pos: {}] starts with dot, not allowed in NQuads syntax.", line_no, current_pos + pos))) }
            Token::COMMENT { pos } => { return Ok(None) }
            _ => { return Err(ParserError::new(format!("Line [#line: {}] unexpected error.", line_no))) }
        };
        let subject = match subject{
            Node::IRINode { iri } => {ResourceNode::IRINode {iri}}
            Node::BNode { bnode } => {ResourceNode::BNode {bnode}}
            _ => {return Err(ParserError::new(format!("WTF")));}

        };
        Ok(Some((subject, current_pos)))
    }

    fn predicate(&self, line: &str, line_no: usize, mut current_pos: usize) -> Result<(IRIResource, usize), ParserError> {
        let predicate = match self.peek_next_token(&line[current_pos..]) {
            Token::IRI { node, pos, len } => {
                current_pos += len;
                node
            }
            Token::BNode { node, pos, len } => { return Err(ParserError::new(format!("Line [#line: {}, pos: {}] BNode not allowed as predicate. Only IRI.", line_no, current_pos + pos))) }
            Token::Literal { node, pos, len } => { return Err(ParserError::new(format!("Line [#line: {}, pos: {}] Literal is not allowed as predicate. Only IRI.", line_no, current_pos + pos))) }
            Token::DOT { pos } => { return Err(ParserError::new(format!("Line [#line: {}, pos: {}] Found dot, but not allowed here.", line_no, current_pos + pos))) }
            Token::COMMENT { pos } => { return Err(ParserError::new(format!("Line [#line: {}, pos: {}] Comment starts within quad, which is not allowed in NQuads Syntax.", line_no, current_pos + pos))) }
            _ => { return Err(ParserError::new(format!("Line [#line: {}] unexpected error.", line_no))) }
        };
        let predicate = IRIResource::create_resource(IRI::create_iri(&predicate.as_uri_resource().expect("").as_string(false)).expect(""));
        Ok((predicate, current_pos))
    }

    fn object(&self, line: &str, line_no: usize, mut current_pos: usize) -> Result<(Node, usize), ParserError> {
        let object = match self.peek_next_token(&line[current_pos..]) {
            Token::IRI { node, pos, len } => {
                current_pos +=  len;
                node
            }
            Token::BNode { node, pos, len } => {
                current_pos +=  len;
                node
            }
            Token::Literal { node, pos, len } => {
                current_pos +=  len;
                node
            }
            Token::DOT { pos } => { return Err(ParserError::new(format!("Line [#line: {}, pos: {}] Found dot, but not allowed here.", line_no, current_pos + pos))) }
            Token::COMMENT { pos } => { return Err(ParserError::new(format!("Line [#line: {}, pos: {}] Comment starts within quad, which is not allowed in NQuads Syntax.", line_no, current_pos + pos))) }
            _ => { return Err(ParserError::new(format!("Line [#line: {}] unexpected error.", line_no))) }
        };
        return Ok((object,current_pos))
    }

    fn graph(&self, line: &str, line_no: usize, mut current_pos: usize) -> Result<(IRIResource, usize), ParserError> {
        let graph_node =  match self.peek_next_token(&line[current_pos..]){
            Token::IRI{node, pos, len} => {current_pos += len; node}
            Token::BNode{node, pos, len} => {return Err(ParserError::new(format!("Line [#line: {}, pos: {}] Graph cannot be blank node.", line_no, current_pos+pos)))}
            Token::Literal{node, pos, len} => {return Err(ParserError::new(format!("Line [#line: {}, pos: {}] Graph cannot be literal.", line_no, current_pos+pos)))}
            Token::DOT{pos} => {return Err(ParserError::new(format!("Line [#line: {}, pos: {}] Found dot, but not allowed here.", line_no, current_pos+pos)))}
            Token::COMMENT{pos} => {return Err(ParserError::new(format!("Line [#line: {}, pos: {}] Comment starts within quad, which is not allowed in NQuads Syntax.", line_no, current_pos+pos)))}
            _  => {return Err(ParserError::new(format!("Line [#line: {}] unexpected error.", line_no)))}
        };
        let graph_node = IRIResource::create_resource(IRI::create_iri(&graph_node.as_uri_resource().expect("").as_string(false)).expect(""));
        Ok((graph_node, current_pos))
    }

}

struct XmlTokenizer{
    //queue open tokens
    //for trix -> root has to be trix
}

impl Tokenizer for XmlTokenizer{
    fn peek_next_token(&self, rest_line: &str) -> Token {
        todo!()
    }
}

impl TurtleReader{

    fn new()->Self{
        let mut allowed_tokens = Vec::new();
        allowed_tokens.push(String::from("iri"));
        allowed_tokens.push(String::from("bnode"));
        allowed_tokens.push(String::from("comment"));
        allowed_tokens.push(String::from("base"));
        allowed_tokens.push(String::from("prefix"));
        allowed_tokens.push(String::from("dot"));
        allowed_tokens.push(String::from("bnode_open"));
        TurtleReader{
            next_token_type: allowed_tokens
        }
    }
}

struct TurtleTokenizer{
    nested_bnode_count: usize,
    simple_tokenizer: SimpleTokenizer,
    prefix_mapping: HashMap<String, String>,
    current_base: Option<String>
}

impl TurtleTokenizer{
    fn new()-> Self{
        TurtleTokenizer{
            nested_bnode_count:0,
            simple_tokenizer: SimpleTokenizer{},
            prefix_mapping: HashMap::new(),
            current_base: None
        }
    }
}

impl Tokenizer for TurtleTokenizer{
    fn peek_next_token(&self, rest_line: &str) -> Token {
        todo!()
        //check ttl specific tokens if next token has to be simple use SimpleTokenizer
    }
}

impl Parser for TurtleReader{


    fn read_from_line(&mut self, line: &str, line_no: usize, database: &mut Database) -> Result<bool, ParserError> {
        todo!()
        //read next token
        //is token allowed?
        // no => error
        // yes:
        //   0. check bnode counter (incr if token is bnode open, decr if close)
        //   0.1 if bnode counter ==0 after decr => next token #;,. if bnode was object . if was subject
        //   1. is triple end? yes=> add triple
        //   2. set new allowed tokens
        //   start from beginning
        //Comment just ignored allowed_tokens stay
    }
}

struct SimpleTokenizer{}

impl Tokenizer for SimpleTokenizer{

    fn peek_next_token(&self, rest_line: &str) -> Token{

        let pos= match rest_line.find(|c: char| !c.is_whitespace()){
            None => {return Token::ERROR}
            Some(position) => {position}
        };

        let line = &rest_line[pos..];
        //end is either end of string or depends
        let end = if line.starts_with("_:"){
            match line.find(|c: char| c.is_whitespace() || c=='<' || c=='\"' || c=='\'') {
                None => { rest_line.len() }
                Some(position) => { position+pos }
            }
        }else if line.starts_with("<"){
                match line.find(|c: char| c=='>') {
                    None => { rest_line.len() }
                    Some(position) => { position+pos+1 }
                }
        }
        else{

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
                        ret = match line[current_start+position..].find(|c: char| c.is_whitespace()) {
                            None => { rest_line.len() }
                            Some(position2) => {current_start+ position2 + position + pos  }
                        };
                        break;
                    }
                    current_start += position + 1;
                }
            }else{
                ret = match line.find(|c: char| c.is_whitespace()) {
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
        let node = match parse_resolved_object(&rest_line[pos..end]){
            Ok(node) => {node}
            Err(err) => {
                println!("{}", err);
                return Token::ERROR}
        };
        match node{
            Node::IRINode { .. } => { Token::IRI {node, pos, len: end }}
            Node::LiteralNode { .. } => {Token::Literal {node, pos, len: end }}
            Node::BNode { .. } => {Token::BNode {node, pos, len: end }}
        }
    }
}



impl Parser for NQuadsReader{
    fn read_from_line(&mut self, line: &str, line_no: usize, database: &mut Database) -> Result<bool, ParserError> {
        if line.is_empty(){
            return Ok(false)
        }
        //peek_next_token() -> expect _: or < expect < -> literal -> expect .
        let mut current_pos =0;
        let (subject, current_pos) = match self.tokenizer.subject(&line, line_no, current_pos){
            Ok(Some(node)) => {
                node
            },
            Ok(None) => return Ok(false),
            Err(err) => return Err(err)
        };
        if !line[current_pos..].starts_with(|c: char| c.is_whitespace()){
            return Err(ParserError::new(format!("Line [no: {}, pos: {}]: Items have to be separated by whitespace char. ", line_no, current_pos)))
        }
        let (predicate, current_pos) = match self.tokenizer.predicate(&line, line_no, current_pos){
            Ok(node) => node,
            Err(err) => return Err(err)
        };
        if !line[current_pos..].starts_with(|c: char| c.is_whitespace()){
            return Err(ParserError::new(format!("Line [no: {}, pos: {}]: Items have to be separated by whitespace char. ", line_no, current_pos)))
        }
        let (object, current_pos)  = match self.tokenizer.object(&line, line_no, current_pos){
            Ok(node) => node,
            Err(err) => return Err(err)
        };
        if !line[current_pos..].starts_with(|c: char| c.is_whitespace()){
            return Err(ParserError::new(format!("Line [no: {}, pos: {}]: Items have to be separated by whitespace char. ", line_no, current_pos)))
        }

        if self.check_end(&line[current_pos..]){
                database.add_statement(None, Statement::create(
                    subject,
                    predicate,
                    object
                ));
                return Ok(true)
        }

        let (graph_node, current_pos) = match self.tokenizer.graph(&line, line_no, current_pos){
            Ok(node) => node,
            Err(err) => return Err(err)
        };


        return if self.check_end(&line[current_pos..]) {
            database.add_statement(Some(graph_node), Statement::create(
                subject,
                predicate,
                object
            ));
            Ok(true)
        } else {
            Err(ParserError::new(format!("Line [no: {}] doesn't end on dot.", line_no)))
        }
    }
}


impl NTripleReader{

}

impl Parser for NTripleReader {

    fn read_from_line(&mut self, line: &str, line_no: usize, database: &mut Database) -> Result<bool, ParserError> {
        if line.is_empty(){
            return Ok(false)
        }
        //peek_next_token() -> expect _: or < expect < -> literal -> expect .
        let mut current_pos =0;
        let (subject, current_pos) = match self.tokenizer.subject(&line, line_no, current_pos){
            Ok(Some(node)) => {
                node
            },
            Ok(None) => return Ok(false),
            Err(err) => return Err(err)
        };
        if !line[current_pos..].starts_with(|c: char| c.is_whitespace()){
            return Err(ParserError::new(format!("Line [no: {}, pos: {}]: Items have to be separated by whitespace char. ", line_no, current_pos)))
        }
        let (predicate, current_pos) = match self.tokenizer.predicate(&line, line_no, current_pos){
            Ok(node) => node,
            Err(err) => return Err(err)
        };
        if !line[current_pos..].starts_with(|c: char| c.is_whitespace()){
            return Err(ParserError::new(format!("Line [no: {}, pos: {}]: Items have to be separated by whitespace char. ", line_no, current_pos)))
        }
        let (object, current_pos)  = match self.tokenizer.object(&line, line_no, current_pos){
            Ok(node) => node,
            Err(err) => return Err(err)
        };

        match self.tokenizer.peek_next_token(&line[current_pos..]){
            Token::DOT { pos } => {
                database.add_statement(None, Statement::create(
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
/// Literals need to be enclodes by " otherwise will fail.
///
/// If you want turtle syntax literals use `parse_object(object, HashMap::new(), false)`
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
/// if let Some(node) = parse_resolved_object("\"true\"^^<http://www.w3.org/2001/XMLSchema#boolean>").ok(){
///     match node{
///         Node::LiteralNode { literal } => {
///             assert_eq!(xsd::xsd_boolean(), literal.get_datatype());
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
    parse_object(object, &HashMap::new(), true)
}

///
/// Parses an `RDFNode` object from a String.
///
/// # Parameters
///
/// * `object` - The object represented as a string
/// * `prefix_mapping` - The prefix mapping to use if the object is an IRI of type `prefix:suffix`
/// * `strict_literal` - literal has to be enclosed by "
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
/// if let Some(node) = parse_object("true", &HashMap::new(), false).ok(){
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
/// if let Some(node) = parse_object("true", &HashMap::new(), false).ok(){
///     match node{
///         Node::LiteralNode { literal } => {
///             assert_eq!(xsd::xsd_boolean(), literal.get_datatype());
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
/// if let Some(node) = parse_object("example:abcdef", &prefixes, true).ok(){
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
/// if let Some(node) = parse_object("_:abc", &HashMap::new(), true).ok(){
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
/// if let Some(node) = parse_object("<http://example.com/abcdef>", &HashMap::new(), true).ok(){
///     match node{
///         Node::IRINode { iri } => {
///             assert_eq!("http://example.com/abcdef", iri.get_value());
///         }
///         _ => {}
///     };
/// }
///
/// ```
pub fn parse_object(object: &str, prefix_mapping: &HashMap<String, String>, strict_literal: bool) -> Result<Node, ObjectParserError> {
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
        if !strict_literal || object.starts_with("\"") {
            return match Literal::parse_literal(object, &prefix_mapping) {
                Ok(literal) => Ok(LiteralNode { literal }),
                Err(err) => Err(ObjectParserError::new(err.msg))
            }
        }
        Err(ObjectParserError::new(format!("Literals need to be enclosed by \" if strict_literal is true (e.g. NT, NQ)")))
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