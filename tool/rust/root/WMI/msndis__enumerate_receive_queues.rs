// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_EnumerateReceiveQueues struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_EnumerateReceiveQueues {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSNdis_EnumerateReceiveQueues {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            active: None,
            instance_name: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

/// 

    /// * `header` -  (MSNdis_WmiMethodHeader)
    /// * `receive_queue_info_array` -  (MSNdis_ReceiveQueueInfoArray)

    /// * `receive_queue_info_array` -  (MSNdis_ReceiveQueueInfoArray)
    pub fn wmi_enum_receive_queues(&self, header: MSNdis_WmiMethodHeader, receive_queue_info_array: &mut MSNdis_ReceiveQueueInfoArray) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Header".to_string(), value: header.into() });

        let result = self.invoke_method("WmiEnumReceiveQueues", &args)?;
        let receive_queue_info_array = result.get_value("ReceiveQueueInfoArray")?;
        Ok(result.return_value)

    }

}

