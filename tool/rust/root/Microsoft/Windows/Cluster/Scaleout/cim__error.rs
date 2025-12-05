// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Cluster.Scaleout
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Error struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Error {

/// The CIM status code that characterizes this instance. 
/// This property defines the status codes that MAY be return by a conforming CIM Server or Listener. Note that not all status codes are valid for each operation. The specification for each operation SHOULD define the status codes that may be returned by that operation. 
/// The following values for CIM status code are defined: 
/// 1 - CIM_ERR_FAILED. A general error occurred that is not covered by a more specific error code. 
/// 2 - CIM_ERR_ACCESS_DENIED. Access to a CIM resource was not available to the client. 
/// 3 - CIM_ERR_INVALID_NAMESPACE. The target namespace does not exist. 
/// 4 - CIM_ERR_INVALID_PARAMETER. One or more parameter values passed to the method were invalid. 
/// 5 - CIM_ERR_INVALID_CLASS. The specified Class does not exist. 
/// 6 - CIM_ERR_NOT_FOUND. The requested object could not be found. 
/// 7 - CIM_ERR_NOT_SUPPORTED. The requested operation is not supported. 
/// 8 - CIM_ERR_CLASS_HAS_CHILDREN. Operation cannot be carried out on this class since it has instances. 
/// 9 - CIM_ERR_CLASS_HAS_INSTANCES. Operation cannot be carried out on this class since it has instances. 
/// 10 - CIM_ERR_INVALID_SUPERCLASS. Operation cannot be carried out since the specified superclass does not exist. 
/// 11 - CIM_ERR_ALREADY_EXISTS. Operation cannot be carried out because an object already exists. 
/// 12 - CIM_ERR_NO_SUCH_PROPERTY. The specified Property does not exist. 
/// 13 - CIM_ERR_TYPE_MISMATCH. The value supplied is incompatible with the type. 
/// 14 - CIM_ERR_QUERY_LANGUAGE_NOT_SUPPORTED. The query language is not recognized or supported. 
/// 15 - CIM_ERR_INVALID_QUERY. The query is not valid for the specified query language. 
/// 16 - CIM_ERR_METHOD_NOT_AVAILABLE. The extrinsic Method could not be executed. 
/// 17 - CIM_ERR_METHOD_NOT_FOUND. The specified extrinsic Method does not exist. 
/// 18 - CIM_ERR_UNEXPECTED_RESPONSE. The returned response to the asynchronous operation was not expected. 
/// 19 - CIM_ERR_INVALID_RESPONSE_DESTINATION. The specified destination for the asynchronous response is not valid. 
/// 20 - CIM_ERR_NAMESPACE_NOT_EMPTY. The specified Namespace is not empty.
/// 21 - CIM_ERR_INVALID_ENUMERATION_CONTEXT. The enumeration context supplied is not valid.
/// 22 - CIM_ERR_INVALID_OPERATION_TIMEOUT. The specified Namespace is not empty.
/// 23 - CIM_ERR_PULL_HAS_BEEN_ABANDONED. The specified Namespace is not empty.
/// 24 - CIM_ERR_PULL_CANNOT_BE_ABANDONED. The attempt to abandon a pull operation has failed.
/// 25 - CIM_ERR_FILTERED_ENUMERATION_NOT_SUPPORTED. Filtered Enumeratrions are not supported.
/// 26 - CIM_ERR_CONTINUATION_ON_ERROR_NOT_SUPPORTED. Continue on error is not supported.
/// 27 - CIM_ERR_SERVER_LIMITS_EXCEEDED. The WBEM Server limits have been exceeded (e.g. memory, connections, ...).
/// 28 - CIM_ERR_SERVER_IS_SHUTTING_DOWN. The WBEM Server is shutting down.
/// 29 - CIM_ERR_QUERY_FEATURE_NOT_SUPPORTED. The specified Query Feature is not supported.
    #[serde(rename = "CIMStatusCode")]
    pub cimstatus_code: Option<Error_CIMStatusCode>,

/// A free-form string containing a human-readable description of CIMStatusCode. This description MAY extend, but MUST be consistent with, the definition of CIMStatusCode.
    #[serde(rename = "CIMStatusCodeDescription")]
    pub cimstatus_code_description: Option<String>,

/// The identifying information of the entity (i.e., the instance) generating the error. If this entity is modeled in the CIM Schema, this property contains the path of the instance encoded as a string parameter. If not modeled, the property contains some identifying string that names the entity that generated the error. The path or identifying string is formatted per the ErrorSourceFormat property.
    #[serde(rename = "ErrorSource")]
    pub error_source: Option<String>,

/// An array containing the dynamic content of the message.
    #[serde(rename = "ErrorSourceFormat")]
    pub error_source_format: Option<Error_ErrorSourceFormat>,

/// Primary classification of the error. The following values are defined: 
/// 2 - Communications Error. Errors of this type are principally associated with the procedures and/or processes required to convey information from one point to another. 
/// 3 - Quality of Service Error. Errors of this type are principally associated with failures that result in reduced functionality or performance. 
/// 4 - Software Error. Error of this type are principally associated with a software or processing fault. 
/// 5 - Hardware Error. Errors of this type are principally associated with an equipment or hardware failure. 
/// 6 - Environmental Error. Errors of this type are principally associated with a failure condition relating the to facility, or other environmental considerations. 
/// 7 - Security Error. Errors of this type are associated with security violations, detection of viruses, and similar issues. 
/// 8 - Oversubscription Error. Errors of this type are principally associated with the failure to allocate sufficient resources to complete the operation. 
/// 9 - Unavailable Resource Error. Errors of this type are principally associated with the failure to access a required resource. 
/// 10 -Unsupported Operation Error. Errors of this type are principally associated with requests that are not supported.
    #[serde(rename = "ErrorType")]
    pub error_type: Option<Error_ErrorType>,

/// The formatted message. This message is constructed by combining some or all of the dynamic elements specified in the MessageArguments property with the static elements uniquely identified by the MessageID in a message registry or other catalog associated with the OwningEntity.
    #[serde(rename = "Message")]
    pub message: Option<String>,

/// An array containing the dynamic content of the message.
    #[serde(rename = "MessageArguments")]
    pub message_arguments: Vec<String>,

/// An opaque string that uniquely identifies, within the scope of the OwningEntity, the format of the Message.
    #[serde(rename = "MessageID")]
    pub message_id: Option<String>,

/// A string defining "Other" values for ErrorSourceFormat. This value MUST be set to a non NULL value when ErrorSourceFormat is set to a value of 1 ("Other"). For all other values of ErrorSourceFormat, the value of this string must be set to NULL.
    #[serde(rename = "OtherErrorSourceFormat")]
    pub other_error_source_format: Option<String>,

/// A free-form string describing the ErrorType when 1, "Other", is specified as the ErrorType.
    #[serde(rename = "OtherErrorType")]
    pub other_error_type: Option<String>,

/// A string that uniquely identifies the entity that owns the definition of the format of the Message described in this instance. OwningEntity MUST include a copyrighted, trademarked or otherwise unique name that is owned by the business entity or standards body defining the format.
    #[serde(rename = "OwningEntity")]
    pub owning_entity: Option<String>,

/// An enumerated value that describes the severity of the Indication from the notifier's point of view: 
/// 0 - the Perceived Severity of the indication is unknown or indeterminate. 
/// 1 - Other, by CIM convention, is used to indicate that the Severity's value can be found in the OtherSeverity property. 
/// 2 - Information should be used when providing an informative response. 
/// 3 - Degraded/Warning should be used when its appropriate to let the user decide if action is needed. 
/// 4 - Minor should be used to indicate action is needed, but the situation is not serious at this time. 
/// 5 - Major should be used to indicate action is needed NOW. 
/// 6 - Critical should be used to indicate action is needed NOW and the scope is broad (perhaps an imminent outage to a critical resource will result). 
/// 7 - Fatal/NonRecoverable should be used to indicate an error occurred, but it's too late to take remedial action. 
/// 2 and 0 - Information and Unknown (respectively) follow common usage. Literally, the Error is purely informational or its severity is simply unknown.
    #[serde(rename = "PerceivedSeverity")]
    pub perceived_severity: Option<Error_PerceivedSeverity>,

/// An enumerated value that describes the probable cause of the error.
    #[serde(rename = "ProbableCause")]
    pub probable_cause: Option<Error_ProbableCause>,

/// A free-form string describing the probable cause of the error.
    #[serde(rename = "ProbableCauseDescription")]
    pub probable_cause_description: Option<String>,

/// A free-form string describing recommended actions to take to resolve the error.
    #[serde(rename = "RecommendedActions")]
    pub recommended_actions: Vec<String>,
}

