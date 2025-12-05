// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_LogicalDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_LogicalDevice {
    #[serde(flatten)]
    pub base: CIM_EnabledLogicalElement,

/// Additional availability and status of the Device, beyond that specified in the Availability property. The Availability property denotes the primary status and availability of the Device. In some cases, this will not be sufficient to denote the complete status of the Device. In those cases, the AdditionalAvailability property can be used to provide further information. For example, a Device's primary Availability may be "Off line" (value=8), but it may also be in a low power state (AdditonalAvailability value=14), or the Device could be running Diagnostics (AdditionalAvailability value=5, "In Test").
    #[serde(rename = "AdditionalAvailability")]
    pub additional_availability: Vec<LogicalDevice_AdditionalAvailability>,

/// The primary availability and status of the Device. (Additional status information can be specified using the Additional Availability array property.) For example, the Availability property indicates that the Device is running and has full power (value=3), or is in a warning (4), test (5), degraded (10) or power save state (values 13-15 and 17). Regarding the Power Save states, these are defined as follows: Value 13 ("Power Save - Unknown") indicates that the Device is known to be in a power save mode, but its exact status in this mode is unknown; 14 ("Power Save - Low Power Mode") indicates that the Device is in a power save state but still functioning, and may exhibit degraded performance; 15 ("Power Save - Standby") describes that the Device is not functioning but could be brought to full power 'quickly'; and value 17 ("Power Save - Warning") indicates that the Device is in a warning state, though also in a power save mode.
    #[serde(rename = "Availability")]
    pub availability: Option<LogicalDevice_Availability>,

/// CreationClassName indicates the name of the class or the subclass used in the creation of an instance. When used with the other key properties of this class, this property allows all instances of this class and its subclasses to be uniquely identified.
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// An address or other identifying information to uniquely name the LogicalDevice.
    #[serde(rename = "DeviceID")]
    pub device_id: Option<String>,

/// ErrorCleared is a boolean property indicating that the error reported in LastErrorCode is now cleared.
    #[serde(rename = "ErrorCleared")]
    pub error_cleared: Option<bool>,

/// ErrorDescription is a free-form string supplying more information about the error recorded in LastErrorCode, and information on any corrective actions that may be taken.
    #[serde(rename = "ErrorDescription")]
    pub error_description: Option<String>,

/// An array of free-form strings providing explanations and details behind the entries in the OtherIdentifyingInfo array. Note, each entry of this array is related to the entry in OtherIdentifyingInfo that is located at the same index.
    #[serde(rename = "IdentifyingDescriptions")]
    pub identifying_descriptions: Vec<String>,

/// LastErrorCode captures the last error code reported by the LogicalDevice.
    #[serde(rename = "LastErrorCode")]
    pub last_error_code: Option<u32>,

/// The MaxQuiesceTime property has been deprecated. When evaluating the use of Quiesce, it was determine that this single property is not adequate for describing when a device will automatically exit a quiescent state. In fact, the most likely scenario for a device to exit a quiescent state was determined to be based on the number of outstanding requests queued rather than on a maximum time. This will be re-evaluated and repositioned later. 
/// Maximum time in milliseconds, that a Device can run in a "Quiesced" state. A Device's state is defined in its Availability and AdditionalAvailability properties, where "Quiesced" is conveyed by the value 21. What occurs at the end of the time limit is device-specific. The Device may unquiesce, may offline or take other action. A value of 0 indicates that a Device can remain quiesced indefinitely.
    #[serde(rename = "MaxQuiesceTime")]
    pub max_quiesce_time: Option<u64>,

/// OtherIdentifyingInfo captures additional data, beyond DeviceID information, that could be used to identify a LogicalDevice. One example would be to hold the Operating System's user friendly name for the Device in this property.
    #[serde(rename = "OtherIdentifyingInfo")]
    pub other_identifying_info: Vec<String>,

/// An enumerated array describing the power management capabilities of the Device. The use of this property has been deprecated. Instead, the PowerCapabilites property in an associated PowerManagementCapabilities class should be used.
    #[serde(rename = "PowerManagementCapabilities")]
    pub power_management_capabilities: Vec<LogicalDevice_PowerManagementCapabilities>,

/// Boolean indicating that the Device can be power managed. The use of this property has been deprecated. Instead, the existence of an associated PowerManagementCapabilities class (associated using the ElementCapabilities relationhip) indicates that power management is supported.
    #[serde(rename = "PowerManagementSupported")]
    pub power_management_supported: Option<bool>,

/// The number of consecutive hours that this Device has been powered, since its last power cycle.
    #[serde(rename = "PowerOnHours")]
    pub power_on_hours: Option<u64>,

