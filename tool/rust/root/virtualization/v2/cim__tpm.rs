// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_TPM struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_TPM {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// AvailableRequestedTPMStates indicates the possible values for the RequestedTPMState parameter of the method RequestTPMStateChange, used to initiate a state change. The values listed shall be a subset of the values contained in the RequestedTPMStatesSupported property of the associated instance of CIM_TPMCapabilities where the values selected are a function of the current TPM state of the TPM.
    #[serde(rename = "AvailableRequestedTPMStates")]
    pub available_requested_tpmstates: Vec<TPM_AvailableRequestedTPMStates>,

/// The TPM manufacturer's major revision.
    #[serde(rename = "TPMManafucturerMajorRevision")]
    pub tpmmanafucturer_major_revision: Option<u32>,

/// The TPM manufacturer Identifier as defined by the TCG.
    #[serde(rename = "TPMManufacturerId")]
    pub tpmmanufacturer_id: Option<u32>,

/// The additional information defined by the TPM manufacturer.
    #[serde(rename = "TPMManufacturerInfo")]
    pub tpmmanufacturer_info: Option<String>,

/// The TPM manufacturer's minor revision.
    #[serde(rename = "TPMManufacturerMinorRevision")]
    pub tpmmanufacturer_minor_revision: Option<u32>,

/// The TPM specification's major version to which the TPM device claims to be conformant.
    #[serde(rename = "TPMSpecMajorVersion")]
    pub tpmspec_major_version: Option<u32>,

/// The TPM specification's minor version to which the TPM device claims to be conformant.
    #[serde(rename = "TPMSpecMinorVersion")]
    pub tpmspec_minor_version: Option<u32>,

/// Indicates the TPM's operational mode by indicating whether TPM is Enabled, Active and Owned.
    #[serde(rename = "TPMState")]
    pub tpmstate: Option<TPM_TPMState>,

/// TransitioningToState indicates the TPM's target state to which the TPM is transitioning. 
/// A value of 11 "No Change" shall indicate that no transition is in progress.A value of 12 "Not Applicable" shall indicate the implementation does not support representing ongoing transitions. 
/// A value other than 11 or 10 shall identify the state to which the element is in the process of transitioning.
    #[serde(rename = "TransitioningToTPMState")]
    pub transitioning_to_tpmstate: Option<TPM_TransitioningToTPMState>,
}

impl CIM_TPM {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            available_requested_tpmstates: Vec::new(),
            tpmmanafucturer_major_revision: None,
            tpmmanufacturer_id: None,
            tpmmanufacturer_info: None,
            tpmmanufacturer_minor_revision: None,
            tpmspec_major_version: None,
            tpmspec_minor_version: None,
            tpmstate: None,
            transitioning_to_tpmstate: None,
        }
    }


    /// Sets the value of AvailableRequestedTPMStates
    pub fn set_available_requested_tpmstates(&mut self, value: Vec<TPM_AvailableRequestedTPMStates>) {
        self.available_requested_tpmstates = value;
    }

    /// Gets the value of AvailableRequestedTPMStates
    pub fn get_available_requested_tpmstates(&self) -> &Vec<TPM_AvailableRequestedTPMStates> {
        &self.available_requested_tpmstates
    }

    /// Sets the value of TPMManafucturerMajorRevision
    pub fn set_tpmmanafucturer_major_revision(&mut self, value: u32) {
        self.tpmmanafucturer_major_revision = Some(value);
    }

    /// Gets the value of TPMManafucturerMajorRevision
    pub fn get_tpmmanafucturer_major_revision(&self) -> Option<&u32> {
        self.tpmmanafucturer_major_revision.as_ref()
    }

    /// Sets the value of TPMManufacturerId
    pub fn set_tpmmanufacturer_id(&mut self, value: u32) {
        self.tpmmanufacturer_id = Some(value);
    }

    /// Gets the value of TPMManufacturerId
    pub fn get_tpmmanufacturer_id(&self) -> Option<&u32> {
        self.tpmmanufacturer_id.as_ref()
    }

    /// Sets the value of TPMManufacturerInfo
    pub fn set_tpmmanufacturer_info(&mut self, value: String) {
        self.tpmmanufacturer_info = Some(value);
    }

    /// Gets the value of TPMManufacturerInfo
    pub fn get_tpmmanufacturer_info(&self) -> Option<&String> {
        self.tpmmanufacturer_info.as_ref()
    }

    /// Sets the value of TPMManufacturerMinorRevision
    pub fn set_tpmmanufacturer_minor_revision(&mut self, value: u32) {
        self.tpmmanufacturer_minor_revision = Some(value);
    }

    /// Gets the value of TPMManufacturerMinorRevision
    pub fn get_tpmmanufacturer_minor_revision(&self) -> Option<&u32> {
        self.tpmmanufacturer_minor_revision.as_ref()
    }

    /// Sets the value of TPMSpecMajorVersion
    pub fn set_tpmspec_major_version(&mut self, value: u32) {
        self.tpmspec_major_version = Some(value);
    }

    /// Gets the value of TPMSpecMajorVersion
    pub fn get_tpmspec_major_version(&self) -> Option<&u32> {
        self.tpmspec_major_version.as_ref()
    }

    /// Sets the value of TPMSpecMinorVersion
    pub fn set_tpmspec_minor_version(&mut self, value: u32) {
        self.tpmspec_minor_version = Some(value);
    }

    /// Gets the value of TPMSpecMinorVersion
    pub fn get_tpmspec_minor_version(&self) -> Option<&u32> {
        self.tpmspec_minor_version.as_ref()
    }

    /// Sets the value of TPMState
    pub fn set_tpmstate(&mut self, value: TPM_TPMState) {
        self.tpmstate = Some(value);
    }

    /// Gets the value of TPMState
    pub fn get_tpmstate(&self) -> Option<&TPM_TPMState> {
        self.tpmstate.as_ref()
    }

    /// Sets the value of TransitioningToTPMState
    pub fn set_transitioning_to_tpmstate(&mut self, value: TPM_TransitioningToTPMState) {
        self.transitioning_to_tpmstate = Some(value);
    }

    /// Gets the value of TransitioningToTPMState
    pub fn get_transitioning_to_tpmstate(&self) -> Option<&TPM_TransitioningToTPMState> {
        self.transitioning_to_tpmstate.as_ref()
    }

