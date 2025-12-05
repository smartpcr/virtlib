// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_BootConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_BootConfiguration {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// If TRUE dynamically discover boot device.
    #[serde(rename = "DiscoverBootDevice")]
    pub discover_boot_device: Option<bool>,

/// The InitiatorNode specifies the iSCSI name of the initiator node to use for the connection. If empty, then the adapter can choose any initiator node name.
    #[serde(rename = "InitiatorNode")]
    pub initiator_node: Option<String>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// Options that affect how login is performed. See ISCSI_LoginOptions
    #[serde(rename = "LoginOptions")]
    pub login_options: Option<ISCSI_LoginOptions>,

/// LUN on target to use as boot device.
    #[serde(rename = "LUN")]
    pub lun: Option<u64>,

/// Authentication Password, for CHAP this is the shared secret to use when generating the response to the target challange. This field is a variable length array.
    #[serde(rename = "Password")]
    pub password: Vec<u8>,

/// Size in bytes of Target Password.
    #[serde(rename = "PasswordSize")]
    pub password_size: Option<u32>,

/// Security flags
    #[serde(rename = "SecurityFlags")]
    pub security_flags: Option<u64>,

/// TargetName specifies the iSCSI target name on which the boot device resides.
    #[serde(rename = "TargetName")]
    pub target_name: Option<String>,

/// Target portal to use for connection to the target.
    #[serde(rename = "TargetPortal")]
    pub target_portal: Option<ISCSI_TargetPortal>,

/// **extra fields** Authentication Username, for CHAP this is the CHAP Name (CHAP_N) use when authenticating with the target. NOTE: This field is a variable length array, the field that follows this field starts immediately after the end of this field subject to appropriate padding.
    #[serde(rename = "Username")]
    pub username: Vec<u8>,

/// Size in bytes of Target Username.
    #[serde(rename = "UsernameSize")]
    pub username_size: Option<u32>,
}

impl MSiSCSI_BootConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            discover_boot_device: None,
            initiator_node: None,
            instance_name: None,
            login_options: None,
            lun: None,
            password: Vec::new(),
            password_size: None,
            security_flags: None,
            target_name: None,
            target_portal: None,
            username: Vec::new(),
            username_size: None,
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

    /// Sets the value of DiscoverBootDevice
    pub fn set_discover_boot_device(&mut self, value: bool) {
        self.discover_boot_device = Some(value);
    }

    /// Gets the value of DiscoverBootDevice
    pub fn get_discover_boot_device(&self) -> Option<&bool> {
        self.discover_boot_device.as_ref()
    }

    /// Sets the value of InitiatorNode
    pub fn set_initiator_node(&mut self, value: String) {
        self.initiator_node = Some(value);
    }

    /// Gets the value of InitiatorNode
    pub fn get_initiator_node(&self) -> Option<&String> {
        self.initiator_node.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of LoginOptions
    pub fn set_login_options(&mut self, value: ISCSI_LoginOptions) {
        self.login_options = Some(value);
    }

    /// Gets the value of LoginOptions
    pub fn get_login_options(&self) -> Option<&ISCSI_LoginOptions> {
        self.login_options.as_ref()
    }

    /// Sets the value of LUN
    pub fn set_lun(&mut self, value: u64) {
        self.lun = Some(value);
    }

    /// Gets the value of LUN
    pub fn get_lun(&self) -> Option<&u64> {
        self.lun.as_ref()
    }

    /// Sets the value of Password
    pub fn set_password(&mut self, value: Vec<u8>) {
        self.password = value;
    }

    /// Gets the value of Password
    pub fn get_password(&self) -> &Vec<u8> {
        &self.password
    }

    /// Sets the value of PasswordSize
    pub fn set_password_size(&mut self, value: u32) {
        self.password_size = Some(value);
    }

    /// Gets the value of PasswordSize
    pub fn get_password_size(&self) -> Option<&u32> {
        self.password_size.as_ref()
    }

    /// Sets the value of SecurityFlags
    pub fn set_security_flags(&mut self, value: u64) {
        self.security_flags = Some(value);
    }

    /// Gets the value of SecurityFlags
    pub fn get_security_flags(&self) -> Option<&u64> {
        self.security_flags.as_ref()
    }

    /// Sets the value of TargetName
    pub fn set_target_name(&mut self, value: String) {
        self.target_name = Some(value);
    }

    /// Gets the value of TargetName
    pub fn get_target_name(&self) -> Option<&String> {
        self.target_name.as_ref()
    }

    /// Sets the value of TargetPortal
    pub fn set_target_portal(&mut self, value: ISCSI_TargetPortal) {
        self.target_portal = Some(value);
    }

    /// Gets the value of TargetPortal
    pub fn get_target_portal(&self) -> Option<&ISCSI_TargetPortal> {
        self.target_portal.as_ref()
    }

    /// Sets the value of Username
    pub fn set_username(&mut self, value: Vec<u8>) {
        self.username = value;
    }

    /// Gets the value of Username
    pub fn get_username(&self) -> &Vec<u8> {
        &self.username
    }

    /// Sets the value of UsernameSize
    pub fn set_username_size(&mut self, value: u32) {
        self.username_size = Some(value);
    }

    /// Gets the value of UsernameSize
    pub fn get_username_size(&self) -> Option<&u32> {
        self.username_size.as_ref()
    }
}