/// The StatusInfo property indicates whether the Logical Device is in an enabled (value = 3), disabled (value = 4) or some other (1) or unknown (2) state. If this property does not apply to the LogicalDevice, the value, 5 ("Not Applicable"), should be used. StatusInfo has been deprecated in lieu of a more clearly named property with additional enumerated values (EnabledState), that is inherited from ManagedSystemElement. 
/// If a Device is ("Enabled")(value=3), it has been powered up, and is configured and operational. The Device may or may not be functionally active, depending on whether its Availability (or AdditionalAvailability) indicate that it is ("Running/Full Power")(value=3) or ("Off line") (value=8). In an enabled but offline mode, a Device may be performing out-of-band requests, such as running Diagnostics. If ("Disabled") StatusInfo value=4), a Device can only be "enabled" or powered off. In a personal computer environment, ("Disabled") means that the Device's driver is not available in the stack. In other environments, a Device can be disabled by removing its configuration file. A disabled device is physically present in a System and consuming resources, but can not be communicated with until a load of a driver, a load of a configuration file or some other "enabling" activity has occurred.
    #[serde(rename = "StatusInfo")]
    pub status_info: Option<LogicalDevice_StatusInfo>,

/// The scoping System's CreationClassName.
    #[serde(rename = "SystemCreationClassName")]
    pub system_creation_class_name: Option<String>,

/// The scoping System's Name.
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,

/// The total number of hours that this Device has been powered.
    #[serde(rename = "TotalPowerOnHours")]
    pub total_power_on_hours: Option<u64>,
}

impl CIM_LogicalDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EnabledLogicalElement::new(),
            additional_availability: Vec::new(),
            availability: None,
            creation_class_name: None,
            device_id: None,
            error_cleared: None,
            error_description: None,
            identifying_descriptions: Vec::new(),
            last_error_code: None,
            max_quiesce_time: None,
            other_identifying_info: Vec::new(),
            power_management_capabilities: Vec::new(),
            power_management_supported: None,
            power_on_hours: None,
            status_info: None,
            system_creation_class_name: None,
            system_name: None,
            total_power_on_hours: None,
        }
    }


    /// Sets the value of AdditionalAvailability
    pub fn set_additional_availability(&mut self, value: Vec<LogicalDevice_AdditionalAvailability>) {
        self.additional_availability = value;
    }

    /// Gets the value of AdditionalAvailability
    pub fn get_additional_availability(&self) -> &Vec<LogicalDevice_AdditionalAvailability> {
        &self.additional_availability
    }

    /// Sets the value of Availability
    pub fn set_availability(&mut self, value: LogicalDevice_Availability) {
        self.availability = Some(value);
    }

    /// Gets the value of Availability
    pub fn get_availability(&self) -> Option<&LogicalDevice_Availability> {
        self.availability.as_ref()
    }

    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of DeviceID
    pub fn set_device_id(&mut self, value: String) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceID
    pub fn get_device_id(&self) -> Option<&String> {
        self.device_id.as_ref()
    }

    /// Sets the value of ErrorCleared
    pub fn set_error_cleared(&mut self, value: bool) {
        self.error_cleared = Some(value);
    }

    /// Gets the value of ErrorCleared
    pub fn get_error_cleared(&self) -> Option<&bool> {
        self.error_cleared.as_ref()
    }

    /// Sets the value of ErrorDescription
    pub fn set_error_description(&mut self, value: String) {
        self.error_description = Some(value);
    }

    /// Gets the value of ErrorDescription
    pub fn get_error_description(&self) -> Option<&String> {
        self.error_description.as_ref()
    }

    /// Sets the value of IdentifyingDescriptions
    pub fn set_identifying_descriptions(&mut self, value: Vec<String>) {
        self.identifying_descriptions = value;
    }

    /// Gets the value of IdentifyingDescriptions
    pub fn get_identifying_descriptions(&self) -> &Vec<String> {
        &self.identifying_descriptions
    }

    /// Sets the value of LastErrorCode
    pub fn set_last_error_code(&mut self, value: u32) {
        self.last_error_code = Some(value);
    }

    /// Gets the value of LastErrorCode
    pub fn get_last_error_code(&self) -> Option<&u32> {
        self.last_error_code.as_ref()
    }

    /// Sets the value of MaxQuiesceTime
    pub fn set_max_quiesce_time(&mut self, value: u64) {
        self.max_quiesce_time = Some(value);
    }

    /// Gets the value of MaxQuiesceTime
    pub fn get_max_quiesce_time(&self) -> Option<&u64> {
        self.max_quiesce_time.as_ref()
    }

    /// Sets the value of OtherIdentifyingInfo
    pub fn set_other_identifying_info(&mut self, value: Vec<String>) {
        self.other_identifying_info = value;
    }

    /// Gets the value of OtherIdentifyingInfo
    pub fn get_other_identifying_info(&self) -> &Vec<String> {
        &self.other_identifying_info
    }

    /// Sets the value of PowerManagementCapabilities
    pub fn set_power_management_capabilities(&mut self, value: Vec<LogicalDevice_PowerManagementCapabilities>) {
        self.power_management_capabilities = value;
    }

    /// Gets the value of PowerManagementCapabilities
    pub fn get_power_management_capabilities(&self) -> &Vec<LogicalDevice_PowerManagementCapabilities> {
        &self.power_management_capabilities
    }

    /// Sets the value of PowerManagementSupported
    pub fn set_power_management_supported(&mut self, value: bool) {
        self.power_management_supported = Some(value);
    }

    /// Gets the value of PowerManagementSupported
    pub fn get_power_management_supported(&self) -> Option<&bool> {
        self.power_management_supported.as_ref()
    }

    /// Sets the value of PowerOnHours
    pub fn set_power_on_hours(&mut self, value: u64) {
        self.power_on_hours = Some(value);
    }

    /// Gets the value of PowerOnHours
    pub fn get_power_on_hours(&self) -> Option<&u64> {
        self.power_on_hours.as_ref()
    }

    /// Sets the value of StatusInfo
    pub fn set_status_info(&mut self, value: LogicalDevice_StatusInfo) {
        self.status_info = Some(value);
    }

    /// Gets the value of StatusInfo
    pub fn get_status_info(&self) -> Option<&LogicalDevice_StatusInfo> {
        self.status_info.as_ref()
    }

    /// Sets the value of SystemCreationClassName
    pub fn set_system_creation_class_name(&mut self, value: String) {
        self.system_creation_class_name = Some(value);
    }

    /// Gets the value of SystemCreationClassName
    pub fn get_system_creation_class_name(&self) -> Option<&String> {
        self.system_creation_class_name.as_ref()
    }

    /// Sets the value of SystemName
    pub fn set_system_name(&mut self, value: String) {
        self.system_name = Some(value);
    }

    /// Gets the value of SystemName
    pub fn get_system_name(&self) -> Option<&String> {
        self.system_name.as_ref()
    }

    /// Sets the value of TotalPowerOnHours
    pub fn set_total_power_on_hours(&mut self, value: u64) {
        self.total_power_on_hours = Some(value);
    }

    /// Gets the value of TotalPowerOnHours
    pub fn get_total_power_on_hours(&self) -> Option<&u64> {
        self.total_power_on_hours.as_ref()
    }

