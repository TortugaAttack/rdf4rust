use crate::util::iri::IRI;
use crate::rdf::xsd::*;
use uuid::Uuid;
use std::fmt;
use std::ops::Add;
use std::error::Error;
use std::collections::HashMap;
use std::any::Any;
use std::str::FromStr;
use std::num::{NonZeroUsize, NonZeroIsize};
use base64::DecodeError;
use crate::util::hex_binary::{binary_to_string, string_to_binary};
use chrono::{Duration, DateTime};
use chrono::format::Numeric::Year;

///
/// The trait for all RDF nodes in a graph or in a BGP of a SPARQL query.
///
/// Implementations are `Literal`, `BlankNode`, `URIResource` and `Variable`
///
///
pub trait RDFNode {
    ///
    /// Checks if the `RDFNode` instance is a `Literal` object
    ///
    /// # Returns
    ///
    /// True if the `RDFNode` is a `Literal`, false otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use rdf4rust::rdf::node_factory::{Literal, IRIResource, Variable, BlankNode};
    /// use rdf4rust::rdf::node_factory::RDFNode;
    /// use rdf4rust::util::iri::IRI;
    ///
    /// // This should be true
    /// let literal_node = Literal::create_byte_literal(8);
    /// assert!(literal_node.is_literal());
    ///
    ///
    /// // Everything else should be false
    /// let uri = IRI::create_iri(&String::from("http://example.com")).ok().expect("Is invalid IRI");
    ///
    /// let uri_node = IRIResource::create_resource(uri);
    /// assert!(!uri_node.is_literal());
    ///
    /// let var_node = Variable::create_var("abc");
    /// assert!(!var_node.is_literal());
    ///
    /// let bnode = BlankNode::generate_blank_node();
    /// assert!(!bnode.is_literal());
    ///
    /// ```
    ///
    fn is_literal(&self) -> bool;

    ///
    /// Checks if the `RDFNode` instance is a `URIResource` object
    ///
    /// # Returns
    ///
    /// True if the `RDFNode` is a `URIResource`, false otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use rdf4rust::rdf::node_factory::{Literal, IRIResource, Variable, BlankNode, RDFNode};
    /// use rdf4rust::rdf::node_factory::RDFNode;
    /// use rdf4rust::util::iri::IRI;
    ///
    /// // This should be true
    /// let uri = IRI::create_iri(&String::from("http://example.com")).ok().expect("Is invalid URI");
    ///
    /// let uri_node = IRIResource::create_resource(uri);
    /// assert!(uri_node.is_iri());
    ///
    /// // Everything else should be false
    /// let literal_node = Literal::create_byte_literal(8);
    /// assert!(!literal_node.is_iri());
    ///
    /// let var_node = Variable::create_var("abc");
    /// assert!(!var_node.is_iri());
    ///
    /// let bnode = BlankNode::generate_blank_node();
    /// assert!(!bnode.is_iri());
    ///
    /// ```
    ///
    fn is_iri(&self) -> bool;

    fn is_var(&self) -> bool;

    fn is_blank(&self) -> bool;

    fn as_string(&self, quoting: bool) -> String;

    fn get_value(&self) -> String;
}

#[derive(PartialOrd, PartialEq, Eq, Hash)]
pub struct Literal{
    value: String,
    dtype: XSDDataType,
    lang: Option<String>
}

impl Literal{

    fn as_object_generic<T: Any + FromStr>(&self) ->Option<Box<dyn Any>> {
        self.as_object_generic_lc::<T>(false)
    }

    fn as_object_generic_lc<T: Any + FromStr>(&self, lower_case: bool) ->Option<Box<dyn Any>>{
        let tmp = if lower_case {
            self.value.to_lowercase().parse::<T>()
        }else{
            self.value.parse::<T>()
        };
        if let Ok(val) = tmp{
            Some(Box::new(val))
        }else{
            None
        }
    }

