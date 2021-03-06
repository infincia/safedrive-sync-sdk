use CONFIGURATION;

#[derive(Debug, Clone)]
pub enum Configuration {
    Staging,
    Production,
}

#[derive(Debug, Clone)]
pub enum Channel {
    Stable,
    Beta,
    Nightly,
}

impl ::std::default::Default for Channel {
    fn default() -> Channel {
        Channel::Nightly
    }
}

impl ::std::fmt::Display for Channel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            Channel::Stable => write!(f, "stable"),
            Channel::Beta => write!(f, "beta"),
            Channel::Nightly => write!(f, "nightly"),
        }
    }
}

pub static DEBUG_STATISTICS: bool = true;

static SD_WEB_DOMAIN_PRODUCTION: &'static str = "safedrive.io";
static SD_WEB_DOMAIN_STAGING: &'static str = "staging.safedrive.io";

static SD_API_DOMAIN_PRODUCTION: &'static str = "safedrive.io";
static SD_API_DOMAIN_STAGING: &'static str = "staging.safedrive.io";

pub static SD_GROUP_NAME: &'static str = "group.io.safedrive.db";

pub static HMAC_KEY_SIZE: usize = ::sodiumoxide::crypto::auth::KEYBYTES;
pub static HMAC_SIZE: usize = ::sodiumoxide::crypto::auth::TAGBYTES;

pub static SECRETBOX_KEY_SIZE: usize = ::sodiumoxide::crypto::secretbox::KEYBYTES;
pub static SECRETBOX_NONCE_SIZE: usize = ::sodiumoxide::crypto::secretbox::NONCEBYTES;
pub static SECRETBOX_MAC_SIZE: usize = ::sodiumoxide::crypto::secretbox::MACBYTES;

/// API constants
pub static MULTIPART_BOUNDARY: &'static str = "SAFEDRIVEBINARY";

/// key constants

pub static KEY_ECC_LEN: usize = 48;

/// keychain constants

static SD_ACCOUNT_CREDENTIAL_DOMAIN_PRODUCTION: &'static str = "safedrive.io";
static SD_ACCOUNT_CREDENTIAL_DOMAIN_STAGING: &'static str = "staging.safedrive.io";

static SD_SSH_CREDENTIAL_DOMAIN_PRODUCTION: &'static str = "ssh.safedrive.io";
static SD_SSH_CREDENTIAL_DOMAIN_STAGING: &'static str = "staging.ssh.safedrive.io";

static SD_AUTH_TOKEN_DOMAIN_PRODUCTION: &'static str = "session.safedrive.io";
static SD_AUTH_TOKEN_DOMAIN_STAGING: &'static str = "staging.session.safedrive.io";

static SD_RECOVERY_KEY_DOMAIN_PRODUCTION: &'static str = "recovery.safedrive.io";
static SD_RECOVERY_KEY_DOMAIN_STAGING: &'static str = "staging.recovery.safedrive.io";

// use the same UCID on production and staging until we have a reason not to
static SD_UNIQUE_CLIENT_ID_DOMAIN_PRODUCTION: &'static str = "ucid.safedrive.io";
static SD_UNIQUE_CLIENT_ID_DOMAIN_STAGING: &'static str = "staging.ucid.safedrive.io";

static SD_CURRENT_USER_DOMAIN_PRODUCTION: &'static str = "currentuser.safedrive.io";
static SD_CURRENT_USER_DOMAIN_STAGING: &'static str = "staging.currentuser.safedrive.io";

pub fn is_production() -> bool {
    let c = CONFIGURATION.read();
    match *c {
        Configuration::Staging => false,
        Configuration::Production => true,
    }
}

pub fn web_domain() -> &'static str {
    if is_production() {
        SD_WEB_DOMAIN_PRODUCTION
    } else {
        SD_WEB_DOMAIN_STAGING
    }
}

pub fn api_domain() -> &'static str {
    if is_production() {
        SD_API_DOMAIN_PRODUCTION
    } else {
        SD_API_DOMAIN_STAGING
    }
}

pub fn token_domain() -> &'static str {
    if is_production() {
        SD_AUTH_TOKEN_DOMAIN_PRODUCTION
    } else {
        SD_AUTH_TOKEN_DOMAIN_STAGING
    }
}

pub fn ssh_credential_domain() -> &'static str {
    if is_production() {
        SD_SSH_CREDENTIAL_DOMAIN_PRODUCTION
    } else {
        SD_SSH_CREDENTIAL_DOMAIN_STAGING
    }
}

pub fn account_credential_domain() -> &'static str {
    if is_production() {
        SD_ACCOUNT_CREDENTIAL_DOMAIN_PRODUCTION
    } else {
        SD_ACCOUNT_CREDENTIAL_DOMAIN_STAGING
    }
}

pub fn recovery_key_domain() -> &'static str {
    if is_production() {
        SD_RECOVERY_KEY_DOMAIN_PRODUCTION
    } else {
        SD_RECOVERY_KEY_DOMAIN_STAGING
    }
}

pub fn current_user_domain() -> &'static str {
    if is_production() {
        SD_CURRENT_USER_DOMAIN_PRODUCTION
    } else {
        SD_CURRENT_USER_DOMAIN_STAGING
    }
}

pub fn unique_client_id_domain() -> &'static str {
    if is_production() {
        SD_UNIQUE_CLIENT_ID_DOMAIN_PRODUCTION
    } else {
        SD_UNIQUE_CLIENT_ID_DOMAIN_STAGING
    }
}
