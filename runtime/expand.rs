#![feature(prelude_import)]
#![recursion_limit = "256"]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub use frame_support::traits::EqualPrivilegeOnly;
use frame_system::EnsureRoot;
use pallet_grandpa::{fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, NumberFor, Verify},
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, MultiSignature,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{
        ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, Randomness, StorageInfo,
    },
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        IdentityFee, Weight,
    },
    StorageValue,
};
pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::CurrencyAdapter;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};
/// Import the template pallet.
pub use pallet_template;
/// An index to a block.
pub type BlockNumber = u32;
/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;
/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
/// Balance of an account.
pub type Balance = u128;
/// Index of a transaction in the chain.
pub type Index = u32;
/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;
/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;
    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
    use ::sp_runtime::serde as __opaque_keys_serde_import__SessionKeys;
    #[serde(crate = "__opaque_keys_serde_import__SessionKeys")]
    pub struct SessionKeys {
        pub aura: <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
        pub grandpa: <Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for SessionKeys {
        #[inline]
        fn clone(&self) -> SessionKeys {
            match *self {
                Self {
                    aura: ref __self_0_0,
                    grandpa: ref __self_0_1,
                } => SessionKeys {
                    aura: ::core::clone::Clone::clone(&(*__self_0_0)),
                    grandpa: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for SessionKeys {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for SessionKeys {
        #[inline]
        fn eq(&self, other: &SessionKeys) -> bool {
            match *other {
                Self {
                    aura: ref __self_1_0,
                    grandpa: ref __self_1_1,
                } => match *self {
                    Self {
                        aura: ref __self_0_0,
                        grandpa: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &SessionKeys) -> bool {
            match *other {
                Self {
                    aura: ref __self_1_0,
                    grandpa: ref __self_1_1,
                } => match *self {
                    Self {
                        aura: ref __self_0_0,
                        grandpa: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for SessionKeys {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for SessionKeys {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<
                    <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                >;
                let _: ::core::cmp::AssertParamIsEq<
                    <Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                >;
            }
        }
    }
    const _: () = {
        impl ::codec::Encode for SessionKeys {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::codec::Encode::encode_to(&self.aura, __codec_dest_edqy);
                ::codec::Encode::encode_to(&self.grandpa, __codec_dest_edqy);
            }
        }
        impl ::codec::EncodeLike for SessionKeys {}
    };
    const _: () = {
        impl ::codec::Decode for SessionKeys {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                ::core::result::Result::Ok(SessionKeys {
                    aura: {
                        let __codec_res_edqy = < < Aura as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `SessionKeys::aura`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    grandpa: {
                        let __codec_res_edqy = < < Grandpa as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `SessionKeys::grandpa`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                })
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for SessionKeys {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new(
                        "SessionKeys",
                        "node_template_runtime::opaque",
                    ))
                    .type_params(::alloc::vec::Vec::new())
                    .docs(&[])
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| {
                                f.ty::<<Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public>()
                                    .name("aura")
                                    .type_name("<Aura as $crate::BoundToRuntimeAppPublic>::Public")
                                    .docs(&[])
                            })
                            .field(|f| {
                                f.ty::<<Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public>()
                                    .name("grandpa")
                                    .type_name(
                                        "<Grandpa as $crate::BoundToRuntimeAppPublic>::Public",
                                    )
                                    .docs(&[])
                            }),
                    )
            }
        };
    };
    impl core::fmt::Debug for SessionKeys {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_struct("SessionKeys")
                .field("aura", &self.aura)
                .field("grandpa", &self.grandpa)
                .finish()
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use __opaque_keys_serde_import__SessionKeys as _serde;
        #[automatically_derived]
        impl __opaque_keys_serde_import__SessionKeys::Serialize for SessionKeys {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> __opaque_keys_serde_import__SessionKeys::__private::Result<__S::Ok, __S::Error>
            where
                __S: __opaque_keys_serde_import__SessionKeys::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "SessionKeys",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "aura",
                    &self.aura,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "grandpa",
                    &self.grandpa,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use __opaque_keys_serde_import__SessionKeys as _serde;
        #[automatically_derived]
        impl<'de> __opaque_keys_serde_import__SessionKeys::Deserialize<'de> for SessionKeys {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> __opaque_keys_serde_import__SessionKeys::__private::Result<Self, __D::Error>
            where
                __D: __opaque_keys_serde_import__SessionKeys::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "aura" => _serde::__private::Ok(__Field::__field0),
                            "grandpa" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"aura" => _serde::__private::Ok(__Field::__field0),
                            b"grandpa" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<SessionKeys>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = SessionKeys;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct SessionKeys")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SessionKeys with 2 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            <Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SessionKeys with 2 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(SessionKeys {
                            aura: __field0,
                            grandpa: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<
                            <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                        > = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<
                            <Grandpa as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "aura",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            <Aura as ::sp_runtime::BoundToRuntimeAppPublic>::Public,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "grandpa",
                                            ),
                                        );
                                    }
                                    __field1 = _serde :: __private :: Some (match _serde :: de :: MapAccess :: next_value :: < < Grandpa as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public > (& mut __map) { _serde :: __private :: Ok (__val) => __val , _serde :: __private :: Err (__err) => { return _serde :: __private :: Err (__err) ; } }) ;
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("aura") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("grandpa") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(SessionKeys {
                            aura: __field0,
                            grandpa: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["aura", "grandpa"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "SessionKeys",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<SessionKeys>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl SessionKeys {
        /// Generate a set of keys with optionally using the given seed.
        ///
        /// The generated key pairs are stored in the keystore.
        ///
        /// Returns the concatenated SCALE encoded public keys.
        pub fn generate(
            seed: Option<::sp_runtime::sp_std::vec::Vec<u8>>,
        ) -> ::sp_runtime::sp_std::vec::Vec<u8> {
            let keys = Self { aura : < < Aura as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: sp_runtime :: RuntimeAppPublic > :: generate_pair (seed . clone ()) , grandpa : < < Grandpa as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: sp_runtime :: RuntimeAppPublic > :: generate_pair (seed . clone ()) , } ;
            ::sp_runtime::codec::Encode::encode(&keys)
        }
        /// Converts `Self` into a `Vec` of `(raw public key, KeyTypeId)`.
        pub fn into_raw_public_keys(
            self,
        ) -> ::sp_runtime::sp_std::vec::Vec<(
            ::sp_runtime::sp_std::vec::Vec<u8>,
            ::sp_runtime::KeyTypeId,
        )> {
            let mut keys = Vec::new();
            keys . push ((:: sp_runtime :: RuntimeAppPublic :: to_raw_vec (& self . aura) , < < Aura as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: sp_runtime :: RuntimeAppPublic > :: ID)) ;
            keys . push ((:: sp_runtime :: RuntimeAppPublic :: to_raw_vec (& self . grandpa) , < < Grandpa as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: sp_runtime :: RuntimeAppPublic > :: ID)) ;
            keys
        }
        /// Decode `Self` from the given `encoded` slice and convert `Self` into the raw public
        /// keys (see [`Self::into_raw_public_keys`]).
        ///
        /// Returns `None` when the decoding failed, otherwise `Some(_)`.
        pub fn decode_into_raw_public_keys(
            encoded: &[u8],
        ) -> Option<
            ::sp_runtime::sp_std::vec::Vec<(
                ::sp_runtime::sp_std::vec::Vec<u8>,
                ::sp_runtime::KeyTypeId,
            )>,
        > {
            <Self as ::sp_runtime::codec::Decode>::decode(&mut &encoded[..])
                .ok()
                .map(|s| s.into_raw_public_keys())
        }
    }
    impl ::sp_runtime::traits::OpaqueKeys for SessionKeys {
        type KeyTypeIdProviders = (Aura, Grandpa);
        fn key_ids() -> &'static [::sp_runtime::KeyTypeId] {
            & [< < Aura as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: sp_runtime :: RuntimeAppPublic > :: ID , < < Grandpa as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: sp_runtime :: RuntimeAppPublic > :: ID]
        }
        fn get_raw(&self, i: ::sp_runtime::KeyTypeId) -> &[u8] {
            match i { i if i == < < Aura as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: sp_runtime :: RuntimeAppPublic > :: ID => self . aura . as_ref () , i if i == < < Grandpa as :: sp_runtime :: BoundToRuntimeAppPublic > :: Public as :: sp_runtime :: RuntimeAppPublic > :: ID => self . grandpa . as_ref () , _ => & [] , }
        }
    }
}
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: { ::sp_runtime::RuntimeString::Borrowed("node-template") },
    impl_name: { ::sp_runtime::RuntimeString::Borrowed("node-template") },
    authoring_version: 1,
    spec_version: 102,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};
const _: () = {};
/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;
/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
pub struct MaxWellKnownNodes;
impl MaxWellKnownNodes {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        8
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxWellKnownNodes {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MaxPeerIdLength;
impl MaxPeerIdLength {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        128
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxPeerIdLength {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl pallet_node_authorization::Config for Runtime {
    type Event = Event;
    type MaxWellKnownNodes = MaxWellKnownNodes;
    type MaxPeerIdLength = MaxPeerIdLength;
    type AddOrigin = EnsureRoot<AccountId>;
    type RemoveOrigin = EnsureRoot<AccountId>;
    type SwapOrigin = EnsureRoot<AccountId>;
    type ResetOrigin = EnsureRoot<AccountId>;
    type WeightInfo = ();
}
pub struct BlockHashCount;
impl BlockHashCount {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        2400
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for BlockHashCount {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct Version;
impl Version {
    /// Returns the value of this parameter type.
    pub const fn get() -> RuntimeVersion {
        VERSION
    }
}
impl<I: From<RuntimeVersion>> ::frame_support::traits::Get<I> for Version {
    fn get() -> I {
        I::from(Self::get())
    }
}
/// We allow for 2 seconds of compute with a 6 second average block time.
pub struct BlockWeights;
impl BlockWeights {
    /// Returns the value of this parameter type.
    pub fn get() -> frame_system::limits::BlockWeights {
        frame_system::limits::BlockWeights::with_sensible_defaults(
            2 * WEIGHT_PER_SECOND,
            NORMAL_DISPATCH_RATIO,
        )
    }
}
impl<I: From<frame_system::limits::BlockWeights>> ::frame_support::traits::Get<I> for BlockWeights {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct BlockLength;
impl BlockLength {
    /// Returns the value of this parameter type.
    pub fn get() -> frame_system::limits::BlockLength {
        frame_system::limits::BlockLength::max_with_normal_ratio(
            5 * 1024 * 1024,
            NORMAL_DISPATCH_RATIO,
        )
    }
}
impl<I: From<frame_system::limits::BlockLength>> ::frame_support::traits::Get<I> for BlockLength {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct SS58Prefix;
impl SS58Prefix {
    /// Returns the value of this parameter type.
    pub const fn get() -> u8 {
        42
    }
}
impl<I: From<u8>> ::frame_support::traits::Get<I> for SS58Prefix {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl frame_system::Config for Runtime {
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = frame_support::traits::Everything;
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = BlockWeights;
    /// The maximum length of a block (in bytes).
    type BlockLength = BlockLength;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The aggregated dispatch type that is available for extrinsics.
    type Call = Call;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// The ubiquitous event type.
    type Event = Event;
    /// The ubiquitous origin type.
    type Origin = Origin;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// Version of the runtime.
    type Version = Version;
    /// Converts a module to the index of the module in `construct_runtime!`.
    ///
    /// This type is being generated by `construct_runtime!`.
    type PalletInfo = PalletInfo;
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
    /// The set code logic, just the default since we're not a parachain.
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}
impl pallet_randomness_collective_flip::Config for Runtime {}
impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<32>;
}
impl pallet_grandpa::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type KeyOwnerProofSystem = ();
    type KeyOwnerProof =
        <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        GrandpaId,
    )>>::IdentificationTuple;
    type HandleEquivocation = ();
    type WeightInfo = ();
    type MaxAuthorities = ConstU32<32>;
}
impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    type WeightInfo = ();
}
impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<1000>;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}
pub struct NickReservationFee;
impl NickReservationFee {
    /// Returns the value of this parameter type.
    pub const fn get() -> u128 {
        100
    }
}
impl<I: From<u128>> ::frame_support::traits::Get<I> for NickReservationFee {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MinNickLength;
impl MinNickLength {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        8
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for MinNickLength {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MaxNickLength;
impl MaxNickLength {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        32
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxNickLength {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl pallet_nicks::Config for Runtime {
    type Currency = Balances;
    type ReservationFee = NickReservationFee;
    type Slashed = ();
    type ForceOrigin = frame_system::EnsureRoot<AccountId>;
    type MinLength = MinNickLength;
    type MaxLength = MaxNickLength;
    type Event = Event;
}
impl pallet_transaction_payment::Config for Runtime {
    type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = IdentityFee<Balance>;
    type LengthToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
}
impl pallet_sudo::Config for Runtime {
    type Event = Event;
    type Call = Call;
}
/// Configure the pallet-template in pallets/template.
impl pallet_template::Config for Runtime {
    type Event = Event;
    type MaxBytesInHash = frame_support::traits::ConstU32<64>;
}
#[doc(hidden)]
mod sp_api_hidden_includes_construct_runtime {
    pub extern crate frame_support as hidden_include;
}
const _: () = {
    #[allow(unused)]
    type __hidden_use_of_unchecked_extrinsic = UncheckedExtrinsic;
};
pub struct Runtime;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Runtime {
    #[inline]
    fn clone(&self) -> Runtime {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for Runtime {}
impl ::core::marker::StructuralPartialEq for Runtime {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Runtime {
    #[inline]
    fn eq(&self, other: &Runtime) -> bool {
        match *other {
            Self => match *self {
                Self => true,
            },
        }
    }
}
impl ::core::marker::StructuralEq for Runtime {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Runtime {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
impl core::fmt::Debug for Runtime {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("Runtime").finish()
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for Runtime {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(::scale_info::Path::new("Runtime", "node_template_runtime"))
                .type_params(::alloc::vec::Vec::new())
                .docs(&[])
                .composite(::scale_info::build::Fields::unit())
        }
    };
};
impl self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: GetNodeBlockType for Runtime { type NodeBlock = opaque :: Block ; }
impl self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: GetRuntimeBlockType for Runtime { type RuntimeBlock = Block ; }
#[allow(non_camel_case_types)]
pub enum Event {
    #[codec(index = 0u8)]
    System(frame_system::Event<Runtime>),
    #[codec(index = 4u8)]
    Grandpa(pallet_grandpa::Event),
    #[codec(index = 5u8)]
    Balances(pallet_balances::Event<Runtime>),
    #[codec(index = 7u8)]
    Sudo(pallet_sudo::Event<Runtime>),
    #[codec(index = 8u8)]
    TemplateModule(pallet_template::Event<Runtime>),
    #[codec(index = 9u8)]
    Nicks(pallet_nicks::Event<Runtime>),
    #[codec(index = 10u8)]
    NodeAuthorization(pallet_node_authorization::Event<Runtime>),
    #[codec(index = 11u8)]
    Scheduler(pallet_scheduler::Event<Runtime>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for Event {
    #[inline]
    fn clone(&self) -> Event {
        match (&*self,) {
            (&Event::System(ref __self_0),) => {
                Event::System(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::Grandpa(ref __self_0),) => {
                Event::Grandpa(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::Balances(ref __self_0),) => {
                Event::Balances(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::Sudo(ref __self_0),) => Event::Sudo(::core::clone::Clone::clone(&(*__self_0))),
            (&Event::TemplateModule(ref __self_0),) => {
                Event::TemplateModule(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::Nicks(ref __self_0),) => {
                Event::Nicks(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::NodeAuthorization(ref __self_0),) => {
                Event::NodeAuthorization(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::Scheduler(ref __self_0),) => {
                Event::Scheduler(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralPartialEq for Event {}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for Event {
    #[inline]
    fn eq(&self, other: &Event) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Event::System(ref __self_0), &Event::System(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::Grandpa(ref __self_0), &Event::Grandpa(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::Balances(ref __self_0), &Event::Balances(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::Sudo(ref __self_0), &Event::Sudo(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (
                        &Event::TemplateModule(ref __self_0),
                        &Event::TemplateModule(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (&Event::Nicks(ref __self_0), &Event::Nicks(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (
                        &Event::NodeAuthorization(ref __self_0),
                        &Event::NodeAuthorization(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (&Event::Scheduler(ref __self_0), &Event::Scheduler(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Event) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Event::System(ref __self_0), &Event::System(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::Grandpa(ref __self_0), &Event::Grandpa(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::Balances(ref __self_0), &Event::Balances(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::Sudo(ref __self_0), &Event::Sudo(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (
                        &Event::TemplateModule(ref __self_0),
                        &Event::TemplateModule(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (&Event::Nicks(ref __self_0), &Event::Nicks(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (
                        &Event::NodeAuthorization(ref __self_0),
                        &Event::NodeAuthorization(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (&Event::Scheduler(ref __self_0), &Event::Scheduler(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralEq for Event {}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::Eq for Event {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<frame_system::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_grandpa::Event>;
            let _: ::core::cmp::AssertParamIsEq<pallet_balances::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_sudo::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_template::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_nicks::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_node_authorization::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<pallet_scheduler::Event<Runtime>>;
        }
    }
}
const _: () = {
    #[allow(non_camel_case_types)]
    impl ::codec::Encode for Event {
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                Event::System(ref aa) => {
                    __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Grandpa(ref aa) => {
                    __codec_dest_edqy.push_byte(4u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Balances(ref aa) => {
                    __codec_dest_edqy.push_byte(5u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Sudo(ref aa) => {
                    __codec_dest_edqy.push_byte(7u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::TemplateModule(ref aa) => {
                    __codec_dest_edqy.push_byte(8u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Nicks(ref aa) => {
                    __codec_dest_edqy.push_byte(9u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::NodeAuthorization(ref aa) => {
                    __codec_dest_edqy.push_byte(10u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Event::Scheduler(ref aa) => {
                    __codec_dest_edqy.push_byte(11u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    impl ::codec::EncodeLike for Event {}
};
const _: () = {
    #[allow(non_camel_case_types)]
    impl ::codec::Decode for Event {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| e.chain("Could not decode `Event`, failed to read variant byte"))?
            {
                __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::System({
                        let __codec_res_edqy =
                            <frame_system::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::System.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 4u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Grandpa({
                        let __codec_res_edqy =
                            <pallet_grandpa::Event as ::codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Grandpa.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 5u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Balances({
                        let __codec_res_edqy =
                            <pallet_balances::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Balances.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 7u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Sudo({
                        let __codec_res_edqy =
                            <pallet_sudo::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Sudo.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 8u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::TemplateModule({
                        let __codec_res_edqy =
                            <pallet_template::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::TemplateModule.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 9u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Nicks({
                        let __codec_res_edqy =
                            <pallet_nicks::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Nicks.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 10u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::NodeAuthorization({
                        let __codec_res_edqy =
                            <pallet_node_authorization::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::NodeAuthorization.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 11u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::Scheduler({
                        let __codec_res_edqy =
                            <pallet_scheduler::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::Scheduler.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                    "Could not decode `Event`, variant doesn't exist",
                )),
            }
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for Event {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(::scale_info::Path::new("Event", "node_template_runtime"))
                .type_params(::alloc::vec::Vec::new())
                .docs(&[])
                .variant(
                    ::scale_info::build::Variants::new()
                        .variant("System", |v| {
                            v.index(0u8 as ::core::primitive::u8)
                                .fields(::scale_info::build::Fields::unnamed().field(|f| {
                                    f.ty::<frame_system::Event<Runtime>>()
                                        .type_name("frame_system::Event<Runtime>")
                                        .docs(&[])
                                }))
                                .docs(&[])
                        })
                        .variant("Grandpa", |v| {
                            v.index(4u8 as ::core::primitive::u8)
                                .fields(::scale_info::build::Fields::unnamed().field(|f| {
                                    f.ty::<pallet_grandpa::Event>()
                                        .type_name("pallet_grandpa::Event")
                                        .docs(&[])
                                }))
                                .docs(&[])
                        })
                        .variant("Balances", |v| {
                            v.index(5u8 as ::core::primitive::u8)
                                .fields(::scale_info::build::Fields::unnamed().field(|f| {
                                    f.ty::<pallet_balances::Event<Runtime>>()
                                        .type_name("pallet_balances::Event<Runtime>")
                                        .docs(&[])
                                }))
                                .docs(&[])
                        })
                        .variant("Sudo", |v| {
                            v.index(7u8 as ::core::primitive::u8)
                                .fields(::scale_info::build::Fields::unnamed().field(|f| {
                                    f.ty::<pallet_sudo::Event<Runtime>>()
                                        .type_name("pallet_sudo::Event<Runtime>")
                                        .docs(&[])
                                }))
                                .docs(&[])
                        })
                        .variant("TemplateModule", |v| {
                            v.index(8u8 as ::core::primitive::u8)
                                .fields(::scale_info::build::Fields::unnamed().field(|f| {
                                    f.ty::<pallet_template::Event<Runtime>>()
                                        .type_name("pallet_template::Event<Runtime>")
                                        .docs(&[])
                                }))
                                .docs(&[])
                        })
                        .variant("Nicks", |v| {
                            v.index(9u8 as ::core::primitive::u8)
                                .fields(::scale_info::build::Fields::unnamed().field(|f| {
                                    f.ty::<pallet_nicks::Event<Runtime>>()
                                        .type_name("pallet_nicks::Event<Runtime>")
                                        .docs(&[])
                                }))
                                .docs(&[])
                        })
                        .variant("NodeAuthorization", |v| {
                            v.index(10u8 as ::core::primitive::u8)
                                .fields(::scale_info::build::Fields::unnamed().field(|f| {
                                    f.ty::<pallet_node_authorization::Event<Runtime>>()
                                        .type_name("pallet_node_authorization::Event<Runtime>")
                                        .docs(&[])
                                }))
                                .docs(&[])
                        })
                        .variant("Scheduler", |v| {
                            v.index(11u8 as ::core::primitive::u8)
                                .fields(::scale_info::build::Fields::unnamed().field(|f| {
                                    f.ty::<pallet_scheduler::Event<Runtime>>()
                                        .type_name("pallet_scheduler::Event<Runtime>")
                                        .docs(&[])
                                }))
                                .docs(&[])
                        }),
                )
        }
    };
};
impl core::fmt::Debug for Event {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::System(ref a0) => fmt.debug_tuple("Event::System").field(a0).finish(),
            Self::Grandpa(ref a0) => fmt.debug_tuple("Event::Grandpa").field(a0).finish(),
            Self::Balances(ref a0) => fmt.debug_tuple("Event::Balances").field(a0).finish(),
            Self::Sudo(ref a0) => fmt.debug_tuple("Event::Sudo").field(a0).finish(),
            Self::TemplateModule(ref a0) => {
                fmt.debug_tuple("Event::TemplateModule").field(a0).finish()
            }
            Self::Nicks(ref a0) => fmt.debug_tuple("Event::Nicks").field(a0).finish(),
            Self::NodeAuthorization(ref a0) => fmt
                .debug_tuple("Event::NodeAuthorization")
                .field(a0)
                .finish(),
            Self::Scheduler(ref a0) => fmt.debug_tuple("Event::Scheduler").field(a0).finish(),
            _ => Ok(()),
        }
    }
}
impl From<frame_system::Event<Runtime>> for Event {
    fn from(x: frame_system::Event<Runtime>) -> Self {
        Event::System(x)
    }
}
impl TryInto<frame_system::Event<Runtime>> for Event {
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        frame_system::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::System(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_grandpa::Event> for Event {
    fn from(x: pallet_grandpa::Event) -> Self {
        Event::Grandpa(x)
    }
}
impl TryInto<pallet_grandpa::Event> for Event {
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_grandpa::Event,
        Self::Error,
    > {
        match self {
            Self::Grandpa(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_balances::Event<Runtime>> for Event {
    fn from(x: pallet_balances::Event<Runtime>) -> Self {
        Event::Balances(x)
    }
}
impl TryInto<pallet_balances::Event<Runtime>> for Event {
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_balances::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::Balances(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_sudo::Event<Runtime>> for Event {
    fn from(x: pallet_sudo::Event<Runtime>) -> Self {
        Event::Sudo(x)
    }
}
impl TryInto<pallet_sudo::Event<Runtime>> for Event {
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_sudo::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::Sudo(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_template::Event<Runtime>> for Event {
    fn from(x: pallet_template::Event<Runtime>) -> Self {
        Event::TemplateModule(x)
    }
}
impl TryInto<pallet_template::Event<Runtime>> for Event {
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_template::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::TemplateModule(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_nicks::Event<Runtime>> for Event {
    fn from(x: pallet_nicks::Event<Runtime>) -> Self {
        Event::Nicks(x)
    }
}
impl TryInto<pallet_nicks::Event<Runtime>> for Event {
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_nicks::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::Nicks(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_node_authorization::Event<Runtime>> for Event {
    fn from(x: pallet_node_authorization::Event<Runtime>) -> Self {
        Event::NodeAuthorization(x)
    }
}
impl TryInto<pallet_node_authorization::Event<Runtime>> for Event {
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_node_authorization::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::NodeAuthorization(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
impl From<pallet_scheduler::Event<Runtime>> for Event {
    fn from(x: pallet_scheduler::Event<Runtime>) -> Self {
        Event::Scheduler(x)
    }
}
impl TryInto<pallet_scheduler::Event<Runtime>> for Event {
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        pallet_scheduler::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::Scheduler(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
/// The runtime origin type representing the origin of a call.
///
/// Origin is always created with the base filter configured in [`frame_system::Config::BaseCallFilter`].
pub struct Origin {
    caller: OriginCaller,
    filter: self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc<
        Box<dyn Fn(&<Runtime as frame_system::Config>::Call) -> bool>,
    >,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Origin {
    #[inline]
    fn clone(&self) -> Origin {
        match *self {
            Self {
                caller: ref __self_0_0,
                filter: ref __self_0_1,
            } => Origin {
                caller: ::core::clone::Clone::clone(&(*__self_0_0)),
                filter: ::core::clone::Clone::clone(&(*__self_0_1)),
            },
        }
    }
}
#[cfg(feature = "std")]
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Debug for Origin {
    fn fmt(
        &self,
        fmt : & mut self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: fmt :: Formatter,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        (),
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Error,
    > {
        fmt.debug_struct("Origin")
            .field("caller", &self.caller)
            .field("filter", &"[function ptr]")
            .finish()
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait
    for Origin
{
    type Call = <Runtime as frame_system::Config>::Call;
    type PalletsOrigin = OriginCaller;
    type AccountId = <Runtime as frame_system::Config>::AccountId;
    fn add_filter(&mut self, filter: impl Fn(&Self::Call) -> bool + 'static) {
        let f = self.filter.clone();
        self.filter =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                Box::new(move |call| f(call) && filter(call)),
            );
    }
    fn reset_filter(&mut self) {
        let filter = < < Runtime as frame_system :: Config > :: BaseCallFilter as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: Contains < < Runtime as frame_system :: Config > :: Call > > :: contains ;
        self.filter =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                Box::new(filter),
            );
    }
    fn set_caller_from(&mut self, other: impl Into<Self>) {
        self.caller = other.into().caller;
    }
    fn filter_call(&self, call: &Self::Call) -> bool {
        match self.caller {
            OriginCaller::system(frame_system::Origin::<Runtime>::Root) => true,
            _ => (self.filter)(call),
        }
    }
    fn caller(&self) -> &Self::PalletsOrigin {
        &self.caller
    }
    fn try_with_caller<R>(
        mut self,
        f: impl FnOnce(Self::PalletsOrigin) -> Result<R, Self::PalletsOrigin>,
    ) -> Result<R, Self> {
        match f(self.caller) {
            Ok(r) => Ok(r),
            Err(caller) => {
                self.caller = caller;
                Err(self)
            }
        }
    }
    fn none() -> Self {
        frame_system::RawOrigin::None.into()
    }
    fn root() -> Self {
        frame_system::RawOrigin::Root.into()
    }
    fn signed(by: <Runtime as frame_system::Config>::AccountId) -> Self {
        frame_system::RawOrigin::Signed(by).into()
    }
}
#[allow(non_camel_case_types)]
pub enum OriginCaller {
    #[codec(index = 0u8)]
    system(frame_system::Origin<Runtime>),
    #[allow(dead_code)]
    Void(self::sp_api_hidden_includes_construct_runtime::hidden_include::Void),
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for OriginCaller {
    #[inline]
    fn clone(&self) -> OriginCaller {
        match (&*self,) {
            (&OriginCaller::system(ref __self_0),) => {
                OriginCaller::system(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&OriginCaller::Void(ref __self_0),) => {
                OriginCaller::Void(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralPartialEq for OriginCaller {}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for OriginCaller {
    #[inline]
    fn eq(&self, other: &OriginCaller) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&OriginCaller::system(ref __self_0), &OriginCaller::system(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&OriginCaller::Void(ref __self_0), &OriginCaller::Void(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &OriginCaller) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&OriginCaller::system(ref __self_0), &OriginCaller::system(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&OriginCaller::Void(ref __self_0), &OriginCaller::Void(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralEq for OriginCaller {}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::Eq for OriginCaller {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<frame_system::Origin<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<
                self::sp_api_hidden_includes_construct_runtime::hidden_include::Void,
            >;
        }
    }
}
impl core::fmt::Debug for OriginCaller {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::system(ref a0) => fmt.debug_tuple("OriginCaller::system").field(a0).finish(),
            Self::Void(ref a0) => fmt.debug_tuple("OriginCaller::Void").field(a0).finish(),
            _ => Ok(()),
        }
    }
}
const _: () = {
    #[allow(non_camel_case_types)]
    impl ::codec::Encode for OriginCaller {
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                OriginCaller::system(ref aa) => {
                    __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                OriginCaller::Void(ref aa) => {
                    __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    impl ::codec::EncodeLike for OriginCaller {}
};
const _: () = {
    #[allow(non_camel_case_types)]
    impl ::codec::Decode for OriginCaller {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy.read_byte().map_err(|e| {
                e.chain("Could not decode `OriginCaller`, failed to read variant byte")
            })? {
                __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(OriginCaller::system({
                        let __codec_res_edqy =
                            <frame_system::Origin<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `OriginCaller::system.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(OriginCaller::Void({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: Void as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `OriginCaller::Void.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                    "Could not decode `OriginCaller`, variant doesn't exist",
                )),
            }
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for OriginCaller {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            :: scale_info :: Type :: builder () . path (:: scale_info :: Path :: new ("OriginCaller" , "node_template_runtime")) . type_params (:: alloc :: vec :: Vec :: new ()) . docs (& []) . variant (:: scale_info :: build :: Variants :: new () . variant ("system" , | v | v . index (0u8 as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < frame_system :: Origin < Runtime > > () . type_name ("frame_system::Origin<Runtime>") . docs (& []))) . docs (& [])) . variant ("Void" , | v | v . index (1usize as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: Void > () . type_name ("self::sp_api_hidden_includes_construct_runtime::hidden_include::Void") . docs (& []))) . docs (& [])))
        }
    };
};
#[allow(dead_code)]
impl Origin {
    /// Create with system none origin and [`frame_system::Config::BaseCallFilter`].
    pub fn none() -> Self {
        < Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: none ()
    }
    /// Create with system root origin and [`frame_system::Config::BaseCallFilter`].
    pub fn root() -> Self {
        < Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: root ()
    }
    /// Create with system signed origin and [`frame_system::Config::BaseCallFilter`].
    pub fn signed(by: <Runtime as frame_system::Config>::AccountId) -> Self {
        < Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: signed (by)
    }
}
impl From<frame_system::Origin<Runtime>> for OriginCaller {
    fn from(x: frame_system::Origin<Runtime>) -> Self {
        OriginCaller::system(x)
    }
}
impl TryFrom<OriginCaller> for frame_system::Origin<Runtime> {
    type Error = OriginCaller;
    fn try_from(
        x: OriginCaller,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        frame_system::Origin<Runtime>,
        OriginCaller,
    > {
        if let OriginCaller::system(l) = x {
            Ok(l)
        } else {
            Err(x)
        }
    }
}
impl From<frame_system::Origin<Runtime>> for Origin {
    /// Convert to runtime origin, using as filter: [`frame_system::Config::BaseCallFilter`].
    fn from(x: frame_system::Origin<Runtime>) -> Self {
        let o: OriginCaller = x.into();
        o.into()
    }
}
impl From<OriginCaller> for Origin {
    fn from(x: OriginCaller) -> Self {
        let mut o = Origin {
            caller: x,
            filter:
                self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                    Box::new(|_| true),
                ),
        };
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait :: reset_filter (& mut o) ;
        o
    }
}
impl From<Origin>
    for self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        frame_system::Origin<Runtime>,
        Origin,
    >
{
    /// NOTE: converting to pallet origin loses the origin filter information.
    fn from(val: Origin) -> Self {
        if let OriginCaller::system(l) = val.caller {
            Ok(l)
        } else {
            Err(val)
        }
    }
}
impl From<Option<<Runtime as frame_system::Config>::AccountId>> for Origin {
    /// Convert to runtime origin with caller being system signed or none and use filter [`frame_system::Config::BaseCallFilter`].
    fn from(x: Option<<Runtime as frame_system::Config>::AccountId>) -> Self {
        <frame_system::Origin<Runtime>>::from(x).into()
    }
}
pub type System = frame_system::Pallet<Runtime>;
pub type RandomnessCollectiveFlip = pallet_randomness_collective_flip::Pallet<Runtime>;
pub type Timestamp = pallet_timestamp::Pallet<Runtime>;
pub type Aura = pallet_aura::Pallet<Runtime>;
pub type Grandpa = pallet_grandpa::Pallet<Runtime>;
pub type Balances = pallet_balances::Pallet<Runtime>;
pub type TransactionPayment = pallet_transaction_payment::Pallet<Runtime>;
pub type Sudo = pallet_sudo::Pallet<Runtime>;
pub type TemplateModule = pallet_template::Pallet<Runtime>;
pub type Nicks = pallet_nicks::Pallet<Runtime>;
pub type NodeAuthorization = pallet_node_authorization::Pallet<Runtime>;
pub type Scheduler = pallet_scheduler::Pallet<Runtime>;
/// All pallets included in the runtime as a nested tuple of types.
#[deprecated(
    note = "The type definition has changed from representing all pallets \
			excluding system, in reversed order to become the representation of all pallets \
			including system pallet in regular order. For this reason it is encouraged to use \
			explicitly one of `AllPalletsWithSystem`, `AllPalletsWithoutSystem`, \
			`AllPalletsWithSystemReversed`, `AllPalletsWithoutSystemReversed`. \
			Note that the type `frame_executive::Executive` expects one of `AllPalletsWithSystem` \
			, `AllPalletsWithSystemReversed`, `AllPalletsReversedWithSystemFirst`. More details in \
			https://github.com/paritytech/substrate/pull/10043"
)]
pub type AllPallets = AllPalletsWithSystem;
/// All pallets included in the runtime as a nested tuple of types.
pub type AllPalletsWithSystem = ((
    System,
    (
        RandomnessCollectiveFlip,
        (
            Timestamp,
            (
                Aura,
                (
                    Grandpa,
                    (
                        Balances,
                        (
                            TransactionPayment,
                            (
                                Sudo,
                                (TemplateModule, (Nicks, (NodeAuthorization, (Scheduler,)))),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    ),
));
/// All pallets included in the runtime as a nested tuple of types.
/// Excludes the System pallet.
pub type AllPalletsWithoutSystem = ((
    RandomnessCollectiveFlip,
    (
        Timestamp,
        (
            Aura,
            (
                Grandpa,
                (
                    Balances,
                    (
                        TransactionPayment,
                        (
                            Sudo,
                            (TemplateModule, (Nicks, (NodeAuthorization, (Scheduler,)))),
                        ),
                    ),
                ),
            ),
        ),
    ),
));
/// All pallets included in the runtime as a nested tuple of types in reversed order.
/// Excludes the System pallet.
pub type AllPalletsWithoutSystemReversed = ((
    Scheduler,
    (
        NodeAuthorization,
        (
            Nicks,
            (
                TemplateModule,
                (
                    Sudo,
                    (
                        TransactionPayment,
                        (
                            Balances,
                            (Grandpa, (Aura, (Timestamp, (RandomnessCollectiveFlip,)))),
                        ),
                    ),
                ),
            ),
        ),
    ),
));
/// All pallets included in the runtime as a nested tuple of types in reversed order.
pub type AllPalletsWithSystemReversed = ((
    Scheduler,
    (
        NodeAuthorization,
        (
            Nicks,
            (
                TemplateModule,
                (
                    Sudo,
                    (
                        TransactionPayment,
                        (
                            Balances,
                            (
                                Grandpa,
                                (Aura, (Timestamp, (RandomnessCollectiveFlip, (System,)))),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    ),
));
/// All pallets included in the runtime as a nested tuple of types in reversed order.
/// With the system pallet first.
pub type AllPalletsReversedWithSystemFirst = (System, AllPalletsWithoutSystemReversed);
/// Provides an implementation of `PalletInfo` to provide information
/// about the pallet setup in the runtime.
pub struct PalletInfo;
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfo
    for PalletInfo
{
    fn index<P: 'static>() -> Option<usize> {
        let type_id =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                P,
            >();
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < System > () { return Some (0usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < RandomnessCollectiveFlip > () { return Some (1usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Timestamp > () { return Some (2usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Aura > () { return Some (3usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Grandpa > () { return Some (4usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Balances > () { return Some (5usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < TransactionPayment > () { return Some (6usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Sudo > () { return Some (7usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < TemplateModule > () { return Some (8usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Nicks > () { return Some (9usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < NodeAuthorization > () { return Some (10usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Scheduler > () { return Some (11usize) }
        None
    }
    fn name<P: 'static>() -> Option<&'static str> {
        let type_id =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                P,
            >();
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < System > () { return Some ("System") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < RandomnessCollectiveFlip > () { return Some ("RandomnessCollectiveFlip") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Timestamp > () { return Some ("Timestamp") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Aura > () { return Some ("Aura") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Grandpa > () { return Some ("Grandpa") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Balances > () { return Some ("Balances") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < TransactionPayment > () { return Some ("TransactionPayment") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Sudo > () { return Some ("Sudo") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < TemplateModule > () { return Some ("TemplateModule") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Nicks > () { return Some ("Nicks") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < NodeAuthorization > () { return Some ("NodeAuthorization") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Scheduler > () { return Some ("Scheduler") }
        None
    }
    fn module_name<P: 'static>() -> Option<&'static str> {
        let type_id =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                P,
            >();
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < System > () { return Some ("frame_system") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < RandomnessCollectiveFlip > () { return Some ("pallet_randomness_collective_flip") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Timestamp > () { return Some ("pallet_timestamp") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Aura > () { return Some ("pallet_aura") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Grandpa > () { return Some ("pallet_grandpa") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Balances > () { return Some ("pallet_balances") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < TransactionPayment > () { return Some ("pallet_transaction_payment") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Sudo > () { return Some ("pallet_sudo") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < TemplateModule > () { return Some ("pallet_template") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Nicks > () { return Some ("pallet_nicks") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < NodeAuthorization > () { return Some ("pallet_node_authorization") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Scheduler > () { return Some ("pallet_scheduler") }
        None
    }
    fn crate_version<P: 'static>(
    ) -> Option<self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::CrateVersion>
    {
        let type_id =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                P,
            >();
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < System > () { return Some (< frame_system :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: PalletInfoAccess > :: crate_version ()) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < RandomnessCollectiveFlip > () { return Some (< pallet_randomness_collective_flip :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: PalletInfoAccess > :: crate_version ()) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Timestamp > () { return Some (< pallet_timestamp :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: PalletInfoAccess > :: crate_version ()) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Aura > () { return Some (< pallet_aura :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: PalletInfoAccess > :: crate_version ()) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Grandpa > () { return Some (< pallet_grandpa :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: PalletInfoAccess > :: crate_version ()) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Balances > () { return Some (< pallet_balances :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: PalletInfoAccess > :: crate_version ()) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < TransactionPayment > () { return Some (< pallet_transaction_payment :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: PalletInfoAccess > :: crate_version ()) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Sudo > () { return Some (< pallet_sudo :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: PalletInfoAccess > :: crate_version ()) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < TemplateModule > () { return Some (< pallet_template :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: PalletInfoAccess > :: crate_version ()) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Nicks > () { return Some (< pallet_nicks :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: PalletInfoAccess > :: crate_version ()) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < NodeAuthorization > () { return Some (< pallet_node_authorization :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: PalletInfoAccess > :: crate_version ()) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Scheduler > () { return Some (< pallet_scheduler :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: PalletInfoAccess > :: crate_version ()) }
        None
    }
}
pub enum Call {
    #[codec(index = 0u8)]
    System(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    ),
    #[codec(index = 2u8)]
    Timestamp(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Timestamp,
            Runtime,
        >,
    ),
    #[codec(index = 4u8)]
    Grandpa(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Grandpa,
            Runtime,
        >,
    ),
    #[codec(index = 5u8)]
    Balances(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Balances,
            Runtime,
        >,
    ),
    #[codec(index = 7u8)]
    Sudo(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Sudo,
            Runtime,
        >,
    ),
    #[codec(index = 8u8)]
    TemplateModule(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            TemplateModule,
            Runtime,
        >,
    ),
    #[codec(index = 9u8)]
    Nicks(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Nicks,
            Runtime,
        >,
    ),
    #[codec(index = 10u8)]
    NodeAuthorization(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            NodeAuthorization,
            Runtime,
        >,
    ),
    #[codec(index = 11u8)]
    Scheduler(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Scheduler,
            Runtime,
        >,
    ),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Call {
    #[inline]
    fn clone(&self) -> Call {
        match (&*self,) {
            (&Call::System(ref __self_0),) => {
                Call::System(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Timestamp(ref __self_0),) => {
                Call::Timestamp(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Grandpa(ref __self_0),) => {
                Call::Grandpa(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Balances(ref __self_0),) => {
                Call::Balances(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Sudo(ref __self_0),) => Call::Sudo(::core::clone::Clone::clone(&(*__self_0))),
            (&Call::TemplateModule(ref __self_0),) => {
                Call::TemplateModule(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Nicks(ref __self_0),) => Call::Nicks(::core::clone::Clone::clone(&(*__self_0))),
            (&Call::NodeAuthorization(ref __self_0),) => {
                Call::NodeAuthorization(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Scheduler(ref __self_0),) => {
                Call::Scheduler(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
impl ::core::marker::StructuralPartialEq for Call {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Call {
    #[inline]
    fn eq(&self, other: &Call) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Call::System(ref __self_0), &Call::System(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Timestamp(ref __self_0), &Call::Timestamp(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Grandpa(ref __self_0), &Call::Grandpa(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Balances(ref __self_0), &Call::Balances(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Sudo(ref __self_0), &Call::Sudo(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::TemplateModule(ref __self_0), &Call::TemplateModule(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Nicks(ref __self_0), &Call::Nicks(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (
                        &Call::NodeAuthorization(ref __self_0),
                        &Call::NodeAuthorization(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (&Call::Scheduler(ref __self_0), &Call::Scheduler(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Call) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Call::System(ref __self_0), &Call::System(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Timestamp(ref __self_0), &Call::Timestamp(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Grandpa(ref __self_0), &Call::Grandpa(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Balances(ref __self_0), &Call::Balances(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Sudo(ref __self_0), &Call::Sudo(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::TemplateModule(ref __self_0), &Call::TemplateModule(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Nicks(ref __self_0), &Call::Nicks(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (
                        &Call::NodeAuthorization(ref __self_0),
                        &Call::NodeAuthorization(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (&Call::Scheduler(ref __self_0), &Call::Scheduler(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
impl ::core::marker::StructuralEq for Call {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Call {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Timestamp , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Grandpa , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Balances , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Sudo , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < TemplateModule , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Nicks , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < NodeAuthorization , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Scheduler , Runtime > > ;
        }
    }
}
const _: () = {
    impl ::codec::Encode for Call {
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                Call::System(ref aa) => {
                    __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Timestamp(ref aa) => {
                    __codec_dest_edqy.push_byte(2u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Grandpa(ref aa) => {
                    __codec_dest_edqy.push_byte(4u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Balances(ref aa) => {
                    __codec_dest_edqy.push_byte(5u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Sudo(ref aa) => {
                    __codec_dest_edqy.push_byte(7u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::TemplateModule(ref aa) => {
                    __codec_dest_edqy.push_byte(8u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Nicks(ref aa) => {
                    __codec_dest_edqy.push_byte(9u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::NodeAuthorization(ref aa) => {
                    __codec_dest_edqy.push_byte(10u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::Scheduler(ref aa) => {
                    __codec_dest_edqy.push_byte(11u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    impl ::codec::EncodeLike for Call {}
};
const _: () = {
    impl ::codec::Decode for Call {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
            {
                __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::System({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::System.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 2u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Timestamp({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Timestamp , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Timestamp.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 4u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Grandpa({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Grandpa , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Grandpa.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 5u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Balances({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Balances , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Balances.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 7u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Sudo({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Sudo , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Sudo.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 8u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::TemplateModule({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < TemplateModule , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::TemplateModule.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 9u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Nicks({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Nicks , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Nicks.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 10u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::NodeAuthorization({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < NodeAuthorization , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::NodeAuthorization.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 11u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::Scheduler({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Scheduler , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::Scheduler.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                    "Could not decode `Call`, variant doesn't exist",
                )),
            }
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for Call {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            :: scale_info :: Type :: builder () . path (:: scale_info :: Path :: new ("Call" , "node_template_runtime")) . type_params (:: alloc :: vec :: Vec :: new ()) . docs (& []) . variant (:: scale_info :: build :: Variants :: new () . variant ("System" , | v | v . index (0u8 as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Runtime > > () . type_name ("self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<System, Runtime>") . docs (& []))) . docs (& [])) . variant ("Timestamp" , | v | v . index (2u8 as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Timestamp , Runtime > > () . type_name ("self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<Timestamp, Runtime>") . docs (& []))) . docs (& [])) . variant ("Grandpa" , | v | v . index (4u8 as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Grandpa , Runtime > > () . type_name ("self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<Grandpa, Runtime>") . docs (& []))) . docs (& [])) . variant ("Balances" , | v | v . index (5u8 as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Balances , Runtime > > () . type_name ("self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<Balances, Runtime>") . docs (& []))) . docs (& [])) . variant ("Sudo" , | v | v . index (7u8 as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Sudo , Runtime > > () . type_name ("self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<Sudo, Runtime>") . docs (& []))) . docs (& [])) . variant ("TemplateModule" , | v | v . index (8u8 as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < TemplateModule , Runtime > > () . type_name ("self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<TemplateModule, Runtime>") . docs (& []))) . docs (& [])) . variant ("Nicks" , | v | v . index (9u8 as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Nicks , Runtime > > () . type_name ("self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<Nicks, Runtime>") . docs (& []))) . docs (& [])) . variant ("NodeAuthorization" , | v | v . index (10u8 as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < NodeAuthorization , Runtime > > () . type_name ("self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<NodeAuthorization, Runtime>") . docs (& []))) . docs (& [])) . variant ("Scheduler" , | v | v . index (11u8 as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Scheduler , Runtime > > () . type_name ("self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<Scheduler, Runtime>") . docs (& []))) . docs (& [])))
        }
    };
};
impl core::fmt::Debug for Call {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::System(ref a0) => fmt.debug_tuple("Call::System").field(a0).finish(),
            Self::Timestamp(ref a0) => fmt.debug_tuple("Call::Timestamp").field(a0).finish(),
            Self::Grandpa(ref a0) => fmt.debug_tuple("Call::Grandpa").field(a0).finish(),
            Self::Balances(ref a0) => fmt.debug_tuple("Call::Balances").field(a0).finish(),
            Self::Sudo(ref a0) => fmt.debug_tuple("Call::Sudo").field(a0).finish(),
            Self::TemplateModule(ref a0) => {
                fmt.debug_tuple("Call::TemplateModule").field(a0).finish()
            }
            Self::Nicks(ref a0) => fmt.debug_tuple("Call::Nicks").field(a0).finish(),
            Self::NodeAuthorization(ref a0) => fmt
                .debug_tuple("Call::NodeAuthorization")
                .field(a0)
                .finish(),
            Self::Scheduler(ref a0) => fmt.debug_tuple("Call::Scheduler").field(a0).finish(),
            _ => Ok(()),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetDispatchInfo
    for Call
{
    fn get_dispatch_info(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchInfo
    {
        match self {
            Call::System(call) => call.get_dispatch_info(),
            Call::Timestamp(call) => call.get_dispatch_info(),
            Call::Grandpa(call) => call.get_dispatch_info(),
            Call::Balances(call) => call.get_dispatch_info(),
            Call::Sudo(call) => call.get_dispatch_info(),
            Call::TemplateModule(call) => call.get_dispatch_info(),
            Call::Nicks(call) => call.get_dispatch_info(),
            Call::NodeAuthorization(call) => call.get_dispatch_info(),
            Call::Scheduler(call) => call.get_dispatch_info(),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetCallMetadata
    for Call
{
    fn get_call_metadata(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata
    {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetCallName;
        match self {
            Call::System(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "System";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Timestamp(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Timestamp";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Grandpa(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Grandpa";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Balances(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Balances";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Sudo(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Sudo";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::TemplateModule(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "TemplateModule";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Nicks(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Nicks";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::NodeAuthorization(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "NodeAuthorization";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::Scheduler(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "Scheduler";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
        }
    }
    fn get_module_names() -> &'static [&'static str] {
        &[
            "System",
            "Timestamp",
            "Grandpa",
            "Balances",
            "Sudo",
            "TemplateModule",
            "Nicks",
            "NodeAuthorization",
            "Scheduler",
        ]
    }
    fn get_call_names(module: &str) -> &'static [&'static str] {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::{
            Callable, GetCallName,
        };
        match module {
            "System" => <<System as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "Timestamp" => {
                <<Timestamp as Callable<Runtime>>::Call as GetCallName>::get_call_names()
            }
            "Grandpa" => <<Grandpa as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "Balances" => <<Balances as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "Sudo" => <<Sudo as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "TemplateModule" => {
                <<TemplateModule as Callable<Runtime>>::Call as GetCallName>::get_call_names()
            }
            "Nicks" => <<Nicks as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "NodeAuthorization" => {
                <<NodeAuthorization as Callable<Runtime>>::Call as GetCallName>::get_call_names()
            }
            "Scheduler" => {
                <<Scheduler as Callable<Runtime>>::Call as GetCallName>::get_call_names()
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::Dispatchable
    for Call
{
    type Origin = Origin;
    type Config = Call;
    type Info =
        self::sp_api_hidden_includes_construct_runtime::hidden_include::weights::DispatchInfo;
    type PostInfo =
        self::sp_api_hidden_includes_construct_runtime::hidden_include::weights::PostDispatchInfo;    fn dispatch (self , origin : Origin) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: DispatchResultWithPostInfo{
        if ! < Self :: Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: filter_call (& origin , & self) { return self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: result :: Result :: Err (frame_system :: Error :: < Runtime > :: CallFiltered . into ()) ; }
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (self , origin)
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable
    for Call
{
    type Origin = Origin;    fn dispatch_bypass_filter (self , origin : Origin) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: DispatchResultWithPostInfo{
        match self { Call :: System (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Timestamp (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Grandpa (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Balances (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Sudo (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: TemplateModule (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Nicks (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: NodeAuthorization (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: Scheduler (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , }
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    > {
        match self {
            Call::System(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Runtime >,
    ) -> Self {
        Call::System(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Timestamp,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Timestamp,
            Runtime,
        >,
    > {
        match self {
            Call::Timestamp(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Timestamp,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Timestamp , Runtime >,
    ) -> Self {
        Call::Timestamp(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Grandpa,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Grandpa,
            Runtime,
        >,
    > {
        match self {
            Call::Grandpa(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Grandpa,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Grandpa , Runtime >,
    ) -> Self {
        Call::Grandpa(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Balances,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Balances,
            Runtime,
        >,
    > {
        match self {
            Call::Balances(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Balances,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Balances , Runtime >,
    ) -> Self {
        Call::Balances(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Sudo,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Sudo,
            Runtime,
        >,
    > {
        match self {
            Call::Sudo(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Sudo,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Sudo , Runtime >,
    ) -> Self {
        Call::Sudo(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            TemplateModule,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            TemplateModule,
            Runtime,
        >,
    > {
        match self {
            Call::TemplateModule(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            TemplateModule,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < TemplateModule , Runtime >,
    ) -> Self {
        Call::TemplateModule(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Nicks,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Nicks,
            Runtime,
        >,
    > {
        match self {
            Call::Nicks(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Nicks,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Nicks , Runtime >,
    ) -> Self {
        Call::Nicks(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            NodeAuthorization,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            NodeAuthorization,
            Runtime,
        >,
    > {
        match self {
            Call::NodeAuthorization(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            NodeAuthorization,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < NodeAuthorization , Runtime >,
    ) -> Self {
        Call::NodeAuthorization(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Scheduler,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Scheduler,
            Runtime,
        >,
    > {
        match self {
            Call::Scheduler(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            Scheduler,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < Scheduler , Runtime >,
    ) -> Self {
        Call::Scheduler(call)
    }
}
impl Runtime {
    pub fn metadata () -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: RuntimeMetadataPrefixed{
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: RuntimeMetadataLastVersion :: new (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletMetadata { name : "System" , index : 0u8 , storage : Some (frame_system :: Pallet :: < Runtime > :: storage_metadata ()) , calls : Some (frame_system :: Pallet :: < Runtime > :: call_functions ()) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletEventMetadata { ty : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: scale_info :: meta_type :: < frame_system :: Event < Runtime > > () , }) , constants : frame_system :: Pallet :: < Runtime > :: pallet_constants_metadata () , error : frame_system :: Pallet :: < Runtime > :: error_metadata () , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletMetadata { name : "RandomnessCollectiveFlip" , index : 1u8 , storage : Some (pallet_randomness_collective_flip :: Pallet :: < Runtime > :: storage_metadata ()) , calls : None , event : None , constants : pallet_randomness_collective_flip :: Pallet :: < Runtime > :: pallet_constants_metadata () , error : pallet_randomness_collective_flip :: Pallet :: < Runtime > :: error_metadata () , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletMetadata { name : "Timestamp" , index : 2u8 , storage : Some (pallet_timestamp :: Pallet :: < Runtime > :: storage_metadata ()) , calls : Some (pallet_timestamp :: Pallet :: < Runtime > :: call_functions ()) , event : None , constants : pallet_timestamp :: Pallet :: < Runtime > :: pallet_constants_metadata () , error : pallet_timestamp :: Pallet :: < Runtime > :: error_metadata () , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletMetadata { name : "Aura" , index : 3u8 , storage : Some (pallet_aura :: Pallet :: < Runtime > :: storage_metadata ()) , calls : None , event : None , constants : pallet_aura :: Pallet :: < Runtime > :: pallet_constants_metadata () , error : pallet_aura :: Pallet :: < Runtime > :: error_metadata () , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletMetadata { name : "Grandpa" , index : 4u8 , storage : Some (pallet_grandpa :: Pallet :: < Runtime > :: storage_metadata ()) , calls : Some (pallet_grandpa :: Pallet :: < Runtime > :: call_functions ()) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletEventMetadata { ty : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: scale_info :: meta_type :: < pallet_grandpa :: Event > () , }) , constants : pallet_grandpa :: Pallet :: < Runtime > :: pallet_constants_metadata () , error : pallet_grandpa :: Pallet :: < Runtime > :: error_metadata () , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletMetadata { name : "Balances" , index : 5u8 , storage : Some (pallet_balances :: Pallet :: < Runtime > :: storage_metadata ()) , calls : Some (pallet_balances :: Pallet :: < Runtime > :: call_functions ()) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletEventMetadata { ty : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: scale_info :: meta_type :: < pallet_balances :: Event < Runtime > > () , }) , constants : pallet_balances :: Pallet :: < Runtime > :: pallet_constants_metadata () , error : pallet_balances :: Pallet :: < Runtime > :: error_metadata () , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletMetadata { name : "TransactionPayment" , index : 6u8 , storage : Some (pallet_transaction_payment :: Pallet :: < Runtime > :: storage_metadata ()) , calls : None , event : None , constants : pallet_transaction_payment :: Pallet :: < Runtime > :: pallet_constants_metadata () , error : pallet_transaction_payment :: Pallet :: < Runtime > :: error_metadata () , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletMetadata { name : "Sudo" , index : 7u8 , storage : Some (pallet_sudo :: Pallet :: < Runtime > :: storage_metadata ()) , calls : Some (pallet_sudo :: Pallet :: < Runtime > :: call_functions ()) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletEventMetadata { ty : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: scale_info :: meta_type :: < pallet_sudo :: Event < Runtime > > () , }) , constants : pallet_sudo :: Pallet :: < Runtime > :: pallet_constants_metadata () , error : pallet_sudo :: Pallet :: < Runtime > :: error_metadata () , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletMetadata { name : "TemplateModule" , index : 8u8 , storage : Some (pallet_template :: Pallet :: < Runtime > :: storage_metadata ()) , calls : Some (pallet_template :: Pallet :: < Runtime > :: call_functions ()) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletEventMetadata { ty : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: scale_info :: meta_type :: < pallet_template :: Event < Runtime > > () , }) , constants : pallet_template :: Pallet :: < Runtime > :: pallet_constants_metadata () , error : pallet_template :: Pallet :: < Runtime > :: error_metadata () , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletMetadata { name : "Nicks" , index : 9u8 , storage : Some (pallet_nicks :: Pallet :: < Runtime > :: storage_metadata ()) , calls : Some (pallet_nicks :: Pallet :: < Runtime > :: call_functions ()) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletEventMetadata { ty : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: scale_info :: meta_type :: < pallet_nicks :: Event < Runtime > > () , }) , constants : pallet_nicks :: Pallet :: < Runtime > :: pallet_constants_metadata () , error : pallet_nicks :: Pallet :: < Runtime > :: error_metadata () , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletMetadata { name : "NodeAuthorization" , index : 10u8 , storage : Some (pallet_node_authorization :: Pallet :: < Runtime > :: storage_metadata ()) , calls : Some (pallet_node_authorization :: Pallet :: < Runtime > :: call_functions ()) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletEventMetadata { ty : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: scale_info :: meta_type :: < pallet_node_authorization :: Event < Runtime > > () , }) , constants : pallet_node_authorization :: Pallet :: < Runtime > :: pallet_constants_metadata () , error : pallet_node_authorization :: Pallet :: < Runtime > :: error_metadata () , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletMetadata { name : "Scheduler" , index : 11u8 , storage : Some (pallet_scheduler :: Pallet :: < Runtime > :: storage_metadata ()) , calls : Some (pallet_scheduler :: Pallet :: < Runtime > :: call_functions ()) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletEventMetadata { ty : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: scale_info :: meta_type :: < pallet_scheduler :: Event < Runtime > > () , }) , constants : pallet_scheduler :: Pallet :: < Runtime > :: pallet_constants_metadata () , error : pallet_scheduler :: Pallet :: < Runtime > :: error_metadata () , }])) , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ExtrinsicMetadata { ty : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: scale_info :: meta_type :: < UncheckedExtrinsic > () , version : < UncheckedExtrinsic as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: ExtrinsicMetadata > :: VERSION , signed_extensions : < < UncheckedExtrinsic as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: ExtrinsicMetadata > :: SignedExtensions as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: SignedExtension > :: metadata () . into_iter () . map (| meta | self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: SignedExtensionMetadata { identifier : meta . identifier , ty : meta . ty , additional_signed : meta . additional_signed , }) . collect () , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: scale_info :: meta_type :: < Runtime > ()) . into ()
    }
}
#[cfg(any(feature = "std", test))]
pub type SystemConfig = frame_system::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type AuraConfig = pallet_aura::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type GrandpaConfig = pallet_grandpa::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type BalancesConfig = pallet_balances::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type TransactionPaymentConfig = pallet_transaction_payment::GenesisConfig;
#[cfg(any(feature = "std", test))]
pub type SudoConfig = pallet_sudo::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type NodeAuthorizationConfig = pallet_node_authorization::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
use self::sp_api_hidden_includes_construct_runtime::hidden_include::serde as __genesis_config_serde_import__;
#[cfg(any(feature = "std", test))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(crate = "__genesis_config_serde_import__")]
pub struct GenesisConfig {
    pub system: SystemConfig,
    pub aura: AuraConfig,
    pub grandpa: GrandpaConfig,
    pub balances: BalancesConfig,
    pub transaction_payment: TransactionPaymentConfig,
    pub sudo: SudoConfig,
    pub node_authorization: NodeAuthorizationConfig,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use __genesis_config_serde_import__ as _serde;
    #[automatically_derived]
    impl __genesis_config_serde_import__::Serialize for GenesisConfig {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> __genesis_config_serde_import__::__private::Result<__S::Ok, __S::Error>
        where
            __S: __genesis_config_serde_import__::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "GenesisConfig",
                false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "system",
                &self.system,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "aura",
                &self.aura,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "grandpa",
                &self.grandpa,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "balances",
                &self.balances,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "transactionPayment",
                &self.transaction_payment,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "sudo",
                &self.sudo,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "nodeAuthorization",
                &self.node_authorization,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use __genesis_config_serde_import__ as _serde;
    #[automatically_derived]
    impl<'de> __genesis_config_serde_import__::Deserialize<'de> for GenesisConfig {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> __genesis_config_serde_import__::__private::Result<Self, __D::Error>
        where
            __D: __genesis_config_serde_import__::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        4u64 => _serde::__private::Ok(__Field::__field4),
                        5u64 => _serde::__private::Ok(__Field::__field5),
                        6u64 => _serde::__private::Ok(__Field::__field6),
                        _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"field index 0 <= i < 7",
                        )),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "system" => _serde::__private::Ok(__Field::__field0),
                        "aura" => _serde::__private::Ok(__Field::__field1),
                        "grandpa" => _serde::__private::Ok(__Field::__field2),
                        "balances" => _serde::__private::Ok(__Field::__field3),
                        "transactionPayment" => _serde::__private::Ok(__Field::__field4),
                        "sudo" => _serde::__private::Ok(__Field::__field5),
                        "nodeAuthorization" => _serde::__private::Ok(__Field::__field6),
                        _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                            __value, FIELDS,
                        )),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"system" => _serde::__private::Ok(__Field::__field0),
                        b"aura" => _serde::__private::Ok(__Field::__field1),
                        b"grandpa" => _serde::__private::Ok(__Field::__field2),
                        b"balances" => _serde::__private::Ok(__Field::__field3),
                        b"transactionPayment" => _serde::__private::Ok(__Field::__field4),
                        b"sudo" => _serde::__private::Ok(__Field::__field5),
                        b"nodeAuthorization" => _serde::__private::Ok(__Field::__field6),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            ))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<GenesisConfig>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = GenesisConfig;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct GenesisConfig")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<SystemConfig>(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct GenesisConfig with 7 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<AuraConfig>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct GenesisConfig with 7 elements",
                                ));
                            }
                        };
                    let __field2 = match match _serde::de::SeqAccess::next_element::<GrandpaConfig>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                2usize,
                                &"struct GenesisConfig with 7 elements",
                            ));
                        }
                    };
                    let __field3 = match match _serde::de::SeqAccess::next_element::<BalancesConfig>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                3usize,
                                &"struct GenesisConfig with 7 elements",
                            ));
                        }
                    };
                    let __field4 = match match _serde::de::SeqAccess::next_element::<
                        TransactionPaymentConfig,
                    >(&mut __seq)
                    {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                4usize,
                                &"struct GenesisConfig with 7 elements",
                            ));
                        }
                    };
                    let __field5 =
                        match match _serde::de::SeqAccess::next_element::<SudoConfig>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct GenesisConfig with 7 elements",
                                ));
                            }
                        };
                    let __field6 = match match _serde::de::SeqAccess::next_element::<
                        NodeAuthorizationConfig,
                    >(&mut __seq)
                    {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                6usize,
                                &"struct GenesisConfig with 7 elements",
                            ));
                        }
                    };
                    _serde::__private::Ok(GenesisConfig {
                        system: __field0,
                        aura: __field1,
                        grandpa: __field2,
                        balances: __field3,
                        transaction_payment: __field4,
                        sudo: __field5,
                        node_authorization: __field6,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<SystemConfig> =
                        _serde::__private::None;
                    let mut __field1: _serde::__private::Option<AuraConfig> =
                        _serde::__private::None;
                    let mut __field2: _serde::__private::Option<GrandpaConfig> =
                        _serde::__private::None;
                    let mut __field3: _serde::__private::Option<BalancesConfig> =
                        _serde::__private::None;
                    let mut __field4: _serde::__private::Option<TransactionPaymentConfig> =
                        _serde::__private::None;
                    let mut __field5: _serde::__private::Option<SudoConfig> =
                        _serde::__private::None;
                    let mut __field6: _serde::__private::Option<NodeAuthorizationConfig> =
                        _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "system",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<SystemConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("aura"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<AuraConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "grandpa",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<GrandpaConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private::Option::is_some(&__field3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "balances",
                                        ),
                                    );
                                }
                                __field3 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<BalancesConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field4 => {
                                if _serde::__private::Option::is_some(&__field4) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transactionPayment",
                                        ),
                                    );
                                }
                                __field4 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<
                                        TransactionPaymentConfig,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field5 => {
                                if _serde::__private::Option::is_some(&__field5) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("sudo"),
                                    );
                                }
                                __field5 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<SudoConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field6 => {
                                if _serde::__private::Option::is_some(&__field6) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nodeAuthorization",
                                        ),
                                    );
                                }
                                __field6 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<NodeAuthorizationConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("system") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("aura") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("grandpa") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field3) => __field3,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("balances") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field4 = match __field4 {
                        _serde::__private::Some(__field4) => __field4,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("transactionPayment") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field5 = match __field5 {
                        _serde::__private::Some(__field5) => __field5,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("sudo") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field6 = match __field6 {
                        _serde::__private::Some(__field6) => __field6,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("nodeAuthorization") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(GenesisConfig {
                        system: __field0,
                        aura: __field1,
                        grandpa: __field2,
                        balances: __field3,
                        transaction_payment: __field4,
                        sudo: __field5,
                        node_authorization: __field6,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &[
                "system",
                "aura",
                "grandpa",
                "balances",
                "transactionPayment",
                "sudo",
                "nodeAuthorization",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "GenesisConfig",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<GenesisConfig>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for GenesisConfig {
    #[inline]
    fn default() -> GenesisConfig {
        GenesisConfig {
            system: ::core::default::Default::default(),
            aura: ::core::default::Default::default(),
            grandpa: ::core::default::Default::default(),
            balances: ::core::default::Default::default(),
            transaction_payment: ::core::default::Default::default(),
            sudo: ::core::default::Default::default(),
            node_authorization: ::core::default::Default::default(),
        }
    }
}
#[cfg(any(feature = "std", test))]
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildStorage
    for GenesisConfig
{
    fn assimilate_storage(
        &self,
        storage : & mut self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: Storage,
    ) -> std::result::Result<(), String> {
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , frame_system :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . system , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_aura :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . aura , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_grandpa :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . grandpa , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_balances :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . balances , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_transaction_payment :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . transaction_payment , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_sudo :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . sudo , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , pallet_node_authorization :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . node_authorization , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: BasicExternalities :: execute_with_storage (storage , | | { < AllPalletsWithSystem as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OnGenesis > :: on_genesis () ; }) ;
        Ok(())
    }
}
trait InherentDataExt {
    fn create_extrinsics (& self) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Vec < < Block as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: BlockT > :: Extrinsic > ;
    fn check_extrinsics (& self , block : & Block) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: CheckInherentsResult ;
}
impl InherentDataExt
    for self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::InherentData
{
    fn create_extrinsics (& self) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Vec < < Block as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: BlockT > :: Extrinsic >{
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::ProvideInherent;
        let mut inherents = Vec::new();
        if let Some(inherent) = Timestamp::create_inherent(self) {
            let inherent = < UncheckedExtrinsic as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Extrinsic > :: new (inherent . into () , None) . expect ("Runtime UncheckedExtrinsic is not Opaque, so it has to return \
							`Some`; qed") ;
            inherents.push(inherent);
        }
        inherents
    }    fn check_extrinsics (& self , block : & Block) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: CheckInherentsResult{
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::{
            ProvideInherent, IsFatalError,
        };
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::{
            IsSubType, ExtrinsicCall,
        };
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block as _;
        let mut result = self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: CheckInherentsResult :: new () ;
        for xt in block.extrinsics() {
            if self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Extrinsic :: is_signed (xt) . unwrap_or (false) { break }
            let mut is_inherent = false;
            {
                let call = <UncheckedExtrinsic as ExtrinsicCall>::call(xt);
                if let Some(call) = IsSubType::<_>::is_sub_type(call) {
                    if Timestamp::is_inherent(call) {
                        is_inherent = true;
                        if let Err(e) = Timestamp::check_inherent(call, self) {
                            result
                                .put_error(Timestamp::INHERENT_IDENTIFIER, &e)
                                .expect("There is only one fatal error; qed");
                            if e.is_fatal_error() {
                                return result;
                            }
                        }
                    }
                }
            }
            if !is_inherent {
                break;
            }
        }
        match Timestamp::is_inherent_required(self) {
            Ok(Some(e)) => {
                let found = block . extrinsics () . iter () . any (| xt | { let is_signed = self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Extrinsic :: is_signed (xt) . unwrap_or (false) ; if ! is_signed { let call = < UncheckedExtrinsic as ExtrinsicCall > :: call (xt) ; if let Some (call) = IsSubType :: < _ > :: is_sub_type (call) { Timestamp :: is_inherent (& call) } else { false } } else { false } }) ;
                if !found {
                    result
                        .put_error(Timestamp::INHERENT_IDENTIFIER, &e)
                        .expect("There is only one fatal error; qed");
                    if e.is_fatal_error() {
                        return result;
                    }
                }
            }
            Ok(None) => (),
            Err(e) => {
                result
                    .put_error(Timestamp::INHERENT_IDENTIFIER, &e)
                    .expect("There is only one fatal error; qed");
                if e.is_fatal_error() {
                    return result;
                }
            }
        }
        result
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::EnsureInherentsAreFirst<
        Block,
    > for Runtime
{
    fn ensure_inherents_are_first(block: &Block) -> Result<(), u32> {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::ProvideInherent;
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::{
            IsSubType, ExtrinsicCall,
        };
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block as _;
        let mut first_signed_observed = false;
        for (i, xt) in block.extrinsics().iter().enumerate() {
            let is_signed = self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Extrinsic :: is_signed (xt) . unwrap_or (false) ;
            let is_inherent = if is_signed {
                false
            } else {
                let mut is_inherent = false;
                {
                    let call = <UncheckedExtrinsic as ExtrinsicCall>::call(xt);
                    if let Some(call) = IsSubType::<_>::is_sub_type(call) {
                        if Timestamp::is_inherent(&call) {
                            is_inherent = true;
                        }
                    }
                }
                is_inherent
            };
            if !is_inherent {
                first_signed_observed = true;
            }
            if first_signed_observed && is_inherent {
                return Err(i as u32);
            }
        }
        Ok(())
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::ValidateUnsigned
    for Runtime
{
    type Call = Call;    fn pre_dispatch (call : & Self :: Call) -> Result < () , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: unsigned :: TransactionValidityError >{
        #[allow(unreachable_patterns)]
        match call {
            Call::Grandpa(inner_call) => Grandpa::pre_dispatch(inner_call),
            _ => Ok(()),
        }
    }
    fn validate_unsigned(
        # [allow (unused_variables)] source : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: unsigned :: TransactionSource,
        call: &Self::Call,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::TransactionValidity
    {
        # [allow (unreachable_patterns)] match call { Call :: Grandpa (inner_call) => Grandpa :: validate_unsigned (source , inner_call) , _ => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: unsigned :: UnknownTransaction :: NoUnsignedValidator . into () , }
    }
}
const _: () =
    if !(<frame_system::Error<Runtime> as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
        <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
    {
        :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["The maximum encoded size of the error type in the `System` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"] , & []))
    };
const _: () =
    if !(<pallet_grandpa::Error<Runtime> as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
        <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
    {
        :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["The maximum encoded size of the error type in the `Grandpa` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"] , & []))
    };
const _: () =
    if !(<pallet_balances::Error<Runtime> as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
        <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
    {
        :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["The maximum encoded size of the error type in the `Balances` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"] , & []))
    };
const _: () =
    if !(<pallet_sudo::Error<Runtime> as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
        <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
    {
        :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["The maximum encoded size of the error type in the `Sudo` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"] , & []))
    };
const _: () =
    if !(<pallet_template::Error<Runtime> as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
        <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
    {
        :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["The maximum encoded size of the error type in the `TemplateModule` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"] , & []))
    };
const _: () =
    if !(<pallet_nicks::Error<Runtime> as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
        <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
    {
        :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["The maximum encoded size of the error type in the `Nicks` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"] , & []))
    };
const _ : () = if ! (< pallet_node_authorization :: Error < Runtime > as :: frame_support :: traits :: PalletError > :: MAX_ENCODED_SIZE <= :: frame_support :: MAX_MODULE_ERROR_ENCODED_SIZE) { :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["The maximum encoded size of the error type in the `NodeAuthorization` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"] , & [])) } ;
const _ : () = if ! (< pallet_scheduler :: Error < Runtime > as :: frame_support :: traits :: PalletError > :: MAX_ENCODED_SIZE <= :: frame_support :: MAX_MODULE_ERROR_ENCODED_SIZE) { :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["The maximum encoded size of the error type in the `Scheduler` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"] , & [])) } ;
/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;
#[doc(hidden)]
mod sp_api_hidden_includes_IMPL_RUNTIME_APIS {
    pub extern crate sp_api as sp_api;
}
pub struct RuntimeApi {}
/// Implements all runtime apis for the client side.
#[cfg(any(feature = "std", test))]
pub struct RuntimeApiImpl<
    Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
    C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block> + 'static,
> {
    call: &'static C,
    commit_on_success: std::cell::RefCell<bool>,
    changes: std::cell::RefCell<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::OverlayedChanges,
    >,
    storage_transaction_cache: std::cell::RefCell<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageTransactionCache<
            Block,
            C::StateBackend,
        >,
    >,
    recorder: std::option::Option<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ProofRecorder<Block>,
    >,
}
#[cfg(any(feature = "std", test))]
unsafe impl<
        Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
        C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
    > Send for RuntimeApiImpl<Block, C>
{
}
#[cfg(any(feature = "std", test))]
unsafe impl<
        Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
        C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
    > Sync for RuntimeApiImpl<Block, C>
{
}
#[cfg(any(feature = "std", test))]
impl<
        Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
        C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
    > self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiExt<Block>
    for RuntimeApiImpl<Block, C>
{
    type StateBackend = C::StateBackend;
    fn execute_in_transaction<
        F: FnOnce(
            &Self,
        )
            -> self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::TransactionOutcome<R>,
        R,
    >(
        &self,
        call: F,
    ) -> R
    where
        Self: Sized,
    {
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::OverlayedChanges::start_transaction(
            &mut std::cell::RefCell::borrow_mut(&self.changes),
        );
        *std::cell::RefCell::borrow_mut(&self.commit_on_success) = false;
        let res = call(self);
        *std::cell::RefCell::borrow_mut(&self.commit_on_success) = true;
        self.commit_or_rollback(match res {
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::TransactionOutcome::Commit(
                _,
            ) => true,
            _ => false,
        });
        res.into_inner()
    }
    fn has_api<A: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::RuntimeApiInfo + ?Sized>(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<Block>,
    ) -> std::result::Result<bool, self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError>
    where
        Self: Sized,
    {
        self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: CallApiAt :: < Block > :: runtime_version_at (self . call , at) . map (| v | self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: RuntimeVersion :: has_api_with (& v , & A :: ID , | v | v == A :: VERSION))
    }
    fn has_api_with<
        A: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::RuntimeApiInfo + ?Sized,
        P: Fn(u32) -> bool,
    >(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<Block>,
        pred: P,
    ) -> std::result::Result<bool, self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError>
    where
        Self: Sized,
    {
        self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: CallApiAt :: < Block > :: runtime_version_at (self . call , at) . map (| v | self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: RuntimeVersion :: has_api_with (& v , & A :: ID , pred))
    }
    fn api_version<
        A: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::RuntimeApiInfo + ?Sized,
    >(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<Block>,
    ) -> std::result::Result<
        Option<u32>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    >
    where
        Self: Sized,
    {
        self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: CallApiAt :: < Block > :: runtime_version_at (self . call , at) . map (| v | self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: RuntimeVersion :: api_version (& v , & A :: ID))
    }
    fn record_proof(&mut self) {
        self.recorder = std::option::Option::Some(std::default::Default::default());
    }
    fn proof_recorder(
        &self,
    ) -> std::option::Option<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ProofRecorder<Block>,
    > {
        std::clone::Clone::clone(&self.recorder)
    }
    fn extract_proof(
        &mut self,
    ) -> std::option::Option<self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageProof>
    {
        std :: option :: Option :: take (& mut self . recorder) . map (| recorder | self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: ProofRecorder :: < Block > :: to_storage_proof (& recorder))
    }
    fn into_storage_changes(
        &self,
        backend: &Self::StateBackend,
        parent_hash: Block::Hash,
    ) -> core::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageChanges<
            C::StateBackend,
            Block,
        >,
        String,
    >
    where
        Self: Sized,
    {
        let at = self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId::Hash(
            std::clone::Clone::clone(&parent_hash),
        );
        let state_version = self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt::<
            Block,
        >::runtime_version_at(self.call, &at)
        .map(|v| {
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::RuntimeVersion::state_version(
                &v,
            )
        })
        .map_err(|e| {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["Failed to get state version: "],
                &[::core::fmt::ArgumentV1::new_display(&e)],
            ));
            res
        })?;
        self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: OverlayedChanges :: into_storage_changes (std :: cell :: RefCell :: take (& self . changes) , backend , core :: cell :: RefCell :: take (& self . storage_transaction_cache) , state_version)
    }
}
#[cfg(any(feature = "std", test))]
impl<Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT, C>
    self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ConstructRuntimeApi<Block, C>
    for RuntimeApi
where
    C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block> + 'static,
{
    type RuntimeApi = RuntimeApiImpl<Block, C>;
    fn construct_runtime_api<'a>(
        call: &'a C,
    ) -> self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiRef<'a, Self::RuntimeApi> {
        RuntimeApiImpl {
            call: unsafe { std::mem::transmute(call) },
            commit_on_success: true.into(),
            changes: std::default::Default::default(),
            recorder: std::default::Default::default(),
            storage_transaction_cache: std::default::Default::default(),
        }
        .into()
    }
}
#[cfg(any(feature = "std", test))]
impl<
        Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
        C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
    > RuntimeApiImpl<Block, C>
{
    fn call_api_at<
        R: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode
            + self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Decode
            + std::cmp::PartialEq,
        F: FnOnce(
            &C,
            &std::cell::RefCell<
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::OverlayedChanges,
            >,
            &std::cell::RefCell<
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageTransactionCache<
                    Block,
                    C::StateBackend,
                >,
            >,
            &std::option::Option<
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ProofRecorder<Block>,
            >,
        ) -> std::result::Result<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<R>,
            E,
        >,
        E,
    >(
        &self,
        call_api_at: F,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<R>,
        E,
    > {
        if *std::cell::RefCell::borrow(&self.commit_on_success) {
            self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: OverlayedChanges :: start_transaction (& mut std :: cell :: RefCell :: borrow_mut (& self . changes)) ;
        }
        let res = call_api_at(
            &self.call,
            &self.changes,
            &self.storage_transaction_cache,
            &self.recorder,
        );
        self.commit_or_rollback(std::result::Result::is_ok(&res));
        res
    }
    fn commit_or_rollback(&self, commit: bool) {
        let proof = "\
					We only close a transaction when we opened one ourself.
					Other parts of the runtime that make use of transactions (state-machine)
					also balance their transactions. The runtime cannot close client initiated
					transactions; qed";
        if *std::cell::RefCell::borrow(&self.commit_on_success) {
            let res = if commit {
                self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: OverlayedChanges :: commit_transaction (& mut std :: cell :: RefCell :: borrow_mut (& self . changes))
            } else {
                self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: OverlayedChanges :: rollback_transaction (& mut std :: cell :: RefCell :: borrow_mut (& self . changes))
            };
            std::result::Result::expect(res, proof);
        }
    }
}
impl sp_api::runtime_decl_for_Core::Core<Block> for Runtime {
    fn version() -> RuntimeVersion {
        VERSION
    }
    fn execute_block(block: Block) {
        Executive::execute_block(block);
    }
    fn initialize_block(header: &<Block as BlockT>::Header) {
        Executive::initialize_block(header)
    }
}
impl sp_api::runtime_decl_for_Metadata::Metadata<Block> for Runtime {
    fn metadata() -> OpaqueMetadata {
        OpaqueMetadata::new(Runtime::metadata().into())
    }
}
impl sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<Block> for Runtime {
    fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
        Executive::apply_extrinsic(extrinsic)
    }
    fn finalize_block() -> <Block as BlockT>::Header {
        Executive::finalize_block()
    }
    fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
        data.create_extrinsics()
    }
    fn check_inherents(
        block: Block,
        data: sp_inherents::InherentData,
    ) -> sp_inherents::CheckInherentsResult {
        data.check_extrinsics(&block)
    }
}
impl sp_transaction_pool :: runtime_api :: runtime_decl_for_TaggedTransactionQueue :: TaggedTransactionQueue < Block > for Runtime { fn validate_transaction (source : TransactionSource , tx : < Block as BlockT > :: Extrinsic , block_hash : < Block as BlockT > :: Hash) -> TransactionValidity { Executive :: validate_transaction (source , tx , block_hash) } }
impl sp_offchain::runtime_decl_for_OffchainWorkerApi::OffchainWorkerApi<Block> for Runtime {
    fn offchain_worker(header: &<Block as BlockT>::Header) {
        Executive::offchain_worker(header)
    }
}
impl sp_consensus_aura::runtime_decl_for_AuraApi::AuraApi<Block, AuraId> for Runtime {
    fn slot_duration() -> sp_consensus_aura::SlotDuration {
        sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
    }
    fn authorities() -> Vec<AuraId> {
        Aura::authorities().into_inner()
    }
}
impl sp_session::runtime_decl_for_SessionKeys::SessionKeys<Block> for Runtime {
    fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
        opaque::SessionKeys::generate(seed)
    }
    fn decode_session_keys(encoded: Vec<u8>) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
        opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
    }
}
impl fg_primitives::runtime_decl_for_GrandpaApi::GrandpaApi<Block> for Runtime {
    fn grandpa_authorities() -> GrandpaAuthorityList {
        Grandpa::grandpa_authorities()
    }
    fn current_set_id() -> fg_primitives::SetId {
        Grandpa::current_set_id()
    }
    fn submit_report_equivocation_unsigned_extrinsic(
        _equivocation_proof: fg_primitives::EquivocationProof<
            <Block as BlockT>::Hash,
            NumberFor<Block>,
        >,
        _key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
    ) -> Option<()> {
        None
    }
    fn generate_key_ownership_proof(
        _set_id: fg_primitives::SetId,
        _authority_id: GrandpaId,
    ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
        None
    }
}
impl
    frame_system_rpc_runtime_api::runtime_decl_for_AccountNonceApi::AccountNonceApi<
        Block,
        AccountId,
        Index,
    > for Runtime
{
    fn account_nonce(account: AccountId) -> Index {
        System::account_nonce(account)
    }
}
impl pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: TransactionPaymentApi < Block , Balance > for Runtime { fn query_info (uxt : < Block as BlockT > :: Extrinsic , len : u32) -> pallet_transaction_payment_rpc_runtime_api :: RuntimeDispatchInfo < Balance > { TransactionPayment :: query_info (uxt , len) } fn query_fee_details (uxt : < Block as BlockT > :: Extrinsic , len : u32) -> pallet_transaction_payment :: FeeDetails < Balance > { TransactionPayment :: query_fee_details (uxt , len) } }
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_api::Core<__SR_API_BLOCK__> for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    RuntimeVersion: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SR_API_BLOCK__ as BlockT>::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn Core_version_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<RuntimeVersion>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self.call_api_at(
            |call_runtime_at, changes, storage_transaction_cache, recorder| {
                sp_api::runtime_decl_for_Core::version_call_api_at(
                    call_runtime_at,
                    at,
                    params_encoded,
                    changes,
                    storage_transaction_cache,
                    params.map(|p| {
                        sp_api::runtime_decl_for_Core::version_native_call_generator::<
                            Runtime,
                            __SR_API_BLOCK__,
                            Block,
                        >()
                    }),
                    context,
                    recorder,
                )
            },
        )
    }
    fn Core_execute_block_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(__SR_API_BLOCK__)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<()>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self.call_api_at(
            |call_runtime_at, changes, storage_transaction_cache, recorder| {
                sp_api::runtime_decl_for_Core::execute_block_call_api_at(
                    call_runtime_at,
                    at,
                    params_encoded,
                    changes,
                    storage_transaction_cache,
                    params.map(|p| {
                        sp_api::runtime_decl_for_Core::execute_block_native_call_generator::<
                            Runtime,
                            __SR_API_BLOCK__,
                            Block,
                        >(p)
                    }),
                    context,
                    recorder,
                )
            },
        )
    }
    fn Core_initialize_block_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(&<__SR_API_BLOCK__ as BlockT>::Header)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<()>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self.call_api_at(
            |call_runtime_at, changes, storage_transaction_cache, recorder| {
                sp_api::runtime_decl_for_Core::initialize_block_call_api_at(
                    call_runtime_at,
                    at,
                    params_encoded,
                    changes,
                    storage_transaction_cache,
                    params.map(|p| {
                        sp_api::runtime_decl_for_Core::initialize_block_native_call_generator::<
                            Runtime,
                            __SR_API_BLOCK__,
                            Block,
                        >(p)
                    }),
                    context,
                    recorder,
                )
            },
        )
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_api::Metadata<__SR_API_BLOCK__> for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    OpaqueMetadata: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn Metadata_metadata_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<OpaqueMetadata>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self.call_api_at(
            |call_runtime_at, changes, storage_transaction_cache, recorder| {
                sp_api::runtime_decl_for_Metadata::metadata_call_api_at(
                    call_runtime_at,
                    at,
                    params_encoded,
                    changes,
                    storage_transaction_cache,
                    params.map(|p| {
                        sp_api::runtime_decl_for_Metadata::metadata_native_call_generator::<
                            Runtime,
                            __SR_API_BLOCK__,
                            Block,
                        >()
                    }),
                    context,
                    recorder,
                )
            },
        )
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_block_builder::BlockBuilder<__SR_API_BLOCK__>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    <__SR_API_BLOCK__ as BlockT>::Extrinsic: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    ApplyExtrinsicResult: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SR_API_BLOCK__ as BlockT>::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    sp_inherents::InherentData: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<<__SR_API_BLOCK__ as BlockT>::Extrinsic>:
        std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    sp_inherents::InherentData: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    sp_inherents::CheckInherentsResult: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn BlockBuilder_apply_extrinsic_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(<__SR_API_BLOCK__ as BlockT>::Extrinsic)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            ApplyExtrinsicResult,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_block_builder :: runtime_decl_for_BlockBuilder :: apply_extrinsic_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_block_builder :: runtime_decl_for_BlockBuilder :: apply_extrinsic_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p) }) , context , recorder) })
    }
    fn BlockBuilder_finalize_block_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            <__SR_API_BLOCK__ as BlockT>::Header,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_block_builder :: runtime_decl_for_BlockBuilder :: finalize_block_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_block_builder :: runtime_decl_for_BlockBuilder :: finalize_block_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > () }) , context , recorder) })
    }
    fn BlockBuilder_inherent_extrinsics_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(sp_inherents::InherentData)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Vec<<__SR_API_BLOCK__ as BlockT>::Extrinsic>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_block_builder :: runtime_decl_for_BlockBuilder :: inherent_extrinsics_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_block_builder :: runtime_decl_for_BlockBuilder :: inherent_extrinsics_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p) }) , context , recorder) })
    }
    fn BlockBuilder_check_inherents_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(__SR_API_BLOCK__, sp_inherents::InherentData)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            sp_inherents::CheckInherentsResult,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_block_builder :: runtime_decl_for_BlockBuilder :: check_inherents_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_block_builder :: runtime_decl_for_BlockBuilder :: check_inherents_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p . 0 , p . 1) }) , context , recorder) })
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_transaction_pool::runtime_api::TaggedTransactionQueue<__SR_API_BLOCK__>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    TransactionSource: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SR_API_BLOCK__ as BlockT>::Extrinsic: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SR_API_BLOCK__ as BlockT>::Hash: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    TransactionValidity: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn TaggedTransactionQueue_validate_transaction_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(
            TransactionSource,
            <__SR_API_BLOCK__ as BlockT>::Extrinsic,
            <__SR_API_BLOCK__ as BlockT>::Hash,
        )>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            TransactionValidity,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_transaction_pool :: runtime_api :: runtime_decl_for_TaggedTransactionQueue :: validate_transaction_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_transaction_pool :: runtime_api :: runtime_decl_for_TaggedTransactionQueue :: validate_transaction_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p . 0 , p . 1 , p . 2) }) , context , recorder) })
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_offchain::OffchainWorkerApi<__SR_API_BLOCK__>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    <__SR_API_BLOCK__ as BlockT>::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn OffchainWorkerApi_offchain_worker_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(&<__SR_API_BLOCK__ as BlockT>::Header)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<()>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_offchain :: runtime_decl_for_OffchainWorkerApi :: offchain_worker_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_offchain :: runtime_decl_for_OffchainWorkerApi :: offchain_worker_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p) }) , context , recorder) })
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_consensus_aura::AuraApi<__SR_API_BLOCK__, AuraId>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    sp_consensus_aura::SlotDuration: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<AuraId>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn AuraApi_slot_duration_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            sp_consensus_aura::SlotDuration,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_consensus_aura :: runtime_decl_for_AuraApi :: slot_duration_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_consensus_aura :: runtime_decl_for_AuraApi :: slot_duration_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block , AuraId > () }) , context , recorder) })
    }
    fn AuraApi_authorities_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<Vec<AuraId>>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_consensus_aura :: runtime_decl_for_AuraApi :: authorities_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_consensus_aura :: runtime_decl_for_AuraApi :: authorities_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block , AuraId > () }) , context , recorder) })
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_session::SessionKeys<__SR_API_BLOCK__>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    Option<Vec<u8>>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<u8>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<u8>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Option<Vec<(Vec<u8>, KeyTypeId)>>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn SessionKeys_generate_session_keys_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(Option<Vec<u8>>)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<Vec<u8>>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_session :: runtime_decl_for_SessionKeys :: generate_session_keys_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_session :: runtime_decl_for_SessionKeys :: generate_session_keys_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p) }) , context , recorder) })
    }
    fn SessionKeys_decode_session_keys_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(Vec<u8>)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Option<Vec<(Vec<u8>, KeyTypeId)>>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_session :: runtime_decl_for_SessionKeys :: decode_session_keys_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_session :: runtime_decl_for_SessionKeys :: decode_session_keys_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p) }) , context , recorder) })
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > fg_primitives::GrandpaApi<__SR_API_BLOCK__>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    GrandpaAuthorityList: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    fg_primitives::SetId: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    fg_primitives::EquivocationProof<
        <__SR_API_BLOCK__ as BlockT>::Hash,
        NumberFor<__SR_API_BLOCK__>,
    >: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    fg_primitives::OpaqueKeyOwnershipProof: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Option<()>: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    fg_primitives::SetId: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    GrandpaId: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Option<fg_primitives::OpaqueKeyOwnershipProof>:
        std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn GrandpaApi_grandpa_authorities_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            GrandpaAuthorityList,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fg_primitives :: runtime_decl_for_GrandpaApi :: grandpa_authorities_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fg_primitives :: runtime_decl_for_GrandpaApi :: grandpa_authorities_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > () }) , context , recorder) })
    }
    fn GrandpaApi_current_set_id_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            fg_primitives::SetId,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fg_primitives :: runtime_decl_for_GrandpaApi :: current_set_id_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fg_primitives :: runtime_decl_for_GrandpaApi :: current_set_id_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > () }) , context , recorder) })
    }
    fn GrandpaApi_submit_report_equivocation_unsigned_extrinsic_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(
            fg_primitives::EquivocationProof<
                <__SR_API_BLOCK__ as BlockT>::Hash,
                NumberFor<__SR_API_BLOCK__>,
            >,
            fg_primitives::OpaqueKeyOwnershipProof,
        )>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<Option<()>>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fg_primitives :: runtime_decl_for_GrandpaApi :: submit_report_equivocation_unsigned_extrinsic_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fg_primitives :: runtime_decl_for_GrandpaApi :: submit_report_equivocation_unsigned_extrinsic_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p . 0 , p . 1) }) , context , recorder) })
    }
    fn GrandpaApi_generate_key_ownership_proof_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(fg_primitives::SetId, GrandpaId)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Option<fg_primitives::OpaqueKeyOwnershipProof>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { fg_primitives :: runtime_decl_for_GrandpaApi :: generate_key_ownership_proof_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { fg_primitives :: runtime_decl_for_GrandpaApi :: generate_key_ownership_proof_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p . 0 , p . 1) }) , context , recorder) })
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > frame_system_rpc_runtime_api::AccountNonceApi<__SR_API_BLOCK__, AccountId, Index>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    AccountId: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Index: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn AccountNonceApi_account_nonce_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(AccountId)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<Index>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { frame_system_rpc_runtime_api :: runtime_decl_for_AccountNonceApi :: account_nonce_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { frame_system_rpc_runtime_api :: runtime_decl_for_AccountNonceApi :: account_nonce_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block , AccountId , Index > (p) }) , context , recorder) })
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<__SR_API_BLOCK__, Balance>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    <__SR_API_BLOCK__ as BlockT>::Extrinsic: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    u32: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance>:
        std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SR_API_BLOCK__ as BlockT>::Extrinsic: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    u32: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    pallet_transaction_payment::FeeDetails<Balance>:
        std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn TransactionPaymentApi_query_info_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(<__SR_API_BLOCK__ as BlockT>::Extrinsic, u32)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: query_info_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: query_info_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block , Balance > (p . 0 , p . 1) }) , context , recorder) })
    }
    fn TransactionPaymentApi_query_fee_details_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(<__SR_API_BLOCK__ as BlockT>::Extrinsic, u32)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            pallet_transaction_payment::FeeDetails<Balance>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: query_fee_details_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: query_fee_details_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block , Balance > (p . 0 , p . 1) }) , context , recorder) })
    }
}
const RUNTIME_API_VERSIONS : self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: ApisVec = :: sp_version :: sp_std :: borrow :: Cow :: Borrowed (& [(sp_api :: runtime_decl_for_Core :: ID , sp_api :: runtime_decl_for_Core :: VERSION) , (sp_api :: runtime_decl_for_Metadata :: ID , sp_api :: runtime_decl_for_Metadata :: VERSION) , (sp_block_builder :: runtime_decl_for_BlockBuilder :: ID , sp_block_builder :: runtime_decl_for_BlockBuilder :: VERSION) , (sp_transaction_pool :: runtime_api :: runtime_decl_for_TaggedTransactionQueue :: ID , sp_transaction_pool :: runtime_api :: runtime_decl_for_TaggedTransactionQueue :: VERSION) , (sp_offchain :: runtime_decl_for_OffchainWorkerApi :: ID , sp_offchain :: runtime_decl_for_OffchainWorkerApi :: VERSION) , (sp_consensus_aura :: runtime_decl_for_AuraApi :: ID , sp_consensus_aura :: runtime_decl_for_AuraApi :: VERSION) , (sp_session :: runtime_decl_for_SessionKeys :: ID , sp_session :: runtime_decl_for_SessionKeys :: VERSION) , (fg_primitives :: runtime_decl_for_GrandpaApi :: ID , fg_primitives :: runtime_decl_for_GrandpaApi :: VERSION) , (frame_system_rpc_runtime_api :: runtime_decl_for_AccountNonceApi :: ID , frame_system_rpc_runtime_api :: runtime_decl_for_AccountNonceApi :: VERSION) , (pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: ID , pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: VERSION)]) ;
const _: () = {};
const _: () = {};
const _: () = {};
const _: () = {};
const _: () = {};
const _: () = {};
const _: () = {};
const _: () = {};
const _: () = {};
const _: () = {};
pub mod api {
    use super::*;
    #[cfg(feature = "std")]
    pub fn dispatch(method: &str, mut __sp_api__input_data: &[u8]) -> Option<Vec<u8>> {
        match method {
            "Core_version" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "version") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_api::runtime_decl_for_Core::Core<Block>>::version()
                }),
            ),
            "Core_execute_block" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (block) : (Block) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "execute_block") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_api::runtime_decl_for_Core::Core<Block>>::execute_block(block)
                }),
            ),
            "Core_initialize_block" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (header) : (< Block as BlockT > :: Header) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "initialize_block") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_api::runtime_decl_for_Core::Core<Block>>::initialize_block(
                        &header,
                    )
                }),
            ),
            "Metadata_metadata" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "metadata") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_api::runtime_decl_for_Metadata::Metadata<Block>>::metadata()
                }),
            ),
            "BlockBuilder_apply_extrinsic" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (extrinsic) : (< Block as BlockT > :: Extrinsic) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "apply_extrinsic") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                        Block,
                    >>::apply_extrinsic(extrinsic)
                }),
            ),
            "BlockBuilder_finalize_block" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "finalize_block") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                        Block,
                    >>::finalize_block()
                }),
            ),
            "BlockBuilder_inherent_extrinsics" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (data) : (sp_inherents :: InherentData) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "inherent_extrinsics") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                        Block,
                    >>::inherent_extrinsics(data)
                }),
            ),
            "BlockBuilder_check_inherents" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (block , data) : (Block , sp_inherents :: InherentData) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "check_inherents") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                        Block,
                    >>::check_inherents(block, data)
                }),
            ),
            "TaggedTransactionQueue_validate_transaction" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (source , tx , block_hash) : (TransactionSource , < Block as BlockT > :: Extrinsic , < Block as BlockT > :: Hash) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "validate_transaction") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    # [allow (deprecated)] < Runtime as sp_transaction_pool :: runtime_api :: runtime_decl_for_TaggedTransactionQueue :: TaggedTransactionQueue < Block > > :: validate_transaction (source , tx , block_hash)
                }),
            ),
            "OffchainWorkerApi_offchain_worker" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (header) : (< Block as BlockT > :: Header) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "offchain_worker") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    # [allow (deprecated)] < Runtime as sp_offchain :: runtime_decl_for_OffchainWorkerApi :: OffchainWorkerApi < Block > > :: offchain_worker (& header)
                }),
            ),
            "AuraApi_slot_duration" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "slot_duration") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_consensus_aura::runtime_decl_for_AuraApi::AuraApi<
                        Block,
                        AuraId,
                    >>::slot_duration()
                }),
            ),
            "AuraApi_authorities" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "authorities") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_consensus_aura::runtime_decl_for_AuraApi::AuraApi<
                        Block,
                        AuraId,
                    >>::authorities()
                }),
            ),
            "SessionKeys_generate_session_keys" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (seed) : (Option < Vec < u8 > >) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "generate_session_keys") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    # [allow (deprecated)] < Runtime as sp_session :: runtime_decl_for_SessionKeys :: SessionKeys < Block > > :: generate_session_keys (seed)
                }),
            ),
            "SessionKeys_decode_session_keys" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (encoded) : (Vec < u8 >) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "decode_session_keys") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    # [allow (deprecated)] < Runtime as sp_session :: runtime_decl_for_SessionKeys :: SessionKeys < Block > > :: decode_session_keys (encoded)
                }),
            ),
            "GrandpaApi_grandpa_authorities" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "grandpa_authorities") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    # [allow (deprecated)] < Runtime as fg_primitives :: runtime_decl_for_GrandpaApi :: GrandpaApi < Block > > :: grandpa_authorities ()
                }),
            ),
            "GrandpaApi_current_set_id" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "current_set_id") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    # [allow (deprecated)] < Runtime as fg_primitives :: runtime_decl_for_GrandpaApi :: GrandpaApi < Block > > :: current_set_id ()
                }),
            ),
            "GrandpaApi_submit_report_equivocation_unsigned_extrinsic" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (_equivocation_proof , _key_owner_proof) : (fg_primitives :: EquivocationProof < < Block as BlockT > :: Hash , NumberFor < Block > > , fg_primitives :: OpaqueKeyOwnershipProof) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "submit_report_equivocation_unsigned_extrinsic") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    # [allow (deprecated)] < Runtime as fg_primitives :: runtime_decl_for_GrandpaApi :: GrandpaApi < Block > > :: submit_report_equivocation_unsigned_extrinsic (_equivocation_proof , _key_owner_proof)
                }),
            ),
            "GrandpaApi_generate_key_ownership_proof" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (_set_id , _authority_id) : (fg_primitives :: SetId , GrandpaId) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "generate_key_ownership_proof") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    # [allow (deprecated)] < Runtime as fg_primitives :: runtime_decl_for_GrandpaApi :: GrandpaApi < Block > > :: generate_key_ownership_proof (_set_id , _authority_id)
                }),
            ),
            "AccountNonceApi_account_nonce" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (account) : (AccountId) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "account_nonce") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    # [allow (deprecated)] < Runtime as frame_system_rpc_runtime_api :: runtime_decl_for_AccountNonceApi :: AccountNonceApi < Block , AccountId , Index > > :: account_nonce (account)
                }),
            ),
            "TransactionPaymentApi_query_info" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (uxt , len) : (< Block as BlockT > :: Extrinsic , u32) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "query_info") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    # [allow (deprecated)] < Runtime as pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: TransactionPaymentApi < Block , Balance > > :: query_info (uxt , len)
                }),
            ),
            "TransactionPaymentApi_query_fee_details" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (uxt , len) : (< Block as BlockT > :: Extrinsic , u32) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "query_fee_details") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    # [allow (deprecated)] < Runtime as pallet_transaction_payment_rpc_runtime_api :: runtime_decl_for_TransactionPaymentApi :: TransactionPaymentApi < Block , Balance > > :: query_fee_details (uxt , len)
                }),
            ),
            _ => None,
        }
    }
}
pub struct MaximumSchedulerWeight;
impl MaximumSchedulerWeight {
    /// Returns the value of this parameter type.
    pub fn get() -> Weight {
        10_000_000
    }
}
impl<I: From<Weight>> ::frame_support::traits::Get<I> for MaximumSchedulerWeight {
    fn get() -> I {
        I::from(Self::get())
    }
}
pub struct MaxScheduledPerBlock;
impl MaxScheduledPerBlock {
    /// Returns the value of this parameter type.
    pub const fn get() -> u32 {
        50
    }
}
impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxScheduledPerBlock {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl pallet_scheduler::Config for Runtime {
    type Event = Event;
    type Origin = Origin;
    type PalletsOrigin = OriginCaller;
    type Call = Call;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = frame_system::EnsureRoot<AccountId>;
    type MaxScheduledPerBlock = MaxScheduledPerBlock;
    type WeightInfo = ();
    type OriginPrivilegeCmp = EqualPrivilegeOnly;
    type PreimageProvider = ();
    type NoPreimagePostponement = ();
}
