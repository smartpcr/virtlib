// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MTEventProvider struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MTEventProvider {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "DisplayPath")]
    pub display_path: Option<String>,

/// 
    #[serde(rename = "ExportedChannelsCount")]
    pub exported_channels_count: Option<u32>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl MSFT_MTEventProvider {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            display_name: None,
            display_path: None,
            exported_channels_count: None,
            name: None,
        }
    }


    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of DisplayPath
    pub fn set_display_path(&mut self, value: String) {
        self.display_path = Some(value);
    }

    /// Gets the value of DisplayPath
    pub fn get_display_path(&self) -> Option<&String> {
        self.display_path.as_ref()
    }

    /// Sets the value of ExportedChannelsCount
    pub fn set_exported_channels_count(&mut self, value: u32) {
        self.exported_channels_count = Some(value);
    }

    /// Gets the value of ExportedChannelsCount
    pub fn get_exported_channels_count(&self) -> Option<&u32> {
        self.exported_channels_count.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

/// 

    /// * `result` -  (MSFT_MTEventChannel[])
    /// * `return_value` -  (u32)
    pub fn get_channels(&self, result: &mut Vec<MSFT_MTEventChannel>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetChannels", &[])?;
        let result = result.get_value("Result")?;
        Ok(result.return_value)

    }


/// 

    /// * `event_providers` -  (MSFT_MTEventProvider[])
    /// * `return_value` -  (u32)
    /// * `windows_event_channels` -  (MSFT_MTEventChannel[])
    pub fn get_providers_and_windows_event_channels(&self, event_providers: &mut Vec<MSFT_MTEventProvider>, windows_event_channels: &mut Vec<MSFT_MTEventChannel>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetProvidersAndWindowsEventChannels", &[])?;
        let event_providers = result.get_value("EventProviders")?;
        let windows_event_channels = result.get_value("WindowsEventChannels")?;
        Ok(result.return_value)

    }

}

