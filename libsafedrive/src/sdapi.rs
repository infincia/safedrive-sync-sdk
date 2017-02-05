#![allow(non_snake_case)]

use std::io::Read;
use std::collections::HashMap;

// external crate imports

use ::rustc_serialize::hex::{ToHex};

// internal imports

use ::error::SDAPIError;
use ::models::*;
use ::block::*;
use ::session::*;
use ::keys::*;
use ::constants::*;
use ::CLIENT_VERSION;



header! { (UserAgent, "User-Agent") => [String] }
header! { (SDAuthToken, "SD-Auth-Token") => [String] }
header! { (ContentType, "Content-Type") => [String] }
header! { (ContentLength, "Content-Length") => [usize] }

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum APIEndpoint<'a> {
    ErrorLog { operatingSystem: &'a str, clientVersion: &'a str, uniqueClientId: &'a str, description: &'a str, context: &'a str, log: &'a [&'a str] },
    RegisterClient { email: &'a str, password: &'a str, operatingSystem: &'a str, language: &'a str, uniqueClientId: &'a str },
    AccountStatus,
    AccountDetails,
    AccountKey { master: &'a str, main: &'a str, hmac: &'a str, tweak: &'a str },
    ReadFolders,
    CreateFolder { folderPath: &'a str, folderName: &'a str, encrypted: bool },
    DeleteFolder { folder_id: u64 },
    RegisterSyncSession { folder_id: u64, name: &'a str, encrypted: bool },
    FinishSyncSession { folder_id: u64, name: &'a str, encrypted: bool, size: usize, session_data: &'a [u8] },
    ReadSyncSession { name: &'a str, encrypted: bool },
    ReadSyncSessions { encrypted: bool },
    CheckBlock { name: &'a str },
    WriteBlock { session: &'a str, name: &'a str },
    WriteBlocks { session: &'a str },
    ReadBlock { name: &'a str },
}

impl<'a> APIEndpoint<'a> {

    pub fn url(&self) -> ::reqwest::Url {
        let mut base = String::new();
        base += &self.protocol();
        base += &self.domain();
        let url_base = ::reqwest::Url::parse(&base).unwrap();
        let mut url = url_base.join(&self.path()).unwrap();
        match *self {
            APIEndpoint::DeleteFolder { folder_id, .. } => {
                url.query_pairs_mut()
                    .clear()
                    .append_pair("folderIds", &format!("{}", folder_id));
            },
            _ => {}
        }

        url
    }

    pub fn domain(&self) -> String {
        api_domain().to_string()
    }

    pub fn protocol(&self) -> String {
        "https://".to_string()
    }

    pub fn method(&self) -> ::reqwest::Method {
        match *self {
            APIEndpoint::ErrorLog { .. } => {
                ::reqwest::Method::Post
            },
            APIEndpoint::RegisterClient { .. } => {
                ::reqwest::Method::Post
            },
            APIEndpoint::AccountStatus => {
                ::reqwest::Method::Get
            },
            APIEndpoint::AccountDetails => {
                ::reqwest::Method::Get
            },
            APIEndpoint::AccountKey { .. } => {
                ::reqwest::Method::Post
            },
            APIEndpoint::ReadFolders => {
                ::reqwest::Method::Get
            },
            APIEndpoint::CreateFolder { .. } => {
                ::reqwest::Method::Post
            },
            APIEndpoint::DeleteFolder { .. } => {
                ::reqwest::Method::Delete
            },
            APIEndpoint::RegisterSyncSession { .. } => {
                ::reqwest::Method::Post
            },
            APIEndpoint::FinishSyncSession { .. } => {
                ::reqwest::Method::Post
            },
            APIEndpoint::ReadSyncSession { .. } => {
                ::reqwest::Method::Get
            },
            APIEndpoint::ReadSyncSessions { .. } => {
                ::reqwest::Method::Get
            },
            APIEndpoint::CheckBlock { .. } => {
                ::reqwest::Method::Head
            },
            APIEndpoint::WriteBlock { .. } => {
                ::reqwest::Method::Post
            },
            APIEndpoint::WriteBlocks { .. } => {
                ::reqwest::Method::Post
            },
            APIEndpoint::ReadBlock { .. } => {
                ::reqwest::Method::Get
            },
        }
    }

    pub fn path(&self) -> String {
        let path = match *self {
            APIEndpoint::ErrorLog { .. } => {
                format!("/api/1/error/log")
            },
            APIEndpoint::RegisterClient { .. } => {
                format!("/api/1/client/register")
            },
            APIEndpoint::AccountStatus => {
                format!("/api/1/account/status")
            },
            APIEndpoint::AccountDetails => {
                format!("/api/1/account/details")
            },
            APIEndpoint::AccountKey { .. } => {
                format!("/api/1/account/key")
            },
            APIEndpoint::ReadFolders => {
                format!("/api/1/folder")
            },
            APIEndpoint::CreateFolder { .. } => {
                format!("/api/1/folder")
            },
            APIEndpoint::DeleteFolder { .. } => {
                format!("/api/1/folder")
            },
            APIEndpoint::RegisterSyncSession { folder_id, name, .. } => {
                format!("/api/1/sync/session/register/{}/{}", folder_id, name)
            },
            APIEndpoint::FinishSyncSession { name, size, .. } => {
                format!("/api/1/sync/session/{}/{}", name, size)
            },
            APIEndpoint::ReadSyncSession { name, .. } => {
                format!("/api/1/sync/session/{}", name)
            },
            APIEndpoint::ReadSyncSessions { .. } => {
                format!("/api/1/sync/session")
            },
            APIEndpoint::CheckBlock { name, .. } => {
                format!("/api/1/sync/block/{}", name)
            },
            APIEndpoint::WriteBlock { session, name, .. } => {
                format!("/api/1/sync/block/{}/{}", name, session)
            },
            APIEndpoint::WriteBlocks { session } => {
                format!("/api/1/sync/blocks/{}", session)
            },
            APIEndpoint::ReadBlock { name, .. } => {
                format!("/api/1/sync/block/{}", name)
            },

        };

        path
    }
}

// SD API
#[allow(dead_code)]
pub fn report_error<'a>(clientVersion: &'a str, uniqueClientId: &'a str, operatingSystem: &'a str, description: &'a str, context: &'a str, log: &'a [&'a str]) -> Result<(), SDAPIError> {

    let endpoint = APIEndpoint::ErrorLog { operatingSystem: operatingSystem, uniqueClientId: uniqueClientId, clientVersion: clientVersion, description: description, context: context, log: log };

    let user_agent = &**CLIENT_VERSION.read();

    let client = ::reqwest::Client::new().unwrap();
    let request = client.request(endpoint.method(), endpoint.url())
        .header(UserAgent(user_agent.to_string()))
        .json(&endpoint);

    let mut result = try!(request.send());

    let mut response = String::new();

    try!(result.read_to_string(&mut response));

    debug!("response: {}", response);

    match result.status() {
        &::reqwest::StatusCode::Ok => {},
        _ => return Err(SDAPIError::Internal(format!("unexpected response(HTTP{}): {}", result.status(), &response)))
    }

    Ok(())
}

pub fn register_client<'a>(operatingSystem: &str, languageCode: &str, uniqueClientId: &'a str, email: &'a str, password: &'a str) -> Result<Token, SDAPIError> {

    let endpoint = APIEndpoint::RegisterClient{ operatingSystem: operatingSystem, email: email, password: password, language: languageCode, uniqueClientId: uniqueClientId };

    let user_agent = &**CLIENT_VERSION.read();

    let client = ::reqwest::Client::new().unwrap();
    let request = client.request(endpoint.method(), endpoint.url())
        .header(UserAgent(user_agent.to_string()))
        .json(&endpoint);

    let mut result = try!(request.send());

    let mut response = String::new();

    try!(result.read_to_string(&mut response));

    debug!("response: {}", response);

    match result.status() {
        &::reqwest::StatusCode::Ok => {},
        &::reqwest::StatusCode::Unauthorized => return Err(SDAPIError::Authentication),

        _ => return Err(SDAPIError::Internal(format!("unexpected response(HTTP{}): {}", result.status(), &response)))
    }


    let token: Token = try!(::serde_json::from_str(&response));

    Ok(token)
}

pub fn account_status(token: &Token) -> Result<AccountStatus, SDAPIError> {
    let endpoint = APIEndpoint::AccountStatus;

    let user_agent = &**CLIENT_VERSION.read();

    let client = ::reqwest::Client::new().unwrap();
    let request = client.request(endpoint.method(), endpoint.url())
        .header(UserAgent(user_agent.to_string()))
        .header(SDAuthToken(token.token.to_owned()));

    let mut result = try!(request.send());

    let mut response = String::new();

    try!(result.read_to_string(&mut response));

    debug!("response: {}", response);

    match result.status() {
        &::reqwest::StatusCode::Ok => {},
        &::reqwest::StatusCode::Unauthorized => return Err(SDAPIError::Authentication),

        _ => return Err(SDAPIError::Internal(format!("unexpected response(HTTP{}): {}", result.status(), &response)))
    }



    let account_status: AccountStatus = try!(::serde_json::from_str(&response));

    Ok(account_status)
}

#[allow(dead_code)]
pub fn account_details(token: &Token) -> Result<AccountDetails, SDAPIError> {
    let endpoint = APIEndpoint::AccountDetails;

    let user_agent = &**CLIENT_VERSION.read();

    let client = ::reqwest::Client::new().unwrap();
    let request = client.request(endpoint.method(), endpoint.url())
        .header(UserAgent(user_agent.to_string()))
        .header(SDAuthToken(token.token.to_owned()));

    let mut result = try!(request.send());

    let mut response = String::new();

    try!(result.read_to_string(&mut response));

    debug!("response: {}", response);

    match result.status() {
        &::reqwest::StatusCode::Ok => {},
        &::reqwest::StatusCode::Unauthorized => return Err(SDAPIError::Authentication),

        _ => return Err(SDAPIError::Internal(format!("unexpected response(HTTP{}): {}", result.status(), &response)))
    }

    let account_details: AccountDetails = try!(::serde_json::from_str(&response));

    Ok(account_details)
}

pub fn account_key(token: &Token, new_wrapped_keyset: &WrappedKeyset) -> Result<WrappedKeyset, SDAPIError> {

    let endpoint = APIEndpoint::AccountKey { master: &new_wrapped_keyset.master.to_hex(), main: &new_wrapped_keyset.main.to_hex(), hmac: &new_wrapped_keyset.hmac.to_hex(), tweak: &new_wrapped_keyset.tweak.to_hex() };

    let user_agent = &**CLIENT_VERSION.read();

    let client = ::reqwest::Client::new().unwrap();
    let request = client.request(endpoint.method(), endpoint.url())
        .json(&endpoint)
        .header(UserAgent(user_agent.to_string()))
        .header(SDAuthToken(token.token.to_owned()));

    let mut result = try!(request.send());

    let mut response = String::new();

    try!(result.read_to_string(&mut response));

    debug!("response: {}", response);

    match result.status() {
        &::reqwest::StatusCode::Ok => {},
        &::reqwest::StatusCode::Unauthorized => return Err(SDAPIError::Authentication),

        _ => return Err(SDAPIError::Internal(format!("unexpected response(HTTP{}): {}", result.status(), &response)))
    }

    let wrapped_keyset_b: WrappedKeysetBody = try!(::serde_json::from_str(&response));

    Ok(WrappedKeyset::from(wrapped_keyset_b))
}

pub fn read_folders(token: &Token) -> Result<Vec<RegisteredFolder>, SDAPIError> {

    let endpoint = APIEndpoint::ReadFolders;

    let user_agent = &**CLIENT_VERSION.read();

    let client = ::reqwest::Client::new().unwrap();
    let request = client.request(endpoint.method(), endpoint.url())
        .header(UserAgent(user_agent.to_string()))
        .header(SDAuthToken(token.token.to_owned()));

    let mut result = try!(request.send());

    let mut response = String::new();

    try!(result.read_to_string(&mut response));

    debug!("response: {}", response);

    match result.status() {
        &::reqwest::StatusCode::Ok => {},
        &::reqwest::StatusCode::Unauthorized => return Err(SDAPIError::Authentication),

        _ => return Err(SDAPIError::Internal(format!("unexpected response(HTTP{}): {}", result.status(), &response)))
    }

    let folders: Vec<RegisteredFolder> = try!(::serde_json::from_str(&response));

    Ok(folders)
}

pub fn create_folder<'a>(token: &Token, path: &'a str, name: &'a str, encrypted: bool) -> Result<u64, SDAPIError> {

    let endpoint = APIEndpoint::CreateFolder { folderPath: path, folderName: name, encrypted: encrypted };

    let user_agent = &**CLIENT_VERSION.read();

    let client = ::reqwest::Client::new().unwrap();
    let request = client.request(endpoint.method(), endpoint.url())
        .json(&endpoint)
        .header(UserAgent(user_agent.to_string()))
        .header(SDAuthToken(token.token.to_owned()));

    let mut result = try!(request.send());

    let mut response = String::new();

    try!(result.read_to_string(&mut response));

    debug!("response: {}", response);

    match result.status() {
        &::reqwest::StatusCode::Ok => {},
        &::reqwest::StatusCode::Unauthorized => return Err(SDAPIError::Authentication),

        _ => return Err(SDAPIError::Internal(format!("unexpected response(HTTP{}): {}", result.status(), &response)))
    }

    let folder_response: CreateFolderResponse = try!(::serde_json::from_str(&response));

    Ok(folder_response.id)
}

pub fn delete_folder(token: &Token, folder_id: u64) -> Result<(), SDAPIError> {
    let endpoint = APIEndpoint::DeleteFolder { folder_id: folder_id };

    let user_agent = &**CLIENT_VERSION.read();

    let client = ::reqwest::Client::new().unwrap();
    let request = client.request(endpoint.method(), endpoint.url())
        .header(UserAgent(user_agent.to_string()))
        .header(SDAuthToken(token.token.to_owned()));

    let mut result = try!(request.send());

    let mut response = String::new();

    try!(result.read_to_string(&mut response));

    debug!("response: {}", response);

    match result.status() {
        &::reqwest::StatusCode::Ok => {},
        &::reqwest::StatusCode::Unauthorized => return Err(SDAPIError::Authentication),

        _ => return Err(SDAPIError::Internal(format!("unexpected response(HTTP{}): {}", result.status(), &response)))
    }

    Ok(())
}

// sync session handling

pub fn read_sessions(token: &Token) -> Result<HashMap<String, HashMap<u64, Vec<SyncSession>>>, SDAPIError> {

    let endpoint = APIEndpoint::ReadSyncSessions { encrypted: true };

    let user_agent = &**CLIENT_VERSION.read();

    let client = ::reqwest::Client::new().unwrap();
    let request = client.request(endpoint.method(), endpoint.url())
        .header(UserAgent(user_agent.to_string()))
        .header(SDAuthToken(token.token.to_owned()));

    let mut result = try!(request.send());

    let mut response = String::new();

    try!(result.read_to_string(&mut response));

    debug!("response: {}", response);

    match result.status() {
        &::reqwest::StatusCode::Ok => {},
        &::reqwest::StatusCode::Unauthorized => return Err(SDAPIError::Authentication),

        _ => return Err(SDAPIError::Internal(format!("unexpected response(HTTP{}): {}", result.status(), &response)))
    }

    let sessions: HashMap<String, HashMap<u64, Vec<SyncSession>>> = try!(::serde_json::from_str(&response));

    Ok(sessions)
}

pub fn register_sync_session<'a>(token: &Token, folder_id: u64, name: &'a str, encrypted: bool) -> Result<(), SDAPIError> {

    let endpoint = APIEndpoint::RegisterSyncSession { folder_id: folder_id, name: name, encrypted: encrypted };

    let user_agent = &**CLIENT_VERSION.read();

    let client = ::reqwest::Client::new().unwrap();
    let request = client.request(endpoint.method(), endpoint.url())
        .header(UserAgent(user_agent.to_string()))
        .header(SDAuthToken(token.token.to_owned()));

    let mut result = try!(request.send());

    let mut response = String::new();

    try!(result.read_to_string(&mut response));

    debug!("response: {}", response);

    match result.status() {
        &::reqwest::StatusCode::Ok => {},
        &::reqwest::StatusCode::Created => {},
        &::reqwest::StatusCode::Unauthorized => return Err(SDAPIError::Authentication),

        _ => return Err(SDAPIError::Internal(format!("unexpected response(HTTP{}): {}", result.status(), &response)))
    }

    Ok(())
}

pub fn finish_sync_session<'a>(token: &Token, folder_id: u64, name: &'a str, encrypted: bool, session_data: &[u8], size: usize) -> Result<(), SDAPIError> {

    let endpoint = APIEndpoint::FinishSyncSession { folder_id: folder_id, name: name, encrypted: encrypted, size: size, session_data: session_data };

    let (body, content_length, boundary) = multipart_for_bytes(session_data, name);

    //debug!("body: {}", String::from_utf8_lossy(&body));

    let user_agent = &**CLIENT_VERSION.read();

    let client = ::reqwest::Client::new().unwrap();
    let request = client.request(endpoint.method(), endpoint.url())
        .body(body)
        .header(UserAgent(user_agent.to_string()))
        .header(SDAuthToken(token.token.to_owned()))
        .header(ContentType(format!("multipart/form-data; boundary={}", boundary.to_owned())))
        .header(ContentLength(content_length));

    let mut result = try!(request.send());

    let mut response = String::new();

    try!(result.read_to_string(&mut response));

    debug!("response: {}", response);

    match result.status() {
        &::reqwest::StatusCode::Ok => Ok(()),
        &::reqwest::StatusCode::Created => Ok(()),
        &::reqwest::StatusCode::Unauthorized => return Err(SDAPIError::Authentication),

        _ => return Err(SDAPIError::Internal(format!("unexpected response(HTTP{}): {}", result.status(), &response)))
    }
}

pub fn read_session<'a>(token: &Token, folder_id: u64, name: &'a str, encrypted: bool) -> Result<SyncSessionResponse<'a>, SDAPIError> {
    let endpoint = APIEndpoint::ReadSyncSession { name: name, encrypted: encrypted };

    let user_agent = &**CLIENT_VERSION.read();

    let client = ::reqwest::Client::new().unwrap();

    let request = client.request(endpoint.method(), endpoint.url())
        .header(UserAgent(user_agent.to_string()))
        .header(SDAuthToken(token.token.to_owned()));

    let mut result = try!(request.send());

    match result.status() {
        &::reqwest::StatusCode::Ok => {},
        &::reqwest::StatusCode::NotFound => return Err(SDAPIError::SessionMissing),
        &::reqwest::StatusCode::Unauthorized => return Err(SDAPIError::Authentication),

        _ => return Err(SDAPIError::Internal(format!("unexpected status code: {}", result.status())))
    };
    let mut buffer = Vec::new();

    try!(result.read_to_end(&mut buffer));

    Ok(SyncSessionResponse { name: name, chunk_data: buffer, folder_id: folder_id })
}


