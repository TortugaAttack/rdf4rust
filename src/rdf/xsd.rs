

use crate::util::iri::IRI;
use std::collections::HashMap;
use std::sync::Mutex;
use std::borrow::Borrow;


lazy_static! {
	static ref ADDITIONAL_TYPES: Mutex<HashMap<String, &'static XSDDataType>> = Mutex::new(HashMap::new());
}

lazy_static! {
	pub static ref XSD_STRING: XSDDataType = xsd_string();
	pub static ref XSD_UNSIGNED_BYTE: XSDDataType = xsd_unsigned_byte();
	pub static ref XSD_UNSIGNED_SHORT: XSDDataType = xsd_unsigned_short();
	pub static ref XSD_UNSIGNED_INT: XSDDataType = xsd_unsigned_int();
	pub static ref XSD_UNSIGNED_LONG: XSDDataType = xsd_unsigned_long();
	pub static ref XSD_BYTE: XSDDataType = xsd_byte();
	pub static ref XSD_SHORT: XSDDataType = xsd_short();
	pub static ref XSD_INT: XSDDataType = xsd_int();
	pub static ref XSD_LONG: XSDDataType = xsd_long();
	pub static ref XSD_INTEGER: XSDDataType = xsd_integer();
	pub static ref XSD_NON_NEGATIVE_INTEGER: XSDDataType = xsd_non_positive_integer();
	pub static ref XSD_NON_POSITIVE_INTEGER: XSDDataType = xsd_non_positive_integer();
	pub static ref XSD_POSITIVE_INTEGER: XSDDataType = xsd_positive_integer();
	pub static ref XSD_NEGATIVE_INTEGER: XSDDataType = xsd_negative_integer();
	pub static ref XSD_DECIMAL: XSDDataType = xsd_decimal();
	pub static ref XSD_FLOAT: XSDDataType = xsd_float();
	pub static ref XSD_DOUBLE: XSDDataType = xsd_double();
	pub static ref XSD_BOOLEAN: XSDDataType = xsd_boolean();
	pub static ref XSD_ANY_TYPE: XSDDataType = xsd_any_type();
	pub static ref XSD_ANY_SIMPLE_TYPE: XSDDataType = xsd_any_simple_type();
	pub static ref XSD_ANY_URI: XSDDataType = xsd_any_uri();
	pub static ref XSD_HEX_BINARY: XSDDataType = xsd_hex_binary();
	pub static ref XSD_BASE64_BINARY: XSDDataType = xsd_base64binary();
	pub static ref XSD_DURATION: XSDDataType = xsd_duration();
	pub static ref XSD_DATE_TIME: XSDDataType = xsd_date_time();
	pub static ref XSD_TIME: XSDDataType = xsd_time();
	pub static ref XSD_DATE: XSDDataType = xsd_date();
	pub static ref XSD_G_YEAR_MONTH: XSDDataType = xsd_g_year_month();
	pub static ref XSD_G_YEAR: XSDDataType = xsd_g_year();
	pub static ref XSD_G_MONTH: XSDDataType = xsd_g_month();
	pub static ref XSD_G_DAY: XSDDataType = xsd_g_day();
	pub static ref XSD_G_MONTH_DAY: XSDDataType = xsd_g_month_day();
}

