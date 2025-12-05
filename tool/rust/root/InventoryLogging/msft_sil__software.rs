// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.InventoryLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MsftSil_Software struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MsftSil_Software {
    #[serde(flatten)]
    pub base: MsftSil_Data,

/// 
    #[serde(rename = "ID")]
    pub id: Option<String>,

/// 
    #[serde(rename = "InstallDate")]
    pub install_date: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Publisher")]
    pub publisher: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl MsftSil_Software {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MsftSil_Data::new(),
            id: None,
            install_date: None,
            name: None,
            publisher: None,
            version: None,
        }
    }


    /// Sets the value of ID
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of ID
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of InstallDate
    pub fn set_install_date(&mut self, value: String) {
        self.install_date = Some(value);
    }

    /// Gets the value of InstallDate
    pub fn get_install_date(&self) -> Option<&String> {
        self.install_date.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Publisher
    pub fn set_publisher(&mut self, value: String) {
        self.publisher = Some(value);
    }

    /// Gets the value of Publisher
    pub fn get_publisher(&self) -> Option<&String> {
        self.publisher.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }
}

