// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_VirtualSystemSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_VirtualSystemSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// Action to take for the virtual system when the software executed by the virtual system fails. Failures in this case means a failure that is detectable by the host platform, such as a non-interuptable wait state condition.
    #[serde(rename = "AutomaticRecoveryAction")]
    pub automatic_recovery_action: Option<VirtualSystemSettingData_AutomaticRecoveryAction>,

/// Action to take for the virtual system when the host is shut down.
    #[serde(rename = "AutomaticShutdownAction")]
    pub automatic_shutdown_action: Option<VirtualSystemSettingData_AutomaticShutdownAction>,

/// Action to take for the virtual system when the host is started.
    #[serde(rename = "AutomaticStartupAction")]
    pub automatic_startup_action: Option<VirtualSystemSettingData_AutomaticStartupAction>,

/// Delay applicable to startup action. The value shall be in the interval variant of the datetime datatype.
    #[serde(rename = "AutomaticStartupActionDelay")]
    pub automatic_startup_action_delay: Option<String>,

/// Number indicating the relative sequence of virtual system activation when the host system is started. A lower number indicates earlier activation. If one or more configurations show the same value, the sequence is implementation dependent. A value of 0 indicates that the sequence is implementation dependent.
    #[serde(rename = "AutomaticStartupActionSequenceNumber")]
    pub automatic_startup_action_sequence_number: Option<u16>,

/// Filepath of a directory where information about the virtual system configuration is stored.Format shall be URI based on RFC 2079.
    #[serde(rename = "ConfigurationDataRoot")]
    pub configuration_data_root: Option<String>,

/// Filepath of a file where information about the virtual system configuration is stored. A relative path appends to the value of the ConfigurationDataRoot property.Format shall be URI based on RFC 2079.
    #[serde(rename = "ConfigurationFile")]
    pub configuration_file: Option<String>,

/// Unique id of the virtual system configuration. Note that the ConfigurationID is different from the InstanceID as it is assigned by the implementation to a virtual system or a virtual system configuration. It is not a key, and the same value may occur within more than one instance.
    #[serde(rename = "ConfigurationID")]
    pub configuration_id: Option<String>,

/// Time when the virtual system configuration was created.
    #[serde(rename = "CreationTime")]
    pub creation_time: Option<String>,

/// Filepath of a directory where log information about the virtual system is stored. A relative path appends to the value of the ConfigurationDataRoot property.Format shall be URI based on RFC 2079.
    #[serde(rename = "LogDataRoot")]
    pub log_data_root: Option<String>,

/// End-user supplied notes that are related to the virtual system.
    #[serde(rename = "Notes")]
    pub notes: Vec<String>,

/// Filepath of a file where recovery relateded information of the virtual system is stored.Format shall be URI based on RFC 2079.
    #[serde(rename = "RecoveryFile")]
    pub recovery_file: Option<String>,

/// Filepath of a directory where information about virtual system snapshots is stored. A relative path appends to the value of the ConfigurationDataRoot property.Format shall be URI based on RFC 2079.
    #[serde(rename = "SnapshotDataRoot")]
    pub snapshot_data_root: Option<String>,

/// Filepath of a directory where suspend related information about the virtual system is stored. A relative path appends to the value of the ConfigurationDataRoot property.Format shall be URI based on RFC 2079.
    #[serde(rename = "SuspendDataRoot")]
    pub suspend_data_root: Option<String>,

/// Filepath of a directory where swapfiles of the virtual system are stored. A relative path appends to the value of the ConfigurationDataRoot property.Format shall be URI based on RFC 2079.
    #[serde(rename = "SwapFileDataRoot")]
    pub swap_file_data_root: Option<String>,

/// VirtualSystemIdentifier shall reflect a unique name for the system as it is used within the virtualization platform. Note that the VirtualSystemIdentifier is not the hostname assigned to the operating system instance running within the virtual system, nor is it an IP address or MAC address assigned to any of its network ports. 
/// On create requests VirtualSystemIdentifier may contain implementation specific rules (like simple patterns or regular expresssion) that may be interpreted by the implementation when assigning a VirtualSystemIdentifier.
    #[serde(rename = "VirtualSystemIdentifier")]
    pub virtual_system_identifier: Option<String>,

