// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbBandwidthLimit struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbBandwidthLimit {

/// 
    #[serde(rename = "BytesPerSecond")]
    pub bytes_per_second: Option<u64>,

/// 
    #[serde(rename = "Category")]
    pub category: Option<SmbBandwidthLimit_Category>,
}

impl MSFT_SmbBandwidthLimit {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            bytes_per_second: None,
            category: None,
        }
    }


    /// Sets the value of BytesPerSecond
    pub fn set_bytes_per_second(&mut self, value: u64) {
        self.bytes_per_second = Some(value);
    }

    /// Gets the value of BytesPerSecond
    pub fn get_bytes_per_second(&self) -> Option<&u64> {
        self.bytes_per_second.as_ref()
    }

    /// Sets the value of Category
    pub fn set_category(&mut self, value: SmbBandwidthLimit_Category) {
        self.category = Some(value);
    }

    /// Gets the value of Category
    pub fn get_category(&self) -> Option<&SmbBandwidthLimit_Category> {
        self.category.as_ref()
    }

/// 

    /// * `bytes_per_second` -  (u64)
    /// * `category` -  (u32)

    /// * `output` -  (MSFT_SmbBandwidthLimit[])
    /// * `return_value` -  (u32)
    pub fn set(&self, category: u32, bytes_per_second: u64, output: &mut Vec<MSFT_SmbBandwidthLimit>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Category".to_string(), value: category.into() });
        args.push(MethodParameter { name: "BytesPerSecond".to_string(), value: bytes_per_second.into() });

        let result = self.invoke_method("Set", &args)?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }

}