impl CIM_Error {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cimstatus_code: None,
            cimstatus_code_description: None,
            error_source: None,
            error_source_format: None,
            error_type: None,
            message: None,
            message_arguments: Vec::new(),
            message_id: None,
            other_error_source_format: None,
            other_error_type: None,
            owning_entity: None,
            perceived_severity: None,
            probable_cause: None,
            probable_cause_description: None,
            recommended_actions: Vec::new(),
        }
    }


    /// Sets the value of CIMStatusCode
    pub fn set_cimstatus_code(&mut self, value: Error_CIMStatusCode) {
        self.cimstatus_code = Some(value);
    }

    /// Gets the value of CIMStatusCode
    pub fn get_cimstatus_code(&self) -> Option<&Error_CIMStatusCode> {
        self.cimstatus_code.as_ref()
    }

    /// Sets the value of CIMStatusCodeDescription
    pub fn set_cimstatus_code_description(&mut self, value: String) {
        self.cimstatus_code_description = Some(value);
    }

    /// Gets the value of CIMStatusCodeDescription
    pub fn get_cimstatus_code_description(&self) -> Option<&String> {
        self.cimstatus_code_description.as_ref()
    }

    /// Sets the value of ErrorSource
    pub fn set_error_source(&mut self, value: String) {
        self.error_source = Some(value);
    }

    /// Gets the value of ErrorSource
    pub fn get_error_source(&self) -> Option<&String> {
        self.error_source.as_ref()
    }

    /// Sets the value of ErrorSourceFormat
    pub fn set_error_source_format(&mut self, value: Error_ErrorSourceFormat) {
        self.error_source_format = Some(value);
    }

    /// Gets the value of ErrorSourceFormat
    pub fn get_error_source_format(&self) -> Option<&Error_ErrorSourceFormat> {
        self.error_source_format.as_ref()
    }

    /// Sets the value of ErrorType
    pub fn set_error_type(&mut self, value: Error_ErrorType) {
        self.error_type = Some(value);
    }

    /// Gets the value of ErrorType
    pub fn get_error_type(&self) -> Option<&Error_ErrorType> {
        self.error_type.as_ref()
    }

    /// Sets the value of Message
    pub fn set_message(&mut self, value: String) {
        self.message = Some(value);
    }

    /// Gets the value of Message
    pub fn get_message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    /// Sets the value of MessageArguments
    pub fn set_message_arguments(&mut self, value: Vec<String>) {
        self.message_arguments = value;
    }

    /// Gets the value of MessageArguments
    pub fn get_message_arguments(&self) -> &Vec<String> {
        &self.message_arguments
    }

    /// Sets the value of MessageID
    pub fn set_message_id(&mut self, value: String) {
        self.message_id = Some(value);
    }

    /// Gets the value of MessageID
    pub fn get_message_id(&self) -> Option<&String> {
        self.message_id.as_ref()
    }

    /// Sets the value of OtherErrorSourceFormat
    pub fn set_other_error_source_format(&mut self, value: String) {
        self.other_error_source_format = Some(value);
    }

    /// Gets the value of OtherErrorSourceFormat
    pub fn get_other_error_source_format(&self) -> Option<&String> {
        self.other_error_source_format.as_ref()
    }

    /// Sets the value of OtherErrorType
    pub fn set_other_error_type(&mut self, value: String) {
        self.other_error_type = Some(value);
    }

    /// Gets the value of OtherErrorType
    pub fn get_other_error_type(&self) -> Option<&String> {
        self.other_error_type.as_ref()
    }

    /// Sets the value of OwningEntity
    pub fn set_owning_entity(&mut self, value: String) {
        self.owning_entity = Some(value);
    }

    /// Gets the value of OwningEntity
    pub fn get_owning_entity(&self) -> Option<&String> {
        self.owning_entity.as_ref()
    }

    /// Sets the value of PerceivedSeverity
    pub fn set_perceived_severity(&mut self, value: Error_PerceivedSeverity) {
        self.perceived_severity = Some(value);
    }

    /// Gets the value of PerceivedSeverity
    pub fn get_perceived_severity(&self) -> Option<&Error_PerceivedSeverity> {
        self.perceived_severity.as_ref()
    }

    /// Sets the value of ProbableCause
    pub fn set_probable_cause(&mut self, value: Error_ProbableCause) {
        self.probable_cause = Some(value);
    }

    /// Gets the value of ProbableCause
    pub fn get_probable_cause(&self) -> Option<&Error_ProbableCause> {
        self.probable_cause.as_ref()
    }

    /// Sets the value of ProbableCauseDescription
    pub fn set_probable_cause_description(&mut self, value: String) {
        self.probable_cause_description = Some(value);
    }

    /// Gets the value of ProbableCauseDescription
    pub fn get_probable_cause_description(&self) -> Option<&String> {
        self.probable_cause_description.as_ref()
    }

    /// Sets the value of RecommendedActions
    pub fn set_recommended_actions(&mut self, value: Vec<String>) {
        self.recommended_actions = value;
    }

    /// Gets the value of RecommendedActions
    pub fn get_recommended_actions(&self) -> &Vec<String> {
        &self.recommended_actions
    }
}

