#![allow(non_snake_case)]

use std;


#[derive(Debug)]
pub enum Configuration {
    Staging,
    Production
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum SyncVersion {
    Version0, // doesn't exist
    Version1, // testing format
    Version2, // production
}
impl std::default::Default for SyncVersion {
    fn default() -> SyncVersion {
        SyncVersion::Version1
    }
}


// request bodies

#[derive(Serialize, Debug)]
pub struct ErrorLogBody<'a> {
    pub operatingSystem: &'a str,
    pub uniqueClientId: &'a str,
    pub clientVersion: &'a str,
    pub description: &'a str,
    pub context: &'a str,
    pub log: &'a [&'a str],
}

#[derive(Serialize, Debug)]
pub struct RegisterClientBody<'a> {
    pub operatingSystem: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub language: &'a str,
    pub uniqueClientId: &'a str
}

#[derive(Serialize, Debug)]
pub struct AccountKeyBody<'a> {
    pub master: &'a str,
    pub main: &'a str,
    pub hmac: &'a str,
    pub tweak: &'a str
}

#[derive(Serialize, Debug)]
pub struct CreateFolderBody<'a> {
    pub folderName: &'a str,
    pub folderPath: &'a str,
    pub encrypted: bool
}

// responses

//private final String token
#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub token: String,
}

//private final String uniqueClientId
#[derive(Serialize, Deserialize, Debug)]
pub struct UniqueClientID {
    pub id: String,
}

#[derive(Debug)]
pub struct Block<'a> {
    pub chunk_data: Vec<u8>,
    pub name: &'a str
}

//private final int id
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFolderResponse {
    pub id: u32,
}

//private final String status;
//private final String host;
//private final int port;
//private final String userName;
//private final Long time;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountStatus {
    pub status: Option<String>,
    pub host: String,
    pub port: u16,
    pub userName: String,
    pub time: Option<u64>
}



//private final long assignedStorage;
//private final long usedStorage;
//private final int lowFreeStorageThreshold;
//private final long expirationDate;
//private final Set<NotificationTO> notifications;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountDetails {
    pub assignedStorage: u64,
    pub usedStorage: u64,
    pub lowFreeStorageThreshold: i64,
    pub expirationDate: u64,
    pub notifications: Option<Vec<Notification>>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub title: String,
    pub message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WrappedKeysetBody {
    pub master: String,
    pub main: String,
    pub hmac: String,
    pub tweak: String
}

pub struct SyncSessionResponse<'a> {
    pub name: &'a str,
    pub folder_id: u32,
    pub chunk_data: Vec<u8>
}


/*
Current sync folder model:

"id" : 1,
"folderName" : "Music",
"folderPath" : /Volumes/MacOS/Music,
"addedDate"  : 1435864769463,
"encrypted"  : false
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisteredFolder {
    pub id: u32,
    pub folderName: String,
    pub folderPath: String,
    pub addedDate: u64,
    pub encrypted: bool
}

