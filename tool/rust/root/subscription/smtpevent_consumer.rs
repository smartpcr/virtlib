// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.subscription
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SMTPEventConsumer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SMTPEventConsumer {
    #[serde(flatten)]
    pub base: __EventConsumer,

/// 
    #[serde(rename = "BccLine")]
    pub bcc_line: Option<String>,

/// 
    #[serde(rename = "CcLine")]
    pub cc_line: Option<String>,

/// 
    #[serde(rename = "FromLine")]
    pub from_line: Option<String>,

/// 
    #[serde(rename = "HeaderFields")]
    pub header_fields: Vec<String>,

/// 
    #[serde(rename = "Message")]
    pub message: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ReplyToLine")]
    pub reply_to_line: Option<String>,

/// 
    #[serde(rename = "SMTPServer")]
    pub smtpserver: Option<String>,

/// 
    #[serde(rename = "Subject")]
    pub subject: Option<String>,

/// 
    #[serde(rename = "ToLine")]
    pub to_line: Option<String>,
}

impl SMTPEventConsumer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __EventConsumer::new(),
            bcc_line: None,
            cc_line: None,
            from_line: None,
            header_fields: Vec::new(),
            message: None,
            name: None,
            reply_to_line: None,
            smtpserver: None,
            subject: None,
            to_line: None,
        }
    }


    /// Sets the value of BccLine
    pub fn set_bcc_line(&mut self, value: String) {
        self.bcc_line = Some(value);
    }

    /// Gets the value of BccLine
    pub fn get_bcc_line(&self) -> Option<&String> {
        self.bcc_line.as_ref()
    }

    /// Sets the value of CcLine
    pub fn set_cc_line(&mut self, value: String) {
        self.cc_line = Some(value);
    }

    /// Gets the value of CcLine
    pub fn get_cc_line(&self) -> Option<&String> {
        self.cc_line.as_ref()
    }

    /// Sets the value of FromLine
    pub fn set_from_line(&mut self, value: String) {
        self.from_line = Some(value);
    }

    /// Gets the value of FromLine
    pub fn get_from_line(&self) -> Option<&String> {
        self.from_line.as_ref()
    }

    /// Sets the value of HeaderFields
    pub fn set_header_fields(&mut self, value: Vec<String>) {
        self.header_fields = value;
    }

    /// Gets the value of HeaderFields
    pub fn get_header_fields(&self) -> &Vec<String> {
        &self.header_fields
    }

    /// Sets the value of Message
    pub fn set_message(&mut self, value: String) {
        self.message = Some(value);
    }

    /// Gets the value of Message
    pub fn get_message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ReplyToLine
    pub fn set_reply_to_line(&mut self, value: String) {
        self.reply_to_line = Some(value);
    }

    /// Gets the value of ReplyToLine
    pub fn get_reply_to_line(&self) -> Option<&String> {
        self.reply_to_line.as_ref()
    }

    /// Sets the value of SMTPServer
    pub fn set_smtpserver(&mut self, value: String) {
        self.smtpserver = Some(value);
    }

    /// Gets the value of SMTPServer
    pub fn get_smtpserver(&self) -> Option<&String> {
        self.smtpserver.as_ref()
    }

    /// Sets the value of Subject
    pub fn set_subject(&mut self, value: String) {
        self.subject = Some(value);
    }

    /// Gets the value of Subject
    pub fn get_subject(&self) -> Option<&String> {
        self.subject.as_ref()
    }

    /// Sets the value of ToLine
    pub fn set_to_line(&mut self, value: String) {
        self.to_line = Some(value);
    }

    /// Gets the value of ToLine
    pub fn get_to_line(&self) -> Option<&String> {
        self.to_line.as_ref()
    }
}

