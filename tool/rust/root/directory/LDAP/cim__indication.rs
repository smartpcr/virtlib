// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.directory.LDAP
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Indication struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Indication {

/// A list of IndicationIdentifiers whose notifications are correlated with (related to) this one.
    #[serde(rename = "CorrelatedIndications")]
    pub correlated_indications: Vec<String>,

/// An identifier for the indication filter that selects this indication and causes it to be sent. This property is to be filled out by the indication sending service. The value shall be correlatable with the Name property of the instance of CIM_IndicationFilter describing the criteria of the indication. The value of the IndicationFilterName should be formatted using the following algorithm: < OrgID > : < LocalID >, where < OrgID > and < LocalID > are separated by a colon (:) and < OrgID > shall include a copyrighted, trademarked, or otherwise unique name that is owned by the business entity that is creating or defining the value or that is a registered ID assigned to the business entity by a recognized global authority. In addition, to ensure uniqueness, < OrgID > shall not contain a colon (:).When using this algorithm, the first colon to appear in the value shall appear between < OrgID > and < LocalID >. < LocalID > is chosen by the business entity and shall be used uniquely.
    #[serde(rename = "IndicationFilterName")]
    pub indication_filter_name: Option<String>,

/// An identifier for the Indication. This property is similar to a key value in that it can be used for identification, when correlating Indications (see the CorrelatedIndications array). Its value SHOULD be unique as long as correlations are reported, but MAY be reused or left NULL if no future Indications will reference it in their CorrelatedIndications array.To ensure uniqueness, the value of IndicationIdentifier should be constructed using the following "preferred" algorithm: 
/// <OrgID>:<LocalID> 
/// Where <OrgID> and <LocalID> are separated by a colon (:), and where <OrgID> must include a copyrighted, trademarked, or otherwise unique name that is owned by the business entity that is creating or defining the IndicationIdentifier or that is a recognized ID that is assigned to the business entity by a recognized global authority. (This requirement is similar to the <Schema Name>_<Class Name> structure of Schema class names.) In addition, to ensure uniqueness <OrgID> must not contain a colon (:). When using this algorithm, the first colon to appear in IndicationIdentifier must appear between <OrgID> and <LocalID>. 
/// <LocalID> is chosen by the business entity and should not be re-used to identify different underlying (real-world) elements. 
/// If the above "preferred" algorithm is not used, the defining entity should assure that the resulting IndicationIdentifier is not re-used across any IndicationIdentifiers that are produced by this or other providers for the NameSpace of this instance. 
/// For DMTF-defined instances, the "preferred" algorithm should be used with the <OrgID> set to CIM.
    #[serde(rename = "IndicationIdentifier")]
    pub indication_identifier: Option<String>,

/// The time and date of creation of the Indication. The property may be set to NULL if the entity creating the Indication is not capable of determining this information. Note that IndicationTime may be the same for two Indications that are generated in rapid succession.
    #[serde(rename = "IndicationTime")]
    pub indication_time: Option<String>,

/// Holds the value of the user defined severity value when 'PerceivedSeverity' is 1 ("Other").
    #[serde(rename = "OtherSeverity")]
    pub other_severity: Option<String>,

/// An enumerated value that describes the severity of the Indication from the notifier's point of view: 
/// 1 - Other, by CIM convention, is used to indicate that the Severity's value can be found in the OtherSeverity property. 
/// 3 - Degraded/Warning should be used when its appropriate to let the user decide if action is needed. 
/// 4 - Minor should be used to indicate action is needed, but the situation is not serious at this time. 
/// 5 - Major should be used to indicate action is needed NOW. 
/// 6 - Critical should be used to indicate action is needed NOW and the scope is broad (perhaps an imminent outage to a critical resource will result). 
/// 7 - Fatal/NonRecoverable should be used to indicate an error occurred, but it's too late to take remedial action. 
/// 2 and 0 - Information and Unknown (respectively) follow common usage. Literally, the Indication is purely informational or its severity is simply unknown.
    #[serde(rename = "PerceivedSeverity")]
    pub perceived_severity: Option<Indication_PerceivedSeverity>,

/// The sequence context portion of a sequence identifier for the indication. The sequence number portion of the sequence identifier is provided by the SequenceNumber property. The combination of both property values represents the sequence identifier for the indication.
/// The sequence identifier for the indication enables a CIM listener to identify duplicate indications when the CIM service attempts the delivery retry of indications, to reorder indications that arrive out-of-order, and to detect lost indications.
/// If a CIM service does not support sequence identifiers for indications, this property shall be NULL.
/// If a CIM service supports sequence identifiers for indications, this property shall be maintained by the CIM service for each registered listener destination, and its value shall uniquely identify the CIM service and the indication service within the CIM service such that restarts of the CIM service and deregistration of listener destinations to the CIM service cause the value to change, without reusing earlier values for a sufficiently long time.
/// When retrying the delivery of an indication, this property shall have the same value as in the original delivery.
/// To guarantee this uniqueness, the property value should be constructed using the following format (defined in ABNF): sequence-context = indication-service-name "#" cim-service-start-id "#" listener-destination-creation-time
/// Where: indication-service-name is the value of the Name property of the CIM_IndicationService instance responsible for delivering the indication. cim-service-start-id is an identifier that uniquely identifies the CIM service start, for example via a timestamp of the start time, or via a counter that increases for each start or restart. listener-destination-creation-time is a timestamp of the creation time of the CIM_ListenerDestination instance representing the listener destination.
/// Since this format is only a recommendation, CIM clients shall treat the value as an opaque identifier for the sequence context and shall not rely on this format.
    #[serde(rename = "SequenceContext")]
    pub sequence_context: Option<String>,

/// The sequence number portion of a sequence identifier for the indication. The sequence context portion of the sequence identifier is provided by the SequenceContext property. The combination of both property values represents the sequence identifier for the indication.
/// The sequence identifier for the indication enables a CIM listener to identify duplicate indications when the CIM service attempts the delivery retry of indications, to reorder indications that arrive out-of-order, and to detect lost indications.
/// If a CIM service does not support sequence identifiers for indications, this property shall be NULL.
/// If a CIM service supports sequence identifiers for indications, this property shall be maintained by the CIM service for each registered listener destination, and its value shall uniquely identify the indication within the sequence context provided by SequenceContext. It shall start at 0 whenever the sequence context string changes. Otherwise, it shall be increased by 1 for every new indication to that listener destination, and it shall wrap to 0 when the value range is exceeded.
/// When retrying the delivery of an indication, this property shall have the same value as in the original delivery.
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: Option<i64>,
}