/// VirtualSystemType shall reflect a particular type of virtual system.
/// The property value shall conform to this format (in ABNF): vs-type = dmtf-value / other-org-value / legacy-value; dmtf-value = "DMTF:" defining-org ":" org-vs-type; other-org-value = defining-org ":" org-vs-type;
/// Where: dmtf-value:
/// is a property value defined by DMTF and is defined in the description of this property. other-org-value:
/// is a property value defined by a business entity other than DMTF and is not defined in the description of this property. legacy-value:
/// is a property value defined by a business entity other than DMTF and is not defined in the description of this property. These values are permitted but recommended to be deprecated over time. defining-org:
/// is an identifier for the business entity that defines the virtual system type. It shall include a copyrighted, trademarked, or otherwise unique name that is owned by that business entity. It shall not be "DMTF" and shall not contain a colon (:). org-vs-type:
/// is an identifier for the virtual system type within the defining business entity. It shall be unique within the defining-org. It may use any character allowed for CIM strings, except for the following: U0000-U001F (Unicode C0 controls) U0020 (space), note that the reason is that OVF allows for multiple space-separated vs-type values in this property. U007F (Unicode C0 controls) U0080-U009F (Unicode C1 controls)
/// If there is a need to structure the value into segments, the segments should be separated with a single colon (:).
/// The values of this property shall be processed case sensitively. They are intended to be processed programmatically (instead of being a display name) and should be short.
/// As stated in the class description, instances of this class may be used for various purposes. A management application intending to use an instance of this class as input parameter to an operation that creates or modifies a virtual system should first determine the set of valid virtual system types that are supported by the virtualization platform hosting the virtual system by inspecting values of array property VirtualSystemTypesSupported of the instance of class CIM_VirtualSystemManagementCapabilities that describes the capabilities of the virtualization platform.
/// The following DMTF values are defined: DMTF:unknown - the virtual system type is unknown or cannot be determined
    #[serde(rename = "VirtualSystemType")]
    pub virtual_system_type: Option<String>,
}