// block handling
#[allow(dead_code)]
pub fn check_block<'a>(token: &Token, name: &'a str) -> Result<bool, SDAPIError> {

    let endpoint = APIEndpoint::CheckBlock { name: name };

    let user_agent = &**CLIENT_VERSION.read();

    let client = ::reqwest::Client::new().unwrap();

    let request = client.request(endpoint.method(), endpoint.url())
        .header(UserAgent(user_agent.to_string()))
        .header(SDAuthToken(token.token.to_owned()));

    let mut result = try!(request.send());

    let mut response = String::new();

    try!(result.read_to_string(&mut response));

    debug!("response: {}", response);

    match result.status() {
        &::reqwest::StatusCode::Ok => Ok(true),
        &::reqwest::StatusCode::NotFound => Ok(false),
        &::reqwest::StatusCode::Unauthorized => return Err(SDAPIError::Authentication),

        _ => return Err(SDAPIError::Internal(format!("unexpected response(HTTP{}): {}", result.status(), &response)))
    }
}

pub fn write_block(token: &Token, session: &str, name: &str, block: &WrappedBlock, should_upload: bool) -> Result<(), SDAPIError> {

    let endpoint = APIEndpoint::WriteBlock { name: name, session: session };

    let user_agent = &**CLIENT_VERSION.read();

    let client = ::reqwest::Client::new().unwrap();
    let mut request = client.request(endpoint.method(), endpoint.url())
        .header(UserAgent(user_agent.to_string()))
        .header(SDAuthToken(token.token.to_owned()));
    if should_upload {
        let (body, content_length, boundary) = multipart_for_bytes(block.as_ref(), name);
        //debug!("body: {}", String::from_utf8_lossy(&body));

        request = request.body(body)
        .header(ContentType(format!("multipart/form-data; boundary={}", boundary.to_owned())))
        .header(ContentLength(content_length));
    }
    let mut result = try!(request.send());

    let mut response = String::new();

    try!(result.read_to_string(&mut response));

    debug!("response: {}", response);

    match result.status() {
        &::reqwest::StatusCode::Ok => Ok(()),
        &::reqwest::StatusCode::Created => Ok(()),
        &::reqwest::StatusCode::BadRequest => Err(SDAPIError::RetryUpload),
        &::reqwest::StatusCode::NotFound => Err(SDAPIError::RetryUpload),
        &::reqwest::StatusCode::Unauthorized => return Err(SDAPIError::Authentication),

        _ => return Err(SDAPIError::Internal(format!("unexpected response(HTTP{}): {}", result.status(), &response)))
    }
}

