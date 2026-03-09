use crate::AuthTokenRequest;
use crate::AuthTokenRequestBuilder;
use crate::SubjectIdentifierType;
use crate::client::KsefClient;
use crate::client::error::KsefError;

pub async fn get_auth_token_request(
    client: &KsefClient,
    subject_type: SubjectIdentifierType,
) -> Result<AuthTokenRequest, KsefError> {
    let challenge = match client.get_auth_challenge().await {
        Ok(ch) => ch.challenge,
        Err(e) => {
            return Err(KsefError::ApplicationError(
                0,
                format!("Unable to get AuthChallenge: {}", e),
            ));
        }
    };

    let built = AuthTokenRequestBuilder::new()
        .with_challenge(&challenge)
        .with_context(client.context.id_type.clone(), &client.context.value)
        .with_subject_type(subject_type)
        .build();

    match built {
        Ok(req) => Ok(req),
        Err(e) => Err(KsefError::ApplicationError(
            0,
            format!("Unable to build AuthTokenRequest: {}", e),
        )),
    }
}
