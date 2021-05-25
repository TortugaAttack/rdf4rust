use crate::util::iri::IRI;
use crate::rdf::xsd::*;
use uuid::Uuid;
use std::fmt;
use std::ops::Add;

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
    /// use rdf4rust::rdf::node_factory::{Literal, URIResource, Variable, BlankNode};
    /// use rdf4rust::rdf::node_factory::RDFNode;
    /// use rdf4rust::util::iri::IRI;
    ///
    /// // This should be true
    /// let literal_node = Literal::create_byte_literal(8);
    /// assert!(literal_node.is_literal());
    ///
    ///
    /// // Everything else should be false
    /// let uri = IRI::create_iri(&String::from("http://example.com")).ok().expect("Is invalid URI");
    ///
    /// let uri_node = URIResource::create_resource(uri);
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
    /// use rdf4rust::rdf::node_factory::{Literal, URIResource, Variable, BlankNode, RDFNode};
    /// use rdf4rust::rdf::node_factory::RDFNode;
    /// use rdf4rust::util::iri::IRI;
    ///
    /// // This should be true
    /// let uri = IRI::create_iri(&String::from("http://example.com")).ok().expect("Is invalid URI");
    ///
    /// let uri_node = URIResource::create_resource(uri);
    /// assert!(uri_node.is_uri());
    ///
    /// // Everything else should be false
    /// let literal_node = Literal::create_byte_literal(8);
    /// assert!(!literal_node.is_uri());
    ///
    /// let var_node = Variable::create_var("abc");
    /// assert!(!var_node.is_uri());
    ///
    /// let bnode = BlankNode::generate_blank_node();
    /// assert!(!bnode.is_uri());
    ///
    /// ```
    ///
    fn is_uri(&self) -> bool;

    fn is_var(&self) -> bool;

    fn is_blank(&self) -> bool;

    fn as_string(&self, quoting: bool) -> String;
    fn get_value(&self) -> String;
}

pub struct Literal{
    value: String,
    dtype: XSDDataType,
    lang: Option<String>
}

impl Literal{
    pub fn create_literal(value: String) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_STRING())
    }

    pub fn create_hex_binary_literal(value: String) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_HEX_BINARY())
    }

    pub fn create_base64_binary_literal(value: String) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_BASE64BINARY())
    }

    pub fn create_any_type_literal(value: String) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_ANY_TYPE())
    }

    pub fn create_any_simple_type_literal(value: String) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_ANY_SIMPLE_TYPE())
    }

    pub fn create_unsigned_long_literal(value: u64) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_UNSIGNED_LONG())
    }

    pub fn create_unsigned_integer_literal(value: u32) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_UNSIGNED_INT())
    }

    pub fn create_unsigned_short_literal(value: u16) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_UNSIGNED_SHORT())
    }

    pub fn create_unsigned_byte_literal(value: u8) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_UNSIGNED_BYTE())
    }

    pub fn create_integer_literal(value: i64) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_INTEGER())
    }

    pub fn create_long_literal(value: i64) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_LONG())
    }

    pub fn create_int_literal(value: i32) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_INT())
    }

    pub fn create_short_literal(value: i16) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_SHORT())
    }

    pub fn create_byte_literal(value: i8) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_BYTE())
    }

    pub fn create_boolean_literal(value: bool) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_BOOLEAN())
    }

    pub fn create_any_uri_literal(value: IRI) -> Literal{
        Literal::create_typed_literal(value.as_string(), XSD_ANY_URI())
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
    /// use rdf4rust::rdf::xsd::{XSD_ANY_URI, XSD_INT, XSD_STRING};
    ///
    /// let uri_literal = Literal::create_typed_literal(String::from("http://my-uri.com/Person1"), XSD_ANY_URI());
    /// let int_literal = Literal::create_typed_literal(String::from("12345"), XSD_INT());
    /// let simple_string_literal = Literal::create_typed_literal(String::from("my-value"), XSD_STRING());
    ///
    /// //Note that if the XSDDataType is not XSD_STRING the quotation boolean in as_string(quotation) will be ignored.
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
                dtype: XSD_STRING(),
                lang: None
            }
        }else {
            Literal {
                value,
                dtype: XSD_STRING(),
                lang: Some(lang)
            }
        }
    }

    pub fn parse_literal(literal: &str) -> Result<Literal, InvalidLiteralError>{
        //true|false|number|quotes " ' ''' | @ | ^^<> | ^^prefix:suffix
        let mut starts_with="";
        if literal == "true"{
            return Ok(Literal::create_boolean_literal(true));
        }
        else if literal == "false"{
            return Ok(Literal::create_boolean_literal(false));
        }
        else if literal.starts_with("\""){
            starts_with="\"";
        }
        else if literal.starts_with("'"){
            starts_with="'";
        }
        else if literal.starts_with("'''"){
            starts_with="'''";
        }
        if starts_with.is_empty(){
            //TODO check if number
            return Result::Ok(Literal::create_literal(String::from(literal)))
        }
        else{
            //swABCsw@?||^^
            let end_of_str = match literal.rfind(starts_with){
                Some(val) => val,
                None => return Result::Err(InvalidLiteralError::new(String::from("Literal starts with ").add(starts_with).add(" but doesn't close.")))
            };
            let val = String::from(&literal[starts_with.len()..end_of_str]);
            if literal[end_of_str+1..].starts_with("@"){
                if literal[end_of_str+2..].contains("^^"){
                    return Result::Err(InvalidLiteralError::new(String::from("Literal cannot contain language tag as well as datatype.")))
                }
                return Ok(Literal::create_lang_literal(val, String::from(&literal[end_of_str+2..])))
            }
            else if literal[end_of_str+1..].starts_with("^^"){
                // datatype
                //TODO prefix mapping
                let dtype = XSD_STRING();
                return Ok(Literal::create_typed_literal(val, dtype));
            }
            else{
                return Ok(Literal::create_literal(val));
            }
        }
        return Err(InvalidLiteralError::new(String::from("Unexpected Error. Shouldn't occur")))
    }

}

impl RDFNode for Literal{

    fn is_literal(&self) -> bool {
        true
    }

    fn is_uri(&self) -> bool {
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

    fn is_uri(&self) -> bool {
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

pub struct URIResource {
    uri: IRI
}

impl URIResource {
    pub fn get_uri(&self) -> &IRI {
        &self.uri
    }

    pub fn create_resource(uri: IRI) -> URIResource{
        URIResource{
            uri
        }
    }
}

impl RDFNode for URIResource {

    fn is_literal(&self) -> bool{
        false
    }

    fn is_uri(&self) -> bool {
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
            return String::from("<").add(&self.uri.as_string()).add(">")
        }else {
            self.uri.as_string()
        }
    }

    fn get_value(&self) -> String {
        self.uri.as_string()
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

    fn is_uri(&self) -> bool {
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

pub struct InvalidLiteralError {
    msg: String
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