/// Requests that the state of the TPM be changed to the value specified in the RequestedTPMState parameter. If the method invokation completes successfuly, the TPMState property shall be equal to the RequestedTPMState parameter. Invoking the RequestTPMStateChange method multiple times could result in earlier requests being overwritten or lost. 
/// A return code of 0 shall indicate the state change was successfully initiated. 
/// A return code of 3 shall indicate that the state transition cannot complete within the interval specified by the TimeoutPeriod parameter. 
/// A return code of 4096 (0x1000) shall indicate the state change was successfully initiated, a ConcreteJob has been created, and its reference returned in the output parameter Job. Any other return code indicates an error condition.

    /// * `authorization_token` - Authorization token that may be required for the action to take effect. The AuthorizationToken parameter may be required to establish Physical Presence, or to pass the OwnerAuth, the TCG defined owner authorization password. In the case of OwnerAuth, the CIM_SharedCredential with non-null value of the CIM_SharedCredential.Secret may be required. The CIM_SharedCredential.Algorithm property may also be specified based on the property CIM_TPMCapabilities.SupportedPasswordAlgorithms. (String)
    /// * `requested_tpmstate` - The requested TPM states. (TPM_RequestedTPMState)
    /// * `timeout_period` - A timeout period that specifies the maximum amount of time that the client expects the transition to the new state to take. The interval format must be used to specify the TimeoutPeriod. A value of 0 or a null parameter indicates that the client has no time requirements for the transition. (String)

    /// * `job` - May contain a reference to the ConcreteJob created to track the state transition initiated by the method invocation. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn request_tpmstate_change(&self, requested_tpmstate: TPM_RequestedTPMState, authorization_token: &String, job: &mut CIM_ConcreteJob, timeout_period: &Option<String>, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestedTPMState".to_string(), value: requested_tpmstate.into() });
        args.push(MethodParameter { name: "AuthorizationToken".to_string(), value: authorization_token.into() });
        if let Some(val) = timeout_period {
            args.push(MethodParameter { name: "TimeoutPeriod".to_string(), value: val.into() });
        }

        let result = self.invoke_method_with_job("RequestTPMStateChange", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// This method changes the owner authorization credential of the TPM device. The old and new owner authorization passwords are required.Reference: See Section 17 (Changing AuthData) of Spec (#3).

    /// * `new_owner_auth` - NewOwnerAuth represents new owner authorization credential required to take ownership of the TPM device.The CIM_SharedCredential subclass may be required with non-null value of the CIM_SharedCredential.Secret property for the parameter. (String)
    /// * `old_owner_auth` - OldOwnerAuth represents old owner authorization credential required to take ownership of the TPM device.The CIM_SharedCredential subclass may be required with non-null value of the CIM_SharedCredential.Secret property for the parameter. (String)

    /// * `return_value` -  (u32)
    pub fn change_owner_auth(&self, old_owner_auth: &String, new_owner_auth: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "OldOwnerAuth".to_string(), value: old_owner_auth.into() });
        args.push(MethodParameter { name: "NewOwnerAuth".to_string(), value: new_owner_auth.into() });
        self.invoke_method("ChangeOwnerAuth", &args)

    }

}