/// Sets the power state of the Device. The use of this method has been deprecated. Instead, use the SetPowerState method in the associated PowerManagementService class.

    /// * `power_state` - The power state to set. (LogicalDevice_PowerState)
    /// * `time` - Time indicates when the power state should be set, either as a regular date-time value or as an interval value (where the interval begins when the method invocation is received. (String)

    /// * `return_value` -  (u32)
    pub fn set_power_state(&self, power_state: LogicalDevice_PowerState, time: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PowerState".to_string(), value: power_state.into() });
        args.push(MethodParameter { name: "Time".to_string(), value: time.into() });
        self.invoke_method("SetPowerState", &args)

    }


/// Requests a reset of the LogicalDevice. The return value should be 0 if the request was successfully executed, 1 if the request is not supported and some other value if an error occurred. In a subclass, the set of possible return codes could be specified, using a ValueMap qualifier on the method. The strings to which the ValueMap contents are 'translated' may also be specified in the subclass as a Values array qualifier.

    /// * `return_value` -  (u32)
    pub fn reset(&self) -> Result<(), WmiError> {
        self.invoke_method("Reset", &[])

    }


/// The EnableDevice method has been deprecated in lieu of the more general RequestStateChange method that directly overlaps with the functionality provided by this method. 
/// Requests that the LogicalDevice be enabled ("Enabled" input parameter = TRUE) or disabled (= FALSE). If successful, the Device's StatusInfo/EnabledState properties should reflect the desired state (enabled/disabled). Note that this method's function overlaps with the RequestedState property. RequestedState was added to the model to maintain a record (i.e., a persisted value) of the last state request. Invoking the EnableDevice method should set the RequestedState property appropriately. 
/// The return code should be 0 if the request was successfully executed, 1 if the request is not supported and some other value if an error occurred. In a subclass, the set of possible return codes could be specified, using a ValueMap qualifier on the method. The strings to which the ValueMap contents are 'translated' may also be specified in the subclass as a Values array qualifier.

    /// * `enabled` - If TRUE enable the device, if FALSE disable the device. (bool)

    /// * `return_value` -  (u32)
    pub fn enable_device(&self, enabled: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Enabled".to_string(), value: enabled.into() });
        self.invoke_method("EnableDevice", &args)

    }


