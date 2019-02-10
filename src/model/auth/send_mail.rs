use data::auth::InvitationToken;
use data::auth::Invitation;
use std::fmt::Debug;
use model::state::ActionState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailError;

pub struct EmailSender {}

pub trait EmailOps {
    fn send_email(&self, invitation_token: InvitationToken) -> Result<Invitation, EmailError>;
}


impl EmailOps for EmailSender {
    fn send_email(&self, invitation_token: InvitationToken) -> Result<Invitation, EmailError> {
        //TODO: send email
        let inviatation = Invitation {
            email: invitation_token.email,
            expires_at: invitation_token.expires_at,
        };

        Ok(inviatation)
    }
}