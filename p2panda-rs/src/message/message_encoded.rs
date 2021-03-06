use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use crate::hash::Hash;
use crate::message::{Message, MessageEncodedError};
#[cfg(not(target_arch = "wasm32"))]
use crate::schema::{validate_schema, MESSAGE_SCHEMA};
use crate::Validate;

/// Message represented in hex encoded CBOR format.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db-sqlx",
    derive(sqlx::Type, sqlx::FromRow),
    sqlx(transparent)
)]
pub struct MessageEncoded(String);

impl MessageEncoded {
    /// Validates and wraps encoded message string into a new `MessageEncoded` instance.
    pub fn new(value: &str) -> Result<MessageEncoded, MessageEncodedError> {
        let inner = Self(value.to_owned());
        inner.validate()?;
        Ok(inner)
    }

    /// Returns the hash of this message.
    pub fn hash(&self) -> Hash {
        // Unwrap as we already know that the inner value is valid
        Hash::new_from_bytes(self.to_bytes()).unwrap()
    }

    /// Returns encoded message as string.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Decodes hex encoding and returns message as bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        // Unwrap as we already know that the inner value is valid
        hex::decode(&self.0).unwrap()
    }

    /// Returns payload size (number of bytes) of encoded message.
    pub fn size(&self) -> i64 {
        // Divide by 2 as every byte is represented by 2 hex chars.
        self.0.len() as i64 / 2
    }
}

/// Returns an encoded version of this message.
impl TryFrom<&Message> for MessageEncoded {
    type Error = MessageEncodedError;

    fn try_from(message: &Message) -> Result<Self, Self::Error> {
        // Encode bytes as hex string
        let encoded = hex::encode(&message.to_cbor());
        Ok(MessageEncoded::new(&encoded)?)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Validate for MessageEncoded {
    type Error = MessageEncodedError;

    /// Checks encoded message value against hex format and CDDL schema.
    fn validate(&self) -> Result<(), Self::Error> {
        // Validate hex encoding
        let bytes = hex::decode(&self.0).map_err(|_| MessageEncodedError::InvalidHexEncoding)?;

        // Validate CDDL schema
        validate_schema(MESSAGE_SCHEMA, bytes)?;

        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl Validate for MessageEncoded {
    type Error = MessageEncodedError;

    /// Checks encoded message value against hex format.
    ///
    /// Skips CDDL schema validation as this is not supported for wasm targets. See:
    /// https://github.com/anweiss/cddl/issues/83
    fn validate(&self) -> Result<(), Self::Error> {
        hex::decode(&self.0).map_err(|_| MessageEncodedError::InvalidHexEncoding)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::hash::Hash;
    use crate::message::{Message, MessageValue};

    use super::MessageEncoded;

    #[test]
    fn validate() {
        // Invalid hex string
        assert!(MessageEncoded::new("123456789Z").is_err());

        // Invalid message
        assert!(MessageEncoded::new("68656c6c6f2062616d626f6f21").is_err());

        // Valid `delete` message
        assert!(MessageEncoded::new("a466616374696f6e6664656c65746566736368656d6178843030343032646332356433326466623430306262323935623636336434373036626334376630636234663165646666323737633733376166633861393233323333306165393838346663663664303231343161373835633566643832633139366239373365383432376566633063303464303434346463633330353932323062396564616776657273696f6e016269647884303034306366393466366436303536353765393063353433623063393139303730636461616637323039633565316561353861636238663335363866613231313432363864633961633362616665313261663237376432383666636537646335396237633063333438393733633465396461636265373934383565353661633261373032").is_ok());
    }

    #[test]
    fn decode() {
        let message_encoded = MessageEncoded::new("a566616374696f6e6675706461746566736368656d6178843030343032646332356433326466623430306262323935623636336434373036626334376630636234663165646666323737633733376166633861393233323333306165393838346663663664303231343161373835633566643832633139366239373365383432376566633063303464303434346463633330353932323062396564616776657273696f6e016269647884303034303564343933303465323964316439333538333134653130383364303564353631356137366636346330393834663531653336353961353361336535643637613262386536396239316533333539373836323765346363616663633534393231316132383363363135346433616634373036393863666332353666626638373030666669656c6473a563616765a2647479706563696e746576616c7565181c66686569676874a2647479706565666c6f61746576616c7565f943006869735f61646d696ea2647479706564626f6f6c6576616c7565f46f70726f66696c655f70696374757265a264747970656872656c6174696f6e6576616c7565788430303430636639346636643630353635376539306335343362306339313930373063646161663732303963356531656135386163623866333536386661323131343236386463396163336261666531326166323737643238366663653764633539623763306333343839373363346539646163626537393438356535366163326137303268757365726e616d65a26474797065637374726576616c75656462756275").unwrap();

        let message = Message::try_from(&message_encoded).unwrap();

        assert!(message.is_update());
        assert!(message.has_id());
        assert_eq!(message.schema().as_str(), "00402dc25d32dfb400bb295b663d4706bc47f0cb4f1edff277c737afc8a9232330ae9884fcf6d02141a785c5fd82c196b973e8427efc0c04d0444dcc3059220b9eda");

        let fields = message.fields().unwrap();

        assert_eq!(
            fields.get("username").unwrap(),
            &MessageValue::Text("bubu".to_owned())
        );
        assert_eq!(fields.get("age").unwrap(), &MessageValue::Integer(28));
        assert_eq!(fields.get("height").unwrap(), &MessageValue::Float(3.5));
        assert_eq!(
            fields.get("is_admin").unwrap(),
            &MessageValue::Boolean(false)
        );
        assert_eq!(
            fields.get("profile_picture").unwrap(),
            &MessageValue::Relation(Hash::new_from_bytes(vec![1, 2, 3]).unwrap())
        );
    }
}