/// Base URL for XSD =  http://www.w3.org/2001/XMLSchema#
pub fn xsd_base() -> String {
    String::from("http://www.w3.org/2001/XMLSchema#")
}
/// XSD Type String = http://www.w3.org/2001/XMLSchema#string
pub fn xsd_string() -> XSDDataType {
    XSDDataType::create_from_string(xsd_base() + "string", |s| true).expect("")
}
/// XSD Type Unsigned Byte = http://www.w3.org/2001/XMLSchema#unsignedByte
pub fn xsd_unsigned_byte() -> XSDDataType {
    XSDDataType::create_from_string(xsd_base() +"unsignedByte", |s| true).expect("")
}
/// XSD Type Unsigned Short = http://www.w3.org/2001/XMLSchema#unsignedShort
pub fn xsd_unsigned_short() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"unsignedShort", |s| true).expect("")
}
/// XSD Type Unsigned Int = http://www.w3.org/2001/XMLSchema#unsignedInt
pub fn xsd_unsigned_int() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"unsignedInt", |s| true).expect("")
}
/// XSD Type Unsigned Long = http://www.w3.org/2001/XMLSchema#unsignedLong
pub fn xsd_unsigned_long() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"unsignedLong", |s| true).expect("")
}
/// XSD Type Byte = http://www.w3.org/2001/XMLSchema#byte
pub fn xsd_byte() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"byte", |s| true).expect("")
}
/// XSD Type Short = http://www.w3.org/2001/XMLSchema#short
pub fn xsd_short() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"short", |s| true).expect("")
}
/// XSD Type Int = http://www.w3.org/2001/XMLSchema#int
pub fn xsd_int() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"int", |s| true).expect("")
}
/// XSD Type Long = http://www.w3.org/2001/XMLSchema#long
pub fn xsd_long() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"long", |s| true).expect("")
}
/// XSD Type Integer = http://www.w3.org/2001/XMLSchema#int
///
/// Will be internally used as i64 due to semantics
pub fn xsd_integer() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"integer", |s| true).expect("")
}
/// XSD Type Non negative Integer = http://www.w3.org/2001/XMLSchema#nonNegativeInteger
///
/// Will be internally used as usize
pub fn xsd_non_negative_integer() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"nonNegativeInteger", |s| true).expect("")
}
/// XSD Type Non positive Integer = http://www.w3.org/2001/XMLSchema#nonPositiveInteger
///
/// Will be internally used as isize
pub fn xsd_non_positive_integer() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"nonPositiveInteger", |s| true).expect("")
}
/// XSD Type Positive Integer = http://www.w3.org/2001/XMLSchema#positiveInteger
///
/// Will be internally used as NonZeroUsize
pub fn xsd_positive_integer() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"positiveInteger", |s| true).expect("")
}
/// XSD Type Negative Integer = http://www.w3.org/2001/XMLSchema#negativeInteger
///
/// Will be internally used as NonZeroIsize
pub fn xsd_negative_integer() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"negativeInteger", |s| true).expect("")
}
/// XSD Type Decimal = http://www.w3.org/2001/XMLSchema#decimal
///
/// Will be internally used as f64
pub fn xsd_decimal() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"decimal", |s| true).expect("")
}
/// XSD Type Float = http://www.w3.org/2001/XMLSchema#float
///
/// will be internally used as f32
pub fn xsd_float() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"float", |s| true).expect("")
}
/// XSD Type Double = http://www.w3.org/2001/XMLSchema#double
///
/// will be internally used as f64
pub fn xsd_double() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"double", |s| true).expect("")
}
/// XSD Type Boolean = http://www.w3.org/2001/XMLSchema#boolean
pub fn xsd_boolean() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"boolean", |s| s == "true" || s=="false").expect("")
}
/// XSD Type Super Type = http://www.w3.org/2001/XMLSchema#anyType
///
/// Will use String for this value
pub fn xsd_any_type() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"anyType", |s| true).expect("")
}
/// XSD Type Super Type for Simple Types = http://www.w3.org/2001/XMLSchema#anySimpleType
///
/// Will use String for this value
pub fn xsd_any_simple_type() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"anySimpleType", |s| true).expect("")
}
/// XSD URI Type = http://www.w3.org/2001/XMLSchema#anyUri
///
/// Will use rdf4rust::util::iri::IRI for the value
pub fn xsd_any_uri() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"anyUri", |s| true).expect("")
}
/// XSD Binary Type in Base64 format = http://www.w3.org/2001/XMLSchema#base64Binary
///
/// Will use &[u8] for this value
pub fn xsd_base64binary() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"base64Binary", |s| true).expect("")
}
/// XSD Binary Type in HEX format = http://www.w3.org/2001/XMLSchema#hexBinary
///
/// Will use &[u8] for this value
pub fn xsd_hex_binary() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"hexBinary", |s| true).expect("")
}
/// XSD Type Duration = http://www.w3.org/2001/XMLSchema#duration
pub fn xsd_duration() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"duration", |s| true).expect("")
}
/// XSD Type Date time = http://www.w3.org/2001/XMLSchema#dateTime
pub fn xsd_date_time() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"dateTime", |s| true).expect("")
}
/// XSD Type time = http://www.w3.org/2001/XMLSchema#time
pub fn xsd_time() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"time", |s| true).expect("")
}
/// XSD Type date = http://www.w3.org/2001/XMLSchema#date
pub fn xsd_date() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"date", |s| true).expect("")
}
/// XSD Type gYearMonth = http://www.w3.org/2001/XMLSchema#gYearMonth
pub fn xsd_g_year_month() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"gYearMonth", |s| true).expect("")
}
/// XSD Type gYear = http://www.w3.org/2001/XMLSchema#gYear
pub fn xsd_g_year() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"gYear", |s| true).expect("")
}
/// XSD Type gMonthDay = http://www.w3.org/2001/XMLSchema#gMonthDay --MM--DD --MM--DD(Z+-)0digit|1[0-3]:0[0..5]digit
pub fn xsd_g_month_day() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"gMonthDay", |s| true).expect("")
}
/// XSD Type gDay = http://www.w3.org/2001/XMLSchema#gDay
pub fn xsd_g_day() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"gDay", |s| true).expect("")
}
/// XSD Type gMonth = http://www.w3.org/2001/XMLSchema#gMonth
pub fn xsd_g_month() -> XSDDataType{
	XSDDataType::create_from_string(xsd_base() +"gMonth", |s| true).expect("")
}

