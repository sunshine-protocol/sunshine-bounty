pub use sunshine_ffi_utils as ffi_utils;
pub mod dto;
pub mod ffi;

#[doc(hidden)]
#[cfg(feature = "bounty-key")]
#[macro_export]
macro_rules! impl_bounty_key_ffi {
    () => {
        use $crate::ffi::Key;
        gen_ffi! {
             /// Check if the Keystore is exist and initialized.
            ///
            /// this is useful if you want to check if there is an already created account or not.
            Key::exists => fn client_key_exists() -> bool;
            /// Set a new Key for this device if not already exist.
            /// you should call `client_has_device_key` first to see if you have already a key.
            ///
            /// suri is used for testing only.
            /// phrase is used to restore a backup
            /// returns a string that is the current device id
            Key::set => fn client_key_set(
                password: *const raw::c_char = cstr!(password),
                suri: *const raw::c_char = cstr!(suri, allow_null),
                paperkey: *const raw::c_char = cstr!(paperkey, allow_null)
            ) -> String;
            /// Lock your account
            /// return `true` if locked, and return an error message if something went wrong
            Key::lock => fn client_key_lock() -> bool;
            /// Unlock your account using the password
            /// return `true` when the account get unlocked, otherwise an error message returned
            Key::unlock => fn client_key_unlock(password: *const raw::c_char = cstr!(password)) -> bool;
            /// Get current UID as string (if any)
            /// otherwise null returned
            Key::uid => fn client_key_uid() -> Option<String>;
        }
    }
}

#[doc(hidden)]
#[cfg(not(feature = "bounty-key"))]
#[macro_export]
macro_rules! impl_bounty_key_ffi {
    () => {};
}

#[doc(hidden)]
#[cfg(feature = "bounty-wallet")]
#[macro_export]
macro_rules! impl_bounty_wallet_ffi {
    () => {
        use $crate::ffi::Wallet;
        gen_ffi! {
            /// Get the balance of an identifier.
            /// returns and string but normally it's a `u128` encoded as string.
            Wallet::balance => fn client_wallet_balance(identifier: *const raw::c_char = cstr!(identifier, allow_null)) -> String;
            /// Transfer tokens to another account using there `identifier`
            /// returns current account balance after the transaction.
            Wallet::transfer => fn client_wallet_transfer(
                to: *const raw::c_char = cstr!(to),
                amount: u64 = amount
            ) -> String;
        }
    };
}

#[doc(hidden)]
#[cfg(not(feature = "bounty-wallet"))]
#[macro_export]
macro_rules! impl_bounty_wallet_ffi {
    () => {};
}

#[doc(hidden)]
#[cfg(feature = "bounty-module")]
#[macro_export]
macro_rules! impl_bounty_ffi {
    () => {
        use $crate::ffi::Bounty;
        gen_ffi! {
            /// Get a bounty Information by using bounty Id
            /// Returns JSON encoded `BountyInformation` as string
            Bounty::get => fn client_bounty_get(
                bounty_id: *const raw::c_char = cstr!(bounty_id)
            ) -> JSON<BountyInformation>;
            /// Get a submission Information by using submission Id
            /// Returns JSON encoded `BountySubmissionInformation` as string
            Bounty::get_submission => fn client_bounty_get_submission(
                submission_id: *const raw::c_char = cstr!(submission_id)
            ) -> JSON<BountySubmissionInformation>;
            /// Create a new Bounty
            /// Returns the `BountyId` as `u64`
            Bounty::post => fn client_bounty_post(
                repo_owner: *const raw::c_char = cstr!(repo_owner),
                repo_name: *const raw::c_char = cstr!(repo_name),
                issue_number: u64 = issue_number,
                amount: *const raw::c_char = cstr!(amount)
            ) -> u64;
            /// Contribute to a bounty.
            /// Returns the new total bounty amount
            Bounty::contribute => fn client_bounty_contribute(
                bounty_id: *const raw::c_char = cstr!(bounty_id),
                amount: *const raw::c_char = cstr!(amount)
            ) -> u128;
            /// Create a submission on a bounty
            /// Returns the `SubmissionId` as `u64`
            Bounty::submit => fn client_bounty_submit(
                bounty_id: *const raw::c_char = cstr!(bounty_id),
                repo_owner: *const raw::c_char = cstr!(repo_owner),
                repo_name: *const raw::c_char = cstr!(repo_name),
                issue_number: u64 = issue_number,
                amount: *const raw::c_char = cstr!(amount)
            ) -> u64;
            /// Approve a Submission using `SubmissionId`
            /// Returns the new total amount on that bounty after this operation
            Bounty::approve => fn client_bounty_approve(
                submission_id: *const raw::c_char = cstr!(submission_id)
            ) -> u128;
            /// Get a list of open bounties.
            /// Returns a JSON encoded list of `BountyInformation` as string.
            Bounty::open_bounties => fn client_bounty_open_bounties(
                min: *const raw::c_char = cstr!(min)
            ) -> JSON<Vec<BountyInformation>>;
            /// Get a list of open submissions on a bounty.
            /// Returns a JSON encoded list of `BountySubmissionInformation` as string.
            Bounty::open_bounty_submissions => fn client_bounty_open_bounty_submissions(
                bounty_id: *const raw::c_char = cstr!(bounty_id)
            ) -> JSON<Vec<BountySubmissionInformation>>;
        }
    };
}

#[doc(hidden)]
#[cfg(not(feature = "bounty-module"))]
#[macro_export]
macro_rules! impl_bounty_ffi {
    () => {};
}

/// Generate the FFI for the provided runtime
///
/// ### Example
/// ```
/// use test_client::Client;
/// use sunshine_bounty_ffi::impl_ffi;
///
/// impl_ffi!(client: Client);
/// ```
#[macro_export]
macro_rules! impl_ffi {
    () => {
        $crate::impl_bounty_ffi!();
        $crate::impl_bounty_key_ffi!();
        $crate::impl_bounty_wallet_ffi!();
    };
    (client: $client: ty) => {
        use ::std::os::raw;
        #[allow(unused)]
        use $crate::ffi_utils::*;
        gen_ffi!(client = $client);
        $crate::impl_ffi!();
    };
}
