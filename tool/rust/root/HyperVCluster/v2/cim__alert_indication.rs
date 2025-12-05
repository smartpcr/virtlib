// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_AlertIndication struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_AlertIndication {
    #[serde(flatten)]
    pub base: CIM_ProcessIndication,

/// The format of the AlertingManagedElement property is interpretable based upon the value of this property. Values are defined as: 
/// 0 - Unknown. The format is unknown or not meaningfully interpretable by a CIM client application. 
/// 1 - Other. The format is defined by the value of the OtherAlertingElementFormat property. 
/// 2 - CIMObjectPath. The format is a CIMObjectPath, with format <NamespacePath>:<ClassName>.<Prop1>="<Value1>", <Prop2>="<Value2>", . . . specifying an instance in the CIM Schema.
    #[serde(rename = "AlertingElementFormat")]
    pub alerting_element_format: Option<AlertIndication_AlertingElementFormat>,

/// The identifying information of the entity (ie, the instance) for which this Indication is generated. The property contains the path of an instance, encoded as a string parameter - if the instance is modeled in the CIM Schema. If not a CIM instance, the property contains some identifying string that names the entity for which the Alert is generated. The path or identifying string is formatted per the AlertingElementFormat property.
    #[serde(rename = "AlertingManagedElement")]
    pub alerting_managed_element: Option<String>,

/// Primary classification of the Indication. The following values are defined: 
/// 1 - Other. The Indication's OtherAlertType property conveys its classification. Use of "Other" in an enumeration is a standard CIM convention. It means that the current Indication does not fit into the categories described by this enumeration. 
/// 2 - Communications Alert. An Indication of this type is principally associated with the procedures and/or processes required to convey information from one point to another. 
/// 3 - Quality of Service Alert. An Indication of this type is principally associated with a degradation or errors in the performance or function of an entity. 
/// 4 - Processing Error. An Indication of this type is principally associated with a software or processing fault. 
/// 5 - Device Alert. An Indication of this type is principally associated with an equipment or hardware fault. 
/// 6 - Environmental Alert. An Indication of this type is principally associated with a condition relating to an enclosure in which the hardware resides, or other environmental considerations. 
/// 7 - Model Change. The Indication addresses changes in the Information Model. For example, it may embed a Lifecycle Indication to convey the specific model change being alerted. 
/// 8 - Security Alert. An Indication of this type is associated with security violations, detection of viruses, and similar issues.
    #[serde(rename = "AlertType")]
    pub alert_type: Option<AlertIndication_AlertType>,

/// A short description of the Indication.
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// An instrumentation or provider specific value that describes the underlying "real-world" event represented by the Indication. Two Indications with the same, non NULL EventID value are considered, by the creating entity, to represent the same event. The comparison of two EventID values is only defined for Alert Indications with identical, non NULL values of SystemCreateClassName, SystemName and ProviderName.
    #[serde(rename = "EventID")]
    pub event_id: Option<String>,

/// The time and date the underlying event was first detected. If specified, this property MUST be set to NULL if the creating entity is not capable of providing this information. This value is based on the notion of local date and time of the Managed System Element generating the Indication.
    #[serde(rename = "EventTime")]
    pub event_time: Option<String>,

/// The formatted message. This message is constructed by combining some or all of the dynamic elements specified in the MessageArguments property with the static elements uniquely identified by the MessageID in a message registry or other catalog associated with the OwningEntity.
    #[serde(rename = "Message")]
    pub message: Option<String>,

/// An array containing the dynamic content of the message.
    #[serde(rename = "MessageArguments")]
    pub message_arguments: Vec<String>,

/// A string that uniquely identifies, within the scope of the OwningEntity, the format of the Message.
    #[serde(rename = "MessageID")]
    pub message_id: Option<String>,

/// A string defining "Other" values for AlertingElementFormat. This value MUST be set to a non NULL value when AlertingElementFormat is set to a value of 1 ("Other"). For all other values of AlertingElementFormat, the value of this string must be set to NULL.
    #[serde(rename = "OtherAlertingElementFormat")]
    pub other_alerting_element_format: Option<String>,

/// A string describing the Alert type - used when the AlertType property is set to 1, "Other State Change".
    #[serde(rename = "OtherAlertType")]
    pub other_alert_type: Option<String>,

/// A string that uniquely identifies the entity that owns the definition of the format of the Message described in this instance. OwningEntity MUST include a copyrighted, trademarked or otherwise unique name that is owned by the business entity or standards body defining the format.
    #[serde(rename = "OwningEntity")]
    pub owning_entity: Option<String>,

/// An enumerated value that describes the probable cause of the situation which resulted in the AlertIndication.
    #[serde(rename = "ProbableCause")]
    pub probable_cause: Option<AlertIndication_ProbableCause>,

/// Provides additional information related to the ProbableCause.
    #[serde(rename = "ProbableCauseDescription")]
    pub probable_cause_description: Option<String>,