    pub fn as_object(&self) -> Option<Box<dyn Any>>{
        if self.dtype.is_xsd_string(){
            return Some(Box::new(String::from(&self.value)))
        }
        match self.dtype.get_value().as_string().as_str(){
            "http://www.w3.org/2001/XMLSchema#unsignedByte" =>  {return self.as_object_generic::<u8>();}
            "http://www.w3.org/2001/XMLSchema#unsignedShort" =>  {return self.as_object_generic::<u16>();}
            "http://www.w3.org/2001/XMLSchema#unsignedInt" =>  {return self.as_object_generic::<u32>();}
            "http://www.w3.org/2001/XMLSchema#unsignedLong" =>  {return self.as_object_generic::<u64>();}
            "http://www.w3.org/2001/XMLSchema#byte" =>  {return self.as_object_generic::<i8>();}
            "http://www.w3.org/2001/XMLSchema#short" =>  {return self.as_object_generic::<i16>();}
            "http://www.w3.org/2001/XMLSchema#int" =>  {return self.as_object_generic::<i32>();}
            "http://www.w3.org/2001/XMLSchema#long" =>  {return self.as_object_generic::<i64>();}
            "http://www.w3.org/2001/XMLSchema#integer" =>  {return self.as_object_generic::<i64>();}
            "http://www.w3.org/2001/XMLSchema#nonNegativeInteger" =>  {return self.as_object_generic::<usize>();}
            "http://www.w3.org/2001/XMLSchema#nonPositiveInteger" =>  {return self.as_object_generic::<isize>();}
            "http://www.w3.org/2001/XMLSchema#positiveInteger" =>  {return self.as_object_generic::<NonZeroUsize>();}
            "http://www.w3.org/2001/XMLSchema#negativeInteger" =>  {return self.as_object_generic::<NonZeroIsize>();}
            "http://www.w3.org/2001/XMLSchema#decimal" =>  {return self.as_object_generic_lc::<f64>(true);}
            //TODO f32 doesn't allow INF and NaN
            "http://www.w3.org/2001/XMLSchema#float" =>  {return self.as_object_generic_lc::<f32>(true);}
            "http://www.w3.org/2001/XMLSchema#double" =>  {return self.as_object_generic_lc::<f64>(true);}
            "http://www.w3.org/2001/XMLSchema#boolean" =>  {return self.as_object_generic::<bool>();}
            "http://www.w3.org/2001/XMLSchema#anyType" =>  {return Some(Box::new(String::from(&self.value)));}
            "http://www.w3.org/2001/XMLSchema#anySimpleType" =>  {return Some(Box::new(String::from(&self.value)));}
            "http://www.w3.org/2001/XMLSchema#anyUri" =>  {return Some(Box::new(IRI::create_iri(self.get_value())));}
            "http://www.w3.org/2001/XMLSchema#hexBinary" =>  {
                return match string_to_binary(self.value.as_str()){
                    None => {None}
                    Some(val) => {return Some(Box::new(val))}
                }
            }
            "http://www.w3.org/2001/XMLSchema#base64Binary" =>  {
                return match base64::decode(self.value.as_str()){
                    Ok(val) => Some(Box::new(val)),
                    Err(_) => None
                }
            }
            //TODO date, time etc.

            _ => {}
        }
        Some(Box::new(String::from(&self.value)))
    }

