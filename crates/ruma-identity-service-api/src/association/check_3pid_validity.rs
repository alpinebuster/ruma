//! `GET /_matrix/identity/*/3pid/getValidated3pid`
//!
//! Endpoint to determine the validity of a 3PID.

pub mod v2 {
    //! `/v2/` ([spec])
    //!
    //! [spec]: https://spec.matrix.org/v1.2/identity-service-api/#get_matrixidentityv23pidgetvalidated3pid

    use js_int::UInt;
    use ruma_common::{api::ruma_api, thirdparty::Medium, ClientSecret, SessionId};

    ruma_api! {
        metadata: {
            description: "Determines if a given 3PID has been validated by a user.",
            method: GET,
            name: "check_3pid_validity",
            stable_path: "/_matrix/identity/v2/3pid/getValidated3pid/",
            rate_limited: false,
            authentication: AccessToken,
            added: 1.0,
        }

        request: {
            /// The Session ID generated by the `requestToken` call.
            #[ruma_api(query)]
            pub sid: &'a SessionId,

            /// The client secret passed to the `requestToken` call.
            #[ruma_api(query)]
            pub client_secret: &'a ClientSecret,
        }

        response: {
            /// The medium type of the 3PID.
            pub medium: Medium,

            /// The address of the 3PID being looked up.
            pub address: String,

            /// Timestamp, in milliseconds, indicating the time that the 3PID was validated.
            pub validated_at: UInt,
        }
    }

    impl<'a> Request<'a> {
        /// Creates a `Request` with the given Session ID and client secret.
        pub fn new(sid: &'a SessionId, client_secret: &'a ClientSecret) -> Self {
            Self { sid, client_secret }
        }
    }

    impl Response {
        /// Creates a `Response` with the given medium, address and validation timestamp.
        pub fn new(medium: Medium, address: String, validated_at: UInt) -> Self {
            Self { medium, address, validated_at }
        }
    }
}
