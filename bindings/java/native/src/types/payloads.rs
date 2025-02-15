// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::{
    types::{index::*, milestone::*, receipt::*, transaction::*, treasury::*},
    Result,
};

use iota_wallet::message::MessagePayload as MessagePayloadRust;

pub enum MessagePayloadType {
    Transaction = 1,
    Milestone = 2,
    Indexation = 3,
    Receipt = 4,
    TreasuryTransaction = 5,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct MessagePayload {
    payload: MessagePayloadRust,
}

impl From<MessagePayloadRust> for MessagePayload {
    fn from(payload: MessagePayloadRust) -> Self {
        Self { payload }
    }
}

impl MessagePayload {
    pub fn deserialize(serialised_data: &str) -> Result<MessagePayload> {
        let res = serde_json::from_str(&serialised_data);

        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    pub fn serialize(&self) -> Result<String> {
        let res = serde_json::to_string(self);

        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    pub fn to_inner(self) -> MessagePayloadRust {
        self.payload
    }

    pub fn payload_type(&self) -> MessagePayloadType {
        match self.payload {
            MessagePayloadRust::Transaction(_) => MessagePayloadType::Transaction,
            MessagePayloadRust::Milestone(_) => MessagePayloadType::Milestone,
            MessagePayloadRust::Indexation(_) => MessagePayloadType::Indexation,
            MessagePayloadRust::Receipt(_) => MessagePayloadType::Receipt,
            MessagePayloadRust::TreasuryTransaction(_) => MessagePayloadType::TreasuryTransaction,
        }
    }

    pub fn as_transaction(&self) -> Result<TransactionPayload> {
        if let MessagePayloadRust::Transaction(payload) = &self.payload {
            Ok(payload.into())
        } else {
            Err(anyhow::anyhow!("Message is not of type Transaction"))
        }
    }

    pub fn as_indexation(&self) -> Result<IndexationPayload> {
        if let MessagePayloadRust::Indexation(index) = &self.payload {
            IndexationPayload::new(index.index(), index.data())
        } else {
            Err(anyhow::anyhow!("Message is not of type Indexation"))
        }
    }

    pub fn as_milestone(&self) -> Result<MilestonePayload> {
        if let MessagePayloadRust::Milestone(payload) = &self.payload {
            Ok(MilestonePayload::new(
                payload.essence().to_owned(),
                payload.signatures().to_owned(),
            ))
        } else {
            Err(anyhow::anyhow!("Message is not of type Milestone"))
        }
    }

    pub fn as_receipt(&self) -> Result<ReceiptPayload> {
        if let MessagePayloadRust::Receipt(payload) = &self.payload {
            Ok((*payload.clone()).into())
        } else {
            Err(anyhow::anyhow!("Message is not of type Receipt"))
        }
    }

    pub fn as_treasury(&self) -> Result<TreasuryPayload> {
        if let MessagePayloadRust::TreasuryTransaction(payload) = &self.payload {
            Ok((*payload.clone()).into())
        } else {
            Err(anyhow::anyhow!("Message is not of type Treasury"))
        }
    }
}

impl core::fmt::Display for MessagePayload {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self.payload)
    }
}
