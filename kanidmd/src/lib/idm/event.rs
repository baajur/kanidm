use crate::actors::v1::IdmAccountSetPasswordMessage;
use crate::audit::AuditScope;
use crate::event::Event;
use crate::server::QueryServerWriteTransaction;

use uuid::Uuid;

use kanidm_proto::v1::OperationError;

#[derive(Debug)]
pub struct PasswordChangeEvent {
    pub event: Event,
    pub target: Uuid,
    pub cleartext: String,
    pub appid: Option<String>,
}

impl PasswordChangeEvent {
    pub fn new_internal(target: &Uuid, cleartext: &str, appid: Option<&str>) -> Self {
        PasswordChangeEvent {
            event: Event::from_internal(),
            target: target.clone(),
            cleartext: cleartext.to_string(),
            appid: appid.map(|v| v.to_string()),
        }
    }

    pub fn from_idm_account_set_password(
        audit: &mut AuditScope,
        qs: &QueryServerWriteTransaction,
        msg: IdmAccountSetPasswordMessage,
    ) -> Result<Self, OperationError> {
        let e = Event::from_rw_uat(audit, qs, msg.uat)?;
        let u = e.get_uuid().ok_or(OperationError::InvalidState)?.clone();

        Ok(PasswordChangeEvent {
            event: e,
            target: u,
            cleartext: msg.cleartext,
            appid: None,
        })
    }
}