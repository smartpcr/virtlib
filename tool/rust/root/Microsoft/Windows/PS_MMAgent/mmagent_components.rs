// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.PS_MMAgent
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MMAgentComponents struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MMAgentComponents {

/// 
    #[serde(rename = "ApplicationLaunchPrefetching")]
    pub application_launch_prefetching: Option<bool>,

/// 
    #[serde(rename = "ApplicationPreLaunch")]
    pub application_pre_launch: Option<bool>,

/// 
    #[serde(rename = "MaxOperationAPIFiles")]
    pub max_operation_apifiles: Option<u32>,

/// 
    #[serde(rename = "MemoryCompression")]
    pub memory_compression: Option<bool>,

/// 
    #[serde(rename = "OperationAPI")]
    pub operation_api: Option<bool>,

/// 
    #[serde(rename = "PageCombining")]
    pub page_combining: Option<bool>,
}

impl MMAgentComponents {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            application_launch_prefetching: None,
            application_pre_launch: None,
            max_operation_apifiles: None,
            memory_compression: None,
            operation_api: None,
            page_combining: None,
        }
    }


    /// Sets the value of ApplicationLaunchPrefetching
    pub fn set_application_launch_prefetching(&mut self, value: bool) {
        self.application_launch_prefetching = Some(value);
    }

    /// Gets the value of ApplicationLaunchPrefetching
    pub fn get_application_launch_prefetching(&self) -> Option<&bool> {
        self.application_launch_prefetching.as_ref()
    }

    /// Sets the value of ApplicationPreLaunch
    pub fn set_application_pre_launch(&mut self, value: bool) {
        self.application_pre_launch = Some(value);
    }

    /// Gets the value of ApplicationPreLaunch
    pub fn get_application_pre_launch(&self) -> Option<&bool> {
        self.application_pre_launch.as_ref()
    }

    /// Sets the value of MaxOperationAPIFiles
    pub fn set_max_operation_apifiles(&mut self, value: u32) {
        self.max_operation_apifiles = Some(value);
    }

    /// Gets the value of MaxOperationAPIFiles
    pub fn get_max_operation_apifiles(&self) -> Option<&u32> {
        self.max_operation_apifiles.as_ref()
    }

    /// Sets the value of MemoryCompression
    pub fn set_memory_compression(&mut self, value: bool) {
        self.memory_compression = Some(value);
    }

    /// Gets the value of MemoryCompression
    pub fn get_memory_compression(&self) -> Option<&bool> {
        self.memory_compression.as_ref()
    }

    /// Sets the value of OperationAPI
    pub fn set_operation_api(&mut self, value: bool) {
        self.operation_api = Some(value);
    }

    /// Gets the value of OperationAPI
    pub fn get_operation_api(&self) -> Option<&bool> {
        self.operation_api.as_ref()
    }

    /// Sets the value of PageCombining
    pub fn set_page_combining(&mut self, value: bool) {
        self.page_combining = Some(value);
    }

    /// Gets the value of PageCombining
    pub fn get_page_combining(&self) -> Option<&bool> {
        self.page_combining.as_ref()
    }
}

