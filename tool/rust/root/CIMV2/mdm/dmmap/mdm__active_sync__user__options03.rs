// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_ActiveSync_User_Options03 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_ActiveSync_User_Options03 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "Logging")]
    pub logging: Option<String>,

/// 
    #[serde(rename = "MailAgeFilter")]
    pub mail_age_filter: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Schedule")]
    pub schedule: Option<String>,

/// 
    #[serde(rename = "UseSSL")]
    pub use_ssl: Option<String>,
}

impl MDM_ActiveSync_User_Options03 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            logging: None,
            mail_age_filter: None,
            parent_id: None,
            schedule: None,
            use_ssl: None,
        }
    }


    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of Logging
    pub fn set_logging(&mut self, value: String) {
        self.logging = Some(value);
    }

    /// Gets the value of Logging
    pub fn get_logging(&self) -> Option<&String> {
        self.logging.as_ref()
    }

    /// Sets the value of MailAgeFilter
    pub fn set_mail_age_filter(&mut self, value: String) {
        self.mail_age_filter = Some(value);
    }

    /// Gets the value of MailAgeFilter
    pub fn get_mail_age_filter(&self) -> Option<&String> {
        self.mail_age_filter.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of Schedule
    pub fn set_schedule(&mut self, value: String) {
        self.schedule = Some(value);
    }

    /// Gets the value of Schedule
    pub fn get_schedule(&self) -> Option<&String> {
        self.schedule.as_ref()
    }

    /// Sets the value of UseSSL
    pub fn set_use_ssl(&mut self, value: String) {
        self.use_ssl = Some(value);
    }

    /// Gets the value of UseSSL
    pub fn get_use_ssl(&self) -> Option<&String> {
        self.use_ssl.as_ref()
    }
}

