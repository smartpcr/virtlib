// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_EnabledLogicalElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_EnabledLogicalElement {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// AvailableRequestedStates indicates the possible values for the RequestedState parameter of the method RequestStateChange, used to initiate a state change. The values listed shall be a subset of the values contained in the RequestedStatesSupported property of the associated instance of CIM_EnabledLogicalElementCapabilities where the values selected are a function of the current state of the CIM_EnabledLogicalElement. This property may be non-null if an implementation is able to advertise the set of possible values as a function of the current state. This property shall be null if an implementation is unable to determine the set of possible values as a function of the current state.
    #[serde(rename = "AvailableRequestedStates")]
    pub available_requested_states: Vec<EnabledLogicalElement_AvailableRequestedStates>,

/// An enumerated value indicating an administrator's default or startup configuration for the Enabled State of an element. By default, the element is "Enabled" (value=2).
    #[serde(rename = "EnabledDefault")]
    pub enabled_default: Option<EnabledLogicalElement_EnabledDefault>,

/// EnabledState is an integer enumeration that indicates the enabled and disabled states of an element. It can also indicate the transitions between these requested states. For example, shutting down (value=4) and starting (value=10) are transient states between enabled and disabled. The following text briefly summarizes the various enabled and disabled states: 
/// Enabled (2) indicates that the element is or could be executing commands, will process any queued commands, and queues new requests. 
/// Disabled (3) indicates that the element will not execute commands and will drop any new requests. 
/// Shutting Down (4) indicates that the element is in the process of going to a Disabled state. 
/// Not Applicable (5) indicates the element does not support being enabled or disabled. 
/// Enabled but Offline (6) indicates that the element might be completing commands, and will drop any new requests. 
/// Test (7) indicates that the element is in a test state. 
/// Deferred (8) indicates that the element might be completing commands, but will queue any new requests. 
/// Quiesce (9) indicates that the element is enabled but in a restricted mode.
/// Starting (10) indicates that the element is in the process of going to an Enabled state. New requests are queued.
    #[serde(rename = "EnabledState")]
    pub enabled_state: Option<EnabledLogicalElement_EnabledState>,

/// A string that describes the enabled or disabled state of the element when the EnabledState property is set to 1 ("Other"). This property must be set to null when EnabledState is any value other than 1.
    #[serde(rename = "OtherEnabledState")]
    pub other_enabled_state: Option<String>,

/// RequestedState is an integer enumeration that indicates the last requested or desired state for the element, irrespective of the mechanism through which it was requested. The actual state of the element is represented by EnabledState. This property is provided to compare the last requested and current enabled or disabled states. Note that when EnabledState is set to 5 ("Not Applicable"), then this property has no meaning. Refer to the EnabledState property description for explanations of the values in the RequestedState enumeration. 
/// "Unknown" (0) indicates the last requested state for the element is unknown.
/// Note that the value "No Change" (5) has been deprecated in lieu of indicating the last requested state is "Unknown" (0). If the last requested or desired state is unknown, RequestedState should have the value "Unknown" (0), but may have the value "No Change" (5).Offline (6) indicates that the element has been requested to transition to the Enabled but Offline EnabledState. 
/// It should be noted that there are two new values in RequestedState that build on the statuses of EnabledState. These are "Reboot" (10) and "Reset" (11). Reboot refers to doing a "Shut Down" and then moving to an "Enabled" state. Reset indicates that the element is first "Disabled" and then "Enabled". The distinction between requesting "Shut Down" and "Disabled" should also be noted. Shut Down requests an orderly transition to the Disabled state, and might involve removing power, to completely erase any existing state. The Disabled state requests an immediate disabling of the element, such that it will not execute or accept any commands or processing requests. 
/// 
/// This property is set as the result of a method invocation (such as Start or StopService on CIM_Service), or can be overridden and defined as WRITEable in a subclass. The method approach is considered superior to a WRITEable property, because it allows an explicit invocation of the operation and the return of a result code. 
/// 
/// If knowledge of the last RequestedState is not supported for the EnabledLogicalElement, the property shall be NULL or have the value 12 "Not Applicable".
    #[serde(rename = "RequestedState")]
    pub requested_state: Option<EnabledLogicalElement_RequestedState>,

/// The date or time when the EnabledState of the element last changed. If the state of the element has not changed and this property is populated, then it must be set to a 0 interval value. If a state change was requested, but rejected or not yet processed, the property must not be updated.
    #[serde(rename = "TimeOfLastStateChange")]
    pub time_of_last_state_change: Option<String>,

/// TransitioningToState indicates the target state to which the instance is transitioning. 
/// A value of 5 "No Change" shall indicate that no transition is in progress.A value of 12 "Not Applicable" shall indicate the implementation does not support representing ongoing transitions. 
/// A value other than 5 or 12 shall identify the state to which the element is in the process of transitioning.
    #[serde(rename = "TransitioningToState")]
    pub transitioning_to_state: Option<EnabledLogicalElement_TransitioningToState>,
}

