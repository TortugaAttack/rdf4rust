use crate::util::iri::IRI;

/// Base URL for XSD =  http://www.w3.org/2001/XMLSchema#
pub fn XSD_BASE() -> String {
    String::from("http://www.w3.org/2001/XMLSchema#")
}
/// XSD Type String = http://www.w3.org/2001/XMLSchema#string
pub fn XSD_STRING() -> XSDDataType {
    XSDDataType::create_from_string((XSD_BASE() + "string")).expect("")
}
/// XSD Type Unsigned Byte = http://www.w3.org/2001/XMLSchema#unsignedByte
pub fn XSD_UNSIGNED_BYTE() -> XSDDataType {
    XSDDataType::create_from_string((XSD_BASE() +"unsignedByte")).expect("")
}
/// XSD Type Unsigned Short = http://www.w3.org/2001/XMLSchema#unsignedShort
pub fn XSD_UNSIGNED_SHORT() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"unsignedShort")).expect("")
}
/// XSD Type Unsigned Int = http://www.w3.org/2001/XMLSchema#unsignedInt
pub fn XSD_UNSIGNED_INT() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"unsignedInt")).expect("")
}
/// XSD Type Unsigned Long = http://www.w3.org/2001/XMLSchema#unsignedLong
pub fn XSD_UNSIGNED_LONG() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"unsignedLong")).expect("")
}
/// XSD Type Byte = http://www.w3.org/2001/XMLSchema#byte
pub fn XSD_BYTE() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"byte")).expect("")
}
/// XSD Type Short = http://www.w3.org/2001/XMLSchema#short
pub fn XSD_SHORT() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"short")).expect("")
}
/// XSD Type Int = http://www.w3.org/2001/XMLSchema#int
pub fn XSD_INT() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"int")).expect("")
}
/// XSD Type Long = http://www.w3.org/2001/XMLSchema#long
pub fn XSD_LONG() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"long")).expect("")
}
/// XSD Type Integer = http://www.w3.org/2001/XMLSchema#int
///
/// Will be internally used as i64 due to semantics
pub fn XSD_INTEGER() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"integer")).expect("")
}
/// XSD Type Non negative Integer = http://www.w3.org/2001/XMLSchema#nonNegativeInteger
///
/// Will be internally used as usize
pub fn XSD_NON_NEGATIVE_INTEGER() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"nonNegativeInteger")).expect("")
}
/// XSD Type Non positive Integer = http://www.w3.org/2001/XMLSchema#nonPositiveInteger
///
/// Will be internally used as isize
pub fn XSD_NON_POSITIVE_INTEGER() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"nonPositiveInteger")).expect("")
}
/// XSD Type Positive Integer = http://www.w3.org/2001/XMLSchema#positiveInteger
///
/// Will be internally used as NonZeroUsize
pub fn XSD_POSITIVE_INTEGER() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"positiveInteger")).expect("")
}
/// XSD Type Negative Integer = http://www.w3.org/2001/XMLSchema#negativeInteger
///
/// Will be internally used as NonZeroIsize
pub fn XSD_NEGATIVE_INTEGER() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"negativeInteger")).expect("")
}
/// XSD Type Decimal = http://www.w3.org/2001/XMLSchema#decimal
///
/// Will be internally used as i64
pub fn XSD_DECIMAL() -> XSDDataType{ 
	XSDDataType::create_from_string(XSD_BASE() +"decimal").expect("")
}
/// XSD Type Float = http://www.w3.org/2001/XMLSchema#float
///
/// will be internally used as f32
pub fn XSD_FLOAT() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"float")).expect("")
}
/// XSD Type Double = http://www.w3.org/2001/XMLSchema#double
///
/// will be internally used as f64
pub fn XSD_DOUBLE() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"double")).expect("")
}
/// XSD Type Boolean = http://www.w3.org/2001/XMLSchema#boolean
pub fn XSD_BOOLEAN() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"boolean")).expect("")
}
/// XSD Type Super Type = http://www.w3.org/2001/XMLSchema#anyType
///
/// Will use String for this value
pub fn XSD_ANY_TYPE() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"anyType")).expect("")
}
/// XSD Type Super Type for Simple Types = http://www.w3.org/2001/XMLSchema#anySimpleType
///
/// Will use String for this value
pub fn XSD_ANY_SIMPLE_TYPE() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"anySimpleType")).expect("")
}
/// XSD URI Type = http://www.w3.org/2001/XMLSchema#anyUri
///
/// Will use rdf4rust::util::iri::URI for the value
pub fn XSD_ANY_URI() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"anyUri")).expect("")
}
/// XSD Binary Type in Base64 format = http://www.w3.org/2001/XMLSchema#base64Binary
///
/// Will use String for this value
pub fn XSD_BASE64BINARY() -> XSDDataType{
	XSDDataType::create_from_string((XSD_BASE() +"base64Binary")).expect("")
}
/// XSD Binary Type in HEX format = http://www.w3.org/2001/XMLSchema#hexBinary
///
/// Will use String for this value
pub fn XSD_HEX_BINARY() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"hexBinary")).expect("")
}
/// XSD Type Duration = http://www.w3.org/2001/XMLSchema#duration
pub fn XSD_DURATION() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"duration")).expect("")
}
/// XSD Type Date time = http://www.w3.org/2001/XMLSchema#dateTime
pub fn XSD_DATE_TIME() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"dateTime")).expect("")
}
/// XSD Type time = http://www.w3.org/2001/XMLSchema#time
pub fn XSD_TIME() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"time")).expect("")
}
/// XSD Type date = http://www.w3.org/2001/XMLSchema#date
pub fn XSD_DATE() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"date")).expect("")
}
/// XSD Type gYearMonth = http://www.w3.org/2001/XMLSchema#gYearMonth
pub fn XSD_G_YEAR_MONTH() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"gYearMonth")).expect("")
}
/// XSD Type gYear = http://www.w3.org/2001/XMLSchema#gYear
pub fn XSD_G_YEAR() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"gYear")).expect("")
}
/// XSD Type gMonthDay = http://www.w3.org/2001/XMLSchema#gMonthDay
pub fn XSD_G_MONTH_DAY() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"gMonthDay")).expect("")
}
/// XSD Type gDay = http://www.w3.org/2001/XMLSchema#gDay
pub fn XSD_G_DAY() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"gDay")).expect("")
}
/// XSD Type gMonth = http://www.w3.org/2001/XMLSchema#gMonth
pub fn XSD_G_MONTH() -> XSDDataType{ 
	XSDDataType::create_from_string((XSD_BASE() +"gMonth")).expect("")
}