impl CIM_VirtualSystemSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            automatic_recovery_action: None,
            automatic_shutdown_action: None,
            automatic_startup_action: None,
            automatic_startup_action_delay: None,
            automatic_startup_action_sequence_number: None,
            configuration_data_root: None,
            configuration_file: None,
            configuration_id: None,
            creation_time: None,
            log_data_root: None,
            notes: Vec::new(),
            recovery_file: None,
            snapshot_data_root: None,
            suspend_data_root: None,
            swap_file_data_root: None,
            virtual_system_identifier: None,
            virtual_system_type: None,
        }
    }


    /// Sets the value of AutomaticRecoveryAction
    pub fn set_automatic_recovery_action(&mut self, value: VirtualSystemSettingData_AutomaticRecoveryAction) {
        self.automatic_recovery_action = Some(value);
    }

    /// Gets the value of AutomaticRecoveryAction
    pub fn get_automatic_recovery_action(&self) -> Option<&VirtualSystemSettingData_AutomaticRecoveryAction> {
        self.automatic_recovery_action.as_ref()
    }

    /// Sets the value of AutomaticShutdownAction
    pub fn set_automatic_shutdown_action(&mut self, value: VirtualSystemSettingData_AutomaticShutdownAction) {
        self.automatic_shutdown_action = Some(value);
    }

    /// Gets the value of AutomaticShutdownAction
    pub fn get_automatic_shutdown_action(&self) -> Option<&VirtualSystemSettingData_AutomaticShutdownAction> {
        self.automatic_shutdown_action.as_ref()
    }

    /// Sets the value of AutomaticStartupAction
    pub fn set_automatic_startup_action(&mut self, value: VirtualSystemSettingData_AutomaticStartupAction) {
        self.automatic_startup_action = Some(value);
    }

    /// Gets the value of AutomaticStartupAction
    pub fn get_automatic_startup_action(&self) -> Option<&VirtualSystemSettingData_AutomaticStartupAction> {
        self.automatic_startup_action.as_ref()
    }

    /// Sets the value of AutomaticStartupActionDelay
    pub fn set_automatic_startup_action_delay(&mut self, value: String) {
        self.automatic_startup_action_delay = Some(value);
    }

    /// Gets the value of AutomaticStartupActionDelay
    pub fn get_automatic_startup_action_delay(&self) -> Option<&String> {
        self.automatic_startup_action_delay.as_ref()
    }

    /// Sets the value of AutomaticStartupActionSequenceNumber
    pub fn set_automatic_startup_action_sequence_number(&mut self, value: u16) {
        self.automatic_startup_action_sequence_number = Some(value);
    }

    /// Gets the value of AutomaticStartupActionSequenceNumber
    pub fn get_automatic_startup_action_sequence_number(&self) -> Option<&u16> {
        self.automatic_startup_action_sequence_number.as_ref()
    }

    /// Sets the value of ConfigurationDataRoot
    pub fn set_configuration_data_root(&mut self, value: String) {
        self.configuration_data_root = Some(value);
    }

    /// Gets the value of ConfigurationDataRoot
    pub fn get_configuration_data_root(&self) -> Option<&String> {
        self.configuration_data_root.as_ref()
    }

    /// Sets the value of ConfigurationFile
    pub fn set_configuration_file(&mut self, value: String) {
        self.configuration_file = Some(value);
    }

    /// Gets the value of ConfigurationFile
    pub fn get_configuration_file(&self) -> Option<&String> {
        self.configuration_file.as_ref()
    }

    /// Sets the value of ConfigurationID
    pub fn set_configuration_id(&mut self, value: String) {
        self.configuration_id = Some(value);
    }

    /// Gets the value of ConfigurationID
    pub fn get_configuration_id(&self) -> Option<&String> {
        self.configuration_id.as_ref()
    }

    /// Sets the value of CreationTime
    pub fn set_creation_time(&mut self, value: String) {
        self.creation_time = Some(value);
    }

    /// Gets the value of CreationTime
    pub fn get_creation_time(&self) -> Option<&String> {
        self.creation_time.as_ref()
    }

    /// Sets the value of LogDataRoot
    pub fn set_log_data_root(&mut self, value: String) {
        self.log_data_root = Some(value);
    }

    /// Gets the value of LogDataRoot
    pub fn get_log_data_root(&self) -> Option<&String> {
        self.log_data_root.as_ref()
    }

    /// Sets the value of Notes
    pub fn set_notes(&mut self, value: Vec<String>) {
        self.notes = value;
    }

    /// Gets the value of Notes
    pub fn get_notes(&self) -> &Vec<String> {
        &self.notes
    }

    /// Sets the value of RecoveryFile
    pub fn set_recovery_file(&mut self, value: String) {
        self.recovery_file = Some(value);
    }

    /// Gets the value of RecoveryFile
    pub fn get_recovery_file(&self) -> Option<&String> {
        self.recovery_file.as_ref()
    }

    /// Sets the value of SnapshotDataRoot
    pub fn set_snapshot_data_root(&mut self, value: String) {
        self.snapshot_data_root = Some(value);
    }

    /// Gets the value of SnapshotDataRoot
    pub fn get_snapshot_data_root(&self) -> Option<&String> {
        self.snapshot_data_root.as_ref()
    }

    /// Sets the value of SuspendDataRoot
    pub fn set_suspend_data_root(&mut self, value: String) {
        self.suspend_data_root = Some(value);
    }

    /// Gets the value of SuspendDataRoot
    pub fn get_suspend_data_root(&self) -> Option<&String> {
        self.suspend_data_root.as_ref()
    }

    /// Sets the value of SwapFileDataRoot
    pub fn set_swap_file_data_root(&mut self, value: String) {
        self.swap_file_data_root = Some(value);
    }

    /// Gets the value of SwapFileDataRoot
    pub fn get_swap_file_data_root(&self) -> Option<&String> {
        self.swap_file_data_root.as_ref()
    }

    /// Sets the value of VirtualSystemIdentifier
    pub fn set_virtual_system_identifier(&mut self, value: String) {
        self.virtual_system_identifier = Some(value);
    }

    /// Gets the value of VirtualSystemIdentifier
    pub fn get_virtual_system_identifier(&self) -> Option<&String> {
        self.virtual_system_identifier.as_ref()
    }

    /// Sets the value of VirtualSystemType
    pub fn set_virtual_system_type(&mut self, value: String) {
        self.virtual_system_type = Some(value);
    }

    /// Gets the value of VirtualSystemType
    pub fn get_virtual_system_type(&self) -> Option<&String> {
        self.virtual_system_type.as_ref()
    }
}

