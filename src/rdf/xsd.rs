use crate::util::iri::URI;

/// Base URL for XSD =  http://www.w3.org/2001/XMLSchema#
pub const XSD_BASE: String = String::from("http://www.w3.org/2001/XMLSchema#");
/// XSD Type String = http://www.w3.org/2001/XMLSchema#string
pub const XSD_STRING: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"string").as_str()).ok().expect("");
/// XSD Type Unsigned Byte = http://www.w3.org/2001/XMLSchema#unsignedByte
pub const XSD_UNSIGNED_BYTE: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"unsignedByte").as_str()).ok().expect("");
/// XSD Type Unsigned Short = http://www.w3.org/2001/XMLSchema#unsignedShort
pub const XSD_UNSIGNED_SHORT: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"unsignedShort").as_str()).ok().expect("");
/// XSD Type Unsigned Int = http://www.w3.org/2001/XMLSchema#unsignedInt
pub const XSD_UNSIGNED_INT: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"unsignedInt").as_str()).ok().expect("");
/// XSD Type Unsigned Long = http://www.w3.org/2001/XMLSchema#unsignedLong
pub const XSD_UNSIGNED_LONG: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"unsignedLong").as_str()).ok().expect("");
/// XSD Type Byte = http://www.w3.org/2001/XMLSchema#byte
pub const XSD_BYTE: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"byte").as_str()).ok().expect("");
/// XSD Type Short = http://www.w3.org/2001/XMLSchema#short
pub const XSD_SHORT: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"short").as_str()).ok().expect("");
/// XSD Type Int = http://www.w3.org/2001/XMLSchema#int
pub const XSD_INT: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"int").as_str()).ok().expect("");
/// XSD Type Long = http://www.w3.org/2001/XMLSchema#long
pub const XSD_LONG: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"long").as_str()).ok().expect("");
/// XSD Type Integer = http://www.w3.org/2001/XMLSchema#int
///
/// Will be internally used as i64 due to semantics
pub const XSD_INTEGER: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"integer").as_str()).ok().expect("");
/// XSD Type Non negative Integer = http://www.w3.org/2001/XMLSchema#nonNegativeInteger
///
/// Will be internally used as usize
pub const XSD_NON_NEGATIVE_INTEGER: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"nonNegativeInteger").as_str()).ok().expect("");
/// XSD Type Non positive Integer = http://www.w3.org/2001/XMLSchema#nonPositiveInteger
///
/// Will be internally used as isize
pub const XSD_NON_POSITIVE_INTEGER: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"nonPositiveInteger").as_str()).ok().expect("");
/// XSD Type Positive Integer = http://www.w3.org/2001/XMLSchema#positiveInteger
///
/// Will be internally used as NonZeroUsize
pub const XSD_POSITIVE_INTEGER: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"positiveInteger").as_str()).ok().expect("");
/// XSD Type Negative Integer = http://www.w3.org/2001/XMLSchema#negativeInteger
///
/// Will be internally used as NonZeroIsize
pub const XSD_NEGATIVE_INTEGER: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"negativeInteger").as_str()).ok().expect("");
/// XSD Type Decimal = http://www.w3.org/2001/XMLSchema#decimal
///
/// Will be internally used as i64
pub const XSD_DECIMAL: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"decimal").as_str()).ok().expect("");
/// XSD Type Float = http://www.w3.org/2001/XMLSchema#float
///
/// will be internally used as f32
pub const XSD_FLOAT: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"float").as_str()).ok().expect("");
/// XSD Type Double = http://www.w3.org/2001/XMLSchema#double
///
/// will be internally used as f64
pub const XSD_DOUBLE: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"double").as_str()).ok().expect("");
/// XSD Type Boolean = http://www.w3.org/2001/XMLSchema#boolean
pub const XSD_BOOLEAN: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"boolean").as_str()).ok().expect("");
/// XSD Type Super Type = http://www.w3.org/2001/XMLSchema#anyType
///
/// Will use String for this value
pub const XSD_ANY_TYPE: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"anyType").as_str()).ok().expect("");
/// XSD Type Super Type for Simple Types = http://www.w3.org/2001/XMLSchema#anySimpleType
///
/// Will use String for this value
pub const XSD_ANY_SIMPLE_TYPE: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"anySimpleType").as_str()).ok().expect("");
/// XSD URI Type = http://www.w3.org/2001/XMLSchema#anyUri
///
/// Will use rdf4rust::util::iri::URI for the value
pub const XSD_ANY_URI: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"anyUri").as_str()).ok().expect("");
/// XSD Binary Type in Base64 format = http://www.w3.org/2001/XMLSchema#base64Binary
///
/// Will use String for this value
pub const XSD_BASE64BINARY: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"base64Binary").as_str()).ok().expect("");
/// XSD Binary Type in HEX format = http://www.w3.org/2001/XMLSchema#hexBinary
///
/// Will use String for this value
pub const XSD_HEX_BINARY: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"hexBinary").as_str()).ok().expect("");
/// XSD Type Duration = http://www.w3.org/2001/XMLSchema#duration
pub const XSD_DURATION: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"duration").as_str()).ok().expect("");
/// XSD Type Date time = http://www.w3.org/2001/XMLSchema#dateTime
pub const XSD_DATE_TIME: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"dateTime").as_str()).ok().expect("");
/// XSD Type time = http://www.w3.org/2001/XMLSchema#time
pub const XSD_TIME: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"time").as_str()).ok().expect("");
/// XSD Type date = http://www.w3.org/2001/XMLSchema#date
pub const XSD_DATE: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"date").as_str()).ok().expect("");
/// XSD Type gYearMonth = http://www.w3.org/2001/XMLSchema#gYearMonth
pub const XSD_G_YEAR_MONTH: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"gYearMonth").as_str()).ok().expect("");
/// XSD Type gYear = http://www.w3.org/2001/XMLSchema#gYear
pub const XSD_G_YEAR: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"gYear").as_str()).ok().expect("");
/// XSD Type gMonthDay = http://www.w3.org/2001/XMLSchema#gMonthDay
pub const XSD_G_MONTH_DAY: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"gMonthDay").as_str()).ok().expect("");
/// XSD Type gDay = http://www.w3.org/2001/XMLSchema#gDay
pub const XSD_G_DAY: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"gDay").as_str()).ok().expect("");
/// XSD Type gMonth = http://www.w3.org/2001/XMLSchema#gMonth
pub const XSD_G_MONTH: XSDDataType = XSDDataType::create_from_string((XSD_BASE+"gMonth").as_str()).ok().expect("");

