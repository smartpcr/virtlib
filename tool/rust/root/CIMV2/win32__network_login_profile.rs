// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_NetworkLoginProfile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_NetworkLoginProfile {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "AccountExpires")]
    pub account_expires: Option<String>,

/// 
    #[serde(rename = "AuthorizationFlags")]
    pub authorization_flags: Option<u32>,

/// 
    #[serde(rename = "BadPasswordCount")]
    pub bad_password_count: Option<u32>,

/// 
    #[serde(rename = "CodePage")]
    pub code_page: Option<u32>,

/// 
    #[serde(rename = "Comment")]
    pub comment: Option<String>,

/// 
    #[serde(rename = "CountryCode")]
    pub country_code: Option<u32>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "FullName")]
    pub full_name: Option<String>,

/// 
    #[serde(rename = "HomeDirectory")]
    pub home_directory: Option<String>,

/// 
    #[serde(rename = "HomeDirectoryDrive")]
    pub home_directory_drive: Option<String>,

/// 
    #[serde(rename = "LastLogoff")]
    pub last_logoff: Option<String>,

/// 
    #[serde(rename = "LastLogon")]
    pub last_logon: Option<String>,

/// 
    #[serde(rename = "LogonHours")]
    pub logon_hours: Option<String>,

/// 
    #[serde(rename = "LogonServer")]
    pub logon_server: Option<String>,

/// 
    #[serde(rename = "MaximumStorage")]
    pub maximum_storage: Option<u64>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NumberOfLogons")]
    pub number_of_logons: Option<u32>,

/// 
    #[serde(rename = "Parameters")]
    pub parameters: Option<String>,

/// 
    #[serde(rename = "PasswordAge")]
    pub password_age: Option<String>,

/// 
    #[serde(rename = "PasswordExpires")]
    pub password_expires: Option<String>,

/// 
    #[serde(rename = "PrimaryGroupId")]
    pub primary_group_id: Option<u32>,

/// 
    #[serde(rename = "Privileges")]
    pub privileges: Option<u32>,

/// 
    #[serde(rename = "Profile")]
    pub profile: Option<String>,

/// 
    #[serde(rename = "ScriptPath")]
    pub script_path: Option<String>,

/// 
    #[serde(rename = "UnitsPerWeek")]
    pub units_per_week: Option<u32>,

/// 
    #[serde(rename = "UserComment")]
    pub user_comment: Option<String>,

/// 
    #[serde(rename = "UserId")]
    pub user_id: Option<u32>,

/// 
    #[serde(rename = "UserType")]
    pub user_type: Option<String>,

/// 
    #[serde(rename = "Workstations")]
    pub workstations: Option<String>,
}