impl CIM_Indication {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            correlated_indications: Vec::new(),
            indication_filter_name: None,
            indication_identifier: None,
            indication_time: None,
            other_severity: None,
            perceived_severity: None,
            sequence_context: None,
            sequence_number: None,
        }
    }


    /// Sets the value of CorrelatedIndications
    pub fn set_correlated_indications(&mut self, value: Vec<String>) {
        self.correlated_indications = value;
    }

    /// Gets the value of CorrelatedIndications
    pub fn get_correlated_indications(&self) -> &Vec<String> {
        &self.correlated_indications
    }

    /// Sets the value of IndicationFilterName
    pub fn set_indication_filter_name(&mut self, value: String) {
        self.indication_filter_name = Some(value);
    }

    /// Gets the value of IndicationFilterName
    pub fn get_indication_filter_name(&self) -> Option<&String> {
        self.indication_filter_name.as_ref()
    }

    /// Sets the value of IndicationIdentifier
    pub fn set_indication_identifier(&mut self, value: String) {
        self.indication_identifier = Some(value);
    }

    /// Gets the value of IndicationIdentifier
    pub fn get_indication_identifier(&self) -> Option<&String> {
        self.indication_identifier.as_ref()
    }

    /// Sets the value of IndicationTime
    pub fn set_indication_time(&mut self, value: String) {
        self.indication_time = Some(value);
    }

    /// Gets the value of IndicationTime
    pub fn get_indication_time(&self) -> Option<&String> {
        self.indication_time.as_ref()
    }

    /// Sets the value of OtherSeverity
    pub fn set_other_severity(&mut self, value: String) {
        self.other_severity = Some(value);
    }

    /// Gets the value of OtherSeverity
    pub fn get_other_severity(&self) -> Option<&String> {
        self.other_severity.as_ref()
    }

    /// Sets the value of PerceivedSeverity
    pub fn set_perceived_severity(&mut self, value: Indication_PerceivedSeverity) {
        self.perceived_severity = Some(value);
    }

    /// Gets the value of PerceivedSeverity
    pub fn get_perceived_severity(&self) -> Option<&Indication_PerceivedSeverity> {
        self.perceived_severity.as_ref()
    }

    /// Sets the value of SequenceContext
    pub fn set_sequence_context(&mut self, value: String) {
        self.sequence_context = Some(value);
    }

    /// Gets the value of SequenceContext
    pub fn get_sequence_context(&self) -> Option<&String> {
        self.sequence_context.as_ref()
    }

    /// Sets the value of SequenceNumber
    pub fn set_sequence_number(&mut self, value: i64) {
        self.sequence_number = Some(value);
    }

    /// Gets the value of SequenceNumber
    pub fn get_sequence_number(&self) -> Option<&i64> {
        self.sequence_number.as_ref()
    }
}

