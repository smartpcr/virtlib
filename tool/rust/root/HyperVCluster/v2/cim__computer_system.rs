// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ComputerSystem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ComputerSystem {
    #[serde(flatten)]
    pub base: CIM_System,

/// Enumeration indicating the purpose(s) to which the ComputerSystem is dedicated, if any, and what functionality is provided. For example, one could specify that the System is dedicated to "Print" (value=11) or acts as a "Hub" (value=8). 
/// Also, one could indicate that this is a general purpose system by indicating 'Not Dedicated' (value=0) but that it also hosts 'Print' (value=11) or mobile phone 'Mobile User Device' (value=17) services. 
/// A clarification is needed with respect to the value 17 ("Mobile User Device"). An example of a dedicated user device is a mobile phone or a barcode scanner in a store that communicates via radio frequency. These systems are quite limited in functionality and programmability, and are not considered 'general purpose' computing platforms. Alternately, an example of a mobile system that is 'general purpose' (i.e., is NOT dedicated) is a hand-held computer. Although limited in its programmability, new software can be downloaded and its functionality expanded by the user. 
/// A value of "Management" indicates this instance is dedicated to hosting system management software.
/// A value of "Management Controller" indicates this instance represents specialized hardware dedicated to systems management (i.e., a Baseboard Management Controller (BMC) or service processor).
/// The management scope of a "Management Controller" is typically a single managed system in which it is contained.
/// A value of "Chassis Manager" indicates this instance represents a system dedicated to management of a blade chassis and its contained devices. This value would be used to represent a Shelf Controller. A "Chassis Manager" is an aggregation point for management and may rely on subordinate management controllers for the management of constituent parts. A value of "Host-based RAID Controller" indicates this instance represents a RAID storage controller contained within a host computer. A value of "Storage Device Enclosure" indicates this instance represents an enclosure that contains storage devices. A "Virtual Tape Library" is the emulation of a tape library by a Virtual Library System. A "Virtual Library System" uses disk storage to emulate tape libraries.A "FC Switch" indicates this instance is dedicated to switching layer 2 fibre channel frames. An "Ethernet Switch" indicates this instance is dedicated to switching layer 2 ethernet frames.
    #[serde(rename = "Dedicated")]
    pub dedicated: Vec<ComputerSystem_Dedicated>,

/// A string describing how or why the system is dedicated when the Dedicated array includes the value 2, "Other".
    #[serde(rename = "OtherDedicatedDescriptions")]
    pub other_dedicated_descriptions: Vec<String>,

/// An enumerated array describing the power management capabilities of the ComputerSystem. The use of this property has been deprecated. Instead, the Power Capabilites property in an associated PowerManagement Capabilities class should be used.
    #[serde(rename = "PowerManagementCapabilities")]
    pub power_management_capabilities: Vec<ComputerSystem_PowerManagementCapabilities>,

/// If enabled (value = 4), the ComputerSystem can be reset via hardware (e.g. the power and reset buttons). If disabled (value = 3), hardware reset is not allowed. In addition to Enabled and Disabled, other Values for the property are also defined - "Not Implemented" (5), "Other" (1) and "Unknown" (2).
    #[serde(rename = "ResetCapability")]
    pub reset_capability: Option<ComputerSystem_ResetCapability>,
}

impl CIM_ComputerSystem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_System::new(),
            dedicated: Vec::new(),
            other_dedicated_descriptions: Vec::new(),
            power_management_capabilities: Vec::new(),
            reset_capability: None,
        }
    }


    /// Sets the value of Dedicated
    pub fn set_dedicated(&mut self, value: Vec<ComputerSystem_Dedicated>) {
        self.dedicated = value;
    }

    /// Gets the value of Dedicated
    pub fn get_dedicated(&self) -> &Vec<ComputerSystem_Dedicated> {
        &self.dedicated
    }

    /// Sets the value of OtherDedicatedDescriptions
    pub fn set_other_dedicated_descriptions(&mut self, value: Vec<String>) {
        self.other_dedicated_descriptions = value;
    }

    /// Gets the value of OtherDedicatedDescriptions
    pub fn get_other_dedicated_descriptions(&self) -> &Vec<String> {
        &self.other_dedicated_descriptions
    }

    /// Sets the value of PowerManagementCapabilities
    pub fn set_power_management_capabilities(&mut self, value: Vec<ComputerSystem_PowerManagementCapabilities>) {
        self.power_management_capabilities = value;
    }

    /// Gets the value of PowerManagementCapabilities
    pub fn get_power_management_capabilities(&self) -> &Vec<ComputerSystem_PowerManagementCapabilities> {
        &self.power_management_capabilities
    }

    /// Sets the value of ResetCapability
    pub fn set_reset_capability(&mut self, value: ComputerSystem_ResetCapability) {
        self.reset_capability = Some(value);
    }

    /// Gets the value of ResetCapability
    pub fn get_reset_capability(&self) -> Option<&ComputerSystem_ResetCapability> {
        self.reset_capability.as_ref()
    }

/// Sets the power state of the computer. The use of this method has been deprecated. Instead, use the SetPowerState method in the associated PowerManagementService class.

    /// * `power_state` - The Desired state for the COmputerSystem. (ComputerSystem_PowerState)
    /// * `time` - Time indicates when the power state should be set, either as a regular date-time value or as an interval value (where the interval begins when the method invocation is received. (String)

    /// * `return_value` -  (u32)
    pub fn set_power_state(&self, power_state: ComputerSystem_PowerState, time: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PowerState".to_string(), value: power_state.into() });
        args.push(MethodParameter { name: "Time".to_string(), value: time.into() });
        self.invoke_method("SetPowerState", &args)

    }

}

