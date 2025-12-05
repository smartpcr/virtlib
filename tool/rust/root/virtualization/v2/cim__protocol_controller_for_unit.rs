// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ProtocolControllerForUnit struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ProtocolControllerForUnit {
    #[serde(flatten)]
    pub base: CIM_ProtocolControllerForDevice,

/// The access rights granted to the referenced logical unit as exposed through referenced ProtocolController. The 'No Access' value is used in implementations where the DeviceNumber is reserved, but no access is granted. 
/// If the instrumentation exposes PrivilegeManagementService, this property MUST be synchronized with the Activities property of any Privilege instances associated with StorageHardwareIDs associated to the referenced ProtocolController and the referenced LogicalDevice. In particular, when this property is 'Read Write', Privilege.Activities MUST include entries for 'Read' and 'Write'. When this property is 'Read-Only', Privilege.Activities MUST include an entry for 'Read'. The corresponding entries for Privilege.ActivityQualifiers MUST be 'CDB=*' and the corresponding entries for Privilege.QualifierFormat MUST be 'SCSI Command'.
    #[serde(rename = "DeviceAccess")]
    pub device_access: Option<ProtocolControllerForUnit_DeviceAccess>,
}

impl CIM_ProtocolControllerForUnit {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ProtocolControllerForDevice::new(),
            device_access: None,
        }
    }


    /// Sets the value of DeviceAccess
    pub fn set_device_access(&mut self, value: ProtocolControllerForUnit_DeviceAccess) {
        self.device_access = Some(value);
    }

    /// Gets the value of DeviceAccess
    pub fn get_device_access(&self) -> Option<&ProtocolControllerForUnit_DeviceAccess> {
        self.device_access.as_ref()
    }
}

