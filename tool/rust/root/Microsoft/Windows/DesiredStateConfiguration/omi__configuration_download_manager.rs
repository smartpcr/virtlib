// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// OMI_ConfigurationDownloadManager struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OMI_ConfigurationDownloadManager {
    #[serde(flatten)]
    pub base: OMI_MetaConfigurationResource,
}

impl OMI_ConfigurationDownloadManager {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: OMI_MetaConfigurationResource::new(),
        }
    }

}

