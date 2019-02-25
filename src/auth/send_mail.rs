use data::auth::InvitationToken;
use data::auth::Invitation;

#[derive(Debug, Fail, Clone, PartialEq, Eq)]
pub enum EmailError {
    #[fail(display = "An unknown error occurred")]
    Unknown,
}

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