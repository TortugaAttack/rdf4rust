use crate::util::iri::URI;
use crate::rdf::xsd::*;
use std::intrinsics::type_name;

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
    /// use rdf4rust::rdf::node_factory::{Literal, URIResource};
    /// use rdf4rust::rdf::node_factory::RDFNode;
    /// use rdf4rust::util::iri::URI;
    ///
    /// let literal_node = Literal::create_byte_literal(8);
    /// assert!(literal_node.is_literal());
    ///
    /// let uri = URI::create_uri("http://example.com").ok().expect("Is invalid URI");
    ///
    /// let uri_node = URIResource::create_resource(uri);
    /// assert!(!uri_node.is_literal());
    ///
    /// ```
    ///
    fn is_literal(&self) -> bool;
    fn as_string(&self, quoting: bool);
    fn get_value(&self) -> &String;
}

pub trait Resource {

}

pub struct Literal{
    value: String,
    dtype: XSDDataType,
    lang: Option<String>
}


impl Literal{
    pub fn create_literal(value: String) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_STRING)
    }

    pub fn create_hex_binary_literal(value: String) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_HEX_BINARY)
    }

    pub fn create_base64_binary_literal(value: String) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_BASE64BINARY)
    }

    pub fn create_any_type_literal(value: String) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_ANY_TYPE)
    }

    pub fn create_any_simple_type_literal(value: String) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_ANY_SIMPLE_TYPE)
    }

    pub fn create_unsigned_long_literal(val: u64) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_UNSIGNED_LONG)
    }

    pub fn create_unsigned_integer_literal(val: u32) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_UNSIGNED_INT)
    }

    pub fn create_unsigned_short_literal(val: u16) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_UNSIGNED_SHORT)
    }

    pub fn create_unsigned_byte_literal(val: u8) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_UNSIGNED_BYTE)
    }

    pub fn create_integer_literal(value: i64) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_INTEGER)
    }

    pub fn create_long_literal(value: i64) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_LONG)
    }

    pub fn create_int_literal(value: i32) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_INT)
    }

    pub fn create_short_literal(value: i16) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_SHORT)
    }

    pub fn create_byte_literal(value: i8) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_BYTE)
    }

    pub fn create_boolean_literal(value: bool) -> Literal{
        Literal::create_typed_literal(value.to_string(), XSD_BOOLEAN)
    }

    pub fn create_any_uri_literal(value: URI) -> Literal{
        Literal::create_typed_literal(value.as_string(), XSD_ANY_URI)
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
    /// let uri_literal = Literal::create_typed_literal(String::from("http://my-uri.com/Person1"), XSD_ANY_URI);
    /// let int_literal = Literal::create_typed_literal(String::from("12345"), XSD_INT);
    /// let simple_string_literal = Literal::create_typed_literal(String::from("my-value"), XSD_STRING);
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
                dtype: XSD_STRING,
                lang: None
            }
        }else {
            Literal {
                value,
                dtype: XSD_STRING,
                lang: Some(lang)
            }
        }
    }

    pub fn parse_literal(literal: &str) -> Result<Literal, E>{
        //true|false|number|quotes " ' ''' | @ | ^^<> | ^^prefix:suffix
        let mut starts_with="";
        if literal == "true"{
            Ok(Literal::create_boolean_literal(true))
        }
        else if literal == "false"{
            Ok(Literal::create_boolean_literal(false))
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
            Ok(Literal::create_literal(String::from(literal)));
        }
        else{
            //swABCsw@?||^^
            let end_of_str = match literal.rfind(starts_with){
                Some(val) => val,
                None => return Result::Err("Literal starts with "+starts_with+" but doesn't close.")
            };
            let val = String::from(&literal[starts_with.len()..end_of_str]);
            if literal[end_of_str+1..].starts_with("@"){
                if literal[end_of_str+2..].contains("^^"){
                    Result::Err("Literal cannot contain language tag as well as datatype.")
                }
                Ok(Literal::create_lang_literal(val, String::from(&literal[end_of_str+2..])))
            }
            else if literal[end_of_str+1..].starts_with("^^"){
                // datatype
                //TODO prefix mapping
                Ok(Literal::create_typed_literal(val, dtype));
            }
            else{
                Ok(Literal::create_literal(val));
            }
        }
        Err("Unexpected Error. Shouldn't occur")
    }

}

impl RDFNode for Literal{

    fn is_literal(&self) -> bool {
        true
    }

    fn as_string(&self, quoting: bool) -> String {
        if self.dtype.is_xsd_string(){
            if let Some(lang) = &self.lang {
                // language is set so append
                return  "\""+&self.value+"\"@"+self.lang.expect("checked");
            }
            //language is not set, so check if quoting is true and quote if so
            if quoting { "\""+&self.value+"\"" } else {&self.value}
        }
        //datatype is not string, so add datatype.
        "\""+&self.value+"\"^^<"+self.dtype.get_value()+">"
    }

    fn get_value(&self) -> &String {
        &self.value
    }
}



pub struct BlankNode{
    id: String
}

pub struct URIResource {
    uri: URI
}

impl URIResource {
    pub fn test(&self){}
    pub fn get_uri(&self) -> &URI{
        &self.uri
    }

    pub fn create_resource(uri: URI) -> URIResource{
        URIResource{
            uri
        }
    }
}

impl Resource for BlankNode{
}

impl Resource for URIResource{

}

impl RDFNode for URIResource {
    fn is_literal(&self) -> bool{
        return false;
    }
    fn as_string(&self, quoting: bool) -> &String{
        if quoting{
            String::from("<"+&self.uri.as_string()+">")
        }
        &self.uri.as_string()
    }

    fn get_value(&self) -> &String {
        &self.uri.as_string()
    }
}