    pub fn get_datatype(&self) -> &XSDDataType{
        &self.dtype
    }
    pub fn get_lang(&self) -> &Option<String> {
        &self.lang
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn create_literal(value: String) -> Literal{
        Literal::create_typed_literal(value.to_string(), xsd_string())
    }

    pub fn create_base64_binary_literal(value: &[u8]) -> Literal{
        Literal::create_typed_literal(base64::encode(value), xsd_hex_binary())
    }

    pub fn create_hex_binary_literal(value: &[u8]) -> Literal{
        let hex_string =  binary_to_string(value);
        Literal::create_typed_literal(hex_string, xsd_base64binary())
    }

    pub fn create_any_type_literal(value: String) -> Literal{
        Literal::create_typed_literal(value.to_string(), xsd_any_type())
    }

    pub fn create_any_simple_type_literal(value: String) -> Literal{
        Literal::create_typed_literal(value.to_string(), xsd_any_simple_type())
    }

    pub fn create_unsigned_long_literal(value: u64) -> Literal{
        Literal::create_typed_literal(value.to_string(), xsd_unsigned_long())
    }

    pub fn create_unsigned_integer_literal(value: u32) -> Literal{
        Literal::create_typed_literal(value.to_string(), xsd_unsigned_int())
    }

    pub fn create_unsigned_short_literal(value: u16) -> Literal{
        Literal::create_typed_literal(value.to_string(), xsd_unsigned_short())
    }

    pub fn create_unsigned_byte_literal(value: u8) -> Literal{
        Literal::create_typed_literal(value.to_string(), xsd_unsigned_byte())
    }

    pub fn create_integer_literal(value: i64) -> Literal{
        Literal::create_typed_literal(value.to_string(), xsd_integer())
    }

    pub fn create_long_literal(value: i64) -> Literal{
        Literal::create_typed_literal(value.to_string(), xsd_long())
    }

    pub fn create_int_literal(value: i32) -> Literal{
        Literal::create_typed_literal(value.to_string(), xsd_int())
    }

    pub fn create_short_literal(value: i16) -> Literal{
        Literal::create_typed_literal(value.to_string(), xsd_short())
    }

    pub fn create_byte_literal(value: i8) -> Literal{
        Literal::create_typed_literal(value.to_string(), xsd_byte())
    }
    //TODO

    pub fn create_decimal_literal(value: f64) -> Literal{
        Literal::create_typed_literal(value.to_string(), xsd_decimal())
    }

    pub fn create_boolean_literal(value: bool) -> Literal{
        Literal::create_typed_literal(value.to_string(), xsd_boolean())
    }

    pub fn create_any_uri_literal(value: IRI) -> Literal{
        Literal::create_typed_literal(value.as_string(), xsd_any_uri())
    }

    ///
    /// Creates a typed literal using a `XSDDataType`
    ///
    /// # Parameters
    ///
    /// * `value` - The value of the literal represented as String
    /// * `dtype` - The `XSDDataType` type the value represents
    ///
    /// # Returns
    ///
    /// A Literal of type `dtype`
    ///
    /// # Example
    ///
    /// ```
    /// use rdf4rust::rdf::node_factory::RDFNode;
    /// use rdf4rust::rdf::node_factory::Literal;
    /// use rdf4rust::rdf::xsd::{xsd_any_uri, xsd_int, xsd_string};
    ///
    /// let uri_literal = Literal::create_typed_literal(String::from("http://my-uri.com/Person1"), xsd_any_uri());
    /// let int_literal = Literal::create_typed_literal(String::from("12345"), xsd_int());
    /// let simple_string_literal = Literal::create_typed_literal(String::from("my-value"), xsd_string());
    ///
    /// //Note that if the XSDDataType is not xsd_string the quotation boolean in as_string(quotation) will be ignored.
    /// assert_eq!("\"http://my-uri.com/Person1\"^^<http://www.w3.org/2001/XMLSchema#anyUri>", uri_literal.as_string(false).as_str());
    /// assert_eq!("\"12345\"^^<http://www.w3.org/2001/XMLSchema#int>", uri_literal.as_string(false).as_str());
    /// assert_eq!("my-value", simple_string_literal.as_string(false).as_str());
    /// assert_eq!("\"my-value\"", simple_string_literal.as_string(true).as_str());
    /// ```
    ///
    pub fn create_typed_literal(value: String, dtype: XSDDataType) -> Literal{
        Literal{
            value,
            dtype,
            lang: None
        }
    }

    ///
    /// Creates a string literal with the specified language code
    ///
    /// # Parameters
    ///
    /// * `value` - The string value of the literal
    /// * `lang` - The language tag
    ///
    /// # Returns
    ///
    /// A Literal of type xsd:string annotated with a language tag
    /// or if `lang` is empty returns a plain xsd:string literal
    ///
    /// # Example
    ///
    /// ```
    /// use rdf4rust::rdf::node_factory::RDFNode;
    /// use rdf4rust::rdf::node_factory::Literal;
    ///
    /// let lang_literal = Literal::create_lang_literal(String::from("my-value"), String::from("en"));
    /// let simple_string_literal = Literal::create_lang_literal(String::from("my-value"), String::from(""));
    ///
    /// assert_eq!("\"my-value\"@en", lang_literal.as_string(false).as_str());
    /// assert_eq!("my-value", simple_string_literal.as_string(false).as_str());
    /// assert_eq!("\"my-value\"", simple_string_literal.as_string(true).as_str());
    /// ```
    ///
    pub fn create_lang_literal(value: String, lang: String) -> Literal{
        if lang.is_empty(){
            Literal{
                value,
                dtype: xsd_string(),
                lang: None
            }
        }else {
            Literal {
                value,
                dtype: xsd_string(),
                lang: Some(lang)
            }
        }
    }

    fn parse_number(literal: &str) -> Result<Literal, InvalidLiteralError>{
        if literal.contains(".") {
            return if let Some(val) = literal.parse::<f64>().ok() {
                Ok(Literal::create_decimal_literal(val))
            } else {
                Err(InvalidLiteralError::new(String::from("Literal is not valid.")))
            }
        }
        if let Some(val) = literal.parse::<i64>().ok() {
            Ok(Literal::create_integer_literal(val))
        } else {
            Err(InvalidLiteralError::new(String::from("Literal is not valid.")))
        }
    }

    fn parse_prefixed_datatype(val: String, literal: &str, prefix_mapping: &HashMap<String, String>) -> Result<Literal, InvalidLiteralError> {
        //prefixed datatype
        if let Some((prefix, suffix)) = literal.split_once(":"){
            if let Some(uri) = prefix_mapping.get(prefix){
                let iri = match IRI::create_iri(&String::from(uri).add(suffix)){
                    Ok(val) => val,
                    Err(err) => return Err(InvalidLiteralError::new(err.msg))
                };
                Ok(Literal::create_typed_literal(val, XSDDataType::create(iri)))
            }
            else{
                Err(InvalidLiteralError::new(String::from("Unresolved prefix mapping for prefix: ").add(prefix)))
            }
        }
        else{
            Err(InvalidLiteralError::new(String::from("Unknown Error occurred")))
        }
    }

    fn parse_datatype(val: String, literal: &str) -> Result<Literal, InvalidLiteralError>{
        let iri = match IRI::create_iri(&String::from(&literal[..literal.len()-1])){
            Ok(iri) => {iri}
            Err(err) => {return Err(InvalidLiteralError::new(err.msg))}
        };
        Ok(Literal::create_typed_literal(val, XSDDataType::create(iri)))
    }

    fn parse_complex_literal(literal: &str, starts_with: &str, prefix_mapping: &HashMap<String, String>) -> Result<Literal, InvalidLiteralError>{
        //swABCsw@?||^^
        let end_of_str = match literal.rfind(starts_with) {
            Some(val) => val,
            None => return Result::Err(InvalidLiteralError::new(String::from("Literal starts with ").add(starts_with).add(" but doesn't close.")))
        };
        let val = String::from(&literal[starts_with.len()..end_of_str]);
        //Check if it has a language tag or Datatype tag
        if literal[end_of_str + 1..].starts_with("@") {
            if literal[end_of_str + 2..].contains("^^") {
                return Result::Err(InvalidLiteralError::new(String::from("Literal cannot contain language tag as well as datatype.")))
            }
            Ok(Literal::create_lang_literal(val, String::from(&literal[end_of_str + 2..])))
        } else if literal[end_of_str + 1..].starts_with("^^") {
            // datatype
            if literal[end_of_str + 3..].starts_with("<"){
                if !literal.ends_with(">") {return Err(InvalidLiteralError::new(String::from("Literal DataType doesn't end on >")))}
                Literal::parse_datatype(val, &literal[end_of_str + 4..])
            }
            else {
                Literal::parse_prefixed_datatype(val, &literal[end_of_str+3..], &prefix_mapping)
            }
        } else {
            Ok(Literal::create_literal(val))
        }
    }

    pub fn parse_literal(literal: &str, prefix_mapping: &HashMap<String, String>) -> Result<Literal, InvalidLiteralError>{
        //true|false|number|quotes " ' ''' | @ | ^^<> | ^^prefix:suffix
        if literal == "true"{
            return Ok(Literal::create_boolean_literal(true));
        }
        else if literal == "false"{
            return Ok(Literal::create_boolean_literal(false));
        }

        let mut starts_with="";
        if literal.starts_with("\""){
            starts_with="\"";
        }
        else if literal.starts_with("'"){
            starts_with="'";
        }
        else if literal.starts_with("'''"){
            starts_with="'''";
        }
        return if starts_with.is_empty() {
            //either integer or decimal
            Literal::parse_number(literal)
        } else {
            Literal::parse_complex_literal(&literal, &starts_with , &prefix_mapping)
        }
    }

}

impl RDFNode for Literal{

