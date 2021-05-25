use std::fmt;
use std::ops::Add;
use std::fmt::Formatter;
use std::collections::HashMap;

pub struct Query {
    key_value: HashMap<String, String>,
}

impl fmt::Display for Query{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl Query {

    pub fn get_key_value_pairs(&self) -> &HashMap<String, String>{
        &self.key_value
    }

    fn as_string(&self) -> String {
        let mut ret = String::from("");
        for (index, (key, value)) in self.key_value.iter().enumerate(){
            ret = ret.add(key).add("=").add(value);
            if index < self.key_value.len()-1{
                ret = ret.add("&");
            }
        }
        ret
    }

    pub fn create_query(query: &str) -> Result<Query, URIInvalidError>{
        let v: Vec<&str> = query.split("&").collect();
        let mut key_value: HashMap<String, String> = HashMap::new();
        for fragment in v.iter() {
            match fragment.find("=") {
                Some(value) => {
                    let key = String::from(&fragment[..value]);
                    let value = String::from(&fragment[value+1..]);
                    key_value.insert(key, value);
                },
                None => return Err(URIInvalidError::new(String::from("Invalid Query part, should be set of <key=value> pairs seperated by <&>")))
            };
        }
        Result::Ok(Query{
            key_value
        })
    }
}

pub struct User {
    user: String,
    password: Option<String>,
}

impl fmt::Display for User{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl User {

    pub fn get_user(&self) -> &String {
        &self.user
    }

    pub fn get_password(&self) -> &Option<String> {
        &self.password
    }


    pub fn create_user(str: &str) -> User{
        let user_end = match str.find(':'){
            Some(num) => num,
            None => str.len()
        };
        let user = String::from(&str[..user_end]);
        if user_end == str.len(){
            User{
                user,
                password: None
            }
        }
        else {
            User {
                user,
                password: Some(String::from(&str[user_end + 1..]))
            }
        }
    }

    pub fn as_string(&self) -> String {
        let mut ret = String::from(&self.user);
        if let Some(pwd) = &self.password{
            ret = ret.add(":").add(pwd);
        }
        return ret;
    }
}

pub struct Authority {
    user: Option<User>,
    host: String,
    port: Option<u32>
}

impl fmt::Display for Authority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.as_string())
    }
}

impl Authority {

    pub fn get_port(&self) -> &Option<u32>{
        &self.port
    }

    pub fn get_host(&self) -> &String{
        &self.host
    }

    pub fn get_user(&self) -> &Option<User>{
        &self.user
    }

    fn create_authority(string: &str)-> Result<Authority, URIInvalidError>{
        //regex check ([^\s:@]+(:[^\s:@]*)?@)?[^:]+(:[0-9]+)?
        //xyz:yul@ <-
        //uli.bla.d:0000
        let mut index= match string.find('@'){
            Some(num) => num,
            None => 0
        };
        let mut user = None;
        let mut port = None;
        if index>0 {
            user = Some(User::create_user(&String::from(string)[..index]));
            index = index+1;
        }

        let host_end_index = match string[index..].find(':'){
            Some(num) => num+index,
            None => string.len()
        };
        let host = String::from(&string[index..host_end_index]);
        if host_end_index+1 <string.len(){
            let port_no:u32 = match string[host_end_index+1..].parse(){
                Ok(num) => num,
                Err(_) => return Err(URIInvalidError::new(String::from("URI is not valid, expected number as port.")))
            };
            port =  Some(port_no);
        }
        Result::Ok(Authority{
            user,
            host,
            port
        })
    }

    pub fn as_string(&self) -> String {
        let mut ret = String::from("");
        if let Some(user) = &self.user{
            ret = ret.add(user.as_string().as_str()).add("@");

        }
        if self.user.is_some(){
        }
        ret = ret.add(self.host.as_str());
        if self.port.is_some(){
            ret = ret.add(":").add(self.port.expect("").to_string().as_str());
        }
        ret
    }
}

pub struct URI {
    scheme: String,
    authority: Option<Authority>,
    path:  String,
    query:  Option<Query>,
    fragment:  Option<String>
}

impl fmt::Display for URI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.as_string())
    }
}

impl URI {

    pub fn get_authority(&self) -> &Option<Authority> {
        &self.authority
    }