/// The name of the Provider generating this Indication.
    #[serde(rename = "ProviderName")]
    pub provider_name: Option<String>,

/// Free form descriptions of the recommended actions to take to resolve the cause of the notification.
    #[serde(rename = "RecommendedActions")]
    pub recommended_actions: Vec<String>,

/// The scoping System's CreationClassName for the Provider generating this Indication.
    #[serde(rename = "SystemCreationClassName")]
    pub system_creation_class_name: Option<String>,

/// The scoping System's Name for the Provider generating this Indication.
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,

/// Provides information on trending - trending up, down or no change.
    #[serde(rename = "Trending")]
    pub trending: Option<AlertIndication_Trending>,
}

impl CIM_AlertIndication {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ProcessIndication::new(),
            alerting_element_format: None,
            alerting_managed_element: None,
            alert_type: None,
            description: None,
            event_id: None,
            event_time: None,
            message: None,
            message_arguments: Vec::new(),
            message_id: None,
            other_alerting_element_format: None,
            other_alert_type: None,
            owning_entity: None,
            probable_cause: None,
            probable_cause_description: None,
            provider_name: None,
            recommended_actions: Vec::new(),
            system_creation_class_name: None,
            system_name: None,
            trending: None,
        }
    }


    /// Sets the value of AlertingElementFormat
    pub fn set_alerting_element_format(&mut self, value: AlertIndication_AlertingElementFormat) {
        self.alerting_element_format = Some(value);
    }

    /// Gets the value of AlertingElementFormat
    pub fn get_alerting_element_format(&self) -> Option<&AlertIndication_AlertingElementFormat> {
        self.alerting_element_format.as_ref()
    }

    /// Sets the value of AlertingManagedElement
    pub fn set_alerting_managed_element(&mut self, value: String) {
        self.alerting_managed_element = Some(value);
    }

    /// Gets the value of AlertingManagedElement
    pub fn get_alerting_managed_element(&self) -> Option<&String> {
        self.alerting_managed_element.as_ref()
    }

    /// Sets the value of AlertType
    pub fn set_alert_type(&mut self, value: AlertIndication_AlertType) {
        self.alert_type = Some(value);
    }

    /// Gets the value of AlertType
    pub fn get_alert_type(&self) -> Option<&AlertIndication_AlertType> {
        self.alert_type.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of EventID
    pub fn set_event_id(&mut self, value: String) {
        self.event_id = Some(value);
    }

    /// Gets the value of EventID
    pub fn get_event_id(&self) -> Option<&String> {
        self.event_id.as_ref()
    }

    /// Sets the value of EventTime
    pub fn set_event_time(&mut self, value: String) {
        self.event_time = Some(value);
    }

    /// Gets the value of EventTime
    pub fn get_event_time(&self) -> Option<&String> {
        self.event_time.as_ref()
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

    /// Sets the value of OtherAlertingElementFormat
    pub fn set_other_alerting_element_format(&mut self, value: String) {
        self.other_alerting_element_format = Some(value);
    }

    /// Gets the value of OtherAlertingElementFormat
    pub fn get_other_alerting_element_format(&self) -> Option<&String> {
        self.other_alerting_element_format.as_ref()
    }

    /// Sets the value of OtherAlertType
    pub fn set_other_alert_type(&mut self, value: String) {
        self.other_alert_type = Some(value);
    }

    /// Gets the value of OtherAlertType
    pub fn get_other_alert_type(&self) -> Option<&String> {
        self.other_alert_type.as_ref()
    }

    /// Sets the value of OwningEntity
    pub fn set_owning_entity(&mut self, value: String) {
        self.owning_entity = Some(value);
    }

    /// Gets the value of OwningEntity
    pub fn get_owning_entity(&self) -> Option<&String> {
        self.owning_entity.as_ref()
    }

    /// Sets the value of ProbableCause
    pub fn set_probable_cause(&mut self, value: AlertIndication_ProbableCause) {
        self.probable_cause = Some(value);
    }

    /// Gets the value of ProbableCause
    pub fn get_probable_cause(&self) -> Option<&AlertIndication_ProbableCause> {
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

    /// Sets the value of ProviderName
    pub fn set_provider_name(&mut self, value: String) {
        self.provider_name = Some(value);
    }

    /// Gets the value of ProviderName
    pub fn get_provider_name(&self) -> Option<&String> {
        self.provider_name.as_ref()
    }

    /// Sets the value of RecommendedActions
    pub fn set_recommended_actions(&mut self, value: Vec<String>) {
        self.recommended_actions = value;
    }

    /// Gets the value of RecommendedActions
    pub fn get_recommended_actions(&self) -> &Vec<String> {
        &self.recommended_actions
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

    /// Sets the value of Trending
    pub fn set_trending(&mut self, value: AlertIndication_Trending) {
        self.trending = Some(value);
    }

    /// Gets the value of Trending
    pub fn get_trending(&self) -> Option<&AlertIndication_Trending> {
        self.trending.as_ref()
    }
}