    fn is_literal(&self) -> bool {
        true
    }

    fn is_iri(&self) -> bool {
        false
    }

    fn is_var(&self) -> bool {
        false
    }

    fn is_blank(&self) -> bool {
        false
    }

    fn as_string(&self, quoting: bool) -> String {
        if self.dtype.is_xsd_string(){
            if let Some(lang) = &self.lang {
                // language is set so append
                return  String::from("\"").add(&self.value).add("\"@").add(&self.lang.as_ref().expect("checked"));
            }
            //language is not set, so check if quoting is true and quote if so
            return if quoting {
                String::from("\"").add(&self.value).add("\"")
            } else {
                String::from(&self.value)
            }
        }
        //datatype is not string, so add datatype.
        return String::from("\"").add(&self.value).add("\"^^<").add(&self.dtype.get_value().as_string()).add(">");
    }

    fn get_value(&self) -> String {
        String::from(&self.value)
    }

}

#[derive(Ord, PartialOrd, PartialEq, Eq, Hash)]
pub struct BlankNode{
    id: String
}

impl BlankNode{

    pub fn generate_blank_node() -> BlankNode{
        let id = Uuid::new_v4();
        BlankNode{
            id: id.to_simple().to_string()
        }
    }

    pub fn generate_from_string(id: &str) -> BlankNode{
        let id = Uuid::new_v5(&Uuid::NAMESPACE_URL, id.as_bytes());
        BlankNode{
            id: id.to_simple().to_string()
        }
    }