///
/// The XSD Data type - usually http://www.w3.org/2001/XMLSchema# and the datatype name
///
/// For distinction between the different types have a look at https://www.eclipse.org/modeling/emf/docs/xsd/dW/os-schema2/os-schema2-3-2.html
pub struct XSDDataType{
    dtype: IRI
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
    /// if let Some(dtype) = XSDDataType::create_from_string(String::from("http://my-example-datatypes.com/datatypes#MyDatatype")){
    ///     //use dtype here
    /// }else{
    ///     //uri wasn't valid
    /// }
    ///
    /// ```
    ///
    pub fn create_from_string(uri: String) -> Option<XSDDataType>{
        if let Some(xsdUri) = IRI::create_iri(&uri).ok(){
            Some(XSDDataType::create(xsdUri))
        }else {
            None
        }
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
    /// use rdf4rust::util::iri::IRI;
    /// use rdf4rust::rdf::xsd::XSDDataType;
    ///
    /// if let Some(uri) = IRI::create_iri(&String::from("http://my-example-datatypes.com/datatypes#MyDatatype")){
    ///     let dtype = XSDDataType::create(uri);
    ///     //use dtype here
    /// }else{
    ///     //uri wasn't valid
    /// }
    /// ```
    ///
    pub fn create(uri : IRI) -> XSDDataType{
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
    /// assert!(XSD_STRING().is_xsd_string());
    ///
    /// assert!(!XSD_ANY_TYPE().is_xsd_string())
    /// ```
    ///
    pub fn is_xsd_string(&self) -> bool{
        self.dtype.as_string() == XSD_STRING().dtype.as_string()
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
    /// let uri = XSD_BOOLEAN().get_value();
    ///
    /// assert_eq!("http://www.w3.org/2001/XMLSchema#boolean", uri.as_string());
    /// ```
    ///
    pub fn get_value(&self) -> &IRI {
        &self.dtype
    }

}