impl CIM_EnabledLogicalElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            available_requested_states: Vec::new(),
            enabled_default: None,
            enabled_state: None,
            other_enabled_state: None,
            requested_state: None,
            time_of_last_state_change: None,
            transitioning_to_state: None,
        }
    }


    /// Sets the value of AvailableRequestedStates
    pub fn set_available_requested_states(&mut self, value: Vec<EnabledLogicalElement_AvailableRequestedStates>) {
        self.available_requested_states = value;
    }

    /// Gets the value of AvailableRequestedStates
    pub fn get_available_requested_states(&self) -> &Vec<EnabledLogicalElement_AvailableRequestedStates> {
        &self.available_requested_states
    }

    /// Sets the value of EnabledDefault
    pub fn set_enabled_default(&mut self, value: EnabledLogicalElement_EnabledDefault) {
        self.enabled_default = Some(value);
    }

    /// Gets the value of EnabledDefault
    pub fn get_enabled_default(&self) -> Option<&EnabledLogicalElement_EnabledDefault> {
        self.enabled_default.as_ref()
    }

    /// Sets the value of EnabledState
    pub fn set_enabled_state(&mut self, value: EnabledLogicalElement_EnabledState) {
        self.enabled_state = Some(value);
    }

    /// Gets the value of EnabledState
    pub fn get_enabled_state(&self) -> Option<&EnabledLogicalElement_EnabledState> {
        self.enabled_state.as_ref()
    }

    /// Sets the value of OtherEnabledState
    pub fn set_other_enabled_state(&mut self, value: String) {
        self.other_enabled_state = Some(value);
    }

    /// Gets the value of OtherEnabledState
    pub fn get_other_enabled_state(&self) -> Option<&String> {
        self.other_enabled_state.as_ref()
    }

    /// Sets the value of RequestedState
    pub fn set_requested_state(&mut self, value: EnabledLogicalElement_RequestedState) {
        self.requested_state = Some(value);
    }

    /// Gets the value of RequestedState
    pub fn get_requested_state(&self) -> Option<&EnabledLogicalElement_RequestedState> {
        self.requested_state.as_ref()
    }

    /// Sets the value of TimeOfLastStateChange
    pub fn set_time_of_last_state_change(&mut self, value: String) {
        self.time_of_last_state_change = Some(value);
    }

    /// Gets the value of TimeOfLastStateChange
    pub fn get_time_of_last_state_change(&self) -> Option<&String> {
        self.time_of_last_state_change.as_ref()
    }

    /// Sets the value of TransitioningToState
    pub fn set_transitioning_to_state(&mut self, value: EnabledLogicalElement_TransitioningToState) {
        self.transitioning_to_state = Some(value);
    }

    /// Gets the value of TransitioningToState
    pub fn get_transitioning_to_state(&self) -> Option<&EnabledLogicalElement_TransitioningToState> {
        self.transitioning_to_state.as_ref()
    }

/// Requests that the state of the element be changed to the value specified in the RequestedState parameter. When the requested state change takes place, the EnabledState and RequestedState of the element will be the same. Invoking the RequestStateChange method multiple times could result in earlier requests being overwritten or lost. 
/// A return code of 0 shall indicate the state change was successfully initiated. 
/// A return code of 3 shall indicate that the state transition cannot complete within the interval specified by the TimeoutPeriod parameter. 
/// A return code of 4096 (0x1000) shall indicate the state change was successfully initiated, a ConcreteJob has been created, and its reference returned in the output parameter Job. Any other return code indicates an error condition.

    /// * `requested_state` - The state requested for the element. This information will be placed into the RequestedState property of the instance if the return code of the RequestStateChange method is 0 ('Completed with No Error'), or 4096 (0x1000) ('Job Started'). Refer to the description of the EnabledState and RequestedState properties for the detailed explanations of the RequestedState values. (EnabledLogicalElement_RequestedState)
    /// * `timeout_period` - A timeout period that specifies the maximum amount of time that the client expects the transition to the new state to take. The interval format must be used to specify the TimeoutPeriod. A value of 0 or a null parameter indicates that the client has no time requirements for the transition.  If this property does not contain 0 or null and the implementation does not support this parameter, a return code of 'Use Of Timeout Parameter Not Supported' shall be returned. (String)

    /// * `job` - May contain a reference to the ConcreteJob created to track the state transition initiated by the method invocation. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn request_state_change(&self, requested_state: EnabledLogicalElement_RequestedState, job: &mut CIM_ConcreteJob, timeout_period: &Option<String>, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestedState".to_string(), value: requested_state.into() });
        if let Some(val) = timeout_period {
            args.push(MethodParameter { name: "TimeoutPeriod".to_string(), value: val.into() });
        }

        let result = self.invoke_method_with_job("RequestStateChange", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