pub fn read_block<'a>(token: &Token, name: &'a str) -> Result<Vec<u8>, SDAPIError> {
    let endpoint = APIEndpoint::ReadBlock { name: name };

    let user_agent = &**CLIENT_VERSION.read();

    let client = ::reqwest::Client::new().unwrap();
    let request = client.request(endpoint.method(), endpoint.url())
        .header(UserAgent(user_agent.to_string()))
        .header(SDAuthToken(token.token.to_owned()));

    let mut result = try!(request.send());

    match result.status() {
        &::reqwest::StatusCode::Ok => {},
        &::reqwest::StatusCode::NotFound => return Err(SDAPIError::BlockMissing),
        &::reqwest::StatusCode::Unauthorized => return Err(SDAPIError::Authentication),

        _ => return Err(SDAPIError::Internal(format!("unexpected status code: {}", result.status())))
    };
    let mut buffer = Vec::new();

    try!(result.read_to_end(&mut buffer));

    Ok(buffer)
}

fn multipart_for_bytes(chunk_data: &[u8], name: &str) -> (Vec<u8>, usize, &'static str) {

    let mut body: Vec<u8> = Vec::new();

    // these are compile time optimizations
    let header_boundary: &'static str = "-----SAFEDRIVEBINARY";
    let rn: &'static [u8; 2] = b"\r\n";
    let body_boundary: &'static [u8; 22] = br"-------SAFEDRIVEBINARY";
    let end_boundary: &'static [u8; 24] =  br"-------SAFEDRIVEBINARY--";
    let content_type: &'static [u8; 38] = br"Content-Type: application/octet-stream";


    let disp = format!("content-disposition: form-data; name=file; filename={}", name);
    let enc: &'static [u8; 33] = br"Content-Transfer-Encoding: binary";

    body.extend(rn);
    body.extend(rn);
    body.extend(body_boundary.as_ref());
    body.extend(rn);

    body.extend(disp.as_bytes());
    body.extend(rn);

    body.extend(content_type.as_ref());
    body.extend(rn);

    body.extend(enc.as_ref());
    body.extend(rn);
    body.extend(rn);

    body.extend(chunk_data);
    body.extend(rn);

    body.extend(end_boundary.as_ref());
    body.extend(rn);
    body.extend(rn);

    let content_length = body.len();

    (body, content_length, header_boundary)
}