    pub fn create_blank_node(id: &str) -> BlankNode{
        BlankNode{
            id: String::from(id)
        }
    }

}

impl RDFNode for BlankNode{
    fn is_literal(&self) -> bool {
        false
    }

    fn is_iri(&self) -> bool {
        false
    }

    fn is_var(&self) -> bool {
        false
    }

    fn is_blank(&self) -> bool {
        true
    }

    fn as_string(&self, quoting: bool) -> String{
        return String::from("_:").add(&self.id)
    }

    fn get_value(&self) -> String {
        String::from(&self.id)
    }
}

#[derive(PartialOrd, PartialEq, Eq, Hash)]
pub struct IRIResource {
    iri: IRI,
}

impl IRIResource {
    pub fn get_iri(&self) -> &IRI {
        &self.iri
    }

    pub fn create_resource(iri: IRI) -> IRIResource {
        IRIResource {
            iri
        }
    }
}

impl RDFNode for IRIResource {

    fn is_literal(&self) -> bool{
        false
    }

    fn is_iri(&self) -> bool {
        true
    }

    fn is_var(&self) -> bool {
        false
    }

    fn is_blank(&self) -> bool {
        false
    }

    fn as_string(&self, quoting: bool) -> String{
        if quoting{
            return String::from("<").add(&self.iri.as_string()).add(">")
        }else {
            self.iri.as_string()
        }
    }

    fn get_value(&self) -> String {
        self.iri.as_string()
    }
}

pub struct Variable{
    name: String
}

impl Variable{

    pub fn create_var(var_name: &str) -> Variable{
        Variable{
            name: String::from(var_name)
        }
    }
}

impl RDFNode for Variable{
    fn is_literal(&self) -> bool {
        false
    }

    fn is_iri(&self) -> bool {
        false
    }

    fn is_var(&self) -> bool {
        true
    }

    fn is_blank(&self) -> bool {
        false
    }

    fn as_string(&self, quoting: bool) -> String {
        if quoting{
            return String::from("?").add(&self.name)
        }
        String::from(&self.name)
    }

    fn get_value(&self) -> String {
        String::from(&self.name)
    }
}

#[derive(Debug)]
pub struct InvalidLiteralError {
    pub msg: String
}

impl Error for InvalidLiteralError{
}

impl InvalidLiteralError{
    pub fn new(msg: String) -> InvalidLiteralError {
        InvalidLiteralError{msg}
    }
}

impl fmt::Display for InvalidLiteralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.msg)
    }
}