// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ManagedSystemElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ManagedSystemElement {

/// 
    #[serde(rename = "Caption")]
    pub caption: Option<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "InstallDate")]
    pub install_date: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

impl CIM_ManagedSystemElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            caption: None,
            description: None,
            install_date: None,
            name: None,
            status: None,
        }
    }


    /// Sets the value of Caption
    pub fn set_caption(&mut self, value: String) {
        self.caption = Some(value);
    }

    /// Gets the value of Caption
    pub fn get_caption(&self) -> Option<&String> {
        self.caption.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
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

    /// Sets the value of Status
    pub fn set_status(&mut self, value: String) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&String> {
        self.status.as_ref()
    }
}