impl Win32_NetworkLoginProfile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            account_expires: None,
            authorization_flags: None,
            bad_password_count: None,
            code_page: None,
            comment: None,
            country_code: None,
            flags: None,
            full_name: None,
            home_directory: None,
            home_directory_drive: None,
            last_logoff: None,
            last_logon: None,
            logon_hours: None,
            logon_server: None,
            maximum_storage: None,
            name: None,
            number_of_logons: None,
            parameters: None,
            password_age: None,
            password_expires: None,
            primary_group_id: None,
            privileges: None,
            profile: None,
            script_path: None,
            units_per_week: None,
            user_comment: None,
            user_id: None,
            user_type: None,
            workstations: None,
        }
    }


    /// Sets the value of AccountExpires
    pub fn set_account_expires(&mut self, value: String) {
        self.account_expires = Some(value);
    }

    /// Gets the value of AccountExpires
    pub fn get_account_expires(&self) -> Option<&String> {
        self.account_expires.as_ref()
    }

    /// Sets the value of AuthorizationFlags
    pub fn set_authorization_flags(&mut self, value: u32) {
        self.authorization_flags = Some(value);
    }

    /// Gets the value of AuthorizationFlags
    pub fn get_authorization_flags(&self) -> Option<&u32> {
        self.authorization_flags.as_ref()
    }

    /// Sets the value of BadPasswordCount
    pub fn set_bad_password_count(&mut self, value: u32) {
        self.bad_password_count = Some(value);
    }

    /// Gets the value of BadPasswordCount
    pub fn get_bad_password_count(&self) -> Option<&u32> {
        self.bad_password_count.as_ref()
    }

    /// Sets the value of CodePage
    pub fn set_code_page(&mut self, value: u32) {
        self.code_page = Some(value);
    }

    /// Gets the value of CodePage
    pub fn get_code_page(&self) -> Option<&u32> {
        self.code_page.as_ref()
    }

    /// Sets the value of Comment
    pub fn set_comment(&mut self, value: String) {
        self.comment = Some(value);
    }

    /// Gets the value of Comment
    pub fn get_comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }

    /// Sets the value of CountryCode
    pub fn set_country_code(&mut self, value: u32) {
        self.country_code = Some(value);
    }

    /// Gets the value of CountryCode
    pub fn get_country_code(&self) -> Option<&u32> {
        self.country_code.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of FullName
    pub fn set_full_name(&mut self, value: String) {
        self.full_name = Some(value);
    }

    /// Gets the value of FullName
    pub fn get_full_name(&self) -> Option<&String> {
        self.full_name.as_ref()
    }

    /// Sets the value of HomeDirectory
    pub fn set_home_directory(&mut self, value: String) {
        self.home_directory = Some(value);
    }

    /// Gets the value of HomeDirectory
    pub fn get_home_directory(&self) -> Option<&String> {
        self.home_directory.as_ref()
    }

    /// Sets the value of HomeDirectoryDrive
    pub fn set_home_directory_drive(&mut self, value: String) {
        self.home_directory_drive = Some(value);
    }

    /// Gets the value of HomeDirectoryDrive
    pub fn get_home_directory_drive(&self) -> Option<&String> {
        self.home_directory_drive.as_ref()
    }

    /// Sets the value of LastLogoff
    pub fn set_last_logoff(&mut self, value: String) {
        self.last_logoff = Some(value);
    }

    /// Gets the value of LastLogoff
    pub fn get_last_logoff(&self) -> Option<&String> {
        self.last_logoff.as_ref()
    }

    /// Sets the value of LastLogon
    pub fn set_last_logon(&mut self, value: String) {
        self.last_logon = Some(value);
    }

    /// Gets the value of LastLogon
    pub fn get_last_logon(&self) -> Option<&String> {
        self.last_logon.as_ref()
    }

    /// Sets the value of LogonHours
    pub fn set_logon_hours(&mut self, value: String) {
        self.logon_hours = Some(value);
    }

    /// Gets the value of LogonHours
    pub fn get_logon_hours(&self) -> Option<&String> {
        self.logon_hours.as_ref()
    }

    /// Sets the value of LogonServer
    pub fn set_logon_server(&mut self, value: String) {
        self.logon_server = Some(value);
    }

    /// Gets the value of LogonServer
    pub fn get_logon_server(&self) -> Option<&String> {
        self.logon_server.as_ref()
    }

    /// Sets the value of MaximumStorage
    pub fn set_maximum_storage(&mut self, value: u64) {
        self.maximum_storage = Some(value);
    }

    /// Gets the value of MaximumStorage
    pub fn get_maximum_storage(&self) -> Option<&u64> {
        self.maximum_storage.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NumberOfLogons
    pub fn set_number_of_logons(&mut self, value: u32) {
        self.number_of_logons = Some(value);
    }

    /// Gets the value of NumberOfLogons
    pub fn get_number_of_logons(&self) -> Option<&u32> {
        self.number_of_logons.as_ref()
    }

    /// Sets the value of Parameters
    pub fn set_parameters(&mut self, value: String) {
        self.parameters = Some(value);
    }

    /// Gets the value of Parameters
    pub fn get_parameters(&self) -> Option<&String> {
        self.parameters.as_ref()
    }

    /// Sets the value of PasswordAge
    pub fn set_password_age(&mut self, value: String) {
        self.password_age = Some(value);
    }

    /// Gets the value of PasswordAge
    pub fn get_password_age(&self) -> Option<&String> {
        self.password_age.as_ref()
    }

    /// Sets the value of PasswordExpires
    pub fn set_password_expires(&mut self, value: String) {
        self.password_expires = Some(value);
    }

    /// Gets the value of PasswordExpires
    pub fn get_password_expires(&self) -> Option<&String> {
        self.password_expires.as_ref()
    }

    /// Sets the value of PrimaryGroupId
    pub fn set_primary_group_id(&mut self, value: u32) {
        self.primary_group_id = Some(value);
    }

    /// Gets the value of PrimaryGroupId
    pub fn get_primary_group_id(&self) -> Option<&u32> {
        self.primary_group_id.as_ref()
    }

    /// Sets the value of Privileges
    pub fn set_privileges(&mut self, value: u32) {
        self.privileges = Some(value);
    }

    /// Gets the value of Privileges
    pub fn get_privileges(&self) -> Option<&u32> {
        self.privileges.as_ref()
    }

    /// Sets the value of Profile
    pub fn set_profile(&mut self, value: String) {
        self.profile = Some(value);
    }

    /// Gets the value of Profile
    pub fn get_profile(&self) -> Option<&String> {
        self.profile.as_ref()
    }

    /// Sets the value of ScriptPath
    pub fn set_script_path(&mut self, value: String) {
        self.script_path = Some(value);
    }

    /// Gets the value of ScriptPath
    pub fn get_script_path(&self) -> Option<&String> {
        self.script_path.as_ref()
    }

    /// Sets the value of UnitsPerWeek
    pub fn set_units_per_week(&mut self, value: u32) {
        self.units_per_week = Some(value);
    }

    /// Gets the value of UnitsPerWeek
    pub fn get_units_per_week(&self) -> Option<&u32> {
        self.units_per_week.as_ref()
    }

    /// Sets the value of UserComment
    pub fn set_user_comment(&mut self, value: String) {
        self.user_comment = Some(value);
    }

    /// Gets the value of UserComment
    pub fn get_user_comment(&self) -> Option<&String> {
        self.user_comment.as_ref()
    }

    /// Sets the value of UserId
    pub fn set_user_id(&mut self, value: u32) {
        self.user_id = Some(value);
    }

    /// Gets the value of UserId
    pub fn get_user_id(&self) -> Option<&u32> {
        self.user_id.as_ref()
    }

    /// Sets the value of UserType
    pub fn set_user_type(&mut self, value: String) {
        self.user_type = Some(value);
    }

    /// Gets the value of UserType
    pub fn get_user_type(&self) -> Option<&String> {
        self.user_type.as_ref()
    }

    /// Sets the value of Workstations
    pub fn set_workstations(&mut self, value: String) {
        self.workstations = Some(value);
    }

    /// Gets the value of Workstations
    pub fn get_workstations(&self) -> Option<&String> {
        self.workstations.as_ref()
    }
}

