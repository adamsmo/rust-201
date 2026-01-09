pub trait CryptoBackend {
    type SignError;
    type VerifyError;

    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, Self::SignError>;
    fn verify(&self, data: &[u8], sig: &[u8]) -> Result<bool, Self::VerifyError>;
}

pub struct InMemoryKeys {
    private_key: [u8; 32],
}

impl CryptoBackend for InMemoryKeys {
    type SignError = !; // Signing cannot fail
    type VerifyError = !; // Verification cannot fail

    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, !> {
        // dummy
        Ok(self.do_sign(data))
    }

    fn verify(&self, _data: &[u8], _sig: &[u8]) -> Result<bool, !> {
        // dummy
        Ok(true)
    }
}

impl InMemoryKeys {
    fn do_sign(&self, _data: &[u8]) -> Vec<u8> {
        // dummy
        vec![0x42, 0x24]
    }
}

pub struct HsmBackend {
    device_id: String,
}

#[derive(Debug)]
pub enum HsmError {
    DeviceNotFound,
    OperationFailed,
    Timeout,
}

impl CryptoBackend for HsmBackend {
    type SignError = HsmError; // HSM signing can fail
    type VerifyError = HsmError; // HSM verification can fail

    fn sign(&self, _data: &[u8]) -> Result<Vec<u8>, HsmError> {
        // dummy
        Ok(vec![0x12, 0x34])
    }

    fn verify(&self, _data: &[u8], _sig: &[u8]) -> Result<bool, HsmError> {
        // dummy
        Ok(true)
    }
}

pub fn sign_document<B: CryptoBackend>(
    backend: &B,
    document: &[u8],
) -> Result<Vec<u8>, B::SignError> {
    backend.sign(document)
}

#[test]
fn test_signing() {
    let in_memory = InMemoryKeys {
        private_key: [0; 32],
    };

    // this can not fail
    let Ok(sig) = sign_document(&in_memory, b"hello");
    println!("In memory signature: {:?}", sig);

    let hsm = HsmBackend {
        device_id: "HSM-001".to_string(),
    };

    // this can fail
    match sign_document(&hsm, b"hello") {
        Ok(sig) => println!("HSM signature: {:?}", sig),
        Err(e) => println!("HSM error: {:?}", e),
    }
}
