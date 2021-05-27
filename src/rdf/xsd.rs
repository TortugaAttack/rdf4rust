use crate::util::iri::IRI;

/// Base URL for XSD =  http://www.w3.org/2001/XMLSchema#
pub fn xsd_base() -> String {
    String::from("http://www.w3.org/2001/XMLSchema#")
}
/// XSD Type String = http://www.w3.org/2001/XMLSchema#string
pub fn xsd_string() -> XSDDataType {
    XSDDataType::create_from_string((xsd_base() + "string")).expect("")
}
/// XSD Type Unsigned Byte = http://www.w3.org/2001/XMLSchema#unsignedByte
pub fn xsd_unsigned_byte() -> XSDDataType {
    XSDDataType::create_from_string((xsd_base() +"unsignedByte")).expect("")
}
/// XSD Type Unsigned Short = http://www.w3.org/2001/XMLSchema#unsignedShort
pub fn xsd_unsigned_short() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"unsignedShort")).expect("")
}
/// XSD Type Unsigned Int = http://www.w3.org/2001/XMLSchema#unsignedInt
pub fn xsd_unsigned_int() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"unsignedInt")).expect("")
}
/// XSD Type Unsigned Long = http://www.w3.org/2001/XMLSchema#unsignedLong
pub fn xsd_unsigned_long() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"unsignedLong")).expect("")
}
/// XSD Type Byte = http://www.w3.org/2001/XMLSchema#byte
pub fn xsd_byte() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"byte")).expect("")
}
/// XSD Type Short = http://www.w3.org/2001/XMLSchema#short
pub fn xsd_short() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"short")).expect("")
}
/// XSD Type Int = http://www.w3.org/2001/XMLSchema#int
pub fn xsd_int() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"int")).expect("")
}
/// XSD Type Long = http://www.w3.org/2001/XMLSchema#long
pub fn xsd_long() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"long")).expect("")
}
/// XSD Type Integer = http://www.w3.org/2001/XMLSchema#int
///
/// Will be internally used as i64 due to semantics
pub fn xsd_integer() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"integer")).expect("")
}
/// XSD Type Non negative Integer = http://www.w3.org/2001/XMLSchema#nonNegativeInteger
///
/// Will be internally used as usize
pub fn xsd_non_negative_integer() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"nonNegativeInteger")).expect("")
}
/// XSD Type Non positive Integer = http://www.w3.org/2001/XMLSchema#nonPositiveInteger
///
/// Will be internally used as isize
pub fn xsd_non_positive_integer() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"nonPositiveInteger")).expect("")
}
/// XSD Type Positive Integer = http://www.w3.org/2001/XMLSchema#positiveInteger
///
/// Will be internally used as NonZeroUsize
pub fn xsd_positive_integer() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"positiveInteger")).expect("")
}
/// XSD Type Negative Integer = http://www.w3.org/2001/XMLSchema#negativeInteger
///
/// Will be internally used as NonZeroIsize
pub fn xsd_negative_integer() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"negativeInteger")).expect("")
}
/// XSD Type Decimal = http://www.w3.org/2001/XMLSchema#decimal
///
/// Will be internally used as f64
pub fn xsd_decimal() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"decimal").expect("")
}
/// XSD Type Float = http://www.w3.org/2001/XMLSchema#float
///
/// will be internally used as f32
pub fn xsd_float() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"float")).expect("")
}
/// XSD Type Double = http://www.w3.org/2001/XMLSchema#double
///
/// will be internally used as f64
pub fn xsd_double() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"double")).expect("")
}
/// XSD Type Boolean = http://www.w3.org/2001/XMLSchema#boolean
pub fn xsd_boolean() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"boolean")).expect("")
}
/// XSD Type Super Type = http://www.w3.org/2001/XMLSchema#anyType
///
/// Will use String for this value
pub fn xsd_any_type() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"anyType")).expect("")
}
/// XSD Type Super Type for Simple Types = http://www.w3.org/2001/XMLSchema#anySimpleType
///
/// Will use String for this value
pub fn xsd_any_simple_type() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"anySimpleType")).expect("")
}
/// XSD URI Type = http://www.w3.org/2001/XMLSchema#anyUri
///
/// Will use rdf4rust::util::iri::IRI for the value
pub fn xsd_any_uri() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"anyUri")).expect("")
}
/// XSD Binary Type in Base64 format = http://www.w3.org/2001/XMLSchema#base64Binary
///
/// Will use &[u8] for this value
pub fn xsd_base64binary() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"base64Binary")).expect("")
}
/// XSD Binary Type in HEX format = http://www.w3.org/2001/XMLSchema#hexBinary
///
/// Will use &[u8] for this value
pub fn xsd_hex_binary() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"hexBinary")).expect("")
}
/// XSD Type Duration = http://www.w3.org/2001/XMLSchema#duration
pub fn xsd_duration() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"duration")).expect("")
}
/// XSD Type Date time = http://www.w3.org/2001/XMLSchema#dateTime
pub fn xsd_date_time() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"dateTime")).expect("")
}
/// XSD Type time = http://www.w3.org/2001/XMLSchema#time
pub fn xsd_time() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"time")).expect("")
}
/// XSD Type date = http://www.w3.org/2001/XMLSchema#date
pub fn xsd_date() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"date")).expect("")
}
/// XSD Type gYearMonth = http://www.w3.org/2001/XMLSchema#gYearMonth
pub fn xsd_g_year_month() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"gYearMonth")).expect("")
}
/// XSD Type gYear = http://www.w3.org/2001/XMLSchema#gYear
pub fn xsd_g_year() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"gYear")).expect("")
}
/// XSD Type gMonthDay = http://www.w3.org/2001/XMLSchema#gMonthDay --MM--DD --MM--DD(Z+-)0digit|1[0-3]:0[0..5]digit
pub fn xsd_g_month_day() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"gMonthDay")).expect("")
}
/// XSD Type gDay = http://www.w3.org/2001/XMLSchema#gDay
pub fn xsd_g_day() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"gDay")).expect("")
}
/// XSD Type gMonth = http://www.w3.org/2001/XMLSchema#gMonth
pub fn xsd_g_month() -> XSDDataType{
	XSDDataType::create_from_string((xsd_base() +"gMonth")).expect("")
}

///
/// The XSD Data type - usually http://www.w3.org/2001/XMLSchema# and the datatype name
///
/// For distinction between the different types have a look at https://www.eclipse.org/modeling/emf/docs/xsd/dW/os-schema2/os-schema2-3-2.html
#[derive(PartialOrd, PartialEq, Eq, Hash)]
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
        if let Some(xsd_uri) = IRI::create_iri(&uri).ok(){
            Some(XSDDataType::create(xsd_uri))
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
    /// use rdf4rust::rdf::xsd::{xsd_string, xsd_any_type};
    ///
    /// assert!(xsd_string().is_xsd_string());
    ///
    /// assert!(!xsd_any_type().is_xsd_string())
    /// ```
    ///
    pub fn is_xsd_string(&self) -> bool{
        self.dtype.as_string() == xsd_string().dtype.as_string()
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
    /// use rdf4rust::rdf::xsd::xsd_boolean;
    ///
    /// let uri = xsd_boolean().get_value();
    ///
    /// assert_eq!("http://www.w3.org/2001/XMLSchema#boolean", uri.as_string());
    /// ```
    ///
    pub fn get_value(&self) -> &IRI {
        &self.dtype
    }

}