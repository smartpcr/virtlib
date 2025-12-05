// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DSCConfigurationOutput struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DSCConfigurationOutput {

/// 
    #[serde(rename = "Bookmark")]
    pub bookmark: Vec<u8>,

/// 
    #[serde(rename = "JobId")]
    pub job_id: Option<String>,
}

impl MSFT_DSCConfigurationOutput {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            bookmark: Vec::new(),
            job_id: None,
        }
    }


    /// Sets the value of Bookmark
    pub fn set_bookmark(&mut self, value: Vec<u8>) {
        self.bookmark = value;
    }

    /// Gets the value of Bookmark
    pub fn get_bookmark(&self) -> &Vec<u8> {
        &self.bookmark
    }

    /// Sets the value of JobId
    pub fn set_job_id(&mut self, value: String) {
        self.job_id = Some(value);
    }

    /// Gets the value of JobId
    pub fn get_job_id(&self) -> Option<&String> {
        self.job_id.as_ref()
    }
}