    pub fn get_fragment(&self) -> &Option<String> {
        &self.fragment
    }

    pub fn get_scheme(&self) -> &String {
        &self.scheme
    }

    pub fn get_query(&self) -> &Option<Query> {
        &self.query
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn as_string(&self) -> String{
        let mut ret = String::from(&self.scheme).add(":");
        if let Some(auth) = &self.authority{
            ret = ret.add("//").add(auth.as_string().as_str());
        }
        ret = ret.add(self.path.as_str());
        if let Some(q) = &self.query{
            ret = ret.add("?").add(q.to_string().as_str());
        }
        if let Some(frag) = &self.fragment{
            ret = ret.add("#").add(frag);
        }
        ret
    }

    pub fn create_uri(uri: &str) -> Result<URI, URIInvalidError> {
        if uri.split_whitespace().count() > 1{
            return Err(URIInvalidError::new(String::from("URI is not valid. Whitespaces are not allowed.")))
        }
        //0. regex check schema:(//(user(:pwd)?@)?[^/]+)?[^\?\#]*(\?[^#\?]+)?(\#[^\s\#]+)?
        //1. split to schema:XYZ
        let mut index = match uri.find(':'){
            Some(num) => num,
            None => return Err(URIInvalidError::new(String::from("URI is not valid, expected <:> after schema.")))
        };
        let scheme = String::from(&uri[..index]);
        //2. if YZ starts with // -> authority
        let mut authority = None;
        let mut path = String::from("/");
        //+1 for :
        if uri[index+1..].starts_with("//"){
            // +3 for ://
            let auth_end_index= match uri[index+3..].find(|c: char| (c=='/' || c=='?' || c=='#')){
                Some(val) => val+index+3,
                None => {
                    //uri is finished
                    uri.len()
                }
            };
            authority = match Authority::create_authority(&uri[index+3..auth_end_index]){
                Ok(auth)   => Some(auth),
                Err(e) => return Err(e)
            };
            index = auth_end_index;
        }else{
            //colon
            index +=1;
        }
        //3. path
        if index != uri.len() && uri.as_bytes()[index] != b'?' && uri.as_bytes()[index] != b'#' {
            if authority.is_some() && uri.as_bytes()[index] != b'/'{
                return Result::Err(URIInvalidError::new(String::from("URI is not valid, authority is set, but path does not begin with </>.")))
            }
            let path_end_index = match uri[index + 1..].find(|c: char| (c == '?' || c == '#')) {
                Some(val) => val+index+1,
                None => uri.len()
            };
            path = String::from(&uri[index..path_end_index]);
            index = path_end_index;
        }
        let mut query = None;
        //4. has ? -> query
        if index != uri.len() && uri.as_bytes()[index] == b'?'{
            let query_end_index = match uri[index + 1..].find(|c: char|  c == '#') {
                Some(val) => val+index+1,
                None => uri.len()
            };
            query = match Query::create_query(&uri[index+1..query_end_index]){
                Ok(val) => Some(val),
                Err(e) => return Err(e)
            };
            index = query_end_index;
        }
        let mut fragment = None;
        //5. has # fragment
        if index != uri.len() && uri.as_bytes()[index] == b'#'{
            let tmp = String::from(&uri[index+1..uri.len()]);
            if tmp.chars().all(|c| ( c != '#' && c!= '%' && c!= '^' && c!= '\\' && c!= '{' && c!= '}' &&
                c!= '[' && c!= ']' && c!= '|') ) {
                println!("{}", tmp);
                fragment = Some(tmp);
            }
            else{
                let mut error = String::from("URI is not valid, URI has invalid part <");
                error = error.add(&tmp).add(">");
                return Result::Err(URIInvalidError::new(error))
            }

        }

        Ok(URI {
            scheme,
            authority,
            path,
            query,
            fragment,
        })

    }

    pub fn is_valid_uri(uri: &str) ->bool{
        URI::create_uri(uri).is_ok()
    }
}

pub struct URIInvalidError {
    msg: String
}

impl URIInvalidError{
    pub fn new(msg: String) -> URIInvalidError {
        URIInvalidError{msg}
    }
}

impl fmt::Display for URIInvalidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.msg)
    }
}

