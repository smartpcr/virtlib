// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSIInitiator_TargetClass struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSIInitiator_TargetClass {

/// 
    #[serde(rename = "DiscoveryMechanism")]
    pub discovery_mechanism: Option<String>,

/// 
    #[serde(rename = "InitiatorName")]
    pub initiator_name: Option<String>,

/// 
    #[serde(rename = "LoginOptions")]
    pub login_options: Option<MSiSCSIInitiator_TargetLoginOptions>,

/// 
    #[serde(rename = "Mappings")]
    pub mappings: Option<MSiSCSIInitiator_TargetMappings>,

/// 
    #[serde(rename = "PortalGroups")]
    pub portal_groups: Vec<MSiSCSIInitiator_PortalGroup>,

/// 
    #[serde(rename = "ProtocolType")]
    pub protocol_type: Option<TargetClass_ProtocolType>,

/// 
    #[serde(rename = "TargetAlias")]
    pub target_alias: Option<String>,

/// 
    #[serde(rename = "TargetFlags")]
    pub target_flags: Option<u32>,

/// 
    #[serde(rename = "TargetName")]
    pub target_name: Option<String>,
}

impl MSiSCSIInitiator_TargetClass {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            discovery_mechanism: None,
            initiator_name: None,
            login_options: None,
            mappings: None,
            portal_groups: Vec::new(),
            protocol_type: None,
            target_alias: None,
            target_flags: None,
            target_name: None,
        }
    }


    /// Sets the value of DiscoveryMechanism
    pub fn set_discovery_mechanism(&mut self, value: String) {
        self.discovery_mechanism = Some(value);
    }

    /// Gets the value of DiscoveryMechanism
    pub fn get_discovery_mechanism(&self) -> Option<&String> {
        self.discovery_mechanism.as_ref()
    }

    /// Sets the value of InitiatorName
    pub fn set_initiator_name(&mut self, value: String) {
        self.initiator_name = Some(value);
    }

    /// Gets the value of InitiatorName
    pub fn get_initiator_name(&self) -> Option<&String> {
        self.initiator_name.as_ref()
    }

    /// Sets the value of LoginOptions
    pub fn set_login_options(&mut self, value: MSiSCSIInitiator_TargetLoginOptions) {
        self.login_options = Some(value);
    }

    /// Gets the value of LoginOptions
    pub fn get_login_options(&self) -> Option<&MSiSCSIInitiator_TargetLoginOptions> {
        self.login_options.as_ref()
    }

    /// Sets the value of Mappings
    pub fn set_mappings(&mut self, value: MSiSCSIInitiator_TargetMappings) {
        self.mappings = Some(value);
    }

    /// Gets the value of Mappings
    pub fn get_mappings(&self) -> Option<&MSiSCSIInitiator_TargetMappings> {
        self.mappings.as_ref()
    }

    /// Sets the value of PortalGroups
    pub fn set_portal_groups(&mut self, value: Vec<MSiSCSIInitiator_PortalGroup>) {
        self.portal_groups = value;
    }

    /// Gets the value of PortalGroups
    pub fn get_portal_groups(&self) -> &Vec<MSiSCSIInitiator_PortalGroup> {
        &self.portal_groups
    }

    /// Sets the value of ProtocolType
    pub fn set_protocol_type(&mut self, value: TargetClass_ProtocolType) {
        self.protocol_type = Some(value);
    }

    /// Gets the value of ProtocolType
    pub fn get_protocol_type(&self) -> Option<&TargetClass_ProtocolType> {
        self.protocol_type.as_ref()
    }

    /// Sets the value of TargetAlias
    pub fn set_target_alias(&mut self, value: String) {
        self.target_alias = Some(value);
    }

    /// Gets the value of TargetAlias
    pub fn get_target_alias(&self) -> Option<&String> {
        self.target_alias.as_ref()
    }

    /// Sets the value of TargetFlags
    pub fn set_target_flags(&mut self, value: u32) {
        self.target_flags = Some(value);
    }

    /// Gets the value of TargetFlags
    pub fn get_target_flags(&self) -> Option<&u32> {
        self.target_flags.as_ref()
    }

    /// Sets the value of TargetName
    pub fn set_target_name(&mut self, value: String) {
        self.target_name = Some(value);
    }

    /// Gets the value of TargetName
    pub fn get_target_name(&self) -> Option<&String> {
        self.target_name.as_ref()
    }

/// 

    /// * `initiator_port_number` -  (u32)
    /// * `is_informational_session` -  (bool)
    /// * `is_persistent` -  (bool)
    /// * `key` -  (u8[])
    /// * `login_options` -  (MSiSCSIInitiator_TargetLoginOptions)
    /// * `mappings` -  (MSiSCSIInitiator_TargetMappings[])
    /// * `security_flags` -  (u64)
    /// * `target_portal` -  (MSiSCSIInitiator_Portal)

    /// * `return_value` -  (u32)
    /// * `unique_connection_id` -  (String)
    /// * `unique_session_id` -  (String)
    pub fn login(&self, is_informational_session: bool, initiator_port_number: u32, target_portal: MSiSCSIInitiator_Portal, security_flags: u64, mappings: &Vec<MSiSCSIInitiator_TargetMappings>, login_options: MSiSCSIInitiator_TargetLoginOptions, key: &Vec<u8>, is_persistent: bool, unique_session_id: &mut String, unique_connection_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IsInformationalSession".to_string(), value: is_informational_session.into() });
        args.push(MethodParameter { name: "InitiatorPortNumber".to_string(), value: initiator_port_number.into() });
        args.push(MethodParameter { name: "TargetPortal".to_string(), value: target_portal.into() });
        args.push(MethodParameter { name: "SecurityFlags".to_string(), value: security_flags.into() });
        args.push(MethodParameter { name: "Mappings".to_string(), value: mappings.into() });
        args.push(MethodParameter { name: "LoginOptions".to_string(), value: login_options.into() });
        args.push(MethodParameter { name: "key".to_string(), value: key.into() });
        args.push(MethodParameter { name: "IsPersistent".to_string(), value: is_persistent.into() });

        let result = self.invoke_method("Login", &args)?;
        let unique_connection_id = result.get_value("UniqueConnectionId")?;
        let unique_session_id = result.get_value("UniqueSessionId")?;
        Ok(result.return_value)

    }

}

