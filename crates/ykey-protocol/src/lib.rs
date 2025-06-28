// crates/xkey-protocol/src/lib.rs
pub mod fido2;
pub mod ctap;
pub mod webauthn;

// crates/xkey-protocol/src/fido2.rs
use xkey_core::{traits::*, types::*, error::XKeyResult};
use async_trait::async_trait;

/// FIDO2 协议实现
pub struct Fido2Client<D: Device> {
    device: D,
    pin_token: Option<Vec<u8>>,
}

impl<D: Device> Fido2Client<D> {
    pub fn new(device: D) -> Self {
        Self {
            device,
            pin_token: None,
        }
    }
}

#[async_trait]
impl<D: Device> Fido2Protocol for Fido2Client<D> {
    async fn get_info(&mut self) -> XKeyResult<AuthenticatorInfo> {
        let command = CtapCommand::GetInfo;
        let response = self.send_ctap_command(command).await?;
        
        match response {
            CtapResponse::GetInfo(info) => Ok(info),
            _ => Err(XKeyError::UnexpectedResponse),
        }
    }
    
    async fn make_credential(
        &mut self, 
        params: MakeCredentialParams
    ) -> XKeyResult<AttestationObject> {
        let command = CtapCommand::MakeCredential(params);
        let response = self.send_ctap_command(command).await?;
        
        match response {
            CtapResponse::MakeCredential(attestation) => Ok(attestation),
            CtapResponse::Error(code) => Err(XKeyError::CtapError(code)),
            _ => Err(XKeyError::UnexpectedResponse),
        }
    }
    
    async fn get_assertion(
        &mut self, 
        params: GetAssertionParams
    ) -> XKeyResult<AssertionObject> {
        let command = CtapCommand::GetAssertion(params);
        let response = self.send_ctap_command(command).await?;
        
        match response {
            CtapResponse::GetAssertion(assertion) => Ok(assertion),
            CtapResponse::Error(code) => Err(XKeyError::CtapError(code)),
            _ => Err(XKeyError::UnexpectedResponse),
        }
    }
    
    // ... 其他方法实现
}

impl<D: Device> Fido2Client<D> {
    async fn send_ctap_command(&mut self, command: CtapCommand) -> XKeyResult<CtapResponse> {
        let data = command.encode()?;
        let response_data = self.device.send_raw(&data).await?;
        CtapResponse::decode(&response_data)
    }
}
