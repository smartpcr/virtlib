// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ClusterUpdate
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CAU_Audi_GenerateDeviceInfoFilesFU_Result struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CAU_Audi_GenerateDeviceInfoFilesFU_Result {

/// 
    #[serde(rename = "GenerateFilesHResult")]
    pub generate_files_hresult: Option<u32>,
}

impl MSFT_CAU_Audi_GenerateDeviceInfoFilesFU_Result {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            generate_files_hresult: None,
        }
    }


    /// Sets the value of GenerateFilesHResult
    pub fn set_generate_files_hresult(&mut self, value: u32) {
        self.generate_files_hresult = Some(value);
    }

    /// Gets the value of GenerateFilesHResult
    pub fn get_generate_files_hresult(&self) -> Option<&u32> {
        self.generate_files_hresult.as_ref()
    }
}