///
/// The XSD Data type - usually http://www.w3.org/2001/XMLSchema# and the datatype name
///
/// For distinction between the different types have a look at https://www.eclipse.org/modeling/emf/docs/xsd/dW/os-schema2/os-schema2-3-2.html
#[derive(PartialOrd, PartialEq, Eq, Hash)]
pub struct XSDDataType{
    dtype: String,
	validation: fn(String) -> bool,
}

impl XSDDataType{

	pub fn get_or_default(iri: IRI) -> &'static XSDDataType{
		return match iri.as_string().as_str() {
			"http://www.w3.org/2001/XMLSchema#string" => { &XSD_STRING as &XSDDataType}
			"http://www.w3.org/2001/XMLSchema#unsignedByte" => { &XSD_UNSIGNED_BYTE as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#unsignedShort" => { &XSD_UNSIGNED_SHORT as &XSDDataType}
			"http://www.w3.org/2001/XMLSchema#unsignedInt" => { &XSD_UNSIGNED_INT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#unsignedLong" => { &XSD_UNSIGNED_LONG as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#byte" => { &XSD_BYTE as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#short" => { &XSD_SHORT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#int" => { &XSD_UNSIGNED_SHORT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#long" => { &XSD_LONG as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#integer" => { &XSD_UNSIGNED_SHORT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#nonNegativeInteger" => { &XSD_UNSIGNED_SHORT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#nonPositiveInteger" => { &XSD_UNSIGNED_SHORT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#positiveInteger" => { &XSD_UNSIGNED_SHORT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#negativeInteger" => { &XSD_UNSIGNED_SHORT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#decimal" => { &XSD_UNSIGNED_SHORT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#float" => { &XSD_UNSIGNED_SHORT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#double" => { &XSD_UNSIGNED_SHORT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#boolean" => { &XSD_UNSIGNED_SHORT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#anyType" => { &XSD_UNSIGNED_SHORT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#anySimpleType" => { &XSD_UNSIGNED_SHORT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#anyUri" => { &XSD_UNSIGNED_SHORT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#hexBinary" => { &XSD_UNSIGNED_SHORT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#base64Binary" => { &XSD_UNSIGNED_SHORT as &XSDDataType }
			"http://www.w3.org/2001/XMLSchema#duration" => {&XSD_UNSIGNED_SHORT as &XSDDataType}
			"http://www.w3.org/2001/XMLSchema#dateTime" => {&XSD_UNSIGNED_SHORT as &XSDDataType}
			"http://www.w3.org/2001/XMLSchema#time" => {&XSD_UNSIGNED_SHORT as &XSDDataType}
			"http://www.w3.org/2001/XMLSchema#date" => {&XSD_UNSIGNED_SHORT as &XSDDataType}
			"http://www.w3.org/2001/XMLSchema#gYearMonth" => {&XSD_UNSIGNED_SHORT as &XSDDataType}
			"http://www.w3.org/2001/XMLSchema#gYear" => {&XSD_UNSIGNED_SHORT as &XSDDataType}
			"http://www.w3.org/2001/XMLSchema#gMonth" => {&XSD_UNSIGNED_SHORT as &XSDDataType}
			"http://www.w3.org/2001/XMLSchema#gDay" => {&XSD_UNSIGNED_SHORT as &XSDDataType}
			"http://www.w3.org/2001/XMLSchema#gMonthDay" => {&XSD_UNSIGNED_SHORT as &XSDDataType}
			//TODO this could create a mem leak, as this will be static and duplocates, use lazy static mut hashmap and check if iri is contained
			_ => {
				if let Some(&xsd) =  ADDITIONAL_TYPES.lock().unwrap().get(&iri.as_string()){
					return xsd
				}
				let xsd: &'static XSDDataType = XSDDataType::create(&iri, |s| true).borrow();
				ADDITIONAL_TYPES.lock().unwrap().insert(String::from(iri.as_string()), &xsd);
				let xsd = ADDITIONAL_TYPES.lock().unwrap().get(&iri.as_string()).expect("");
				return xsd
			}
		}
	}

	//TODO fn create_static_ref() -> &'static XSDDataType

	fn create_static(iri: &IRI, valid: fn(String)->bool) -> XSDDataType {
		let ret:XSDDataType = XSDDataType{
			dtype: iri.as_string(),
			validation: valid
		};
		ret
	}

    ///
    /// Creates a DataType from a String.
    ///
    /// Will be None if `uri` is not a valid URI
    ///
    /// # Parameters
    ///
    /// * `uri` - String literal which represents the DataType
    /// * `validation` - The method to validate that a string object fits your datatype
    ///
    /// # Example
    ///
    /// ```
    ///   use rdf4rust::rdf::xsd::XSDDataType;
    ///
    /// // Our validation simply says a valid value of MyDatatype is not empty
	/// if let Some(dtype) = XSDDataType::create_from_string(String::from("http://my-example-datatypes.com/datatypes#MyDatatype"),
	/// 	|s: String| !s.is_empty()
	/// ){
    ///     //use dtype here
    /// }else{
    ///     //uri wasn't valid
    /// }
    ///
    /// ```
    ///
    pub fn create_from_string(uri: String, validation: fn(String) -> bool) -> Option<XSDDataType>{
        if let Some(xsd_uri) = IRI::create_iri(&uri).ok(){
            Some(XSDDataType::create(&xsd_uri, validation))
        }else {
            None
        }
    }

    ///
    /// Creates a DataType from a `IRI` object
    ///
    /// # Parameters
    ///
    /// * `uri` - The URI representing the data type
    /// * `validation` - The method to validate that a string is valid under the data type
    ///
    /// # Example
    ///
    /// ```
    /// use rdf4rust::util::iri::IRI;
    /// use rdf4rust::rdf::xsd::XSDDataType;
    ///
    /// if let Some(uri) = IRI::create_iri(&String::from("http://my-example-datatypes.com/datatypes#MyDatatype")){
    /// 	// Our validation simply says a valid value of MyDatatype is not empty
    ///     let dtype = XSDDataType::create(uri, |s: String| !s.is_empty() );
    ///     //use dtype here
    /// }else{
    ///     //uri wasn't valid
    /// }
    /// ```
    ///
    pub fn create(uri : &IRI, validation: fn(String) -> bool) -> XSDDataType{
        XSDDataType{
            dtype: uri.as_string(),
			validation
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
        self.dtype == xsd_string().dtype
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
    pub fn get_value(&self) -> &String {
        &self.dtype
    }

}