///
/// The XSD Data type - usually http://www.w3.org/2001/XMLSchema# and the datatype name
///
/// For distinction between the different types have a look at https://www.eclipse.org/modeling/emf/docs/xsd/dW/os-schema2/os-schema2-3-2.html
#[derive(Debug)]
pub struct XSDDataType{
    dtype: URI
}

impl XSDDataType{

    ///
    /// Creates a DataType from a String.
    ///
    /// Will be None if `uri` is not a valid URI
    ///
    /// # Parameters
    ///
    /// * `uri` - String literal which represents the DataType
    ///
    /// # Example
    ///
    /// ```
    ///   use rdf4rust::rdf::xsd::XSDDataType;
    ///
    /// if let Some(dtype) = XSDDataType::create_from_string("http://my-example-datatypes.com/datatypes#MyDatatype"){
    ///     //use dtype here
    /// }else{
    ///     //uri wasn't valid
    /// }
    ///
    /// ```
    ///
    pub fn create_from_string(uri: &str) -> Option<XSDDataType>{
        if let Some(xsdUri) = URI::create_uri(uri).ok(){
            Some(xsdUri)
        }
        None
    }

    ///
    /// Creates a DataType from a `URI` object
    ///
    /// # Parameters
    ///
    /// * `uri` - The URI representing the data type
    ///
    /// # Example
    ///
    /// ```
    /// use rdf4rust::util::iri::URI;
    /// use rdf4rust::rdf::xsd::XSDDataType;
    ///
    /// if let Some(uri) = URI::create_uri("http://my-example-datatypes.com/datatypes#MyDatatype"){
    ///     let dtype = XSDDataType::create(uri);
    ///     //use dtype here
    /// }else{
    ///     //uri wasn't valid
    /// }
    /// ```
    ///
    pub fn create(uri : URI) -> XSDDataType{
        XSDDataType{
            dtype: uri
        }
    }

    ///
    /// Shortcut to check if the XSDDataType is `xsd:string`
    ///
    /// # Returns
    ///
    /// * `bool` - True if this instances is xsd:string, false otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use rdf4rust::rdf::xsd::{XSD_STRING, XSD_ANY_TYPE};
    ///
    /// assert!(XSD_STRING.is_xsd_string());
    ///
    /// assert!(!XSD_ANY_TYPE.is_xsd_string())
    /// ```
    ///
    pub fn is_xsd_string(&self) -> bool{
        self.dtype == XSD_STRING.dtype
    }

    ///
    /// Gets you a reference to the underlying URI object of this data type
    ///
    /// # Returns
    ///
    /// * `URI` - The URI representing this XSDDataType
    ///
    /// # Example
    ///
    /// ```
    /// use rdf4rust::rdf::xsd::XSD_BOOLEAN;
    ///
    /// let uri = XSD_BOOLEAN.get_value();
    ///
    /// assert_eq!("http://www.w3.org/2001/XMLSchema#boolean", uri.as_string());
    /// ```
    ///
    pub fn get_value(&self) -> &URI {
        &self.dtype
    }

}