// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MTEventChannel struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MTEventChannel {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "ClassicLog")]
    pub classic_log: Option<bool>,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "DisplayPath")]
    pub display_path: Option<String>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "EventsCount")]
    pub events_count: Option<String>,

/// 
    #[serde(rename = "LogFilePath")]
    pub log_file_path: Option<String>,

/// 
    #[serde(rename = "LogFileSize")]
    pub log_file_size: Option<u64>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl MSFT_MTEventChannel {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            classic_log: None,
            display_name: None,
            display_path: None,
            enabled: None,
            events_count: None,
            log_file_path: None,
            log_file_size: None,
            name: None,
            type: None,
        }
    }


    /// Sets the value of ClassicLog
    pub fn set_classic_log(&mut self, value: bool) {
        self.classic_log = Some(value);
    }

    /// Gets the value of ClassicLog
    pub fn get_classic_log(&self) -> Option<&bool> {
        self.classic_log.as_ref()
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

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of EventsCount
    pub fn set_events_count(&mut self, value: String) {
        self.events_count = Some(value);
    }

    /// Gets the value of EventsCount
    pub fn get_events_count(&self) -> Option<&String> {
        self.events_count.as_ref()
    }

    /// Sets the value of LogFilePath
    pub fn set_log_file_path(&mut self, value: String) {
        self.log_file_path = Some(value);
    }

    /// Gets the value of LogFilePath
    pub fn get_log_file_path(&self) -> Option<&String> {
        self.log_file_path.as_ref()
    }

    /// Sets the value of LogFileSize
    pub fn set_log_file_size(&mut self, value: u64) {
        self.log_file_size = Some(value);
    }

    /// Gets the value of LogFileSize
    pub fn get_log_file_size(&self) -> Option<&u64> {
        self.log_file_size.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn enable(&self) -> Result<(), WmiError> {
        self.invoke_method("Enable", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn disable(&self) -> Result<(), WmiError> {
        self.invoke_method("Disable", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn clear_log_file(&self) -> Result<(), WmiError> {
        self.invoke_method("ClearLogFile", &[])

    }


/// 

    /// * `batch_size` -  (u32)
    /// * `filter_xml` -  (String)
    /// * `reverse_direction` -  (bool)
    /// * `skip` -  (u64)
    /// * `top` -  (u64)

    /// * `result` -  (MSFT_MTEventRecord[])
    /// * `return_value` -  (u32)
    pub fn get_event_records(&self, filter_xml: &String, skip: u64, top: u64, reverse_direction: bool, batch_size: u32, result: &mut Vec<MSFT_MTEventRecord>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FilterXml".to_string(), value: filter_xml.into() });
        args.push(MethodParameter { name: "Skip".to_string(), value: skip.into() });
        args.push(MethodParameter { name: "Top".to_string(), value: top.into() });
        args.push(MethodParameter { name: "ReverseDirection".to_string(), value: reverse_direction.into() });
        args.push(MethodParameter { name: "BatchSize".to_string(), value: batch_size.into() });

        let result = self.invoke_method("GetEventRecords", &args)?;
        let result = result.get_value("Result")?;
        Ok(result.return_value)

    }


/// 

    /// * `result` -  (MSFT_MTEventChannel[])
    /// * `return_value` -  (u32)
    pub fn get_windows_event_channels(&self, result: &mut Vec<MSFT_MTEventChannel>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetWindowsEventChannels", &[])?;
        let result = result.get_value("Result")?;
        Ok(result.return_value)

    }

}

