use parity_scale_codec::{Decode, Encode};
// use sp_runtime::DispatchError;

// Almost identical with the DispatchError
// The PopApiError. The idea is that it majorily returns the `UseCase` error.
// Conversion is handled on the runtime side so that new (or missed) errors,
// coming from polkadot sdk upgrades can be handled via runtime upgrades. In
// addition, all this conversion logic is now handled at the runtime in stead
// of the contract which doesn't increase the size of the contract binary, aka
// the PoV.
#[derive(Debug, PartialEq, Clone, Copy, Encode, Decode)]
enum PopApiError {
    Other(u8),
    CannotLookup,
    BadOrigin,
    // This is only returned if the error originates from a pallet and the
    // conversion logic hasn't picked it up.
    Module(ModuleError),
    ConsumerRemaining,
    NoProviders,
    TooManyConsumers,
    Token(TokenError),
    Arithmetic(ArithmeticError),
    Transactional(TransactionalError),
    Exhausted,
    Corruption,
    Unavailable,
    RootNotAllowed,
    // This error is carefully defined based on the use case and the errors that
    // we want to output to the developers.
    UseCase(UseCaseError),
    // This error is for deployed contracts that encounter a new error that
    // wasn't in the sdk at the time of deployment. The pop api is upgradeable
    // and can therefore convert that error in this error so that the contract
    // maintainers are still able to figure out what the error is by looking at
    // the provided info.
    Unspecified {
        // Index within the DispatchError
        dispatch_error_index: u8,
        // Index within the DispatchError variant. `0` if the above is nested.
        error_index: u8,
        // For struct variant with an index and error. `0` if the above is nested.
        error: u8,
    },
}

#[derive(Debug, PartialEq, Clone, Copy, Encode, Decode)]
enum UseCaseError {
    Fungibles(FungiblesError),
    // NonFungibles(NonFungiblesError),
    // etc
}

#[derive(Debug, PartialEq, Clone, Copy, Encode, Decode)]
pub enum FungiblesError {
    /// The asset is not live; either frozen or being destroyed.
    AssetNotLive,
    /// The amount to mint is less than the existential deposit.
    BelowMinimum,
    /// Not enough allowance to fulfill a request is available.
    InsufficientAllowance,
    /// Not enough balance to fulfill a request is available.
    InsufficientBalance,
    /// The asset ID is already taken.
    InUse,
    /// Minimum balance should be non-zero.
    MinBalanceZero,
    /// The account to alter does not exist.
    NoAccount,
    /// The signing account has no permission to do the operation.
    NoPermission,
    /// The given asset ID is unknown.
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Copy, Encode, Decode)]
struct ModuleError {
    // Pallet index.
    pub index: u8,
    // Error within the pallet's error, nested errors can not be further defined.
    pub error: u8,
}

#[derive(Debug, PartialEq, Clone, Copy, Encode, Decode)]
enum TokenError {
    Unknown,
    // etc
}

#[derive(Debug, PartialEq, Clone, Copy, Encode, Decode)]
enum ArithmeticError {
    Overflow,
    // etc
}

#[derive(Debug, PartialEq, Clone, Copy, Encode, Decode)]
enum TransactionalError {
    MaxLayersReached,
    // etc
}

// Helper function to encode DispatchError to u32
fn encode_and_decode_to_u32(error: PopApiError) -> u32 {
    let mut encoded = error.encode();
    encoded.resize(4, 0);
    println!("Encoded error: {encoded:?}");
    u32::decode(&mut &encoded[..]).unwrap()
}

// Helper function to decode DispatchError from u32
fn encode_and_decode_to_pop_api_error(value: u32) -> PopApiError {
    let encoded = value.encode();
    PopApiError::decode(&mut &encoded[..]).unwrap()
}

fn main() {}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub(crate) fn test_module_error_encoding_decoding() {
        let error = PopApiError::Module(ModuleError { index: 1, error: 2 });
        println!("Error: {error:?}");
        let value_u32 = encode_and_decode_to_u32(error);
        println!("U32: {value_u32}");
        let decoded_error = encode_and_decode_to_pop_api_error(value_u32);
        assert_eq!(error, decoded_error);
    }

    #[test]
    pub(crate) fn test_use_case_error_encoding_decoding() {
        let error =
            PopApiError::UseCase(UseCaseError::Fungibles(FungiblesError::InsufficientBalance));
        println!("Error: {error:?}");
        let value_u32 = encode_and_decode_to_u32(error);
        println!("U32: {value_u32}");
        let decoded_error = encode_and_decode_to_pop_api_error(value_u32);
        assert_eq!(error, decoded_error);
    }

    #[test]
    pub(crate) fn test_unspecified_error_encoding_decoding() {
        let error = PopApiError::Unspecified {
            dispatch_error_index: 3,
            error_index: 2,
            error: 1,
        };
        println!("Error: {error:?}");
        let value_u32 = encode_and_decode_to_u32(error);
        println!("U32: {value_u32}");
        let decoded_error = encode_and_decode_to_pop_api_error(value_u32);
        assert_eq!(error, decoded_error);
    }
}