/// The OnlineDevice method has been deprecated in lieu of the more general RequestStateChange method that directly overlaps with the functionality provided by this method. 
/// Requests that the LogicalDevice be brought online ("Online" input parameter = TRUE) or taken offline (= FALSE). "Online" indicates that the Device is ready to accept requests, and is operational and fully functioning. In this case, the Device's Availability property would be set to a value of 3 ("Running/Full Power"). "Offline" indicates that a Device is powered up and operational, but not processing functional requests. In an offline state, a Device may be capable of running diagnostics or generating operational alerts. For example, when the "Offline" button is pushed on a Printer, the Device is no longer available to process print jobs, but could be available for diagnostics or maintenance. 
/// If this method is successful, the Device's Availability and AdditionalAvailability properties should reflect the updated status. If a failure occurs trying to bring the Device online or offline, it should remain in its current state. IE, the request, if unsuccessful, should not leave the Device in an indeterminate state. When bringing a Device back "Online", from an "Offline" mode, the Device should be restored to its last "Online" state, if at all possible. Only a Device that has an EnabledState/StatusInfo of "Enabled" and has been configured can be brought online or taken offline. 
/// OnlineDevice should return 0 if successful, 1 if the request is not supported at all, 2 if the request is not supported due to the current state of the Device, and some other value if any other error occurred. In a subclass, the set of possible return codes could be specified, using a ValueMap qualifier on the method. The strings to which the ValueMap contents are 'translated' may also be specified in the subclass as a Values array qualifier. 
/// Note that this method's function overlaps with the RequestedState property. RequestedState was added to the model to maintain a record (i.e., a persisted value) of the last state request. Invoking the OnlineDevice method should set the RequestedState property appropriately.

    /// * `online` - If TRUE, take the device online, if FALSE, take the device OFFLINE. (bool)

    /// * `return_value` -  (u32)
    pub fn online_device(&self, online: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Online".to_string(), value: online.into() });
        self.invoke_method("OnlineDevice", &args)

    }


/// The QuiesceDevice method has been deprecated in lieu of the more general RequestStateChange method that directly overlaps with the functionality provided by this method. 
/// Requests that the LogicalDevice cleanly cease all current activity ("Quiesce" input parameter = TRUE) or resume activity (= FALSE). For this method to quiesce a Device, that Device should have an Availability (or Additional Availability) of "Running/Full Power" (value=3) and an EnabledStatus/StatusInfo of "Enabled". For example, if quiesced, a Device may then be offlined for diagnostics, or disabled for power off and hot swap. For the method to "unquiesce" a Device, that Device should have an Availability (or AdditionalAvailability) of "Quiesced" (value=21) and an EnabledStatus/StatusInfo of "Enabled". In this case, the Device would be returned to an "Enabled" and "Running/Full Power" status. 
/// The method's return code should indicate the success or failure of the quiesce. It should return 0 if successful, 1 if the request is not supported at all, 2 if the request is not supported due to the current state of the Device, and some other value if any other error occurred. In a subclass, the set of possible return codes could be specified, using a ValueMap qualifier on the method. The strings to which the ValueMap contents are 'translated' may also be specified in the subclass as a Values array qualifier.

    /// * `quiesce` - If set to TRUE then cleanly cease all activity, if FALSE resume activity. (bool)

    /// * `return_value` -  (u32)
    pub fn quiesce_device(&self, quiesce: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Quiesce".to_string(), value: quiesce.into() });
        self.invoke_method("QuiesceDevice", &args)

    }


/// Requests that the Device capture its current configuration, setup and/or state information in a backing store. The goal would be to use this information at a later time (via the RestoreProperties method), to return a Device to its present "condition". This method may not be supported by all Devices. The method should return 0 if successful, 1 if the request is not supported, and some other value if any other error occurred. In a subclass, the set of possible return codes could be specified, using a ValueMap qualifier on the method. The strings to which the ValueMap contents are 'translated' may also be specified in the subclass as a Values array qualifier.

    /// * `return_value` -  (u32)
    pub fn save_properties(&self) -> Result<(), WmiError> {
        self.invoke_method("SaveProperties", &[])

    }


/// Requests that the Device re-establish its configuration, setup and/or state information from a backing store. The intent is to capture this information at an earlier time (via the SaveProperties method), and use it to return a Device to this earlier "condition". This method may not be supported by all Devices. The method should return 0 if successful, 1 if the request is not supported, and some other value if any other error occurred. In a subclass, the set of possible return codes could be specified, using a ValueMap qualifier on the method. The strings to which the ValueMap contents are 'translated' may also be specified in the subclass as a Values array qualifier.

    /// * `return_value` -  (u32)
    pub fn restore_properties(&self) -> Result<(), WmiError> {
        self.invoke_method("RestoreProperties", &[])

    }

}

