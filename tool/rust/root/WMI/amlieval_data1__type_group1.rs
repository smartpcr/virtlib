// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// AMLIEvalData1_TypeGroup1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AMLIEvalData1_TypeGroup1 {
    #[serde(flatten)]
    pub base: AMLIEvalData1,

/// 
    #[serde(rename = "DataString")]
    pub data_string: Option<String>,
}

impl AMLIEvalData1_TypeGroup1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: AMLIEvalData1::new(),
            data_string: None,
        }
    }


    /// Sets the value of DataString
    pub fn set_data_string(&mut self, value: String) {
        self.data_string = Some(value);
    }

    /// Gets the value of DataString
    pub fn get_data_string(&self) -> Option<&String> {
        self.data_string.as_ref()
    }
}

