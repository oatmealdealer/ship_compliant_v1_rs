mod progenitor_client;

#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
#[doc = r" Types used as operation parameters and responses."]
#[allow(clippy::all)]
pub mod types {
    #[doc = r" Error types."]
    pub mod error {
        #[doc = r" Error from a TryFrom or FromStr implementation."]
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    #[doc = "AddressStatus"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"AddressStatus\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Null\","]
    #[doc = "    \"AddressOutOfRange\","]
    #[doc = "    \"AddressSuggested\","]
    #[doc = "    \"ComponentMismatch\","]
    #[doc = "    \"MultipleMatches\","]
    #[doc = "    \"NonDeliverableAddress\","]
    #[doc = "    \"NoStreetData\","]
    #[doc = "    \"UnknownStreet\","]
    #[doc = "    \"Validated\","]
    #[doc = "    \"ZipCodeDoesNotExist\","]
    #[doc = "    \"ZipCodeDoesNotMatchCityState\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AddressStatus {
        Null,
        AddressOutOfRange,
        AddressSuggested,
        ComponentMismatch,
        MultipleMatches,
        NonDeliverableAddress,
        NoStreetData,
        UnknownStreet,
        Validated,
        ZipCodeDoesNotExist,
        ZipCodeDoesNotMatchCityState,
    }

    impl From<&AddressStatus> for AddressStatus {
        fn from(value: &AddressStatus) -> Self {
            value.clone()
        }
    }

    impl ToString for AddressStatus {
        fn to_string(&self) -> String {
            match *self {
                Self::Null => "Null".to_string(),
                Self::AddressOutOfRange => "AddressOutOfRange".to_string(),
                Self::AddressSuggested => "AddressSuggested".to_string(),
                Self::ComponentMismatch => "ComponentMismatch".to_string(),
                Self::MultipleMatches => "MultipleMatches".to_string(),
                Self::NonDeliverableAddress => "NonDeliverableAddress".to_string(),
                Self::NoStreetData => "NoStreetData".to_string(),
                Self::UnknownStreet => "UnknownStreet".to_string(),
                Self::Validated => "Validated".to_string(),
                Self::ZipCodeDoesNotExist => "ZipCodeDoesNotExist".to_string(),
                Self::ZipCodeDoesNotMatchCityState => "ZipCodeDoesNotMatchCityState".to_string(),
            }
        }
    }

    impl std::str::FromStr for AddressStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Null" => Ok(Self::Null),
                "AddressOutOfRange" => Ok(Self::AddressOutOfRange),
                "AddressSuggested" => Ok(Self::AddressSuggested),
                "ComponentMismatch" => Ok(Self::ComponentMismatch),
                "MultipleMatches" => Ok(Self::MultipleMatches),
                "NonDeliverableAddress" => Ok(Self::NonDeliverableAddress),
                "NoStreetData" => Ok(Self::NoStreetData),
                "UnknownStreet" => Ok(Self::UnknownStreet),
                "Validated" => Ok(Self::Validated),
                "ZipCodeDoesNotExist" => Ok(Self::ZipCodeDoesNotExist),
                "ZipCodeDoesNotMatchCityState" => Ok(Self::ZipCodeDoesNotMatchCityState),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for AddressStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for AddressStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for AddressStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "ComplianceDetailType"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ComplianceDetailType\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"BrandNameNotRegistered\","]
    #[doc = "    \"LabelNotRegistered\","]
    #[doc = "    \"NextShipDate\","]
    #[doc = "    \"VolumeOverLimit\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ComplianceDetailType {
        BrandNameNotRegistered,
        LabelNotRegistered,
        NextShipDate,
        VolumeOverLimit,
    }

    impl From<&ComplianceDetailType> for ComplianceDetailType {
        fn from(value: &ComplianceDetailType) -> Self {
            value.clone()
        }
    }

    impl ToString for ComplianceDetailType {
        fn to_string(&self) -> String {
            match *self {
                Self::BrandNameNotRegistered => "BrandNameNotRegistered".to_string(),
                Self::LabelNotRegistered => "LabelNotRegistered".to_string(),
                Self::NextShipDate => "NextShipDate".to_string(),
                Self::VolumeOverLimit => "VolumeOverLimit".to_string(),
            }
        }
    }

    impl std::str::FromStr for ComplianceDetailType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "BrandNameNotRegistered" => Ok(Self::BrandNameNotRegistered),
                "LabelNotRegistered" => Ok(Self::LabelNotRegistered),
                "NextShipDate" => Ok(Self::NextShipDate),
                "VolumeOverLimit" => Ok(Self::VolumeOverLimit),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ComplianceDetailType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ComplianceDetailType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ComplianceDetailType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "CustomerType"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"CustomerType\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Null\","]
    #[doc = "    \"Household\","]
    #[doc = "    \"Individual\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CustomerType {
        Null,
        Household,
        Individual,
    }

    impl From<&CustomerType> for CustomerType {
        fn from(value: &CustomerType) -> Self {
            value.clone()
        }
    }

    impl ToString for CustomerType {
        fn to_string(&self) -> String {
            match *self {
                Self::Null => "Null".to_string(),
                Self::Household => "Household".to_string(),
                Self::Individual => "Individual".to_string(),
            }
        }
    }

    impl std::str::FromStr for CustomerType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Null" => Ok(Self::Null),
                "Household" => Ok(Self::Household),
                "Individual" => Ok(Self::Individual),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for CustomerType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for CustomerType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for CustomerType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "ErrorTarget"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ErrorTarget\","]
    #[doc = "  \"default\": \"Address\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Null\","]
    #[doc = "    \"Address\","]
    #[doc = "    \"Age\","]
    #[doc = "    \"Batch\","]
    #[doc = "    \"Brand\","]
    #[doc = "    \"Distributor\","]
    #[doc = "    \"Product\","]
    #[doc = "    \"SalesOrder\","]
    #[doc = "    \"Shipment\","]
    #[doc = "    \"Tax\","]
    #[doc = "    \"SSO\","]
    #[doc = "    \"License\","]
    #[doc = "    \"Tracking\","]
    #[doc = "    \"HoldLocation\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ErrorTarget {
        Null,
        Address,
        Age,
        Batch,
        Brand,
        Distributor,
        Product,
        SalesOrder,
        Shipment,
        Tax,
        #[serde(rename = "SSO")]
        Sso,
        License,
        Tracking,
        HoldLocation,
    }

    impl From<&ErrorTarget> for ErrorTarget {
        fn from(value: &ErrorTarget) -> Self {
            value.clone()
        }
    }

    impl ToString for ErrorTarget {
        fn to_string(&self) -> String {
            match *self {
                Self::Null => "Null".to_string(),
                Self::Address => "Address".to_string(),
                Self::Age => "Age".to_string(),
                Self::Batch => "Batch".to_string(),
                Self::Brand => "Brand".to_string(),
                Self::Distributor => "Distributor".to_string(),
                Self::Product => "Product".to_string(),
                Self::SalesOrder => "SalesOrder".to_string(),
                Self::Shipment => "Shipment".to_string(),
                Self::Tax => "Tax".to_string(),
                Self::Sso => "SSO".to_string(),
                Self::License => "License".to_string(),
                Self::Tracking => "Tracking".to_string(),
                Self::HoldLocation => "HoldLocation".to_string(),
            }
        }
    }

    impl std::str::FromStr for ErrorTarget {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Null" => Ok(Self::Null),
                "Address" => Ok(Self::Address),
                "Age" => Ok(Self::Age),
                "Batch" => Ok(Self::Batch),
                "Brand" => Ok(Self::Brand),
                "Distributor" => Ok(Self::Distributor),
                "Product" => Ok(Self::Product),
                "SalesOrder" => Ok(Self::SalesOrder),
                "Shipment" => Ok(Self::Shipment),
                "Tax" => Ok(Self::Tax),
                "SSO" => Ok(Self::Sso),
                "License" => Ok(Self::License),
                "Tracking" => Ok(Self::Tracking),
                "HoldLocation" => Ok(Self::HoldLocation),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ErrorTarget {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ErrorTarget {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ErrorTarget {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl Default for ErrorTarget {
        fn default() -> Self {
            ErrorTarget::Address
        }
    }

    #[doc = "ErrorType"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ErrorType\","]
    #[doc = "  \"default\": \"Validation\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Authentication\","]
    #[doc = "    \"Server\","]
    #[doc = "    \"Validation\","]
    #[doc = "    \"Request\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ErrorType {
        Authentication,
        Server,
        Validation,
        Request,
    }

    impl From<&ErrorType> for ErrorType {
        fn from(value: &ErrorType) -> Self {
            value.clone()
        }
    }

    impl ToString for ErrorType {
        fn to_string(&self) -> String {
            match *self {
                Self::Authentication => "Authentication".to_string(),
                Self::Server => "Server".to_string(),
                Self::Validation => "Validation".to_string(),
                Self::Request => "Request".to_string(),
            }
        }
    }

    impl std::str::FromStr for ErrorType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Authentication" => Ok(Self::Authentication),
                "Server" => Ok(Self::Server),
                "Validation" => Ok(Self::Validation),
                "Request" => Ok(Self::Request),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ErrorType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ErrorType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ErrorType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl Default for ErrorType {
        fn default() -> Self {
            ErrorType::Validation
        }
    }

    #[doc = "FulfillmentExceptionTypes"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"FulfillmentExceptionTypes\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Null\","]
    #[doc = "    \"None\","]
    #[doc = "    \"Inventory\","]
    #[doc = "    \"NonCompliant\","]
    #[doc = "    \"Other\","]
    #[doc = "    \"Updated\","]
    #[doc = "    \"Setup\","]
    #[doc = "    \"Temperature\","]
    #[doc = "    \"Voided\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum FulfillmentExceptionTypes {
        Null,
        None,
        Inventory,
        NonCompliant,
        Other,
        Updated,
        Setup,
        Temperature,
        Voided,
    }

    impl From<&FulfillmentExceptionTypes> for FulfillmentExceptionTypes {
        fn from(value: &FulfillmentExceptionTypes) -> Self {
            value.clone()
        }
    }

    impl ToString for FulfillmentExceptionTypes {
        fn to_string(&self) -> String {
            match *self {
                Self::Null => "Null".to_string(),
                Self::None => "None".to_string(),
                Self::Inventory => "Inventory".to_string(),
                Self::NonCompliant => "NonCompliant".to_string(),
                Self::Other => "Other".to_string(),
                Self::Updated => "Updated".to_string(),
                Self::Setup => "Setup".to_string(),
                Self::Temperature => "Temperature".to_string(),
                Self::Voided => "Voided".to_string(),
            }
        }
    }

    impl std::str::FromStr for FulfillmentExceptionTypes {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Null" => Ok(Self::Null),
                "None" => Ok(Self::None),
                "Inventory" => Ok(Self::Inventory),
                "NonCompliant" => Ok(Self::NonCompliant),
                "Other" => Ok(Self::Other),
                "Updated" => Ok(Self::Updated),
                "Setup" => Ok(Self::Setup),
                "Temperature" => Ok(Self::Temperature),
                "Voided" => Ok(Self::Voided),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for FulfillmentExceptionTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for FulfillmentExceptionTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for FulfillmentExceptionTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "FulfillmentType"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"FulfillmentType\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Null\","]
    #[doc = "    \"Club\","]
    #[doc = "    \"Daily\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum FulfillmentType {
        Null,
        Club,
        Daily,
    }

    impl From<&FulfillmentType> for FulfillmentType {
        fn from(value: &FulfillmentType) -> Self {
            value.clone()
        }
    }

    impl ToString for FulfillmentType {
        fn to_string(&self) -> String {
            match *self {
                Self::Null => "Null".to_string(),
                Self::Club => "Club".to_string(),
                Self::Daily => "Daily".to_string(),
            }
        }
    }

    impl std::str::FromStr for FulfillmentType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Null" => Ok(Self::Null),
                "Club" => Ok(Self::Club),
                "Daily" => Ok(Self::Daily),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for FulfillmentType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for FulfillmentType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for FulfillmentType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "LicenseRelationshipTypes"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"LicenseRelationshipTypes\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Default\","]
    #[doc = "    \"Pickup\","]
    #[doc = "    \"RetailerToConsumer\","]
    #[doc = "    \"RetailerToThreeTier\","]
    #[doc = "    \"SupplierToConsumer\","]
    #[doc = "    \"SupplierToThreeTier\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum LicenseRelationshipTypes {
        Default,
        Pickup,
        RetailerToConsumer,
        RetailerToThreeTier,
        SupplierToConsumer,
        SupplierToThreeTier,
    }

    impl From<&LicenseRelationshipTypes> for LicenseRelationshipTypes {
        fn from(value: &LicenseRelationshipTypes) -> Self {
            value.clone()
        }
    }

    impl ToString for LicenseRelationshipTypes {
        fn to_string(&self) -> String {
            match *self {
                Self::Default => "Default".to_string(),
                Self::Pickup => "Pickup".to_string(),
                Self::RetailerToConsumer => "RetailerToConsumer".to_string(),
                Self::RetailerToThreeTier => "RetailerToThreeTier".to_string(),
                Self::SupplierToConsumer => "SupplierToConsumer".to_string(),
                Self::SupplierToThreeTier => "SupplierToThreeTier".to_string(),
            }
        }
    }

    impl std::str::FromStr for LicenseRelationshipTypes {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Default" => Ok(Self::Default),
                "Pickup" => Ok(Self::Pickup),
                "RetailerToConsumer" => Ok(Self::RetailerToConsumer),
                "RetailerToThreeTier" => Ok(Self::RetailerToThreeTier),
                "SupplierToConsumer" => Ok(Self::SupplierToConsumer),
                "SupplierToThreeTier" => Ok(Self::SupplierToThreeTier),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for LicenseRelationshipTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for LicenseRelationshipTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for LicenseRelationshipTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "OrderTypes"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"OrderTypes\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Club\","]
    #[doc = "    \"Fax\","]
    #[doc = "    \"InPerson\","]
    #[doc = "    \"Internet\","]
    #[doc = "    \"Mail\","]
    #[doc = "    \"Phone\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum OrderTypes {
        Club,
        Fax,
        InPerson,
        Internet,
        Mail,
        Phone,
    }

    impl From<&OrderTypes> for OrderTypes {
        fn from(value: &OrderTypes) -> Self {
            value.clone()
        }
    }

    impl ToString for OrderTypes {
        fn to_string(&self) -> String {
            match *self {
                Self::Club => "Club".to_string(),
                Self::Fax => "Fax".to_string(),
                Self::InPerson => "InPerson".to_string(),
                Self::Internet => "Internet".to_string(),
                Self::Mail => "Mail".to_string(),
                Self::Phone => "Phone".to_string(),
            }
        }
    }

    impl std::str::FromStr for OrderTypes {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Club" => Ok(Self::Club),
                "Fax" => Ok(Self::Fax),
                "InPerson" => Ok(Self::InPerson),
                "Internet" => Ok(Self::Internet),
                "Mail" => Ok(Self::Mail),
                "Phone" => Ok(Self::Phone),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for OrderTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for OrderTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for OrderTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "PaymentTypes"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PaymentTypes\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Cash\","]
    #[doc = "    \"Check\","]
    #[doc = "    \"Creditcard\","]
    #[doc = "    \"Giftcard\","]
    #[doc = "    \"Giftcertificate\","]
    #[doc = "    \"Invoice\","]
    #[doc = "    \"Moneyorder\","]
    #[doc = "    \"Other\","]
    #[doc = "    \"Storeaccount\","]
    #[doc = "    \"Travelerscheck\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PaymentTypes {
        Cash,
        Check,
        Creditcard,
        Giftcard,
        Giftcertificate,
        Invoice,
        Moneyorder,
        Other,
        Storeaccount,
        Travelerscheck,
    }

    impl From<&PaymentTypes> for PaymentTypes {
        fn from(value: &PaymentTypes) -> Self {
            value.clone()
        }
    }

    impl ToString for PaymentTypes {
        fn to_string(&self) -> String {
            match *self {
                Self::Cash => "Cash".to_string(),
                Self::Check => "Check".to_string(),
                Self::Creditcard => "Creditcard".to_string(),
                Self::Giftcard => "Giftcard".to_string(),
                Self::Giftcertificate => "Giftcertificate".to_string(),
                Self::Invoice => "Invoice".to_string(),
                Self::Moneyorder => "Moneyorder".to_string(),
                Self::Other => "Other".to_string(),
                Self::Storeaccount => "Storeaccount".to_string(),
                Self::Travelerscheck => "Travelerscheck".to_string(),
            }
        }
    }

    impl std::str::FromStr for PaymentTypes {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Cash" => Ok(Self::Cash),
                "Check" => Ok(Self::Check),
                "Creditcard" => Ok(Self::Creditcard),
                "Giftcard" => Ok(Self::Giftcard),
                "Giftcertificate" => Ok(Self::Giftcertificate),
                "Invoice" => Ok(Self::Invoice),
                "Moneyorder" => Ok(Self::Moneyorder),
                "Other" => Ok(Self::Other),
                "Storeaccount" => Ok(Self::Storeaccount),
                "Travelerscheck" => Ok(Self::Travelerscheck),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for PaymentTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for PaymentTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for PaymentTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "ResponseStatus"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ResponseStatus\","]
    #[doc = "  \"default\": \"Success\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Failure\","]
    #[doc = "    \"Success\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ResponseStatus {
        Failure,
        Success,
    }

    impl From<&ResponseStatus> for ResponseStatus {
        fn from(value: &ResponseStatus) -> Self {
            value.clone()
        }
    }

    impl ToString for ResponseStatus {
        fn to_string(&self) -> String {
            match *self {
                Self::Failure => "Failure".to_string(),
                Self::Success => "Success".to_string(),
            }
        }
    }

    impl std::str::FromStr for ResponseStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Failure" => Ok(Self::Failure),
                "Success" => Ok(Self::Success),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ResponseStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl Default for ResponseStatus {
        fn default() -> Self {
            ResponseStatus::Success
        }
    }

    #[doc = "RuleComplianceStatus"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"RuleComplianceStatus\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Compliant\","]
    #[doc = "    \"NotCompliant\","]
    #[doc = "    \"Bypassed\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum RuleComplianceStatus {
        Compliant,
        NotCompliant,
        Bypassed,
    }

    impl From<&RuleComplianceStatus> for RuleComplianceStatus {
        fn from(value: &RuleComplianceStatus) -> Self {
            value.clone()
        }
    }

    impl ToString for RuleComplianceStatus {
        fn to_string(&self) -> String {
            match *self {
                Self::Compliant => "Compliant".to_string(),
                Self::NotCompliant => "NotCompliant".to_string(),
                Self::Bypassed => "Bypassed".to_string(),
            }
        }
    }

    impl std::str::FromStr for RuleComplianceStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Compliant" => Ok(Self::Compliant),
                "NotCompliant" => Ok(Self::NotCompliant),
                "Bypassed" => Ok(Self::Bypassed),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for RuleComplianceStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for RuleComplianceStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for RuleComplianceStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsAddressesEntitiesAddress"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"Address\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"city\","]
    #[doc = "    \"state\","]
    #[doc = "    \"street1\","]
    #[doc = "    \"zip1\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"city\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"company\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"country\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"county\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"dateOfBirth\": {"]
    #[doc = "      \"title\": \"DateTime\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"email\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fax\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"firstName\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"lastName\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"phone\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"street1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"street2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"zip1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"zip2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsAddressesEntitiesAddress {
        pub city: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub country: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub county: Option<String>,
        #[serde(
            rename = "dateOfBirth",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub date_of_birth: Option<chrono::NaiveDateTime>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fax: Option<String>,
        #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
        pub first_name: Option<String>,
        #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
        pub last_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        pub state: String,
        pub street1: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street2: Option<String>,
        pub zip1: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zip2: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsAddressesEntitiesAddress>
        for ShipCompliantRestApiDomainsAddressesEntitiesAddress
    {
        fn from(value: &ShipCompliantRestApiDomainsAddressesEntitiesAddress) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsAddressesEntitiesAddressBase"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"AddressBase\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"address\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"address\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.Addresses.Entities.Address\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsAddressesEntitiesAddressBase {
        pub address: ShipCompliantRestApiDomainsAddressesEntitiesAddress,
    }

    impl From<&ShipCompliantRestApiDomainsAddressesEntitiesAddressBase>
        for ShipCompliantRestApiDomainsAddressesEntitiesAddressBase
    {
        fn from(value: &ShipCompliantRestApiDomainsAddressesEntitiesAddressBase) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsAddressesEntitiesValidateAddressResult"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ValidateAddressResult\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"addressStatus\": {"]
    #[doc = "      \"title\": \"AddressStatus\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"AddressOutOfRange\","]
    #[doc = "        \"AddressSuggested\","]
    #[doc = "        \"ComponentMismatch\","]
    #[doc = "        \"MultipleMatches\","]
    #[doc = "        \"NonDeliverableAddress\","]
    #[doc = "        \"NoStreetData\","]
    #[doc = "        \"UnknownStreet\","]
    #[doc = "        \"Validated\","]
    #[doc = "        \"ZipCodeDoesNotExist\","]
    #[doc = "        \"ZipCodeDoesNotMatchCityState\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"addressSuggestion\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.AddressSuggestion\""]
    #[doc = "    },"]
    #[doc = "    \"isBusinessAddress\": {"]
    #[doc = "      \"title\": \"Boolean\","]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsAddressesEntitiesValidateAddressResult {
        #[serde(
            rename = "addressStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub address_status: Option<AddressStatus>,
        #[serde(
            rename = "addressSuggestion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub address_suggestion:
            Option<ShipCompliantRestApiDomainsSalesOrderEntitiesAddressSuggestion>,
        #[serde(
            rename = "isBusinessAddress",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub is_business_address: Option<bool>,
    }

    impl From<&ShipCompliantRestApiDomainsAddressesEntitiesValidateAddressResult>
        for ShipCompliantRestApiDomainsAddressesEntitiesValidateAddressResult
    {
        fn from(value: &ShipCompliantRestApiDomainsAddressesEntitiesValidateAddressResult) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsBrandEntitiesBrand"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"Brand\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"name\","]
    #[doc = "    \"thisBrandIsBottledByAThirdParty\","]
    #[doc = "    \"thisBrandIsProducedByAThirdParty\","]
    #[doc = "    \"thisBrandOperatesUnderATradeName\","]
    #[doc = "    \"thisBrandWasAcquiredFromAThirdParty\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"owner\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.Brand.Entities.BrandOwner\""]
    #[doc = "    },"]
    #[doc = "    \"thisBrandIsBottledByAThirdParty\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"boolean\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"thisBrandIsProducedByAThirdParty\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"boolean\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"thisBrandOperatesUnderATradeName\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"boolean\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"thisBrandWasAcquiredFromAThirdParty\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"boolean\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"ttbBrandKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsBrandEntitiesBrand {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub owner: Option<ShipCompliantRestApiDomainsBrandEntitiesBrandOwner>,
        #[serde(rename = "thisBrandIsBottledByAThirdParty")]
        pub this_brand_is_bottled_by_a_third_party: Option<bool>,
        #[serde(rename = "thisBrandIsProducedByAThirdParty")]
        pub this_brand_is_produced_by_a_third_party: Option<bool>,
        #[serde(rename = "thisBrandOperatesUnderATradeName")]
        pub this_brand_operates_under_a_trade_name: Option<bool>,
        #[serde(rename = "thisBrandWasAcquiredFromAThirdParty")]
        pub this_brand_was_acquired_from_a_third_party: Option<bool>,
        #[serde(
            rename = "ttbBrandKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ttb_brand_key: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsBrandEntitiesBrand>
        for ShipCompliantRestApiDomainsBrandEntitiesBrand
    {
        fn from(value: &ShipCompliantRestApiDomainsBrandEntitiesBrand) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsBrandEntitiesBrandOwner"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"BrandOwner\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"city\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"country\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"street1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"street2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"zip\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsBrandEntitiesBrandOwner {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub country: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street2: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zip: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsBrandEntitiesBrandOwner>
        for ShipCompliantRestApiDomainsBrandEntitiesBrandOwner
    {
        fn from(value: &ShipCompliantRestApiDomainsBrandEntitiesBrandOwner) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsBrandEntitiesPostBrand"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PostBrand\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"key\","]
    #[doc = "    \"name\","]
    #[doc = "    \"thisBrandIsBottledByAThirdParty\","]
    #[doc = "    \"thisBrandIsProducedByAThirdParty\","]
    #[doc = "    \"thisBrandOperatesUnderATradeName\","]
    #[doc = "    \"thisBrandWasAcquiredFromAThirdParty\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"owner\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.Brand.Entities.BrandOwner\""]
    #[doc = "    },"]
    #[doc = "    \"thisBrandIsBottledByAThirdParty\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"boolean\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"thisBrandIsProducedByAThirdParty\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"boolean\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"thisBrandOperatesUnderATradeName\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"boolean\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"thisBrandWasAcquiredFromAThirdParty\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"boolean\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"ttbBrandKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsBrandEntitiesPostBrand {
        pub key: String,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub owner: Option<ShipCompliantRestApiDomainsBrandEntitiesBrandOwner>,
        #[serde(rename = "thisBrandIsBottledByAThirdParty")]
        pub this_brand_is_bottled_by_a_third_party: Option<bool>,
        #[serde(rename = "thisBrandIsProducedByAThirdParty")]
        pub this_brand_is_produced_by_a_third_party: Option<bool>,
        #[serde(rename = "thisBrandOperatesUnderATradeName")]
        pub this_brand_operates_under_a_trade_name: Option<bool>,
        #[serde(rename = "thisBrandWasAcquiredFromAThirdParty")]
        pub this_brand_was_acquired_from_a_third_party: Option<bool>,
        #[serde(
            rename = "ttbBrandKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ttb_brand_key: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsBrandEntitiesPostBrand>
        for ShipCompliantRestApiDomainsBrandEntitiesPostBrand
    {
        fn from(value: &ShipCompliantRestApiDomainsBrandEntitiesPostBrand) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsHoldLocationEntitiesAddUpdateHoldLocation"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"AddUpdateHoldLocation\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"address\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"address\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.Address\""]
    #[doc = "    },"]
    #[doc = "    \"shipmentKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsHoldLocationEntitiesAddUpdateHoldLocation {
        pub address: ShipCompliantRestApiDomainsSalesOrderEntitiesAddress,
        #[serde(
            rename = "shipmentKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_key: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsHoldLocationEntitiesAddUpdateHoldLocation>
        for ShipCompliantRestApiDomainsHoldLocationEntitiesAddUpdateHoldLocation
    {
        fn from(
            value: &ShipCompliantRestApiDomainsHoldLocationEntitiesAddUpdateHoldLocation,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationDetail"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"HoldLocationDetail\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"city\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"companyName\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"countryCode\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"distanceMiles\": {"]
    #[doc = "      \"title\": \"Single\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"float\""]
    #[doc = "    },"]
    #[doc = "    \"email\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"faxNumber\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fridayHours\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.HoldLocation.Entities.HoldLocationOpenHours\""]
    #[doc = "    },"]
    #[doc = "    \"geographicCoordinates\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"isResidential\": {"]
    #[doc = "      \"title\": \"Boolean\","]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    },"]
    #[doc = "    \"mapUrl\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"mondayHours\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.HoldLocation.Entities.HoldLocationOpenHours\""]
    #[doc = "    },"]
    #[doc = "    \"phoneNumber\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"postalCode\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"saturdayHours\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.HoldLocation.Entities.HoldLocationOpenHours\""]
    #[doc = "    },"]
    #[doc = "    \"stateCode\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"street1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"street2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"sundayHours\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.HoldLocation.Entities.HoldLocationOpenHours\""]
    #[doc = "    },"]
    #[doc = "    \"thursdayHours\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.HoldLocation.Entities.HoldLocationOpenHours\""]
    #[doc = "    },"]
    #[doc = "    \"tuesdayHours\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.HoldLocation.Entities.HoldLocationOpenHours\""]
    #[doc = "    },"]
    #[doc = "    \"wednesdayHours\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.HoldLocation.Entities.HoldLocationOpenHours\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationDetail {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[serde(
            rename = "companyName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub company_name: Option<String>,
        #[serde(
            rename = "countryCode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub country_code: Option<String>,
        #[serde(
            rename = "distanceMiles",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub distance_miles: Option<f32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(rename = "faxNumber", default, skip_serializing_if = "Option::is_none")]
        pub fax_number: Option<String>,
        #[serde(
            rename = "fridayHours",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub friday_hours:
            Option<ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationOpenHours>,
        #[serde(
            rename = "geographicCoordinates",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub geographic_coordinates: Option<String>,
        #[serde(
            rename = "isResidential",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub is_residential: Option<bool>,
        #[serde(rename = "mapUrl", default, skip_serializing_if = "Option::is_none")]
        pub map_url: Option<String>,
        #[serde(
            rename = "mondayHours",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub monday_hours:
            Option<ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationOpenHours>,
        #[serde(
            rename = "phoneNumber",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub phone_number: Option<String>,
        #[serde(
            rename = "postalCode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub postal_code: Option<String>,
        #[serde(
            rename = "saturdayHours",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub saturday_hours:
            Option<ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationOpenHours>,
        #[serde(rename = "stateCode", default, skip_serializing_if = "Option::is_none")]
        pub state_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street2: Option<String>,
        #[serde(
            rename = "sundayHours",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sunday_hours:
            Option<ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationOpenHours>,
        #[serde(
            rename = "thursdayHours",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub thursday_hours:
            Option<ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationOpenHours>,
        #[serde(
            rename = "tuesdayHours",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub tuesday_hours:
            Option<ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationOpenHours>,
        #[serde(
            rename = "wednesdayHours",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub wednesday_hours:
            Option<ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationOpenHours>,
    }

    impl From<&ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationDetail>
        for ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationDetail
    {
        fn from(value: &ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationDetail) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationOpenHours"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"HoldLocationOpenHours\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"closingTime\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"openingTime\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"operationalHoursType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationOpenHours {
        #[serde(
            rename = "closingTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub closing_time: Option<String>,
        #[serde(
            rename = "openingTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub opening_time: Option<String>,
        #[serde(
            rename = "operationalHoursType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub operational_hours_type: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationOpenHours>
        for ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationOpenHours
    {
        fn from(
            value: &ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationOpenHours,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "Combo Component class."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ComboComponent\","]
    #[doc = "  \"description\": \"Combo Component class.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"brandKey\","]
    #[doc = "    \"productKey\","]
    #[doc = "    \"quantity\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"brandKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"description\": \"BrandKey field.\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"productKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"description\": \"ProductKey field.\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"quantity\": {"]
    #[doc = "      \"title\": \"Int32\","]
    #[doc = "      \"description\": \"Quantity field.\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsProductEntitiesComboComponent {
        #[doc = "BrandKey field."]
        #[serde(rename = "brandKey")]
        pub brand_key: String,
        #[doc = "ProductKey field."]
        #[serde(rename = "productKey")]
        pub product_key: String,
        #[doc = "Quantity field."]
        pub quantity: i32,
    }

    impl From<&ShipCompliantRestApiDomainsProductEntitiesComboComponent>
        for ShipCompliantRestApiDomainsProductEntitiesComboComponent
    {
        fn from(value: &ShipCompliantRestApiDomainsProductEntitiesComboComponent) -> Self {
            value.clone()
        }
    }

    #[doc = "Contains the fields who can be include in the request to add a new combo."]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ComboInput\","]
    #[doc = "  \"description\": \"Contains the fields who can be include in the request to add a new combo.\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"brandKey\","]
    #[doc = "    \"components\","]
    #[doc = "    \"description\","]
    #[doc = "    \"distributionType\","]
    #[doc = "    \"productComboKey\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"brandKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"description\": \"Brand Key field.\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"components\": {"]
    #[doc = "      \"title\": \"ComboComponent[]\","]
    #[doc = "      \"description\": \"Combo Components.ShipCompliantRestAPI.Domains.Product.Entities.ComboComponent\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.Product.Entities.ComboComponent\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"description\": \"Description field.\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"distributionType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"description\": \"Distribution Type Direct or Wholesale\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"gtin\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"description\": \"GTIN field.\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"itemsPerCase\": {"]
    #[doc = "      \"title\": \"Int32\","]
    #[doc = "      \"description\": \"Number of items per combo.\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"nabca\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"description\": \"NABCA field.\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"productComboKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"description\": \"ProductComboKey field.\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"scc\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"description\": \"SCC field.\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"totalWeight\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"description\": \"Total Weight field.\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"unimerc\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"description\": \"UNIMERC field.\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"upc\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"description\": \"UPC field.\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"wholesaleCasePrice\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"description\": \"Wholesale Case Price field.\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsProductEntitiesComboInput {
        #[doc = "Brand Key field."]
        #[serde(rename = "brandKey")]
        pub brand_key: String,
        #[doc = "Combo Components.ShipCompliantRestAPI.Domains.Product.Entities.ComboComponent"]
        pub components: Vec<ShipCompliantRestApiDomainsProductEntitiesComboComponent>,
        #[doc = "Description field."]
        pub description: String,
        #[doc = "Distribution Type Direct or Wholesale"]
        #[serde(rename = "distributionType")]
        pub distribution_type: String,
        #[doc = "GTIN field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub gtin: Option<String>,
        #[doc = "Number of items per combo."]
        #[serde(
            rename = "itemsPerCase",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub items_per_case: Option<i32>,
        #[doc = "NABCA field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nabca: Option<String>,
        #[doc = "ProductComboKey field."]
        #[serde(rename = "productComboKey")]
        pub product_combo_key: String,
        #[doc = "SCC field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scc: Option<String>,
        #[serde(
            rename = "totalWeight",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub total_weight: Option<f64>,
        #[doc = "UNIMERC field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unimerc: Option<String>,
        #[doc = "UPC field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub upc: Option<String>,
        #[serde(
            rename = "wholesaleCasePrice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub wholesale_case_price: Option<f64>,
    }

    impl From<&ShipCompliantRestApiDomainsProductEntitiesComboInput>
        for ShipCompliantRestApiDomainsProductEntitiesComboInput
    {
        fn from(value: &ShipCompliantRestApiDomainsProductEntitiesComboInput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsProductEntitiesComponent"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"Component\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"brandKey\","]
    #[doc = "    \"productKey\","]
    #[doc = "    \"quantity\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"brandKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"defaultPrice\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"productKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"quantity\": {"]
    #[doc = "      \"title\": \"Int32\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsProductEntitiesComponent {
        #[serde(rename = "brandKey")]
        pub brand_key: String,
        #[serde(
            rename = "defaultPrice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_price: Option<f64>,
        #[serde(rename = "productKey")]
        pub product_key: String,
        pub quantity: i32,
    }

    impl From<&ShipCompliantRestApiDomainsProductEntitiesComponent>
        for ShipCompliantRestApiDomainsProductEntitiesComponent
    {
        fn from(value: &ShipCompliantRestApiDomainsProductEntitiesComponent) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsProductEntitiesKitInput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"KitInput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"brandKey\","]
    #[doc = "    \"components\","]
    #[doc = "    \"description\","]
    #[doc = "    \"distributionType\","]
    #[doc = "    \"productKey\","]
    #[doc = "    \"type\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"brandKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"components\": {"]
    #[doc = "      \"title\": \"Component[]\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.Product.Entities.Component\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"distributionType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"productKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"totalWeight\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsProductEntitiesKitInput {
        #[serde(rename = "brandKey")]
        pub brand_key: String,
        pub components: Vec<ShipCompliantRestApiDomainsProductEntitiesComponent>,
        pub description: String,
        #[serde(rename = "distributionType")]
        pub distribution_type: String,
        #[serde(rename = "productKey")]
        pub product_key: String,
        #[serde(
            rename = "totalWeight",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub total_weight: Option<f64>,
        #[serde(rename = "type")]
        pub type_: String,
    }

    impl From<&ShipCompliantRestApiDomainsProductEntitiesKitInput>
        for ShipCompliantRestApiDomainsProductEntitiesKitInput
    {
        fn from(value: &ShipCompliantRestApiDomainsProductEntitiesKitInput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsProductEntitiesLabelOutput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"LabelOutput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"cola\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"serialNumber\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsProductEntitiesLabelOutput {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cola: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(
            rename = "serialNumber",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub serial_number: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsProductEntitiesLabelOutput>
        for ShipCompliantRestApiDomainsProductEntitiesLabelOutput
    {
        fn from(value: &ShipCompliantRestApiDomainsProductEntitiesLabelOutput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsProductEntitiesPostProductInput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PostProductInput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"productKey\","]
    #[doc = "    \"productType\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"age\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"integer\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"bottleSizeML\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"integer\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"brandKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"containerType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"containersPerSellingUnit\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"integer\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"defaultCase\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"defaultRetailUnitPrice\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"defaultWholesaleCasePrice\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"flavor\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"gtin\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"label\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"nabca\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"percentAlcohol\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"productDistribution\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"productKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"productType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"scc\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"shippingWeightInLbs\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"style\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"subBrand\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"unimerc\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"unitPrice\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"upc\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"varietal\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"vintage\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"integer\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"volumeAmount\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"volumeUnit\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsProductEntitiesPostProductInput {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub age: Option<i32>,
        #[serde(
            rename = "bottleSizeML",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub bottle_size_ml: Option<i32>,
        #[serde(rename = "brandKey", default, skip_serializing_if = "Option::is_none")]
        pub brand_key: Option<String>,
        #[serde(
            rename = "containerType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub container_type: Option<String>,
        #[serde(
            rename = "containersPerSellingUnit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub containers_per_selling_unit: Option<i32>,
        #[serde(
            rename = "defaultCase",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_case: Option<String>,
        #[serde(
            rename = "defaultRetailUnitPrice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_retail_unit_price: Option<f64>,
        #[serde(
            rename = "defaultWholesaleCasePrice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_wholesale_case_price: Option<f64>,
        pub description: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub flavor: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub gtin: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nabca: Option<String>,
        #[serde(
            rename = "percentAlcohol",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub percent_alcohol: Option<f64>,
        #[serde(
            rename = "productDistribution",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub product_distribution: Option<String>,
        #[serde(rename = "productKey")]
        pub product_key: String,
        #[serde(rename = "productType")]
        pub product_type: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scc: Option<String>,
        #[serde(
            rename = "shippingWeightInLbs",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipping_weight_in_lbs: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub style: Option<String>,
        #[serde(rename = "subBrand", default, skip_serializing_if = "Option::is_none")]
        pub sub_brand: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unimerc: Option<String>,
        #[serde(rename = "unitPrice", default, skip_serializing_if = "Option::is_none")]
        pub unit_price: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub upc: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub varietal: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub vintage: Option<i32>,
        #[serde(
            rename = "volumeAmount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub volume_amount: Option<f64>,
        #[serde(
            rename = "volumeUnit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub volume_unit: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsProductEntitiesPostProductInput>
        for ShipCompliantRestApiDomainsProductEntitiesPostProductInput
    {
        fn from(value: &ShipCompliantRestApiDomainsProductEntitiesPostProductInput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsProductEntitiesProductInput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ProductInput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"description\","]
    #[doc = "    \"productType\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"age\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"integer\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"bottleSizeML\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"integer\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"brandKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"containerType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"containersPerSellingUnit\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"integer\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"defaultCase\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"defaultRetailUnitPrice\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"defaultWholesaleCasePrice\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"flavor\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"gtin\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"label\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"nabca\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"percentAlcohol\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"productDistribution\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"productType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"scc\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"shippingWeightInLbs\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"style\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"subBrand\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"unimerc\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"unitPrice\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"upc\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"varietal\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"vintage\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"integer\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"volumeAmount\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"volumeUnit\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsProductEntitiesProductInput {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub age: Option<i32>,
        #[serde(
            rename = "bottleSizeML",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub bottle_size_ml: Option<i32>,
        #[serde(rename = "brandKey", default, skip_serializing_if = "Option::is_none")]
        pub brand_key: Option<String>,
        #[serde(
            rename = "containerType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub container_type: Option<String>,
        #[serde(
            rename = "containersPerSellingUnit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub containers_per_selling_unit: Option<i32>,
        #[serde(
            rename = "defaultCase",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_case: Option<String>,
        #[serde(
            rename = "defaultRetailUnitPrice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_retail_unit_price: Option<f64>,
        #[serde(
            rename = "defaultWholesaleCasePrice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_wholesale_case_price: Option<f64>,
        pub description: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub flavor: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub gtin: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nabca: Option<String>,
        #[serde(
            rename = "percentAlcohol",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub percent_alcohol: Option<f64>,
        #[serde(
            rename = "productDistribution",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub product_distribution: Option<String>,
        #[serde(rename = "productType")]
        pub product_type: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scc: Option<String>,
        #[serde(
            rename = "shippingWeightInLbs",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipping_weight_in_lbs: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub style: Option<String>,
        #[serde(rename = "subBrand", default, skip_serializing_if = "Option::is_none")]
        pub sub_brand: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unimerc: Option<String>,
        #[serde(rename = "unitPrice", default, skip_serializing_if = "Option::is_none")]
        pub unit_price: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub upc: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub varietal: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub vintage: Option<i32>,
        #[serde(
            rename = "volumeAmount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub volume_amount: Option<f64>,
        #[serde(
            rename = "volumeUnit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub volume_unit: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsProductEntitiesProductInput>
        for ShipCompliantRestApiDomainsProductEntitiesProductInput
    {
        fn from(value: &ShipCompliantRestApiDomainsProductEntitiesProductInput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsProductEntitiesProductOutput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ProductOutput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"age\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"integer\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"brandDescription\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"brandKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"brandName\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"containerType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"containersPerSellingUnit\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"integer\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"defaultCase\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"defaultRetailUnitPrice\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"defaultWholesaleCasePrice\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"flavor\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"gtin\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"label\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.Product.Entities.LabelOutput\""]
    #[doc = "    },"]
    #[doc = "    \"marketplaceSKUs\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"title\": \"String\","]
    #[doc = "        \"type\": \"string\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"nabca\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"percentAlcohol\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"productDistribution\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"productKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"productType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"scc\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"unimerc\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"unitPrice\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"upc\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"varietal\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"vintage\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"integer\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"volumeAmount\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"volumeUnit\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsProductEntitiesProductOutput {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub age: Option<i32>,
        #[serde(
            rename = "brandDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub brand_description: Option<String>,
        #[serde(rename = "brandKey", default, skip_serializing_if = "Option::is_none")]
        pub brand_key: Option<String>,
        #[serde(rename = "brandName", default, skip_serializing_if = "Option::is_none")]
        pub brand_name: Option<String>,
        #[serde(
            rename = "containerType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub container_type: Option<String>,
        #[serde(
            rename = "containersPerSellingUnit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub containers_per_selling_unit: Option<i32>,
        #[serde(
            rename = "defaultCase",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_case: Option<String>,
        #[serde(
            rename = "defaultRetailUnitPrice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_retail_unit_price: Option<f64>,
        #[serde(
            rename = "defaultWholesaleCasePrice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_wholesale_case_price: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub flavor: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub gtin: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub label: Option<ShipCompliantRestApiDomainsProductEntitiesLabelOutput>,
        #[serde(
            rename = "marketplaceSKUs",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub marketplace_sk_us: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nabca: Option<String>,
        #[serde(
            rename = "percentAlcohol",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub percent_alcohol: Option<f64>,
        #[serde(
            rename = "productDistribution",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub product_distribution: Option<String>,
        #[serde(
            rename = "productKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub product_key: Option<String>,
        #[serde(
            rename = "productType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub product_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scc: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unimerc: Option<String>,
        #[serde(rename = "unitPrice", default, skip_serializing_if = "Option::is_none")]
        pub unit_price: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub upc: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub varietal: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub vintage: Option<i32>,
        #[serde(
            rename = "volumeAmount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub volume_amount: Option<f64>,
        #[serde(
            rename = "volumeUnit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub volume_unit: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsProductEntitiesProductOutput>
        for ShipCompliantRestApiDomainsProductEntitiesProductOutput
    {
        fn from(value: &ShipCompliantRestApiDomainsProductEntitiesProductOutput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesAddress"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"Address\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"city\","]
    #[doc = "    \"firstName\","]
    #[doc = "    \"lastName\","]
    #[doc = "    \"state\","]
    #[doc = "    \"street1\","]
    #[doc = "    \"zip1\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"city\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"company\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"country\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"county\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"dateOfBirth\": {"]
    #[doc = "      \"title\": \"DateTime\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"email\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fax\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"firstName\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"lastName\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"phone\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"street1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"street2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"zip1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"zip2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesAddress {
        pub city: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub country: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub county: Option<String>,
        #[serde(
            rename = "dateOfBirth",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub date_of_birth: Option<chrono::NaiveDateTime>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fax: Option<String>,
        #[serde(rename = "firstName")]
        pub first_name: String,
        #[serde(rename = "lastName")]
        pub last_name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        pub state: String,
        pub street1: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street2: Option<String>,
        pub zip1: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zip2: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesAddress>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesAddress
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesAddress) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesAddressDetails"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"AddressDetails\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"cityAbbreviation\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"congressionalDistrict\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"countyFips\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"timeZone\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"timeZoneCode\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesAddressDetails {
        #[serde(
            rename = "cityAbbreviation",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub city_abbreviation: Option<String>,
        #[serde(
            rename = "congressionalDistrict",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub congressional_district: Option<String>,
        #[serde(
            rename = "countyFips",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub county_fips: Option<String>,
        #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
        pub time_zone: Option<String>,
        #[serde(
            rename = "timeZoneCode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub time_zone_code: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesAddressDetails>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesAddressDetails
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesAddressDetails) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesAddressOption"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"AddressOption\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"ignoreStreetLevelErrors\": {"]
    #[doc = "      \"title\": \"Boolean\","]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    },"]
    #[doc = "    \"rejectIfAddressSuggested\": {"]
    #[doc = "      \"title\": \"Boolean\","]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesAddressOption {
        #[serde(
            rename = "ignoreStreetLevelErrors",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ignore_street_level_errors: Option<bool>,
        #[serde(
            rename = "rejectIfAddressSuggested",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reject_if_address_suggested: Option<bool>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesAddressOption>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesAddressOption
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesAddressOption) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesAddressParts"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"AddressParts\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"company\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"mailBoxName\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"mailBoxNumber\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"postDirection\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"preDirection\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"streetName\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"streetNumber\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"streetSuffix\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"suiteName\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"suiteNumber\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesAddressParts {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
        #[serde(
            rename = "mailBoxName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub mail_box_name: Option<String>,
        #[serde(
            rename = "mailBoxNumber",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub mail_box_number: Option<String>,
        #[serde(
            rename = "postDirection",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub post_direction: Option<String>,
        #[serde(
            rename = "preDirection",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub pre_direction: Option<String>,
        #[serde(
            rename = "streetName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub street_name: Option<String>,
        #[serde(
            rename = "streetNumber",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub street_number: Option<String>,
        #[serde(
            rename = "streetSuffix",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub street_suffix: Option<String>,
        #[serde(rename = "suiteName", default, skip_serializing_if = "Option::is_none")]
        pub suite_name: Option<String>,
        #[serde(
            rename = "suiteNumber",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub suite_number: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesAddressParts>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesAddressParts
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesAddressParts) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesAddressSuggestion"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"AddressSuggestion\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"city\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"county\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"details\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.AddressDetails\""]
    #[doc = "    },"]
    #[doc = "    \"parts\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.AddressParts\""]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"street1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"street2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"zip1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"zip2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesAddressSuggestion {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub county: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub details: Option<ShipCompliantRestApiDomainsSalesOrderEntitiesAddressDetails>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parts: Option<ShipCompliantRestApiDomainsSalesOrderEntitiesAddressParts>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street2: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zip1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zip2: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesAddressSuggestion>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesAddressSuggestion
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesAddressSuggestion) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesCheckAndCommitSalesOrder"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"CheckAndCommitSalesOrder\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"salesOrder\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"addressOption\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.AddressOption\""]
    #[doc = "    },"]
    #[doc = "    \"commitOption\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"persistOption\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesOrder\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.SalesOrder\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesCheckAndCommitSalesOrder {
        #[serde(
            rename = "addressOption",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub address_option: Option<ShipCompliantRestApiDomainsSalesOrderEntitiesAddressOption>,
        #[serde(
            rename = "commitOption",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub commit_option: Option<String>,
        #[serde(
            rename = "persistOption",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub persist_option: Option<String>,
        #[serde(rename = "salesOrder")]
        pub sales_order: ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrder,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesCheckAndCommitSalesOrder>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesCheckAndCommitSalesOrder
    {
        fn from(
            value: &ShipCompliantRestApiDomainsSalesOrderEntitiesCheckAndCommitSalesOrder,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesCommitSalesOrder"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"CommitSalesOrder\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"commitOption\","]
    #[doc = "    \"salesOrderKey\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"commitOption\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"payments\": {"]
    #[doc = "      \"title\": \"PaymentCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.Payment\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"salesOrderKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"salesTaxCollected\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesCommitSalesOrder {
        #[serde(rename = "commitOption")]
        pub commit_option: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payments: Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesPayment>>,
        #[serde(rename = "salesOrderKey")]
        pub sales_order_key: String,
        #[serde(
            rename = "salesTaxCollected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_collected: Option<f64>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesCommitSalesOrder>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesCommitSalesOrder
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesCommitSalesOrder) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesComplianceDetailResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ComplianceDetailResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"ComplianceDetailType\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"BrandNameNotRegistered\","]
    #[doc = "        \"LabelNotRegistered\","]
    #[doc = "        \"NextShipDate\","]
    #[doc = "        \"VolumeOverLimit\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"value\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesComplianceDetailResponse {
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<ComplianceDetailType>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesComplianceDetailResponse>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesComplianceDetailResponse
    {
        fn from(
            value: &ShipCompliantRestApiDomainsSalesOrderEntitiesComplianceDetailResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesCustomerAggregateVolumeLimitDetail"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"CustomerAggregateVolumeLimitDetail\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"customerType\": {"]
    #[doc = "      \"title\": \"CustomerType\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Household\","]
    #[doc = "        \"Individual\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"timeFrameCount\": {"]
    #[doc = "      \"title\": \"Int32\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"timeFrameType\": {"]
    #[doc = "      \"title\": \"TimeFrameType\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Calendar\","]
    #[doc = "        \"Rolling\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"timeFrameUnit\": {"]
    #[doc = "      \"title\": \"TimeFrameUnit\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Minutes\","]
    #[doc = "        \"Hours\","]
    #[doc = "        \"Days\","]
    #[doc = "        \"Weeks\","]
    #[doc = "        \"Months\","]
    #[doc = "        \"Years\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"unitOfMeasure\": {"]
    #[doc = "      \"title\": \"VolumeUnit\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Bottle\","]
    #[doc = "        \"Case\","]
    #[doc = "        \"Gallon\","]
    #[doc = "        \"Liter\","]
    #[doc = "        \"Milliliter\","]
    #[doc = "        \"Ounce\","]
    #[doc = "        \"Quart\","]
    #[doc = "        \"Barrel\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"volumeCount\": {"]
    #[doc = "      \"title\": \"Int32\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesCustomerAggregateVolumeLimitDetail {
        #[serde(
            rename = "customerType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub customer_type: Option<CustomerType>,
        #[serde(
            rename = "timeFrameCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub time_frame_count: Option<i32>,
        #[serde(
            rename = "timeFrameType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub time_frame_type: Option<TimeFrameType>,
        #[serde(
            rename = "timeFrameUnit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub time_frame_unit: Option<TimeFrameUnit>,
        #[serde(
            rename = "unitOfMeasure",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub unit_of_measure: Option<VolumeUnit>,
        #[serde(
            rename = "volumeCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub volume_count: Option<i32>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesCustomerAggregateVolumeLimitDetail>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesCustomerAggregateVolumeLimitDetail
    {
        fn from(
            value: &ShipCompliantRestApiDomainsSalesOrderEntitiesCustomerAggregateVolumeLimitDetail,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesFreightSalesTaxRate"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"FreightSalesTaxRate\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"salesTaxDue\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"salesTaxRate\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesFreightSalesTaxRate {
        #[serde(
            rename = "salesTaxDue",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_due: Option<f64>,
        #[serde(
            rename = "salesTaxRate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_rate: Option<f64>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesFreightSalesTaxRate>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesFreightSalesTaxRate
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesFreightSalesTaxRate) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesPackage"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"Package\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"trackingNumber\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesPackage {
        #[serde(
            rename = "trackingNumber",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub tracking_number: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesPackage>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesPackage
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesPackage) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesPackageOutput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PackageOutput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"trackingNumber\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"trackingStatus\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesPackageOutput {
        #[serde(
            rename = "trackingNumber",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub tracking_number: Option<String>,
        #[serde(
            rename = "trackingStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub tracking_status: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesPackageOutput>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesPackageOutput
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesPackageOutput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesPayment"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"Payment\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"amount\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"subType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"transactionID\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesPayment {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
        #[serde(rename = "subType", default, skip_serializing_if = "Option::is_none")]
        pub sub_type: Option<String>,
        #[serde(
            rename = "transactionID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub transaction_id: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesPayment>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesPayment
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesPayment) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesPaymentOutput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PaymentOutput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"amount\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"subType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"transactionID\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"PaymentTypes\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Cash\","]
    #[doc = "        \"Check\","]
    #[doc = "        \"Creditcard\","]
    #[doc = "        \"Giftcard\","]
    #[doc = "        \"Giftcertificate\","]
    #[doc = "        \"Invoice\","]
    #[doc = "        \"Moneyorder\","]
    #[doc = "        \"Other\","]
    #[doc = "        \"Storeaccount\","]
    #[doc = "        \"Travelerscheck\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesPaymentOutput {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
        #[serde(rename = "subType", default, skip_serializing_if = "Option::is_none")]
        pub sub_type: Option<String>,
        #[serde(
            rename = "transactionID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub transaction_id: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<PaymentTypes>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesPaymentOutput>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesPaymentOutput
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesPaymentOutput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesPerBottleVolumeLimitDetail"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PerBottleVolumeLimitDetail\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"volumeCount\": {"]
    #[doc = "      \"title\": \"Int32\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"volumeUnit\": {"]
    #[doc = "      \"title\": \"VolumeUnit\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Bottle\","]
    #[doc = "        \"Case\","]
    #[doc = "        \"Gallon\","]
    #[doc = "        \"Liter\","]
    #[doc = "        \"Milliliter\","]
    #[doc = "        \"Ounce\","]
    #[doc = "        \"Quart\","]
    #[doc = "        \"Barrel\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesPerBottleVolumeLimitDetail {
        #[serde(
            rename = "volumeCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub volume_count: Option<i32>,
        #[serde(
            rename = "volumeUnit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub volume_unit: Option<VolumeUnit>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesPerBottleVolumeLimitDetail>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesPerBottleVolumeLimitDetail
    {
        fn from(
            value: &ShipCompliantRestApiDomainsSalesOrderEntitiesPerBottleVolumeLimitDetail,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesPerShipmentVolumeLimitDetail"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PerShipmentVolumeLimitDetail\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"volumeCount\": {"]
    #[doc = "      \"title\": \"Int32\","]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"format\": \"int32\""]
    #[doc = "    },"]
    #[doc = "    \"volumeUnit\": {"]
    #[doc = "      \"title\": \"VolumeUnit\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Bottle\","]
    #[doc = "        \"Case\","]
    #[doc = "        \"Gallon\","]
    #[doc = "        \"Liter\","]
    #[doc = "        \"Milliliter\","]
    #[doc = "        \"Ounce\","]
    #[doc = "        \"Quart\","]
    #[doc = "        \"Barrel\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesPerShipmentVolumeLimitDetail {
        #[serde(
            rename = "volumeCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub volume_count: Option<i32>,
        #[serde(
            rename = "volumeUnit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub volume_unit: Option<VolumeUnit>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesPerShipmentVolumeLimitDetail>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesPerShipmentVolumeLimitDetail
    {
        fn from(
            value: &ShipCompliantRestApiDomainsSalesOrderEntitiesPerShipmentVolumeLimitDetail,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesProductSalesTaxRate"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ProductSalesTaxRate\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"brandKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"productKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesTaxDue\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"salesTaxRate\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesProductSalesTaxRate {
        #[serde(rename = "brandKey", default, skip_serializing_if = "Option::is_none")]
        pub brand_key: Option<String>,
        #[serde(
            rename = "productKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub product_key: Option<String>,
        #[serde(
            rename = "salesTaxDue",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_due: Option<f64>,
        #[serde(
            rename = "salesTaxRate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_rate: Option<f64>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesProductSalesTaxRate>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesProductSalesTaxRate
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesProductSalesTaxRate) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteSalesOrder"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"QuoteSalesOrder\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"billTo\","]
    #[doc = "    \"customerKey\","]
    #[doc = "    \"orderType\","]
    #[doc = "    \"purchaseDate\","]
    #[doc = "    \"shipments\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"billTo\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.Address\""]
    #[doc = "    },"]
    #[doc = "    \"customerKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"discounts\": {"]
    #[doc = "      \"title\": \"SalesOrderDiscountCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.SalesOrderDiscount\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"orderType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"payments\": {"]
    #[doc = "      \"title\": \"PaymentCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.Payment\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"purchaseDate\": {"]
    #[doc = "      \"title\": \"DateTime\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"salesOrderKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesTaxCollected\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"shipments\": {"]
    #[doc = "      \"title\": \"ShipmentCollection\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.Shipment\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"tags\": {"]
    #[doc = "      \"title\": \"TagCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.Tag\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteSalesOrder {
        #[serde(rename = "billTo")]
        pub bill_to: ShipCompliantRestApiDomainsSalesOrderEntitiesAddress,
        #[serde(rename = "customerKey")]
        pub customer_key: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub discounts: Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderDiscount>>,
        #[serde(
            rename = "fulfillmentType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_type: Option<String>,
        #[serde(rename = "orderType")]
        pub order_type: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payments: Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesPayment>>,
        #[serde(rename = "purchaseDate")]
        pub purchase_date: chrono::NaiveDateTime,
        #[serde(
            rename = "salesOrderKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_order_key: Option<String>,
        #[serde(
            rename = "salesTaxCollected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_collected: Option<f64>,
        pub shipments: Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesShipment>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesTag>>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteSalesOrder>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteSalesOrder
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteSalesOrder) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteSalesOrderInput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"QuoteSalesOrderInput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"addressOption\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.AddressOption\""]
    #[doc = "    },"]
    #[doc = "    \"salesOrder\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.QuoteSalesOrder\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteSalesOrderInput {
        #[serde(
            rename = "addressOption",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub address_option: Option<ShipCompliantRestApiDomainsSalesOrderEntitiesAddressOption>,
        #[serde(
            rename = "salesOrder",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_order: Option<ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteSalesOrder>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteSalesOrderInput>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteSalesOrderInput
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteSalesOrderInput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteTaxForSalesOrder"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"QuoteTaxForSalesOrder\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"orderItems\","]
    #[doc = "    \"shipToAddress\","]
    #[doc = "    \"taxSaleType\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"effectiveDate\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"orderItems\": {"]
    #[doc = "      \"title\": \"ShipmentItemTaxCollection\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.ShipmentItemForTax\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"shipToAddress\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.TaxRateAddressInput\""]
    #[doc = "    },"]
    #[doc = "    \"shippingAndHandlingCollected\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"taxSaleType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteTaxForSalesOrder {
        #[serde(
            rename = "effectiveDate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub effective_date: Option<chrono::NaiveDateTime>,
        #[serde(rename = "orderItems")]
        pub order_items: Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItemForTax>,
        #[serde(rename = "shipToAddress")]
        pub ship_to_address: ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateAddressInput,
        #[serde(
            rename = "shippingAndHandlingCollected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipping_and_handling_collected: Option<f64>,
        #[serde(rename = "taxSaleType")]
        pub tax_sale_type: String,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteTaxForSalesOrder>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteTaxForSalesOrder
    {
        fn from(
            value: &ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteTaxForSalesOrder,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "Fees that the states require e.g 'CO Retail Delivery Fee'"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"RetailDeliveryFees\","]
    #[doc = "  \"description\": \"Fees that the states require e.g 'CO Retail Delivery Fee'\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"feeAmount\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"description\": \"Total amount that you pay for this Fee\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"description\": \"Name of the Fees require by State\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesRetailDeliveryFees {
        #[serde(rename = "feeAmount", default, skip_serializing_if = "Option::is_none")]
        pub fee_amount: Option<f64>,
        #[doc = "Name of the Fees require by State"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesRetailDeliveryFees>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesRetailDeliveryFees
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesRetailDeliveryFees) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrder"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrder\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"billTo\","]
    #[doc = "    \"customerKey\","]
    #[doc = "    \"orderType\","]
    #[doc = "    \"purchaseDate\","]
    #[doc = "    \"salesOrderKey\","]
    #[doc = "    \"shipments\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"billTo\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.Address\""]
    #[doc = "    },"]
    #[doc = "    \"customerKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"discounts\": {"]
    #[doc = "      \"title\": \"SalesOrderDiscountCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.SalesOrderDiscount\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"orderType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"payments\": {"]
    #[doc = "      \"title\": \"PaymentCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.Payment\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"purchaseDate\": {"]
    #[doc = "      \"title\": \"DateTime\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"salesOrderKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"salesTaxCollected\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"shipments\": {"]
    #[doc = "      \"title\": \"ShipmentCollection\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.Shipment\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"tags\": {"]
    #[doc = "      \"title\": \"TagCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.Tag\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrder {
        #[serde(rename = "billTo")]
        pub bill_to: ShipCompliantRestApiDomainsSalesOrderEntitiesAddress,
        #[serde(rename = "customerKey")]
        pub customer_key: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub discounts: Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderDiscount>>,
        #[serde(
            rename = "fulfillmentType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_type: Option<String>,
        #[serde(rename = "orderType")]
        pub order_type: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payments: Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesPayment>>,
        #[serde(rename = "purchaseDate")]
        pub purchase_date: chrono::NaiveDateTime,
        #[serde(rename = "salesOrderKey")]
        pub sales_order_key: String,
        #[serde(
            rename = "salesTaxCollected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_collected: Option<f64>,
        pub shipments: Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesShipment>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesTag>>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrder>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrder
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrder) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderBase"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderBase\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"salesOrder\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"addressOption\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.AddressOption\""]
    #[doc = "    },"]
    #[doc = "    \"persistOption\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesOrder\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.SalesOrder\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderBase {
        #[serde(
            rename = "addressOption",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub address_option: Option<ShipCompliantRestApiDomainsSalesOrderEntitiesAddressOption>,
        #[serde(
            rename = "persistOption",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub persist_option: Option<String>,
        #[serde(rename = "salesOrder")]
        pub sales_order: ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrder,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderBase>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderBase
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderBase) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderDiscount"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderDiscount\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"amount\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderDiscount {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderDiscount>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderDiscount
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderDiscount) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderOutput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderOutput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"billTo\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.Address\""]
    #[doc = "    },"]
    #[doc = "    \"customerKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"discounts\": {"]
    #[doc = "      \"title\": \"SalesOrderDiscountCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.SalesOrderDiscount\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentType\": {"]
    #[doc = "      \"title\": \"FulfillmentType\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Club\","]
    #[doc = "        \"Daily\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"orderType\": {"]
    #[doc = "      \"title\": \"OrderTypes\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Club\","]
    #[doc = "        \"Fax\","]
    #[doc = "        \"InPerson\","]
    #[doc = "        \"Internet\","]
    #[doc = "        \"Mail\","]
    #[doc = "        \"Phone\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"payments\": {"]
    #[doc = "      \"title\": \"PaymentOutputCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.PaymentOutput\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"purchaseDate\": {"]
    #[doc = "      \"title\": \"DateTime\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"salesOrderKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesTaxCollected\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"salesTaxDue\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"shipments\": {"]
    #[doc = "      \"title\": \"ShipmentOutputCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.ShipmentOutput\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"tags\": {"]
    #[doc = "      \"title\": \"TagCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.Tag\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderOutput {
        #[serde(rename = "billTo", default, skip_serializing_if = "Option::is_none")]
        pub bill_to: Option<ShipCompliantRestApiDomainsSalesOrderEntitiesAddress>,
        #[serde(
            rename = "customerKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub customer_key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub discounts: Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderDiscount>>,
        #[serde(
            rename = "fulfillmentType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_type: Option<FulfillmentType>,
        #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
        pub order_type: Option<OrderTypes>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payments: Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesPaymentOutput>>,
        #[serde(
            rename = "purchaseDate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub purchase_date: Option<chrono::NaiveDateTime>,
        #[serde(
            rename = "salesOrderKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_order_key: Option<String>,
        #[serde(
            rename = "salesTaxCollected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_collected: Option<f64>,
        #[serde(
            rename = "salesTaxDue",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_due: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub shipments: Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentOutput>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesTag>>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderOutput>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderOutput
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderOutput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesSalesTaxRates"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesTaxRates\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"recommendedSalesTaxDue\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"retailDeliveryFees\": {"]
    #[doc = "      \"title\": \"IList`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.RetailDeliveryFees\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"shipmentSalesTaxRates\": {"]
    #[doc = "      \"title\": \"ShipmentSalesTaxRateCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.ShipmentSalesTaxRate\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesSalesTaxRates {
        #[serde(
            rename = "recommendedSalesTaxDue",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub recommended_sales_tax_due: Option<f64>,
        #[serde(
            rename = "retailDeliveryFees",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub retail_delivery_fees:
            Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesRetailDeliveryFees>>,
        #[serde(
            rename = "shipmentSalesTaxRates",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_sales_tax_rates:
            Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentSalesTaxRate>>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesSalesTaxRates>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesSalesTaxRates
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesSalesTaxRates) -> Self {
            value.clone()
        }
    }

    #[doc = "Request send by Client"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesTaxRatesByAddress\","]
    #[doc = "  \"description\": \"Request send by Client\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"address\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.TaxRateAddressInput\""]
    #[doc = "    },"]
    #[doc = "    \"effectiveDate\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"description\": \"EffectiveDate\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"licenseRelationship\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"description\": \"LicenseRelationship\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"taxSaleType\": {"]
    #[doc = "      \"title\": \"TaxSaleType\","]
    #[doc = "      \"description\": \"TaxSaleType\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Offsite\","]
    #[doc = "        \"Onsite\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesSalesTaxRatesByAddress {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateAddressInput>,
        #[doc = "EffectiveDate"]
        #[serde(
            rename = "effectiveDate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub effective_date: Option<chrono::NaiveDateTime>,
        #[doc = "LicenseRelationship"]
        #[serde(
            rename = "licenseRelationship",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub license_relationship: Option<String>,
        #[doc = "TaxSaleType"]
        #[serde(
            rename = "taxSaleType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub tax_sale_type: Option<TaxSaleType>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesSalesTaxRatesByAddress>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesSalesTaxRatesByAddress
    {
        fn from(
            value: &ShipCompliantRestApiDomainsSalesOrderEntitiesSalesTaxRatesByAddress,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesShipment"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"Shipment\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"licenseRelationship\","]
    #[doc = "    \"shipDate\","]
    #[doc = "    \"shipTo\","]
    #[doc = "    \"shipmentItems\","]
    #[doc = "    \"shipmentStatus\","]
    #[doc = "    \"shippingService\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"discounts\": {"]
    #[doc = "      \"title\": \"ShipmentDiscountCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.ShipmentDiscount\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentAccount\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentExceptionReason\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentExceptionType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentHouse\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"giftNote\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"handling\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"licenseRelationship\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"packages\": {"]
    #[doc = "      \"title\": \"PackageCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.Package\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"shipDate\": {"]
    #[doc = "      \"title\": \"DateTime\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"shipTo\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.Address\""]
    #[doc = "    },"]
    #[doc = "    \"shipmentItems\": {"]
    #[doc = "      \"title\": \"ShipmentItemCollection\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.ShipmentItem\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"shipmentKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"shipmentStatus\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"shipping\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"shippingService\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"specialInstructions\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesShipment {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub discounts: Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentDiscount>>,
        #[serde(
            rename = "fulfillmentAccount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_account: Option<String>,
        #[serde(
            rename = "fulfillmentExceptionReason",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_exception_reason: Option<String>,
        #[serde(
            rename = "fulfillmentExceptionType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_exception_type: Option<String>,
        #[serde(
            rename = "fulfillmentHouse",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_house: Option<String>,
        #[serde(rename = "giftNote", default, skip_serializing_if = "Option::is_none")]
        pub gift_note: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub handling: Option<f64>,
        #[serde(rename = "licenseRelationship")]
        pub license_relationship: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub packages: Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesPackage>>,
        #[serde(rename = "shipDate")]
        pub ship_date: chrono::NaiveDateTime,
        #[serde(rename = "shipTo")]
        pub ship_to: ShipCompliantRestApiDomainsSalesOrderEntitiesAddress,
        #[serde(rename = "shipmentItems")]
        pub shipment_items: Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItem>,
        #[serde(
            rename = "shipmentKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_key: Option<String>,
        #[serde(rename = "shipmentStatus")]
        pub shipment_status: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub shipping: Option<f64>,
        #[serde(rename = "shippingService")]
        pub shipping_service: String,
        #[serde(
            rename = "specialInstructions",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub special_instructions: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesShipment>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesShipment
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesShipment) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentDiscount"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentDiscount\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"amount\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentDiscount {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentDiscount>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentDiscount
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentDiscount) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItem"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentItem\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"productKey\","]
    #[doc = "    \"productQuantity\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"brandKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"citb\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"discounts\": {"]
    #[doc = "      \"title\": \"ShipmentItemDiscountCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.ShipmentItemDiscount\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"productKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"productQuantity\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"productUnitPrice\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItem {
        #[serde(rename = "brandKey", default, skip_serializing_if = "Option::is_none")]
        pub brand_key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub citb: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub discounts:
            Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItemDiscount>>,
        #[serde(rename = "productKey")]
        pub product_key: String,
        #[serde(rename = "productQuantity")]
        pub product_quantity: f64,
        #[serde(
            rename = "productUnitPrice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub product_unit_price: Option<f64>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItem>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItem
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItem) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItemDiscount"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentItemDiscount\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"amount\": {"]
    #[doc = "      \"title\": \"Decimal\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItemDiscount {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItemDiscount>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItemDiscount
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItemDiscount) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItemForTax"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentItemForTax\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"productKey\","]
    #[doc = "    \"productQuantity\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"brandKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"productKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"productQuantity\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"productUnitPrice\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItemForTax {
        #[serde(rename = "brandKey", default, skip_serializing_if = "Option::is_none")]
        pub brand_key: Option<String>,
        #[serde(rename = "productKey")]
        pub product_key: String,
        #[serde(rename = "productQuantity")]
        pub product_quantity: f64,
        #[serde(
            rename = "productUnitPrice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub product_unit_price: Option<f64>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItemForTax>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItemForTax
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItemForTax) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentOutput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentOutput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"discounts\": {"]
    #[doc = "      \"title\": \"ShipmentDiscountCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.ShipmentDiscount\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentAccount\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentExceptionReason\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentExceptionType\": {"]
    #[doc = "      \"title\": \"FulfillmentExceptionTypes\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"None\","]
    #[doc = "        \"Inventory\","]
    #[doc = "        \"NonCompliant\","]
    #[doc = "        \"Other\","]
    #[doc = "        \"Updated\","]
    #[doc = "        \"Setup\","]
    #[doc = "        \"Temperature\","]
    #[doc = "        \"Voided\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fulfillmentHouse\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"giftNote\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"handling\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"licenseRelationship\": {"]
    #[doc = "      \"title\": \"LicenseRelationshipTypes\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Default\","]
    #[doc = "        \"Pickup\","]
    #[doc = "        \"RetailerToConsumer\","]
    #[doc = "        \"RetailerToThreeTier\","]
    #[doc = "        \"SupplierToConsumer\","]
    #[doc = "        \"SupplierToThreeTier\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"packages\": {"]
    #[doc = "      \"title\": \"PackageOutputCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.PackageOutput\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"shipDate\": {"]
    #[doc = "      \"title\": \"DateTime\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"shipTo\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.Address\""]
    #[doc = "    },"]
    #[doc = "    \"shipmentItems\": {"]
    #[doc = "      \"title\": \"ShipmentItemCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.ShipmentItem\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"shipmentKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"shipmentStatus\": {"]
    #[doc = "      \"title\": \"ShipmentStatusTypes\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Delivered\","]
    #[doc = "        \"InProcess\","]
    #[doc = "        \"Shipped\","]
    #[doc = "        \"Voided\","]
    #[doc = "        \"SentToFulfillment\","]
    #[doc = "        \"PaymentAccepted\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"shipping\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"shippingService\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"specialInstructions\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentOutput {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub discounts: Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentDiscount>>,
        #[serde(
            rename = "fulfillmentAccount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_account: Option<String>,
        #[serde(
            rename = "fulfillmentExceptionReason",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_exception_reason: Option<String>,
        #[serde(
            rename = "fulfillmentExceptionType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_exception_type: Option<FulfillmentExceptionTypes>,
        #[serde(
            rename = "fulfillmentHouse",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fulfillment_house: Option<String>,
        #[serde(rename = "giftNote", default, skip_serializing_if = "Option::is_none")]
        pub gift_note: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub handling: Option<f64>,
        #[serde(
            rename = "licenseRelationship",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub license_relationship: Option<LicenseRelationshipTypes>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub packages: Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesPackageOutput>>,
        #[serde(rename = "shipDate", default, skip_serializing_if = "Option::is_none")]
        pub ship_date: Option<chrono::NaiveDateTime>,
        #[serde(rename = "shipTo", default, skip_serializing_if = "Option::is_none")]
        pub ship_to: Option<ShipCompliantRestApiDomainsSalesOrderEntitiesAddress>,
        #[serde(
            rename = "shipmentItems",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_items: Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentItem>>,
        #[serde(
            rename = "shipmentKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_key: Option<String>,
        #[serde(
            rename = "shipmentStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_status: Option<ShipmentStatusTypes>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub shipping: Option<f64>,
        #[serde(
            rename = "shippingService",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipping_service: Option<String>,
        #[serde(
            rename = "specialInstructions",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub special_instructions: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentOutput>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentOutput
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentOutput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentSalesTaxRate"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentSalesTaxRate\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"freightSalesTaxRate\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.FreightSalesTaxRate\""]
    #[doc = "    },"]
    #[doc = "    \"productSalesTaxRates\": {"]
    #[doc = "      \"title\": \"ProductSalesTaxRateCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.ProductSalesTaxRate\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"shipmentKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentSalesTaxRate {
        #[serde(
            rename = "freightSalesTaxRate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub freight_sales_tax_rate:
            Option<ShipCompliantRestApiDomainsSalesOrderEntitiesFreightSalesTaxRate>,
        #[serde(
            rename = "productSalesTaxRates",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub product_sales_tax_rates:
            Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesProductSalesTaxRate>>,
        #[serde(
            rename = "shipmentKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_key: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentSalesTaxRate>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentSalesTaxRate
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesShipmentSalesTaxRate) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesTag"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"Tag\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesTag {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesTag>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesTag
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesTag) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateAddressInput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TaxRateAddressInput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"zip1\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"city\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"street1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"street2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"zip1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"zip2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateAddressInput {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street2: Option<String>,
        pub zip1: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zip2: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateAddressInput>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateAddressInput
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateAddressInput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateAddressOutput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TaxRateAddressOutput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"addressStatus\": {"]
    #[doc = "      \"title\": \"AddressStatus\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"AddressOutOfRange\","]
    #[doc = "        \"AddressSuggested\","]
    #[doc = "        \"ComponentMismatch\","]
    #[doc = "        \"MultipleMatches\","]
    #[doc = "        \"NonDeliverableAddress\","]
    #[doc = "        \"NoStreetData\","]
    #[doc = "        \"UnknownStreet\","]
    #[doc = "        \"Validated\","]
    #[doc = "        \"ZipCodeDoesNotExist\","]
    #[doc = "        \"ZipCodeDoesNotMatchCityState\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"city\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"county\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"state\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"street1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"street2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"zip1\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"zip2\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateAddressOutput {
        #[serde(
            rename = "addressStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub address_status: Option<AddressStatus>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub city: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub county: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street2: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zip1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zip2: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateAddressOutput>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateAddressOutput
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateAddressOutput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateOutput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TaxRateOutput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"description\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"foodSalesTaxPercent\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"freightSalesTaxPercent\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"merchandiseSalesTaxPercent\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"wineSalesTaxPercent\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateOutput {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(
            rename = "foodSalesTaxPercent",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub food_sales_tax_percent: Option<f64>,
        #[serde(
            rename = "freightSalesTaxPercent",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub freight_sales_tax_percent: Option<f64>,
        #[serde(
            rename = "merchandiseSalesTaxPercent",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub merchandise_sales_tax_percent: Option<f64>,
        #[serde(
            rename = "wineSalesTaxPercent",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub wine_sales_tax_percent: Option<f64>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateOutput>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateOutput
    {
        fn from(value: &ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateOutput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderEntitiesUpdateShipmentStatusInput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"UpdateShipmentStatusInput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"salesOrderKey\","]
    #[doc = "    \"shipmentKey\","]
    #[doc = "    \"shipmentStatus\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"salesOrderKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"shipmentKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"shipmentStatus\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderEntitiesUpdateShipmentStatusInput {
        #[serde(rename = "salesOrderKey")]
        pub sales_order_key: String,
        #[serde(rename = "shipmentKey")]
        pub shipment_key: String,
        #[serde(rename = "shipmentStatus")]
        pub shipment_status: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderEntitiesUpdateShipmentStatusInput>
        for ShipCompliantRestApiDomainsSalesOrderEntitiesUpdateShipmentStatusInput
    {
        fn from(
            value: &ShipCompliantRestApiDomainsSalesOrderEntitiesUpdateShipmentStatusInput,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderResponsesRuleComplianceResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"RuleComplianceResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"complianceDescription\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"complianceDetails\": {"]
    #[doc = "      \"title\": \"ComplianceDetailResponseCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.ComplianceDetailResponse\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"complianceStatus\": {"]
    #[doc = "      \"title\": \"RuleComplianceStatus\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Compliant\","]
    #[doc = "        \"NotCompliant\","]
    #[doc = "        \"Bypassed\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"customerAggregateVolumeLimitDetail\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.CustomerAggregateVolumeLimitDetail\""]
    #[doc = "    },"]
    #[doc = "    \"licenseRelationship\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"perBottleVolumeLimitDetail\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.PerBottleVolumeLimitDetail\""]
    #[doc = "    },"]
    #[doc = "    \"perShipmentVolumeLimitDetail\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.PerShipmentVolumeLimitDetail\""]
    #[doc = "    },"]
    #[doc = "    \"ruleDescription\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"ruleType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"supplier\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderResponsesRuleComplianceResponse {
        #[serde(
            rename = "complianceDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub compliance_description: Option<String>,
        #[serde(
            rename = "complianceDetails",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub compliance_details:
            Option<Vec<ShipCompliantRestApiDomainsSalesOrderEntitiesComplianceDetailResponse>>,
        #[serde(
            rename = "complianceStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub compliance_status: Option<RuleComplianceStatus>,
        #[serde(
            rename = "customerAggregateVolumeLimitDetail",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub customer_aggregate_volume_limit_detail:
            Option<ShipCompliantRestApiDomainsSalesOrderEntitiesCustomerAggregateVolumeLimitDetail>,
        #[serde(
            rename = "licenseRelationship",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub license_relationship: Option<String>,
        #[serde(
            rename = "perBottleVolumeLimitDetail",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub per_bottle_volume_limit_detail:
            Option<ShipCompliantRestApiDomainsSalesOrderEntitiesPerBottleVolumeLimitDetail>,
        #[serde(
            rename = "perShipmentVolumeLimitDetail",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub per_shipment_volume_limit_detail:
            Option<ShipCompliantRestApiDomainsSalesOrderEntitiesPerShipmentVolumeLimitDetail>,
        #[serde(
            rename = "ruleDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub rule_description: Option<String>,
        #[serde(rename = "ruleType", default, skip_serializing_if = "Option::is_none")]
        pub rule_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub supplier: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderResponsesRuleComplianceResponse>
        for ShipCompliantRestApiDomainsSalesOrderResponsesRuleComplianceResponse
    {
        fn from(
            value: &ShipCompliantRestApiDomainsSalesOrderResponsesRuleComplianceResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderResponsesShipmentCommitResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentCommitResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"isCommitted\": {"]
    #[doc = "      \"title\": \"Boolean\","]
    #[doc = "      \"type\": \"boolean\""]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderResponsesShipmentCommitResponse {
        #[serde(
            rename = "isCommitted",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub is_committed: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderResponsesShipmentCommitResponse>
        for ShipCompliantRestApiDomainsSalesOrderResponsesShipmentCommitResponse
    {
        fn from(
            value: &ShipCompliantRestApiDomainsSalesOrderResponsesShipmentCommitResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsSalesOrderResponsesShipmentComplianceResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentComplianceResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"complianceStatus\": {"]
    #[doc = "      \"title\": \"RuleComplianceStatus\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Compliant\","]
    #[doc = "        \"NotCompliant\","]
    #[doc = "        \"Bypassed\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"rules\": {"]
    #[doc = "      \"title\": \"RuleComplianceResponseCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Responses.RuleComplianceResponse\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsSalesOrderResponsesShipmentComplianceResponse {
        #[serde(
            rename = "complianceStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub compliance_status: Option<RuleComplianceStatus>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rules:
            Option<Vec<ShipCompliantRestApiDomainsSalesOrderResponsesRuleComplianceResponse>>,
    }

    impl From<&ShipCompliantRestApiDomainsSalesOrderResponsesShipmentComplianceResponse>
        for ShipCompliantRestApiDomainsSalesOrderResponsesShipmentComplianceResponse
    {
        fn from(
            value: &ShipCompliantRestApiDomainsSalesOrderResponsesShipmentComplianceResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsTrackingEntitiesTrackingInput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TrackingInput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"trackingNumbers\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"shipmentKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"trackingNumbers\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"title\": \"String\","]
    #[doc = "        \"type\": \"string\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsTrackingEntitiesTrackingInput {
        #[serde(
            rename = "shipmentKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_key: Option<String>,
        #[serde(rename = "trackingNumbers")]
        pub tracking_numbers: Vec<String>,
    }

    impl From<&ShipCompliantRestApiDomainsTrackingEntitiesTrackingInput>
        for ShipCompliantRestApiDomainsTrackingEntitiesTrackingInput
    {
        fn from(value: &ShipCompliantRestApiDomainsTrackingEntitiesTrackingInput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsTrackingEntitiesTrackingOutput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TrackingOutput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"shipmentKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"trackingNumbers\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"title\": \"String\","]
    #[doc = "        \"type\": \"string\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsTrackingEntitiesTrackingOutput {
        #[serde(
            rename = "shipmentKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_key: Option<String>,
        #[serde(
            rename = "trackingNumbers",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub tracking_numbers: Option<Vec<String>>,
    }

    impl From<&ShipCompliantRestApiDomainsTrackingEntitiesTrackingOutput>
        for ShipCompliantRestApiDomainsTrackingEntitiesTrackingOutput
    {
        fn from(value: &ShipCompliantRestApiDomainsTrackingEntitiesTrackingOutput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsWholesaleEntitiesOrder"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"Order\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"orderNumber\","]
    #[doc = "    \"shipments\","]
    #[doc = "    \"transactionDate\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"billingCustomerKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"externalSalesOrderKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"orderNumber\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"orderReserved\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"referenceNumber\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"refundedOrderReference\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"shipments\": {"]
    #[doc = "      \"title\": \"ShipmentCollection\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.Wholesale.Entities.Shipment\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"tags\": {"]
    #[doc = "      \"title\": \"TagCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.Wholesale.Entities.Tag\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"taxCollected\": {"]
    #[doc = "      \"title\": \"Nullable`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"number\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"transactionDate\": {"]
    #[doc = "      \"title\": \"DateTime\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsWholesaleEntitiesOrder {
        #[serde(
            rename = "billingCustomerKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub billing_customer_key: Option<String>,
        #[serde(
            rename = "externalSalesOrderKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub external_sales_order_key: Option<String>,
        #[serde(rename = "orderNumber")]
        pub order_number: String,
        #[serde(
            rename = "orderReserved",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub order_reserved: Option<String>,
        #[serde(
            rename = "referenceNumber",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reference_number: Option<String>,
        #[serde(
            rename = "refundedOrderReference",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub refunded_order_reference: Option<String>,
        pub shipments: Vec<ShipCompliantRestApiDomainsWholesaleEntitiesShipment>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<ShipCompliantRestApiDomainsWholesaleEntitiesTag>>,
        #[serde(
            rename = "taxCollected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub tax_collected: Option<f64>,
        #[serde(rename = "transactionDate")]
        pub transaction_date: chrono::NaiveDateTime,
    }

    impl From<&ShipCompliantRestApiDomainsWholesaleEntitiesOrder>
        for ShipCompliantRestApiDomainsWholesaleEntitiesOrder
    {
        fn from(value: &ShipCompliantRestApiDomainsWholesaleEntitiesOrder) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsWholesaleEntitiesPostInvoiceInput"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PostInvoiceInput\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"order\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"order\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.Wholesale.Entities.Order\""]
    #[doc = "    },"]
    #[doc = "    \"persistOption\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsWholesaleEntitiesPostInvoiceInput {
        pub order: ShipCompliantRestApiDomainsWholesaleEntitiesOrder,
        #[serde(
            rename = "persistOption",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub persist_option: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsWholesaleEntitiesPostInvoiceInput>
        for ShipCompliantRestApiDomainsWholesaleEntitiesPostInvoiceInput
    {
        fn from(value: &ShipCompliantRestApiDomainsWholesaleEntitiesPostInvoiceInput) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsWholesaleEntitiesShipment"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"Shipment\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"shipDate\","]
    #[doc = "    \"shipmentItems\","]
    #[doc = "    \"shippingCustomerKey\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"defaultWarehouse\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"freight\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"shipDate\": {"]
    #[doc = "      \"title\": \"DateTime\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"format\": \"date-time\""]
    #[doc = "    },"]
    #[doc = "    \"shipmentItems\": {"]
    #[doc = "      \"title\": \"ShipmentItemCollection\","]
    #[doc = "      \"type\": \"array\","]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.Wholesale.Entities.ShipmentItem\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"shipmentKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"shipmentReserved\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"shippingCustomerKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"shippingService\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsWholesaleEntitiesShipment {
        #[serde(
            rename = "defaultWarehouse",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_warehouse: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub freight: Option<f64>,
        #[serde(rename = "shipDate")]
        pub ship_date: chrono::NaiveDateTime,
        #[serde(rename = "shipmentItems")]
        pub shipment_items: Vec<ShipCompliantRestApiDomainsWholesaleEntitiesShipmentItem>,
        #[serde(
            rename = "shipmentKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_key: Option<String>,
        #[serde(
            rename = "shipmentReserved",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipment_reserved: Option<String>,
        #[serde(rename = "shippingCustomerKey")]
        pub shipping_customer_key: String,
        #[serde(
            rename = "shippingService",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shipping_service: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsWholesaleEntitiesShipment>
        for ShipCompliantRestApiDomainsWholesaleEntitiesShipment
    {
        fn from(value: &ShipCompliantRestApiDomainsWholesaleEntitiesShipment) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsWholesaleEntitiesShipmentItem"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentItem\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"required\": ["]
    #[doc = "    \"productKey\","]
    #[doc = "    \"quantity\""]
    #[doc = "  ],"]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"brandKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"fobPointKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"itemReserved\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"productKey\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": \"string\""]
    #[doc = "    },"]
    #[doc = "    \"quantity\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"quantityType\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"unitPrice\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsWholesaleEntitiesShipmentItem {
        #[serde(rename = "brandKey", default, skip_serializing_if = "Option::is_none")]
        pub brand_key: Option<String>,
        #[serde(
            rename = "fobPointKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fob_point_key: Option<String>,
        #[serde(
            rename = "itemReserved",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub item_reserved: Option<String>,
        #[serde(rename = "productKey")]
        pub product_key: String,
        pub quantity: f64,
        #[serde(
            rename = "quantityType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub quantity_type: Option<String>,
        #[serde(rename = "unitPrice", default, skip_serializing_if = "Option::is_none")]
        pub unit_price: Option<f64>,
    }

    impl From<&ShipCompliantRestApiDomainsWholesaleEntitiesShipmentItem>
        for ShipCompliantRestApiDomainsWholesaleEntitiesShipmentItem
    {
        fn from(value: &ShipCompliantRestApiDomainsWholesaleEntitiesShipmentItem) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiDomainsWholesaleEntitiesTag"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"Tag\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"name\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiDomainsWholesaleEntitiesTag {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&ShipCompliantRestApiDomainsWholesaleEntitiesTag>
        for ShipCompliantRestApiDomainsWholesaleEntitiesTag
    {
        fn from(value: &ShipCompliantRestApiDomainsWholesaleEntitiesTag) -> Self {
            value.clone()
        }
    }

    #[doc = "For Quote Salestax we need to suggest an Address with the Error Message"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ErrorData\","]
    #[doc = "  \"description\": \"For Quote Salestax we need to suggest an Address with the Error Message\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"addressSuggestion\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.AddressSuggestion\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiErrorsErrorData {
        #[serde(
            rename = "addressSuggestion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub address_suggestion:
            Option<ShipCompliantRestApiDomainsSalesOrderEntitiesAddressSuggestion>,
    }

    impl From<&ShipCompliantRestApiErrorsErrorData> for ShipCompliantRestApiErrorsErrorData {
        fn from(value: &ShipCompliantRestApiErrorsErrorData) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesAddressBadRequestResponseError"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"AddressBadRequestResponseError\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"message\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 400,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"title\": \"ErrorTarget\","]
    #[doc = "      \"default\": \"Address\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Address\","]
    #[doc = "        \"Age\","]
    #[doc = "        \"Batch\","]
    #[doc = "        \"Brand\","]
    #[doc = "        \"Distributor\","]
    #[doc = "        \"Product\","]
    #[doc = "        \"SalesOrder\","]
    #[doc = "        \"Shipment\","]
    #[doc = "        \"Tax\","]
    #[doc = "        \"SSO\","]
    #[doc = "        \"License\","]
    #[doc = "        \"Tracking\","]
    #[doc = "        \"HoldLocation\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"ErrorType\","]
    #[doc = "      \"default\": \"Validation\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Authentication\","]
    #[doc = "        \"Server\","]
    #[doc = "        \"Validation\","]
    #[doc = "        \"Request\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesAddressBadRequestResponseError {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_address_bad_request_response_error_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_address_bad_request_response_error_target"
        )]
        pub target: ErrorTarget,
        #[serde(
            rename = "type",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_address_bad_request_response_error_type"
        )]
        pub type_: ErrorType,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesAddressBadRequestResponseError>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesAddressBadRequestResponseError
    {
        fn from(
            value : & ShipCompliantRestApiMiddlewaresSwaggerResponseTypesAddressBadRequestResponseError,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"BrandBadRequestResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"errors\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Middlewares.SwaggerResponseTypes.BrandBadRequestResponseError\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Failure\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 400,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub errors: Option<
            Vec<ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponseError>,
        >,
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_brand_bad_request_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_brand_bad_request_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponseError"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"BrandBadRequestResponseError\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"message\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 400,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"title\": \"ErrorTarget\","]
    #[doc = "      \"default\": \"Brand\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Address\","]
    #[doc = "        \"Age\","]
    #[doc = "        \"Batch\","]
    #[doc = "        \"Brand\","]
    #[doc = "        \"Distributor\","]
    #[doc = "        \"Product\","]
    #[doc = "        \"SalesOrder\","]
    #[doc = "        \"Shipment\","]
    #[doc = "        \"Tax\","]
    #[doc = "        \"SSO\","]
    #[doc = "        \"License\","]
    #[doc = "        \"Tracking\","]
    #[doc = "        \"HoldLocation\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"ErrorType\","]
    #[doc = "      \"default\": \"Validation\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Authentication\","]
    #[doc = "        \"Server\","]
    #[doc = "        \"Validation\","]
    #[doc = "        \"Request\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponseError {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_brand_bad_request_response_error_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_brand_bad_request_response_error_target"
        )]
        pub target: ErrorTarget,
        #[serde(
            rename = "type",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_brand_bad_request_response_error_type"
        )]
        pub type_: ErrorType,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponseError>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponseError
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponseError,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"BrandNotFoundResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"errors\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Middlewares.SwaggerResponseTypes.BrandNotFoundResponseError\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Failure\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 404,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub errors: Option<
            Vec<ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseError>,
        >,
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_brand_not_found_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_brand_not_found_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseError"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"BrandNotFoundResponseError\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"message\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 404,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"title\": \"ErrorTarget\","]
    #[doc = "      \"default\": \"Brand\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Address\","]
    #[doc = "        \"Age\","]
    #[doc = "        \"Batch\","]
    #[doc = "        \"Brand\","]
    #[doc = "        \"Distributor\","]
    #[doc = "        \"Product\","]
    #[doc = "        \"SalesOrder\","]
    #[doc = "        \"Shipment\","]
    #[doc = "        \"Tax\","]
    #[doc = "        \"SSO\","]
    #[doc = "        \"License\","]
    #[doc = "        \"Tracking\","]
    #[doc = "        \"HoldLocation\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"ErrorType\","]
    #[doc = "      \"default\": \"Validation\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Authentication\","]
    #[doc = "        \"Server\","]
    #[doc = "        \"Validation\","]
    #[doc = "        \"Request\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseError {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_brand_not_found_response_error_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_brand_not_found_response_error_target"
        )]
        pub target: ErrorTarget,
        #[serde(
            rename = "type",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_brand_not_found_response_error_type"
        )]
        pub type_: ErrorType,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseError>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseError
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseError,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesCheckComplianceResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"CheckComplianceResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesOrder\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Middlewares.SwaggerResponseTypes.SalesOrderComplianceTaxResponse\""]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesCheckComplianceResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_check_compliance_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "salesOrder",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_order: Option<
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderComplianceTaxResponse,
        >,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_check_compliance_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesCheckComplianceResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesCheckComplianceResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesCheckComplianceResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesCommitResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"CommitResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"shipments\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Responses.ShipmentCommitResponse\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesCommitResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_commit_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub shipments:
            Option<Vec<ShipCompliantRestApiDomainsSalesOrderResponsesShipmentCommitResponse>>,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_commit_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesCommitResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesCommitResponse
    {
        fn from(value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesCommitResponse) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteHoldLocationResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"DeleteHoldLocationResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteHoldLocationResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_delete_hold_location_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_delete_hold_location_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteHoldLocationResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteHoldLocationResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteHoldLocationResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteProductResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"DeleteProductResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteProductResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_delete_product_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_delete_product_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteProductResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteProductResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteProductResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteSalesOrderResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"DeleteSalesOrderResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteSalesOrderResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_delete_sales_order_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_delete_sales_order_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteSalesOrderResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteSalesOrderResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteSalesOrderResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteTrackingResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"DeleteTrackingResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteTrackingResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_delete_tracking_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_delete_tracking_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteTrackingResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteTrackingResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteTrackingResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetBrandSuccessResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"GetBrandSuccessResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"brand\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.Brand.Entities.Brand\""]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetBrandSuccessResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub brand: Option<ShipCompliantRestApiDomainsBrandEntitiesBrand>,
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_get_brand_success_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_get_brand_success_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetBrandSuccessResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetBrandSuccessResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetBrandSuccessResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetHoldLocationResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"GetHoldLocationResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"locations\": {"]
    #[doc = "      \"title\": \"HoldLocationDetailCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.HoldLocation.Entities.HoldLocationDetail\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetHoldLocationResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub locations:
            Option<Vec<ShipCompliantRestApiDomainsHoldLocationEntitiesHoldLocationDetail>>,
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_get_hold_location_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_get_hold_location_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetHoldLocationResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetHoldLocationResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetHoldLocationResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetProductSuccessResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"GetProductSuccessResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"product\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.Product.Entities.ProductOutput\""]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetProductSuccessResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub product: Option<ShipCompliantRestApiDomainsProductEntitiesProductOutput>,
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_get_product_success_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_get_product_success_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetProductSuccessResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetProductSuccessResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetProductSuccessResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetSalesOrderSuccessResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"GetSalesOrderSuccessResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"complianceResults\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Middlewares.SwaggerResponseTypes.SalesOrderComplianceResults\""]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesOrder\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.SalesOrderOutput\""]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetSalesOrderSuccessResponse {
        #[serde(
            rename = "complianceResults",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub compliance_results:
            Option<ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderComplianceResults>,
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_get_sales_order_success_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "salesOrder",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_order: Option<ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderOutput>,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_get_sales_order_success_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetSalesOrderSuccessResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetSalesOrderSuccessResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetSalesOrderSuccessResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetTrackingByKeyResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"GetTrackingByKeyResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"trackings\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.Tracking.Entities.TrackingOutput\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetTrackingByKeyResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_get_tracking_by_key_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_get_tracking_by_key_response_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub trackings: Option<Vec<ShipCompliantRestApiDomainsTrackingEntitiesTrackingOutput>>,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetTrackingByKeyResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetTrackingByKeyResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetTrackingByKeyResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"HoldLocationBadRequestResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"errors\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Middlewares.SwaggerResponseTypes.HoldLocationBadRequestResponseError\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Failure\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 400,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponse { # [serde (default , skip_serializing_if = "Option::is_none")] pub errors : Option < Vec < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseError > > , # [serde (rename = "responseStatus" , default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_bad_request_response_response_status")] pub response_status : ResponseStatus , # [serde (rename = "statusCode" , default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_bad_request_response_status_code")] pub status_code : StatusCode , }
    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponse
    {
        fn from(
            value : & ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseError"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"HoldLocationBadRequestResponseError\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"message\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 400,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"title\": \"ErrorTarget\","]
    #[doc = "      \"default\": \"HoldLocation\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Address\","]
    #[doc = "        \"Age\","]
    #[doc = "        \"Batch\","]
    #[doc = "        \"Brand\","]
    #[doc = "        \"Distributor\","]
    #[doc = "        \"Product\","]
    #[doc = "        \"SalesOrder\","]
    #[doc = "        \"Shipment\","]
    #[doc = "        \"Tax\","]
    #[doc = "        \"SSO\","]
    #[doc = "        \"License\","]
    #[doc = "        \"Tracking\","]
    #[doc = "        \"HoldLocation\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"ErrorType\","]
    #[doc = "      \"default\": \"Validation\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Authentication\","]
    #[doc = "        \"Server\","]
    #[doc = "        \"Validation\","]
    #[doc = "        \"Request\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseError {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_bad_request_response_error_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_bad_request_response_error_target"
        )]
        pub target: ErrorTarget,
        #[serde(
            rename = "type",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_bad_request_response_error_type"
        )]
        pub type_: ErrorType,
    }

    impl
        From<
            &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseError,
        >
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseError
    {
        fn from(
            value : & ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseError,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"HoldLocationNotFoundResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"errors\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Middlewares.SwaggerResponseTypes.HoldLocationNotFoundResponseError\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Failure\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 404,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponse { # [serde (default , skip_serializing_if = "Option::is_none")] pub errors : Option < Vec < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponseError > > , # [serde (rename = "responseStatus" , default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_not_found_response_response_status")] pub response_status : ResponseStatus , # [serde (rename = "statusCode" , default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_not_found_response_status_code")] pub status_code : StatusCode , }
    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponseError"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"HoldLocationNotFoundResponseError\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"message\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 404,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"title\": \"ErrorTarget\","]
    #[doc = "      \"default\": \"HoldLocation\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Address\","]
    #[doc = "        \"Age\","]
    #[doc = "        \"Batch\","]
    #[doc = "        \"Brand\","]
    #[doc = "        \"Distributor\","]
    #[doc = "        \"Product\","]
    #[doc = "        \"SalesOrder\","]
    #[doc = "        \"Shipment\","]
    #[doc = "        \"Tax\","]
    #[doc = "        \"SSO\","]
    #[doc = "        \"License\","]
    #[doc = "        \"Tracking\","]
    #[doc = "        \"HoldLocation\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"ErrorType\","]
    #[doc = "      \"default\": \"Validation\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Authentication\","]
    #[doc = "        \"Server\","]
    #[doc = "        \"Validation\","]
    #[doc = "        \"Request\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponseError {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_not_found_response_error_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_not_found_response_error_target"
        )]
        pub target: ErrorTarget,
        #[serde(
            rename = "type",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_not_found_response_error_type"
        )]
        pub type_: ErrorType,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponseError>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponseError
    {
        fn from(
            value : & ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponseError,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostBrandResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PostBrandResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostBrandResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_post_brand_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_post_brand_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostBrandResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostBrandResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostBrandResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostHoldLocationResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PostHoldLocationResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostHoldLocationResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_post_hold_location_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_post_hold_location_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostHoldLocationResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostHoldLocationResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostHoldLocationResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostProductResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PostProductResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostProductResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_post_product_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_post_product_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostProductResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostProductResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostProductResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostSalesOrderResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PostSalesOrderResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostSalesOrderResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_post_sales_order_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_post_sales_order_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostSalesOrderResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostSalesOrderResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostSalesOrderResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostTrackingResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PostTrackingResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostTrackingResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_post_tracking_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_post_tracking_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostTrackingResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostTrackingResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostTrackingResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostWholesaleInvoiceResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PostWholesaleInvoiceResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostWholesaleInvoiceResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_post_wholesale_invoice_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_post_wholesale_invoice_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostWholesaleInvoiceResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostWholesaleInvoiceResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostWholesaleInvoiceResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ProductBadRequestResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"errors\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Middlewares.SwaggerResponseTypes.ProductBadRequestResponseError\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Failure\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 400,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub errors: Option<
            Vec<ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseError>,
        >,
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_product_bad_request_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_product_bad_request_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseError"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ProductBadRequestResponseError\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"message\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 400,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"title\": \"ErrorTarget\","]
    #[doc = "      \"default\": \"Product\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Address\","]
    #[doc = "        \"Age\","]
    #[doc = "        \"Batch\","]
    #[doc = "        \"Brand\","]
    #[doc = "        \"Distributor\","]
    #[doc = "        \"Product\","]
    #[doc = "        \"SalesOrder\","]
    #[doc = "        \"Shipment\","]
    #[doc = "        \"Tax\","]
    #[doc = "        \"SSO\","]
    #[doc = "        \"License\","]
    #[doc = "        \"Tracking\","]
    #[doc = "        \"HoldLocation\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"ErrorType\","]
    #[doc = "      \"default\": \"Validation\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Authentication\","]
    #[doc = "        \"Server\","]
    #[doc = "        \"Validation\","]
    #[doc = "        \"Request\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseError {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_product_bad_request_response_error_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_product_bad_request_response_error_target"
        )]
        pub target: ErrorTarget,
        #[serde(
            rename = "type",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_product_bad_request_response_error_type"
        )]
        pub type_: ErrorType,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseError>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseError
    {
        fn from(
            value : & ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseError,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ProductNotFoundResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"errors\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Middlewares.SwaggerResponseTypes.ProductNotFoundResponseError\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Failure\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 404,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub errors: Option<
            Vec<ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponseError>,
        >,
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_product_not_found_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_product_not_found_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponseError"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ProductNotFoundResponseError\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"message\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 404,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"title\": \"ErrorTarget\","]
    #[doc = "      \"default\": \"Product\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Address\","]
    #[doc = "        \"Age\","]
    #[doc = "        \"Batch\","]
    #[doc = "        \"Brand\","]
    #[doc = "        \"Distributor\","]
    #[doc = "        \"Product\","]
    #[doc = "        \"SalesOrder\","]
    #[doc = "        \"Shipment\","]
    #[doc = "        \"Tax\","]
    #[doc = "        \"SSO\","]
    #[doc = "        \"License\","]
    #[doc = "        \"Tracking\","]
    #[doc = "        \"HoldLocation\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"ErrorType\","]
    #[doc = "      \"default\": \"Validation\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Authentication\","]
    #[doc = "        \"Server\","]
    #[doc = "        \"Validation\","]
    #[doc = "        \"Request\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponseError {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_product_not_found_response_error_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_product_not_found_response_error_target"
        )]
        pub target: ErrorTarget,
        #[serde(
            rename = "type",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_product_not_found_response_error_type"
        )]
        pub type_: ErrorType,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponseError>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponseError
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponseError,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPutBrandResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PutBrandResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPutBrandResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_put_brand_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_put_brand_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPutBrandResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPutBrandResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPutBrandResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPutProductResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"PutProductResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPutProductResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_put_product_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_put_product_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPutProductResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPutProductResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPutProductResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesQuoteComplianceResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"QuoteComplianceResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"complianceResults\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Middlewares.SwaggerResponseTypes.SalesOrderComplianceResults\""]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesQuoteComplianceResponse {
        #[serde(
            rename = "complianceResults",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub compliance_results:
            Option<ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderComplianceResults>,
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_quote_compliance_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_quote_compliance_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesQuoteComplianceResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesQuoteComplianceResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesQuoteComplianceResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesQuoteTaxSalesOrderResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"QuoteTaxSalesOrderResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"processedAddress\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.TaxRateAddressOutput\""]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesTaxDue\": {"]
    #[doc = "      \"title\": \"Double\","]
    #[doc = "      \"type\": \"number\","]
    #[doc = "      \"format\": \"double\""]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesQuoteTaxSalesOrderResponse {
        #[serde(
            rename = "processedAddress",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub processed_address:
            Option<ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateAddressOutput>,
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_quote_tax_sales_order_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "salesTaxDue",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_due: Option<f64>,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_quote_tax_sales_order_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesQuoteTaxSalesOrderResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesQuoteTaxSalesOrderResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesQuoteTaxSalesOrderResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "SalesOrderAddressValidationResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderAddressValidationResponse\","]
    #[doc = "  \"description\": \"SalesOrderAddressValidationResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"errors\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Middlewares.SwaggerResponseTypes.SalesOrderAddressValidationResponseError\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Failure\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 400,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponse { # [serde (default , skip_serializing_if = "Option::is_none")] pub errors : Option < Vec < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseError > > , # [serde (rename = "responseStatus" , default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_address_validation_response_response_status")] pub response_status : ResponseStatus , # [serde (rename = "statusCode" , default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_address_validation_response_status_code")] pub status_code : StatusCode , }
    impl
        From<
            &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponse,
        >
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponse
    {
        fn from(
            value : & ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseError"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderAddressValidationResponseError\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"data\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Errors.ErrorData\""]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"message\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 400,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"title\": \"ErrorTarget\","]
    #[doc = "      \"default\": \"SalesOrder\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Address\","]
    #[doc = "        \"Age\","]
    #[doc = "        \"Batch\","]
    #[doc = "        \"Brand\","]
    #[doc = "        \"Distributor\","]
    #[doc = "        \"Product\","]
    #[doc = "        \"SalesOrder\","]
    #[doc = "        \"Shipment\","]
    #[doc = "        \"Tax\","]
    #[doc = "        \"SSO\","]
    #[doc = "        \"License\","]
    #[doc = "        \"Tracking\","]
    #[doc = "        \"HoldLocation\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"ErrorType\","]
    #[doc = "      \"default\": \"Validation\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Authentication\","]
    #[doc = "        \"Server\","]
    #[doc = "        \"Validation\","]
    #[doc = "        \"Request\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": true"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseError
    {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data: Option<ShipCompliantRestApiErrorsErrorData>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_address_validation_response_error_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_address_validation_response_error_target"
        )]
        pub target: ErrorTarget,
        #[serde(
            rename = "type",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_address_validation_response_error_type"
        )]
        pub type_: ErrorType,
    }

    impl From < & ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseError > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseError { fn from (value : & ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseError) -> Self { value . clone () } }
    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderBadRequestResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"errors\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Middlewares.SwaggerResponseTypes.SalesOrderBadRequestResponseError\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Failure\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 400,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponse { # [serde (default , skip_serializing_if = "Option::is_none")] pub errors : Option < Vec < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponseError > > , # [serde (rename = "responseStatus" , default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_bad_request_response_response_status")] pub response_status : ResponseStatus , # [serde (rename = "statusCode" , default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_bad_request_response_status_code")] pub status_code : StatusCode , }
    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponseError"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderBadRequestResponseError\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"message\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 400,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"title\": \"ErrorTarget\","]
    #[doc = "      \"default\": \"SalesOrder\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Address\","]
    #[doc = "        \"Age\","]
    #[doc = "        \"Batch\","]
    #[doc = "        \"Brand\","]
    #[doc = "        \"Distributor\","]
    #[doc = "        \"Product\","]
    #[doc = "        \"SalesOrder\","]
    #[doc = "        \"Shipment\","]
    #[doc = "        \"Tax\","]
    #[doc = "        \"SSO\","]
    #[doc = "        \"License\","]
    #[doc = "        \"Tracking\","]
    #[doc = "        \"HoldLocation\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"ErrorType\","]
    #[doc = "      \"default\": \"Validation\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Authentication\","]
    #[doc = "        \"Server\","]
    #[doc = "        \"Validation\","]
    #[doc = "        \"Request\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponseError {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_bad_request_response_error_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_bad_request_response_error_target"
        )]
        pub target: ErrorTarget,
        #[serde(
            rename = "type",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_bad_request_response_error_type"
        )]
        pub type_: ErrorType,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponseError>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponseError
    {
        fn from(
            value : & ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponseError,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderComplianceResults"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderComplianceResults\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"complianceStatus\": {"]
    #[doc = "      \"title\": \"RuleComplianceStatus\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Compliant\","]
    #[doc = "        \"NotCompliant\","]
    #[doc = "        \"Bypassed\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"shipments\": {"]
    #[doc = "      \"title\": \"ShipmentComplianceResponseCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Responses.ShipmentComplianceResponse\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderComplianceResults {
        #[serde(
            rename = "complianceStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub compliance_status: Option<RuleComplianceStatus>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub shipments:
            Option<Vec<ShipCompliantRestApiDomainsSalesOrderResponsesShipmentComplianceResponse>>,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderComplianceResults>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderComplianceResults
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderComplianceResults,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderComplianceTaxResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderComplianceTaxResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"complianceStatus\": {"]
    #[doc = "      \"title\": \"RuleComplianceStatus\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Compliant\","]
    #[doc = "        \"NotCompliant\","]
    #[doc = "        \"Bypassed\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"salesTaxRates\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.SalesTaxRates\""]
    #[doc = "    },"]
    #[doc = "    \"shipments\": {"]
    #[doc = "      \"title\": \"ShipmentComplianceResponseCollection\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Responses.ShipmentComplianceResponse\""]
    #[doc = "      }"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderComplianceTaxResponse {
        #[serde(
            rename = "complianceStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub compliance_status: Option<RuleComplianceStatus>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(
            rename = "salesTaxRates",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sales_tax_rates: Option<ShipCompliantRestApiDomainsSalesOrderEntitiesSalesTaxRates>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub shipments:
            Option<Vec<ShipCompliantRestApiDomainsSalesOrderResponsesShipmentComplianceResponse>>,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderComplianceTaxResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderComplianceTaxResponse
    {
        fn from(
            value : & ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderComplianceTaxResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderNotFoundResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"errors\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Middlewares.SwaggerResponseTypes.SalesOrderNotFoundResponseError\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Failure\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 404,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub errors: Option<
            Vec<ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseError>,
        >,
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_not_found_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_not_found_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseError"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"SalesOrderNotFoundResponseError\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"message\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 404,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"title\": \"ErrorTarget\","]
    #[doc = "      \"default\": \"SalesOrder\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Address\","]
    #[doc = "        \"Age\","]
    #[doc = "        \"Batch\","]
    #[doc = "        \"Brand\","]
    #[doc = "        \"Distributor\","]
    #[doc = "        \"Product\","]
    #[doc = "        \"SalesOrder\","]
    #[doc = "        \"Shipment\","]
    #[doc = "        \"Tax\","]
    #[doc = "        \"SSO\","]
    #[doc = "        \"License\","]
    #[doc = "        \"Tracking\","]
    #[doc = "        \"HoldLocation\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"ErrorType\","]
    #[doc = "      \"default\": \"Validation\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Authentication\","]
    #[doc = "        \"Server\","]
    #[doc = "        \"Validation\","]
    #[doc = "        \"Request\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseError {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_not_found_response_error_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_not_found_response_error_target"
        )]
        pub target: ErrorTarget,
        #[serde(
            rename = "type",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_not_found_response_error_type"
        )]
        pub type_: ErrorType,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseError>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseError
    {
        fn from(
            value : & ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseError,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "TaxRateSalesOrderByAddressResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TaxRateSalesOrderByAddressResponse\","]
    #[doc = "  \"description\": \"TaxRateSalesOrderByAddressResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"processedAddress\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.TaxRateAddressOutput\""]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"taxRates\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.SalesOrder.Entities.TaxRateOutput\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTaxRateSalesOrderByAddressResponse {
        #[serde(
            rename = "processedAddress",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub processed_address:
            Option<ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateAddressOutput>,
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_tax_rate_sales_order_by_address_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_tax_rate_sales_order_by_address_response_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(rename = "taxRates", default, skip_serializing_if = "Option::is_none")]
        pub tax_rates: Option<ShipCompliantRestApiDomainsSalesOrderEntitiesTaxRateOutput>,
    }

    impl
        From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTaxRateSalesOrderByAddressResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTaxRateSalesOrderByAddressResponse
    {
        fn from(
            value : & ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTaxRateSalesOrderByAddressResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TrackingBadRequestResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"errors\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Middlewares.SwaggerResponseTypes.TrackingBadRequestResponseError\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Failure\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 400,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub errors: Option<
            Vec<ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponseError>,
        >,
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_tracking_bad_request_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_tracking_bad_request_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponseError"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TrackingBadRequestResponseError\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"message\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 400,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"title\": \"ErrorTarget\","]
    #[doc = "      \"default\": \"Tracking\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Address\","]
    #[doc = "        \"Age\","]
    #[doc = "        \"Batch\","]
    #[doc = "        \"Brand\","]
    #[doc = "        \"Distributor\","]
    #[doc = "        \"Product\","]
    #[doc = "        \"SalesOrder\","]
    #[doc = "        \"Shipment\","]
    #[doc = "        \"Tax\","]
    #[doc = "        \"SSO\","]
    #[doc = "        \"License\","]
    #[doc = "        \"Tracking\","]
    #[doc = "        \"HoldLocation\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"ErrorType\","]
    #[doc = "      \"default\": \"Validation\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Authentication\","]
    #[doc = "        \"Server\","]
    #[doc = "        \"Validation\","]
    #[doc = "        \"Request\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponseError {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_tracking_bad_request_response_error_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_tracking_bad_request_response_error_target"
        )]
        pub target: ErrorTarget,
        #[serde(
            rename = "type",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_tracking_bad_request_response_error_type"
        )]
        pub type_: ErrorType,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponseError>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponseError
    {
        fn from(
            value : & ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponseError,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TrackingNotFoundResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"errors\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Middlewares.SwaggerResponseTypes.TrackingNotFoundResponseError\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Failure\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 404,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub errors: Option<
            Vec<ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseError>,
        >,
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_tracking_not_found_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_tracking_not_found_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseError"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TrackingNotFoundResponseError\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"code\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"key\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"message\": {"]
    #[doc = "      \"title\": \"String\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"string\","]
    #[doc = "        \"null\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 404,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"target\": {"]
    #[doc = "      \"title\": \"ErrorTarget\","]
    #[doc = "      \"default\": \"Tracking\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Null\","]
    #[doc = "        \"Address\","]
    #[doc = "        \"Age\","]
    #[doc = "        \"Batch\","]
    #[doc = "        \"Brand\","]
    #[doc = "        \"Distributor\","]
    #[doc = "        \"Product\","]
    #[doc = "        \"SalesOrder\","]
    #[doc = "        \"Shipment\","]
    #[doc = "        \"Tax\","]
    #[doc = "        \"SSO\","]
    #[doc = "        \"License\","]
    #[doc = "        \"Tracking\","]
    #[doc = "        \"HoldLocation\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"type\": {"]
    #[doc = "      \"title\": \"ErrorType\","]
    #[doc = "      \"default\": \"Validation\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Authentication\","]
    #[doc = "        \"Server\","]
    #[doc = "        \"Validation\","]
    #[doc = "        \"Request\""]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseError {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_tracking_not_found_response_error_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_tracking_not_found_response_error_target"
        )]
        pub target: ErrorTarget,
        #[serde(
            rename = "type",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_tracking_not_found_response_error_type"
        )]
        pub type_: ErrorType,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseError>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseError
    {
        fn from(
            value : & ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseError,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressBadRequestResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ValidateAddressBadRequestResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"errors\": {"]
    #[doc = "      \"title\": \"List`1\","]
    #[doc = "      \"type\": ["]
    #[doc = "        \"array\","]
    #[doc = "        \"null\""]
    #[doc = "      ],"]
    #[doc = "      \"items\": {"]
    #[doc = "        \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Middlewares.SwaggerResponseTypes.AddressBadRequestResponseError\""]
    #[doc = "      }"]
    #[doc = "    },"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Failure\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 400,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressBadRequestResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub errors: Option<
            Vec<ShipCompliantRestApiMiddlewaresSwaggerResponseTypesAddressBadRequestResponseError>,
        >,
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_validate_address_bad_request_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_validate_address_bad_request_response_status_code"
        )]
        pub status_code: StatusCode,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressBadRequestResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressBadRequestResponse
    {
        fn from(
            value : & ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressBadRequestResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressResponse"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ValidateAddressResponse\","]
    #[doc = "  \"type\": \"object\","]
    #[doc = "  \"properties\": {"]
    #[doc = "    \"responseStatus\": {"]
    #[doc = "      \"title\": \"ResponseStatus\","]
    #[doc = "      \"default\": \"Success\","]
    #[doc = "      \"type\": \"string\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        \"Failure\","]
    #[doc = "        \"Success\""]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"statusCode\": {"]
    #[doc = "      \"title\": \"StatusCode\","]
    #[doc = "      \"default\": 200,"]
    #[doc = "      \"type\": \"integer\","]
    #[doc = "      \"enum\": ["]
    #[doc = "        200,"]
    #[doc = "        400,"]
    #[doc = "        401,"]
    #[doc = "        403,"]
    #[doc = "        404,"]
    #[doc = "        429,"]
    #[doc = "        500"]
    #[doc = "      ]"]
    #[doc = "    },"]
    #[doc = "    \"validateAddressResult\": {"]
    #[doc = "      \"$ref\": \"#/components/schemas/ShipCompliantRestAPI.Domains.Addresses.Entities.ValidateAddressResult\""]
    #[doc = "    }"]
    #[doc = "  },"]
    #[doc = "  \"additionalProperties\": false"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressResponse {
        #[serde(
            rename = "responseStatus",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_validate_address_response_response_status"
        )]
        pub response_status: ResponseStatus,
        #[serde(
            rename = "statusCode",
            default = "defaults::ship_compliant_rest_api_middlewares_swagger_response_types_validate_address_response_status_code"
        )]
        pub status_code: StatusCode,
        #[serde(
            rename = "validateAddressResult",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub validate_address_result:
            Option<ShipCompliantRestApiDomainsAddressesEntitiesValidateAddressResult>,
    }

    impl From<&ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressResponse
    {
        fn from(
            value: &ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressResponse,
        ) -> Self {
            value.clone()
        }
    }

    #[doc = "ShipmentStatusTypes"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"ShipmentStatusTypes\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Delivered\","]
    #[doc = "    \"InProcess\","]
    #[doc = "    \"Shipped\","]
    #[doc = "    \"Voided\","]
    #[doc = "    \"SentToFulfillment\","]
    #[doc = "    \"PaymentAccepted\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ShipmentStatusTypes {
        Delivered,
        InProcess,
        Shipped,
        Voided,
        SentToFulfillment,
        PaymentAccepted,
    }

    impl From<&ShipmentStatusTypes> for ShipmentStatusTypes {
        fn from(value: &ShipmentStatusTypes) -> Self {
            value.clone()
        }
    }

    impl ToString for ShipmentStatusTypes {
        fn to_string(&self) -> String {
            match *self {
                Self::Delivered => "Delivered".to_string(),
                Self::InProcess => "InProcess".to_string(),
                Self::Shipped => "Shipped".to_string(),
                Self::Voided => "Voided".to_string(),
                Self::SentToFulfillment => "SentToFulfillment".to_string(),
                Self::PaymentAccepted => "PaymentAccepted".to_string(),
            }
        }
    }

    impl std::str::FromStr for ShipmentStatusTypes {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Delivered" => Ok(Self::Delivered),
                "InProcess" => Ok(Self::InProcess),
                "Shipped" => Ok(Self::Shipped),
                "Voided" => Ok(Self::Voided),
                "SentToFulfillment" => Ok(Self::SentToFulfillment),
                "PaymentAccepted" => Ok(Self::PaymentAccepted),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ShipmentStatusTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ShipmentStatusTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ShipmentStatusTypes {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "StatusCode"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"StatusCode\","]
    #[doc = "  \"default\": 200,"]
    #[doc = "  \"type\": \"integer\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    200,"]
    #[doc = "    400,"]
    #[doc = "    401,"]
    #[doc = "    403,"]
    #[doc = "    404,"]
    #[doc = "    429,"]
    #[doc = "    500"]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(:: serde :: Serialize, Clone, Debug)]
    pub struct StatusCode(i64);
    impl ::std::ops::Deref for StatusCode {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }

    impl From<StatusCode> for i64 {
        fn from(value: StatusCode) -> Self {
            value.0
        }
    }

    impl From<&StatusCode> for StatusCode {
        fn from(value: &StatusCode) -> Self {
            value.clone()
        }
    }

    impl Default for StatusCode {
        fn default() -> Self {
            StatusCode(200_i64)
        }
    }

    impl std::convert::TryFrom<i64> for StatusCode {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
            if ![
                200_i64, 400_i64, 401_i64, 403_i64, 404_i64, 429_i64, 500_i64,
            ]
            .contains(&value)
            {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for StatusCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }

    #[doc = "TaxSaleType"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TaxSaleType\","]
    #[doc = "  \"description\": \"TaxSaleType\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Null\","]
    #[doc = "    \"Offsite\","]
    #[doc = "    \"Onsite\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum TaxSaleType {
        Null,
        Offsite,
        Onsite,
    }

    impl From<&TaxSaleType> for TaxSaleType {
        fn from(value: &TaxSaleType) -> Self {
            value.clone()
        }
    }

    impl ToString for TaxSaleType {
        fn to_string(&self) -> String {
            match *self {
                Self::Null => "Null".to_string(),
                Self::Offsite => "Offsite".to_string(),
                Self::Onsite => "Onsite".to_string(),
            }
        }
    }

    impl std::str::FromStr for TaxSaleType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Null" => Ok(Self::Null),
                "Offsite" => Ok(Self::Offsite),
                "Onsite" => Ok(Self::Onsite),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for TaxSaleType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for TaxSaleType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for TaxSaleType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "TimeFrameType"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TimeFrameType\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Calendar\","]
    #[doc = "    \"Rolling\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum TimeFrameType {
        Calendar,
        Rolling,
    }

    impl From<&TimeFrameType> for TimeFrameType {
        fn from(value: &TimeFrameType) -> Self {
            value.clone()
        }
    }

    impl ToString for TimeFrameType {
        fn to_string(&self) -> String {
            match *self {
                Self::Calendar => "Calendar".to_string(),
                Self::Rolling => "Rolling".to_string(),
            }
        }
    }

    impl std::str::FromStr for TimeFrameType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Calendar" => Ok(Self::Calendar),
                "Rolling" => Ok(Self::Rolling),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for TimeFrameType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for TimeFrameType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for TimeFrameType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "TimeFrameUnit"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"TimeFrameUnit\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Null\","]
    #[doc = "    \"Minutes\","]
    #[doc = "    \"Hours\","]
    #[doc = "    \"Days\","]
    #[doc = "    \"Weeks\","]
    #[doc = "    \"Months\","]
    #[doc = "    \"Years\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum TimeFrameUnit {
        Null,
        Minutes,
        Hours,
        Days,
        Weeks,
        Months,
        Years,
    }

    impl From<&TimeFrameUnit> for TimeFrameUnit {
        fn from(value: &TimeFrameUnit) -> Self {
            value.clone()
        }
    }

    impl ToString for TimeFrameUnit {
        fn to_string(&self) -> String {
            match *self {
                Self::Null => "Null".to_string(),
                Self::Minutes => "Minutes".to_string(),
                Self::Hours => "Hours".to_string(),
                Self::Days => "Days".to_string(),
                Self::Weeks => "Weeks".to_string(),
                Self::Months => "Months".to_string(),
                Self::Years => "Years".to_string(),
            }
        }
    }

    impl std::str::FromStr for TimeFrameUnit {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Null" => Ok(Self::Null),
                "Minutes" => Ok(Self::Minutes),
                "Hours" => Ok(Self::Hours),
                "Days" => Ok(Self::Days),
                "Weeks" => Ok(Self::Weeks),
                "Months" => Ok(Self::Months),
                "Years" => Ok(Self::Years),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for TimeFrameUnit {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for TimeFrameUnit {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for TimeFrameUnit {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = "VolumeUnit"]
    #[doc = r""]
    #[doc = r" <details><summary>JSON schema</summary>"]
    #[doc = r""]
    #[doc = r" ```json"]
    #[doc = "{"]
    #[doc = "  \"title\": \"VolumeUnit\","]
    #[doc = "  \"type\": \"string\","]
    #[doc = "  \"enum\": ["]
    #[doc = "    \"Null\","]
    #[doc = "    \"Bottle\","]
    #[doc = "    \"Case\","]
    #[doc = "    \"Gallon\","]
    #[doc = "    \"Liter\","]
    #[doc = "    \"Milliliter\","]
    #[doc = "    \"Ounce\","]
    #[doc = "    \"Quart\","]
    #[doc = "    \"Barrel\""]
    #[doc = "  ]"]
    #[doc = "}"]
    #[doc = r" ```"]
    #[doc = r" </details>"]
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum VolumeUnit {
        Null,
        Bottle,
        Case,
        Gallon,
        Liter,
        Milliliter,
        Ounce,
        Quart,
        Barrel,
    }

    impl From<&VolumeUnit> for VolumeUnit {
        fn from(value: &VolumeUnit) -> Self {
            value.clone()
        }
    }

    impl ToString for VolumeUnit {
        fn to_string(&self) -> String {
            match *self {
                Self::Null => "Null".to_string(),
                Self::Bottle => "Bottle".to_string(),
                Self::Case => "Case".to_string(),
                Self::Gallon => "Gallon".to_string(),
                Self::Liter => "Liter".to_string(),
                Self::Milliliter => "Milliliter".to_string(),
                Self::Ounce => "Ounce".to_string(),
                Self::Quart => "Quart".to_string(),
                Self::Barrel => "Barrel".to_string(),
            }
        }
    }

    impl std::str::FromStr for VolumeUnit {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Null" => Ok(Self::Null),
                "Bottle" => Ok(Self::Bottle),
                "Case" => Ok(Self::Case),
                "Gallon" => Ok(Self::Gallon),
                "Liter" => Ok(Self::Liter),
                "Milliliter" => Ok(Self::Milliliter),
                "Ounce" => Ok(Self::Ounce),
                "Quart" => Ok(Self::Quart),
                "Barrel" => Ok(Self::Barrel),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for VolumeUnit {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for VolumeUnit {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for VolumeUnit {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    #[doc = r" Generation of default values for serde."]
    pub mod defaults {
        pub(super) fn default_u64<T, const V: u64>() -> T
        where
            T: std::convert::TryFrom<u64>,
            <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
        {
            T::try_from(V).unwrap()
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_address_bad_request_response_error_status_code(
        ) -> super::StatusCode {
            super::StatusCode(400_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_address_bad_request_response_error_target(
        ) -> super::ErrorTarget {
            super::ErrorTarget::Address
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_address_bad_request_response_error_type(
        ) -> super::ErrorType {
            super::ErrorType::Validation
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_brand_bad_request_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Failure
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_brand_bad_request_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(400_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_brand_bad_request_response_error_status_code(
        ) -> super::StatusCode {
            super::StatusCode(400_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_brand_bad_request_response_error_target(
        ) -> super::ErrorTarget {
            super::ErrorTarget::Brand
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_brand_bad_request_response_error_type(
        ) -> super::ErrorType {
            super::ErrorType::Validation
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_brand_not_found_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Failure
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_brand_not_found_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(404_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_brand_not_found_response_error_status_code(
        ) -> super::StatusCode {
            super::StatusCode(404_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_brand_not_found_response_error_target(
        ) -> super::ErrorTarget {
            super::ErrorTarget::Brand
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_brand_not_found_response_error_type(
        ) -> super::ErrorType {
            super::ErrorType::Validation
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_check_compliance_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_check_compliance_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_commit_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_commit_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_delete_hold_location_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_delete_hold_location_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_delete_product_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_delete_product_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_delete_sales_order_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_delete_sales_order_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_delete_tracking_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_delete_tracking_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_get_brand_success_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_get_brand_success_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_get_hold_location_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_get_hold_location_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_get_product_success_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_get_product_success_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_get_sales_order_success_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_get_sales_order_success_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_get_tracking_by_key_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_get_tracking_by_key_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_bad_request_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Failure
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_bad_request_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(400_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_bad_request_response_error_status_code(
        ) -> super::StatusCode {
            super::StatusCode(400_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_bad_request_response_error_target(
        ) -> super::ErrorTarget {
            super::ErrorTarget::HoldLocation
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_bad_request_response_error_type(
        ) -> super::ErrorType {
            super::ErrorType::Validation
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_not_found_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Failure
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_not_found_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(404_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_not_found_response_error_status_code(
        ) -> super::StatusCode {
            super::StatusCode(404_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_not_found_response_error_target(
        ) -> super::ErrorTarget {
            super::ErrorTarget::HoldLocation
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_hold_location_not_found_response_error_type(
        ) -> super::ErrorType {
            super::ErrorType::Validation
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_post_brand_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_post_brand_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_post_hold_location_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_post_hold_location_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_post_product_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_post_product_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_post_sales_order_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_post_sales_order_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_post_tracking_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_post_tracking_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_post_wholesale_invoice_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_post_wholesale_invoice_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_product_bad_request_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Failure
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_product_bad_request_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(400_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_product_bad_request_response_error_status_code(
        ) -> super::StatusCode {
            super::StatusCode(400_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_product_bad_request_response_error_target(
        ) -> super::ErrorTarget {
            super::ErrorTarget::Product
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_product_bad_request_response_error_type(
        ) -> super::ErrorType {
            super::ErrorType::Validation
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_product_not_found_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Failure
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_product_not_found_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(404_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_product_not_found_response_error_status_code(
        ) -> super::StatusCode {
            super::StatusCode(404_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_product_not_found_response_error_target(
        ) -> super::ErrorTarget {
            super::ErrorTarget::Product
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_product_not_found_response_error_type(
        ) -> super::ErrorType {
            super::ErrorType::Validation
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_put_brand_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_put_brand_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_put_product_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_put_product_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_quote_compliance_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_quote_compliance_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_quote_tax_sales_order_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_quote_tax_sales_order_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_address_validation_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Failure
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_address_validation_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(400_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_address_validation_response_error_status_code(
        ) -> super::StatusCode {
            super::StatusCode(400_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_address_validation_response_error_target(
        ) -> super::ErrorTarget {
            super::ErrorTarget::SalesOrder
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_address_validation_response_error_type(
        ) -> super::ErrorType {
            super::ErrorType::Validation
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_bad_request_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Failure
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_bad_request_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(400_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_bad_request_response_error_status_code(
        ) -> super::StatusCode {
            super::StatusCode(400_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_bad_request_response_error_target(
        ) -> super::ErrorTarget {
            super::ErrorTarget::SalesOrder
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_bad_request_response_error_type(
        ) -> super::ErrorType {
            super::ErrorType::Validation
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_not_found_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Failure
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_not_found_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(404_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_not_found_response_error_status_code(
        ) -> super::StatusCode {
            super::StatusCode(404_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_not_found_response_error_target(
        ) -> super::ErrorTarget {
            super::ErrorTarget::SalesOrder
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_sales_order_not_found_response_error_type(
        ) -> super::ErrorType {
            super::ErrorType::Validation
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_tax_rate_sales_order_by_address_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_tax_rate_sales_order_by_address_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_tracking_bad_request_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Failure
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_tracking_bad_request_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(400_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_tracking_bad_request_response_error_status_code(
        ) -> super::StatusCode {
            super::StatusCode(400_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_tracking_bad_request_response_error_target(
        ) -> super::ErrorTarget {
            super::ErrorTarget::Tracking
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_tracking_bad_request_response_error_type(
        ) -> super::ErrorType {
            super::ErrorType::Validation
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_tracking_not_found_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Failure
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_tracking_not_found_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(404_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_tracking_not_found_response_error_status_code(
        ) -> super::StatusCode {
            super::StatusCode(404_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_tracking_not_found_response_error_target(
        ) -> super::ErrorTarget {
            super::ErrorTarget::Tracking
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_tracking_not_found_response_error_type(
        ) -> super::ErrorType {
            super::ErrorType::Validation
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_validate_address_bad_request_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Failure
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_validate_address_bad_request_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(400_i64)
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_validate_address_response_response_status(
        ) -> super::ResponseStatus {
            super::ResponseStatus::Success
        }

        pub(super) fn ship_compliant_rest_api_middlewares_swagger_response_types_validate_address_response_status_code(
        ) -> super::StatusCode {
            super::StatusCode(200_i64)
        }
    }

    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseOrNone {
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponse,
        ),
        None,
    }

    impl From<ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseOrNone
    {
        fn from(
            value: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponse,
        ) -> Self {
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponse (value)
        }
    }

    impl From<()> for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseOrNone {
        fn from(_: ()) -> Self {
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseOrNone::None
        }
    }

    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponseOrNone
    {
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponse,
        ),
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponse,
        ),
        None,
    }

    impl From < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponse > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponseOrNone { fn from (value : ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponse) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponse (value) } }
    impl From < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponse > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponseOrNone { fn from (value : ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponse) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponse (value) } }
    impl From < () > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponseOrNone { fn from (_ : ()) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponseOrNone :: None } }
    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseOrNone {
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponse,
        ),
        None,
    }

    impl From<ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseOrNone
    {
        fn from(
            value : ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponse,
        ) -> Self {
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponse (value)
        }
    }

    impl From<()>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseOrNone
    {
        fn from(_: ()) -> Self {
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseOrNone :: None
        }
    }

    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponseOrNone
    {
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponse,
        ),
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponse,
        ),
        None,
    }

    impl From < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponse > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponseOrNone { fn from (value : ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponse) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponse (value) } }
    impl From < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponse > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponseOrNone { fn from (value : ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponse) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponse (value) } }
    impl From < () > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponseOrNone { fn from (_ : ()) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponseOrNone :: None } }
    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrNone {
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponse,
        ),
        None,
    }

    impl From<ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrNone
    {
        fn from(
            value: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponse,
        ) -> Self {
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponse (value)
        }
    }

    impl From<()>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrNone
    {
        fn from(_: ()) -> Self {
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrNone::None
        }
    }

    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponseOrNone
    {
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponse,
        ),
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponse,
        ),
        None,
    }

    impl From < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponse > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponseOrNone { fn from (value : ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponse) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponse (value) } }
    impl From < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponse > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponseOrNone { fn from (value : ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponse) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponse (value) } }
    impl From < () > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponseOrNone { fn from (_ : ()) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponseOrNone :: None } }
    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone
    {
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponse,
        ),
        None,
    }

    impl From < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponse > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone { fn from (value : ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponse) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponse (value) } }
    impl From < () > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone { fn from (_ : ()) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone :: None } }
    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrNone {
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse,
        ),
        None,
    }

    impl From<ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrNone
    {
        fn from(
            value: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse,
        ) -> Self {
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse (value)
        }
    }

    impl From<()>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrNone
    {
        fn from(_: ()) -> Self {
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrNone :: None
        }
    }

    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone
    {
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse,
        ),
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponse,
        ),
        None,
    }

    impl From < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone { fn from (value : ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse (value) } }
    impl From < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponse > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone { fn from (value : ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponse) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponse (value) } }
    impl From < () > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone { fn from (_ : ()) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone :: None } }
    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponseOrNone
    {
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse,
        ),
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponse,
        ),
        None,
    }

    impl From < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponseOrNone { fn from (value : ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponse (value) } }
    impl From < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponse > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponseOrNone { fn from (value : ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponse) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponse (value) } }
    impl From < () > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponseOrNone { fn from (_ : ()) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponseOrNone :: None } }
    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrNone {
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponse,
        ),
        None,
    }

    impl From<ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponse>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrNone
    {
        fn from(
            value: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponse,
        ) -> Self {
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponse (value)
        }
    }

    impl From<()>
        for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrNone
    {
        fn from(_: ()) -> Self {
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrNone::None
        }
    }

    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponseOrNone
    {
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponse,
        ),
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponse,
        ),
        None,
    }

    impl From < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponse > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponseOrNone { fn from (value : ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponse) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponse (value) } }
    impl From < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponse > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponseOrNone { fn from (value : ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponse) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponse (value) } }
    impl From < () > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponseOrNone { fn from (_ : ()) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponseOrNone :: None } }
    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressBadRequestResponseOrNone
    {
        ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressBadRequestResponse(
            ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressBadRequestResponse,
        ),
        None,
    }

    impl From < ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressBadRequestResponse > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressBadRequestResponseOrNone { fn from (value : ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressBadRequestResponse) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressBadRequestResponseOrNone :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressBadRequestResponse (value) } }
    impl From < () > for ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressBadRequestResponseOrNone { fn from (_ : ()) -> Self { ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressBadRequestResponseOrNone :: None } }
}

#[derive(Clone, Debug)]
#[doc = "Client for ShipCompliant Connect\n\nShipCompliant REST API supporting eCommerce and Point of Sale workflows. <br /> * required fields.\n\nhttps://www.sovos.com/shipcompliant/terms-and-conditions/\n\nVersion: v1"]
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    #[doc = r" Create a new client."]
    #[doc = r""]
    #[doc = r" `baseurl` is the base URL provided to the internal"]
    #[doc = r" `reqwest::Client`, and should include a scheme and hostname,"]
    #[doc = r" as well as port and a path stem if applicable."]
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    #[doc = r" Construct a new client with an existing `reqwest::Client`,"]
    #[doc = r" allowing more control over its configuration."]
    #[doc = r""]
    #[doc = r" `baseurl` is the base URL provided to the internal"]
    #[doc = r" `reqwest::Client`, and should include a scheme and hostname,"]
    #[doc = r" as well as port and a path stem if applicable."]
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    #[doc = r" Get the base URL to which requests are made."]
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    #[doc = r" Get the internal `reqwest::Client` used to make requests."]
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    #[doc = r" Get the version of this API."]
    #[doc = r""]
    #[doc = r" This string is pulled directly from the source OpenAPI"]
    #[doc = r" document and may be in any format the API selects."]
    pub fn api_version(&self) -> &'static str {
        "v1"
    }
}

#[allow(clippy::all)]
impl Client {
    #[doc = "Validates the provided address\n\nSample request:\r\n            \r\n    POST /api/v1/addresses/validate\r\n    {\r\n        \"Address\": {\r\n            \"City\": \"Boulder\",\r\n            \"Company\": \"Sovos\",\r\n            \"Country\": \"US\",\r\n            \"County\": \"Boulder\",\r\n            \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n            \"Email\": \"example@sovos.com\",\r\n            \"Fax\": \"555-555-5555\",\r\n            \"FirstName\": \"Test First Name\",\r\n            \"LastName\": \"Test Last Name\",\r\n            \"Phone\": \"555-555-5555\",\r\n            \"State\": \"CO\",\r\n            \"Street1\": \"2465 Central Ave\",\r\n            \"Street2\": \"Ste 110\",\r\n            \"Zip1\": \"80301\",\r\n            \"Zip2\": \"5728\"\r\n         }\r\n    }\n\nSends a `POST` request to `/api/v1/addresses/validate`\n\nArguments:\n- `body`: The request body with the address to be validated\n"]    pub async fn post_addresses_validate < 'a > (& 'a self , body : & 'a types :: ShipCompliantRestApiDomainsAddressesEntitiesAddressBase) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesValidateAddressBadRequestResponseOrNone > , >{
        let url = format!("{}/api/v1/addresses/validate", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Retrieves brand information\n\nSample request:\r\n            \r\n    GET /api/v1/brands/BRAND123\n\nSends a `GET` request to `/api/v1/brands/{brandKey}`\n\nArguments:\n- `brand_key`: The brand key\n"]
    pub async fn get_brands_brand_key<'a>(
        &'a self,
        brand_key: &'a str,
    ) -> Result<
        ResponseValue<
            types::ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetBrandSuccessResponse,
        >,
        Error<
            types::ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseOrNone,
        >,
    > {
        let url = format!(
            "{}/api/v1/brands/{}",
            self.baseurl,
            encode_path(&brand_key.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Updates an existing brand\n\nSample request:\r\n            \r\n    PUT /api/v1/brands/BRAND123\r\n    {\r\n      \"Key\": \"BRAND123\",\r\n      \"TTBBrandKey\": \"TTBBrandKey\",\r\n      \"Name\": \"Example Brand\",\r\n      \"Owner\": {\r\n        \"City\": \"Boulder\",\r\n        \"Country\": \"US\",\r\n        \"Name\": \"Example Owner\",\r\n        \"State\": \"CO\",\r\n        \"Street1\": \"2465 Central Ave\",\r\n        \"Street2\": \"Ste 110\",\r\n        \"Zip\": \"80301\"\r\n      },\r\n      \"ThisBrandIsBottledByAThirdParty\": true,\r\n      \"ThisBrandIsProducedByAThirdParty\": true,\r\n      \"ThisBrandOperatesUnderATradeName\": true,\r\n      \"ThisBrandWasAcquiredFromAThirdParty\": false\r\n    }\n\nSends a `PUT` request to `/api/v1/brands/{brandKey}`\n\nArguments:\n- `brand_key`: The brand key\n- `body`: The new Brand data\n"]    pub async fn put_brands_brand_key < 'a > (& 'a self , brand_key : & 'a str , body : & 'a types :: ShipCompliantRestApiDomainsBrandEntitiesBrand) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPutBrandResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponseOrNone > , >{
        let url = format!(
            "{}/api/v1/brands/{}",
            self.baseurl,
            encode_path(&brand_key.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Adds a new brand\n\nSample request: \r\n            \r\n    POST /api/v1/brands\r\n    {\r\n        \"Key\": \"BRAND123\",\r\n        \"TTBBrandKey\": \"TTBBrandKey\",\r\n        \"Name\": \"Example Brand\",\r\n        \"Owner\": {\r\n            \"City\": \"Boulder\",\r\n            \"Country\": \"US\",\r\n            \"Name\": \"Example Owner\",\r\n            \"State\": \"CO\",\r\n            \"Street1\": \"2465 Central Ave\",\r\n            \"Street2\": \"Ste 110\",\r\n            \"Zip\": \"80301\"\r\n         },\r\n        \"ThisBrandIsBottledByAThirdParty\": true,\r\n        \"ThisBrandIsProducedByAThirdParty\": true,\r\n        \"ThisBrandOperatesUnderATradeName\": true,\r\n        \"ThisBrandWasAcquiredFromAThirdParty\": false\r\n    }\n\nSends a `POST` request to `/api/v1/brands`\n\nArguments:\n- `body`: The new Brand\n"]    pub async fn post_brands < 'a > (& 'a self , body : & 'a types :: ShipCompliantRestApiDomainsBrandEntitiesPostBrand) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostBrandResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesBrandBadRequestResponseOrNone > , >{
        let url = format!("{}/api/v1/brands", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Gets hold locations for an address within a specified radius\n\nSample request:\r\n            \r\n\tGET /api/v1/holdLocations/FedEx?zipCode=80301&streetAddress=2465+Central+Ave&city=Boulder&stateOrProvince=CO&searchRadius=150&countryCode=US\n\nSends a `GET` request to `/api/v1/holdLocations/{carrier}`\n\nArguments:\n- `carrier`: Either \"FedEx\" or \"UPS\"\n- `city`: Required param for city name\n- `country_code`: Required two digit country code\n- `search_radius`: Optional parameter that indicates mile radius for search. Defaults to 10\n- `state_or_province`: Required two digit state code\n- `street_address`: Required param to add precision to the search\n- `zip_code`: Required query parameter representing postal code\n"]    pub async fn get_hold_locations_carrier < 'a > (& 'a self , carrier : & 'a str , city : & 'a str , country_code : & 'a str , search_radius : Option < i32 > , state_or_province : & 'a str , street_address : & 'a str , zip_code : & 'a str) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetHoldLocationResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseOrNone > , >{
        let url = format!(
            "{}/api/v1/holdLocations/{}",
            self.baseurl,
            encode_path(&carrier.to_string()),
        );
        let mut query = Vec::with_capacity(6usize);
        query.push(("city", city.to_string()));
        query.push(("countryCode", country_code.to_string()));
        if let Some(v) = &search_radius {
            query.push(("searchRadius", v.to_string()));
        }

        query.push(("stateOrProvince", state_or_province.to_string()));
        query.push(("streetAddress", street_address.to_string()));
        query.push(("zipCode", zip_code.to_string()));
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Retrieves a specific Product by Product Key and Brand Key (optional)\n\nSample request:\r\n            \r\n    GET /api/v1/products/PROD123\r\n            \r\n If more than one brand contains the same product key:\r\n\r\n    GET /api/v1/products/PROD123?brandKey=BRAND123\n\nSends a `GET` request to `/api/v1/products/{productKey}`\n\nArguments:\n- `product_key`: The product number\n- `brand_key`: The brand key\n"]    pub async fn get_products_product_key < 'a > (& 'a self , product_key : & 'a str , brand_key : Option < & 'a str >) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetProductSuccessResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponseOrNone > , >{
        let url = format!(
            "{}/api/v1/products/{}",
            self.baseurl,
            encode_path(&product_key.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &brand_key {
            query.push(("brandKey", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Updates an existing product\n\n Sample request:\r\n\r\n     PUT /api/v1/products/PROD123\r\n     {\r\n\t\t\t\"Age\": 2010,\r\n\t    \t\"BottleSizeML\": 700,\r\n\t    \t\"BrandKey\": \"BRAND123\",\r\n\t    \t\"DefaultCase\": \"12\",\r\n\t    \t\"DefaultRetailUnitPrice\": 10.00,\r\n\t    \t\"DefaultWholesaleCasePrice\": 100.00,\r\n\t    \t\"Description\": \"Example Product\",\r\n\t    \t\"Flavor\": \"Merlot\",\r\n\t    \t\"Style\": \"Merlot\",\r\n\t    \t\"GTIN\": \"012345678905\",\r\n\t    \t\"Label\": \"Example Label\",\r\n\t    \t\"NABCA\": \"01234567891\",\r\n\t    \t\"PercentAlcohol\": 6.8,\r\n\t    \t\"ProductDistribution\": \"Both\",\r\n\t    \t\"ProductType\": \"Wine\",\r\n\t    \t\"SCC\": \"10012345678902\",\r\n\t    \t\"UNIMERC\": \"012345\",\r\n\t    \t\"UnitPrice\": 10.00,\r\n\t    \t\"UPC\": \"012345678905\",\r\n\t    \t\"Varietal\": \"Merlot\",\r\n\t    \t\"Vintage\": 2010,\r\n\t    \t\"VolumeAmount\": 700.00,\r\n\t    \t\"VolumeUnit\": \"Milliliter\",\r\n\t    \t\"SubBrand\": \"Example Sub Brand\",\r\n\t    \t\"ContainerType\": \"Bottle\",\r\n\t    \t\"ContainersPerSellingUnit\": 1,\r\n\t    \t\"ShippingWeightInLbs\": 1.2\r\n     }\n\nSends a `PUT` request to `/api/v1/products/{productKey}`\n\nArguments:\n- `product_key`: The Product key\n- `brand_key`: The Brand Key (optional)\n- `body`: The new Product data\n"]    pub async fn put_products_product_key < 'a > (& 'a self , product_key : & 'a str , brand_key : Option < & 'a str > , body : & 'a types :: ShipCompliantRestApiDomainsProductEntitiesProductInput) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPutProductResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrNone > , >{
        let url = format!(
            "{}/api/v1/products/{}",
            self.baseurl,
            encode_path(&product_key.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &brand_key {
            query.push(("brandKey", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Voids a Product by Product Key and Brand Key (optional)\n\nSample request:\r\n            \r\n\tDELETE /api/v1/products/PROD123\r\n\r\nIf more than one brand contains the same product key:\r\n            \r\n    DELETE /api/v1/products/PROD123?brandKey=BRAND123\n\nSends a `DELETE` request to `/api/v1/products/{productKey}`\n\nArguments:\n- `product_key`: The product number or sku\n- `brand_key`: The brand key\n"]    pub async fn delete_products_product_key < 'a > (& 'a self , product_key : & 'a str , brand_key : Option < & 'a str >) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteProductResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductNotFoundResponseOrNone > , >{
        let url = format!(
            "{}/api/v1/products/{}",
            self.baseurl,
            encode_path(&product_key.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &brand_key {
            query.push(("brandKey", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Adds a new product\n\nSample request:\r\n            \r\n    POST /api/v1/products\r\n    {\r\n        \"Age\": 2010,\r\n        \"BottleSizeML\": 700,\r\n        \"BrandKey\": \"BRAND123\",\r\n        \"DefaultCase\": \"12\",\r\n        \"DefaultRetailUnitPrice\": 10.00,\r\n        \"DefaultWholesaleCasePrice\": 100.00,\r\n        \"Description\": \"Example Product\",\r\n        \"Flavor\": \"Merlot\",\r\n        \"Style\": \"Merlot\",\r\n        \"GTIN\": \"012345678905\",\r\n        \"Label\": \"Example Label\",\r\n        \"NABCA\": \"01234567891\",\r\n        \"PercentAlcohol\": 6.8,\r\n        \"ProductDistribution\": \"Both\",\r\n        \"ProductKey\": \"EP\",\r\n        \"ProductType\": \"Wine\",\r\n        \"SCC\": \"10012345678902\",\r\n        \"UNIMERC\": \"012345\",\r\n        \"UnitPrice\": 10.00,\r\n        \"UPC\": \"012345678905\",\r\n        \"Varietal\": \"Merlot\",\r\n        \"Vintage\": 2010,\r\n        \"VolumeAmount\": 700.00,\r\n        \"VolumeUnit\": \"Milliliter\",\r\n        \"SubBrand\": \"Example Sub Brand\",\r\n        \"ContainerType\": \"Bottle\",\r\n        \"ContainersPerSellingUnit\": 1,\r\n        \"ShippingWeightInLbs\": 1.2\r\n    }\n\nSends a `POST` request to `/api/v1/products`\n\nArguments:\n- `body`: The new Product\n"]    pub async fn post_products < 'a > (& 'a self , body : & 'a types :: ShipCompliantRestApiDomainsProductEntitiesPostProductInput) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostProductResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrNone > , >{
        let url = format!("{}/api/v1/products", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Adds a new Kit\n\nSample request:\r\n            \r\n    POST /api/v1/products/kits\r\n    {\r\n        \"type\": \"Prebuilt\",\r\n        \"brandKey\": \"BRAND123\",\r\n        \"productKey\": \"KIT123\",\r\n        \"description\": \"Example Kit\",\r\n        \"totalWeight\": 10.95,\r\n        \"components\": [\r\n          {\r\n            \"productKey\": \"PROD123\",\r\n            \"brandKey\": \"BRAND123\",\r\n            \"quantity\": 2\r\n          }\r\n        ],\r\n        \"distributionType\": \"Direct\"\r\n    }\n\nSends a `POST` request to `/api/v1/products/kits`\n\nArguments:\n- `body`: The new Kit data\n"]    pub async fn post_products_kits < 'a > (& 'a self , body : & 'a types :: ShipCompliantRestApiDomainsProductEntitiesKitInput) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostProductResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrNone > , >{
        let url = format!("{}/api/v1/products/kits", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Adds a new Combo\n\nSample request:\r\n            \r\n    POST /api/v1/products/combos\r\n    {\r\n        \"brandKey\": \"BRAND123\",\r\n        \"productComboKey\": \"COMBO123\",\r\n        \"description\": \"Example Combo\",\r\n        \"itemsPerCase\": 2,\r\n        \"wholesaleCasePrice\": 10.50,\r\n        \"totalWeight\": 20.00,\r\n        \"nabca\": \"string\",\r\n        \"scc\": \"string\",\r\n        \"unimerc\": \"string\",\r\n        \"upc\": \"string\",\r\n        \"gtin\": \"string\",\r\n        \"components\": [\r\n        {\r\n            \"productKey\": \"PROD123\",\r\n            \"brandKey\": \"BRAND123\",\r\n            \"quantity\": 3,\r\n        }\r\n        ],\r\n        \"distributionType\": \"Direct\"\r\n   }\n\nSends a `POST` request to `/api/v1/products/combos`\n\nArguments:\n- `body`: The new Combo data\n"]    pub async fn post_products_combos < 'a > (& 'a self , body : & 'a types :: ShipCompliantRestApiDomainsProductEntitiesComboInput) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostProductResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesProductBadRequestResponseOrNone > , >{
        let url = format!("{}/api/v1/products/combos", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Retrieves the specified sales order\n\nSample request:\r\n            \r\n    GET /api/v1/salesOrders/ORDER123\n\nSends a `GET` request to `/api/v1/salesOrders/{salesOrderKey}`\n\nArguments:\n- `sales_order_key`: The sales order number\n"]    pub async fn get_sales_orders_sales_order_key < 'a > (& 'a self , sales_order_key : & 'a str) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetSalesOrderSuccessResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrNone > , >{
        let url = format!(
            "{}/api/v1/salesOrders/{}",
            self.baseurl,
            encode_path(&sales_order_key.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Voids a sales order\n\nSample request:\r\n            \r\n    DELETE /api/v1/salesOrders/ORDER123\n\nSends a `DELETE` request to `/api/v1/salesOrders/{salesOrderKey}`\n\nArguments:\n- `sales_order_key`: The sales order number\n"]    pub async fn delete_sales_orders_sales_order_key < 'a > (& 'a self , sales_order_key : & 'a str) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteSalesOrderResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrNone > , >{
        let url = format!(
            "{}/api/v1/salesOrders/{}",
            self.baseurl,
            encode_path(&sales_order_key.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Saves a sales order to the system without checking compliance or tax due\n\nSample request:\r\n            \r\n\tPOST /api/v1/salesOrders\r\n\t{\r\n\t  \"SalesOrder\": {\r\n\t    \"BillTo\": {\r\n\t      \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n\t      \"City\": \"Boulder\",\r\n\t      \"Company\": \"Sovos\",\r\n\t      \"Country\": \"US\",\r\n\t      \"County\": \"Boulder\",\r\n\t      \"Email\": \"example@sovos.com\",\r\n\t      \"Fax\": \"555-555-5555\",\r\n\t      \"FirstName\": \"Test First Name\",\r\n\t      \"LastName\": \"Test Last Name\",\r\n\t      \"Phone\": \"555-555-5555\",\r\n\t      \"State\": \"CO\",\r\n\t      \"Street1\": \"2465 Central Ave\",\r\n\t      \"Street2\": \"Ste 110\",\r\n\t      \"Zip1\": \"80301\",\r\n\t      \"Zip2\": \"5728\"\r\n\t    },\r\n\t    \"CustomerKey\": \"Custom123\",\r\n\t    \"Discounts\": [\r\n\t      {\r\n\t        \"Amount\": 0.00,\r\n\t        \"Code\": \"string\",\r\n\t        \"Type\": \"string\"\r\n\t      }\r\n\t    ],\r\n\t    \"FulfillmentType\": \"Daily\",\r\n\t    \"OrderType\": \"Internet\",\r\n\t    \"Payments\": [\r\n\t      {\r\n\t        \"Amount\": 0.00,\r\n\t        \"SubType\": \"VISA\",\r\n\t        \"TransactionID\": \"string\",\r\n\t        \"Type\": \"CreditCard\"\r\n\t      }\r\n\t    ],\r\n\t    \"PurchaseDate\": \"2020-11-01T00:00:00Z\",\r\n\t    \"SalesOrderKey\": \"Order123\",\r\n\t    \"SalesTaxCollected\": 0.00,\r\n\t    \"Shipments\": [\r\n\t      {\r\n\t        \"Discounts\": [\r\n\t          {\r\n\t            \"Amount\": 0.00,\r\n\t            \"Code\": \"string\",\r\n\t            \"Type\": \"string\"\r\n\t          }\r\n\t        ],\r\n\t        \"FulfillmentAccount\": \"Account123\",\r\n\t        \"FulfillmentHouse\": \"WineShipping\",\r\n\t        \"FulfillmentExceptionReason\": \"Test Exception Reason\",\r\n\t        \"FulfillmentExceptionType\": \"Test Exception Type\",\r\n\t        \"GiftNote\": \"Happy Birthday\",\r\n\t        \"Handling\": 0.00,\r\n\t        \"LicenseRelationship\": \"SupplierToConsumer\",\r\n\t        \"Packages\": [\r\n\t          {\r\n\t            \"TrackingNumber\": \"ABC123456789\"\r\n\t          }\r\n\t        ],\r\n\t        \"ShipDate\": \"2020-11-01T00:00:00Z\",\r\n\t        \"ShipmentItems\": [{\r\n\t          \"BrandKey\": \"Brand123\",\r\n\t          \"Discounts\": [\r\n\t            {\r\n\t              \"Amount\": 0.00,\r\n\t              \"Code\": \"string\",\r\n\t              \"Type\": \"string\"\r\n\t            }\r\n\t          ],\r\n\t          \"ProductKey\": \"Product123\",\r\n\t          \"ProductQuantity\": 2,\r\n\t          \"ProductUnitPrice\": 19.99,\r\n\t          \"CITB\": \"CITB\"\r\n\t        }],\r\n\t        \"ShipmentKey\": \"1\",\r\n\t        \"ShipmentStatus\": \"SentToFulfillment\",\r\n\t        \"Shipping\": 0.00,\r\n\t        \"ShippingService\": \"FEX\",\r\n\t        \"ShipTo\": {\r\n\t           \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n\t            \"City\": \"Boulder\",\r\n\t            \"Company\": \"Sovos\",\r\n\t            \"Country\": \"US\",\r\n\t            \"County\": \"Boulder\",\r\n\t            \"Email\": \"example@sovos.com\",\r\n\t            \"Fax\": \"555-555-5555\",\r\n\t            \"FirstName\": \"Test First Name\",\r\n\t            \"LastName\": \"Test Last Name\",\r\n\t            \"Phone\": \"555-555-5555\",\r\n\t            \"State\": \"CO\",\r\n\t            \"Street1\": \"2465 Central Ave\",\r\n\t            \"Street2\": \"Ste 110\",\r\n\t            \"Zip1\": \"80301\",\r\n\t            \"Zip2\": \"5728\"\r\n\t         },\r\n\t        \"SpecialInstructions\": \"Test Instructions\"\r\n\t      }\r\n\t    ],\r\n\t    \"Tags\": [\r\n\t      {\r\n\t        \"Name\": \"Test Tag\"\r\n\t      }\r\n\t    ]\r\n\t  },\r\n\t  \"AddressOption\": {\r\n\t    \"IgnoreStreetLevelErrors\": false,\r\n\t    \"RejectIfAddressSuggested\": true\r\n\t  },\r\n\t  \"PersistOption\": \"Null\"\r\n\t}\n\nSends a `POST` request to `/api/v1/salesOrders`\n\nArguments:\n- `body`: The sales order to Persist\n"]    pub async fn post_sales_orders < 'a > (& 'a self , body : & 'a types :: ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderBase) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostSalesOrderResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone > , >{
        let url = format!("{}/api/v1/salesOrders", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Checks the compliance and tax due of a sales order that will be committed at a later time\n\nSample request:\r\n            \r\n\tPOST /api/v1/salesOrders/check-compliance\r\n\t{\r\n\t  \"SalesOrder\": {\r\n\t    \"BillTo\": {\r\n\t      \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n\t      \"City\": \"Boulder\",\r\n\t      \"Company\": \"Sovos\",\r\n\t      \"Country\": \"US\",\r\n\t      \"County\": \"Boulder\",\r\n\t      \"Email\": \"example@sovos.com\",\r\n\t      \"Fax\": \"555-555-5555\",\r\n\t      \"FirstName\": \"Test First Name\",\r\n\t      \"LastName\": \"Test Last Name\",\r\n\t      \"Phone\": \"555-555-5555\",\r\n\t      \"State\": \"CO\",\r\n\t      \"Street1\": \"2465 Central Ave\",\r\n\t      \"Street2\": \"Ste 110\",\r\n\t      \"Zip1\": \"80301\",\r\n\t      \"Zip2\": \"5728\"\r\n\t    },\r\n\t    \"CustomerKey\": \"Custom123\",\r\n\t    \"Discounts\": [\r\n\t      {\r\n\t        \"Amount\": 0.00,\r\n\t        \"Code\": \"string\",\r\n\t        \"Type\": \"string\"\r\n\t      }\r\n\t    ],\r\n\t    \"FulfillmentType\": \"Daily\",\r\n\t    \"OrderType\": \"Internet\",\r\n\t    \"Payments\": [\r\n\t      {\r\n\t        \"Amount\": 0.00,\r\n\t        \"SubType\": \"VISA\",\r\n\t        \"TransactionID\": \"string\",\r\n\t        \"Type\": \"CreditCard\"\r\n\t      }\r\n\t    ],\r\n\t    \"PurchaseDate\": \"2020-11-01T00:00:00Z\",\r\n\t    \"SalesOrderKey\": \"Order123\",\r\n\t    \"SalesTaxCollected\": 0.00,\r\n\t    \"Shipments\": [\r\n\t      {\r\n\t        \"Discounts\": [\r\n\t          {\r\n\t            \"Amount\": 0.00,\r\n\t            \"Code\": \"string\",\r\n\t            \"Type\": \"string\"\r\n\t          }\r\n\t        ],\r\n\t        \"FulfillmentAccount\": \"Account123\",\r\n\t        \"FulfillmentHouse\": \"WineShipping\",\r\n\t        \"FulfillmentExceptionReason\": \"Test Exception Reason\",\r\n\t        \"FulfillmentExceptionType\": \"Test Exception Type\",\r\n\t        \"GiftNote\": \"Happy Birthday\",\r\n\t        \"Handling\": 0.00,\r\n\t        \"LicenseRelationship\": \"SupplierToConsumer\",\r\n\t        \"Packages\": [\r\n\t          {\r\n\t            \"TrackingNumber\": \"ABC123456789\"\r\n\t          }\r\n\t        ],\r\n\t        \"ShipDate\": \"2020-11-01T00:00:00Z\",\r\n\t        \"ShipmentItems\": [{\r\n\t          \"BrandKey\": \"Brand123\",\r\n\t          \"Discounts\": [\r\n\t            {\r\n\t              \"Amount\": 0.00,\r\n\t              \"Code\": \"string\",\r\n\t              \"Type\": \"string\"\r\n\t            }\r\n\t          ],\r\n\t          \"ProductKey\": \"Product123\",\r\n\t          \"ProductQuantity\": 2,\r\n\t          \"ProductUnitPrice\": 19.99,\r\n\t          \"CITB\": \"CITB\"\r\n\t        }],\r\n\t        \"ShipmentKey\": \"1\",\r\n\t        \"ShipmentStatus\": \"SentToFulfillment\",\r\n\t        \"Shipping\": 0.00,\r\n\t        \"ShippingService\": \"FEX\",\r\n\t        \"ShipTo\": {\r\n\t           \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n\t            \"City\": \"Boulder\",\r\n\t            \"Company\": \"Sovos\",\r\n\t            \"Country\": \"US\",\r\n\t            \"County\": \"Boulder\",\r\n\t            \"Email\": \"example@sovos.com\",\r\n\t            \"Fax\": \"555-555-5555\",\r\n\t            \"FirstName\": \"Test First Name\",\r\n\t            \"LastName\": \"Test Last Name\",\r\n\t            \"Phone\": \"555-555-5555\",\r\n\t            \"State\": \"CO\",\r\n\t            \"Street1\": \"2465 Central Ave\",\r\n\t            \"Street2\": \"Ste 110\",\r\n\t            \"Zip1\": \"80301\",\r\n\t            \"Zip2\": \"5728\"\r\n\t         },\r\n\t        \"SpecialInstructions\": \"Test Instructions\"\r\n\t      }\r\n\t    ],\r\n\t    \"Tags\": [\r\n\t      {\r\n\t        \"Name\": \"Test Tag\"\r\n\t      }\r\n\t    ]\r\n\t  },\r\n\t  \"AddressOption\": {\r\n\t    \"IgnoreStreetLevelErrors\": false,\r\n\t    \"RejectIfAddressSuggested\": true\r\n\t  },\r\n\t  \"PersistOption\": \"Null\"\r\n\t}\n\nSends a `POST` request to `/api/v1/salesOrders/check-compliance`\n\nArguments:\n- `body`: The sales order to check compliance\n"]    pub async fn post_sales_orders_check_compliance < 'a > (& 'a self , body : & 'a types :: ShipCompliantRestApiDomainsSalesOrderEntitiesSalesOrderBase) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesCheckComplianceResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone > , >{
        let url = format!("{}/api/v1/salesOrders/check-compliance", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Checks the compliance and tax due of a sales order and saves the order to the system\n\nSample request:\r\n            \r\n\tPOST /api/v1/salesOrders/check-commit\r\n\t{\r\n\t  \"CommitOption\": \"AllShipments\",\r\n\t  \"SalesOrder\": {\r\n\t    \"BillTo\": {\r\n\t      \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n\t      \"City\": \"Boulder\",\r\n\t      \"Company\": \"Sovos\",\r\n\t      \"Country\": \"US\",\r\n\t      \"County\": \"Boulder\",\r\n\t      \"Email\": \"example@sovos.com\",\r\n\t      \"Fax\": \"555-555-5555\",\r\n\t      \"FirstName\": \"Test First Name\",\r\n\t      \"LastName\": \"Test Last Name\",\r\n\t      \"Phone\": \"555-555-5555\",\r\n\t      \"State\": \"CO\",\r\n\t      \"Street1\": \"2465 Central Ave\",\r\n\t      \"Street2\": \"Ste 110\",\r\n\t      \"Zip1\": \"80301\",\r\n\t      \"Zip2\": \"5728\"\r\n\t    },\r\n\t    \"CustomerKey\": \"Custom123\",\r\n\t    \"Discounts\": [\r\n\t      {\r\n\t        \"Amount\": 0.00,\r\n\t        \"Code\": \"string\",\r\n\t        \"Type\": \"string\"\r\n\t      }\r\n\t    ],\r\n\t    \"FulfillmentType\": \"Daily\",\r\n\t    \"OrderType\": \"Internet\",\r\n\t    \"Payments\": [\r\n\t      {\r\n\t        \"Amount\": 0.00,\r\n\t        \"SubType\": \"VISA\",\r\n\t        \"TransactionID\": \"string\",\r\n\t        \"Type\": \"CreditCard\"\r\n\t      }\r\n\t    ],\r\n\t    \"PurchaseDate\": \"2020-11-01T00:00:00Z\",\r\n\t    \"SalesOrderKey\": \"Order123\",\r\n\t    \"SalesTaxCollected\": 0.00,\r\n\t    \"Shipments\": [\r\n\t      {\r\n\t        \"Discounts\": [\r\n\t          {\r\n\t            \"Amount\": 0.00,\r\n\t            \"Code\": \"string\",\r\n\t            \"Type\": \"string\"\r\n\t          }\r\n\t        ],\r\n\t        \"FulfillmentAccount\": \"Account123\",\r\n\t        \"FulfillmentHouse\": \"WineShipping\",\r\n\t        \"FulfillmentExceptionReason\": \"Test Exception Reason\",\r\n\t        \"FulfillmentExceptionType\": \"Test Exception Type\",\r\n\t        \"GiftNote\": \"Happy Birthday\",\r\n\t        \"Handling\": 0.00,\r\n\t        \"LicenseRelationship\": \"SupplierToConsumer\",\r\n\t        \"Packages\": [\r\n\t          {\r\n\t            \"TrackingNumber\": \"ABC123456789\"\r\n\t          }\r\n\t        ],\r\n\t        \"ShipDate\": \"2020-11-01T00:00:00Z\",\r\n\t        \"ShipmentItems\": [{\r\n\t          \"BrandKey\": \"Brand123\",\r\n\t          \"Discounts\": [\r\n\t            {\r\n\t              \"Amount\": 0.00,\r\n\t              \"Code\": \"string\",\r\n\t              \"Type\": \"string\"\r\n\t            }\r\n\t          ],\r\n\t          \"ProductKey\": \"Product123\",\r\n\t          \"ProductQuantity\": 2,\r\n\t          \"ProductUnitPrice\": 19.99,\r\n\t          \"CITB\": \"CITB\"\r\n\t        }],\r\n\t        \"ShipmentKey\": \"1\",\r\n\t        \"ShipmentStatus\": \"SentToFulfillment\",\r\n\t        \"Shipping\": 0.00,\r\n\t        \"ShippingService\": \"FEX\",\r\n\t        \"ShipTo\": {\r\n\t           \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n\t            \"City\": \"Boulder\",\r\n\t            \"Company\": \"Sovos\",\r\n\t            \"Country\": \"US\",\r\n\t            \"County\": \"Boulder\",\r\n\t            \"Email\": \"example@sovos.com\",\r\n\t            \"Fax\": \"555-555-5555\",\r\n\t            \"FirstName\": \"Test First Name\",\r\n\t            \"LastName\": \"Test Last Name\",\r\n\t            \"Phone\": \"555-555-5555\",\r\n\t            \"State\": \"CO\",\r\n\t            \"Street1\": \"2465 Central Ave\",\r\n\t            \"Street2\": \"Ste 110\",\r\n\t            \"Zip1\": \"80301\",\r\n\t            \"Zip2\": \"5728\"\r\n\t         },\r\n\t        \"SpecialInstructions\": \"Test Instructions\"\r\n\t      }\r\n\t    ],\r\n\t    \"Tags\": [\r\n\t      {\r\n\t        \"Name\": \"Test Tag\"\r\n\t      }\r\n\t    ]\r\n\t  },\r\n\t  \"AddressOption\": {\r\n\t    \"IgnoreStreetLevelErrors\": false,\r\n\t    \"RejectIfAddressSuggested\": true\r\n\t  },\r\n\t  \"PersistOption\": \"Null\"\r\n\t}\n\nSends a `POST` request to `/api/v1/salesOrders/check-commit`\n\nArguments:\n- `body`: The sales order to check compliance\n"]    pub async fn post_sales_orders_check_commit < 'a > (& 'a self , body : & 'a types :: ShipCompliantRestApiDomainsSalesOrderEntitiesCheckAndCommitSalesOrder) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesCheckComplianceResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone > , >{
        let url = format!("{}/api/v1/salesOrders/check-commit", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Saves a sales order to the system that has previously been checked for compliance\n\nSample request:\r\n            \r\n\tPOST /api/v1/salesOrders/commit\r\n\t{\r\n\t  \"CommitOption\": \"AllShipments\",\r\n\t  \"Payments\": [\r\n\t    {\r\n\t      \"Amount\": 0.00,\r\n\t      \"SubType\": \"VISA\",\r\n\t      \"TransactionID\": \"string\",\r\n\t      \"Type\": \"CreditCard\"\r\n\t    }\r\n\t  ],\r\n\t  \"SalesTaxCollected\": 0.00,\r\n\t  \"SalesOrderKey\": \"Order123\"\r\n\t}\n\nSends a `POST` request to `/api/v1/salesOrders/commit`\n\nArguments:\n- `body`: The sales order to commit\n"]    pub async fn post_sales_orders_commit < 'a > (& 'a self , body : & 'a types :: ShipCompliantRestApiDomainsSalesOrderEntitiesCommitSalesOrder) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesCommitResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponseOrNone > , >{
        let url = format!("{}/api/v1/salesOrders/commit", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Retrieves all tracking numbers by salesOrderKey or tracking numbers for a specified (optional) shipment key\n\nSample request:\r\n            \r\n\tGET /api/v1/salesOrders/Order123/tracking\r\n            \r\nIf a specific shipment tracking is required.\r\n            \r\n    GET /api/v1/salesOrders/Order123/tracking?shipmentKey=Ship1\n\nSends a `GET` request to `/api/v1/salesOrders/{salesOrderKey}/tracking`\n\nArguments:\n- `sales_order_key`: The sales order number\n- `shipment_key`: The Shipment key\n"]    pub async fn get_sales_orders_sales_order_key_tracking < 'a > (& 'a self , sales_order_key : & 'a str , shipment_key : Option < & 'a Vec < String > >) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesGetTrackingByKeyResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrNone > , >{
        let url = format!(
            "{}/api/v1/salesOrders/{}/tracking",
            self.baseurl,
            encode_path(&sales_order_key.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &shipment_key {
            for k in v.into_iter() {
                query.push(("shipmentKey[]", k.to_string()));
            }
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Adds a new Tracking\n\nSample request:\r\n            \r\n\tPOST /api/v1/salesOrders/Order123/tracking\r\n\t{\r\n\t    \"ShipmentKey\": \"Ship1\",\r\n\t    \"TrackingNumbers\": [\r\n\t    \t\"ABC123456789\"\r\n\t    ]\r\n\t}\n\nSends a `POST` request to `/api/v1/salesOrders/{salesOrderKey}/tracking`\n\nArguments:\n- `sales_order_key`: The sales order number\n- `body`: The new Trackings\n"]    pub async fn post_sales_orders_sales_order_key_tracking < 'a > (& 'a self , sales_order_key : & 'a str , body : & 'a types :: ShipCompliantRestApiDomainsTrackingEntitiesTrackingInput) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostTrackingResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingBadRequestResponseOrNone > , >{
        let url = format!(
            "{}/api/v1/salesOrders/{}/tracking",
            self.baseurl,
            encode_path(&sales_order_key.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Voids all Trackings for a sales order\n\nSample request:\r\n            \r\n\tDELETE /api/v1/salesOrders/Order123/tracking\n\nSends a `DELETE` request to `/api/v1/salesOrders/{salesOrderKey}/tracking`\n\nArguments:\n- `sales_order_key`: The sales order number\n"]    pub async fn delete_sales_orders_sales_order_key_tracking < 'a > (& 'a self , sales_order_key : & 'a str) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteTrackingResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrNone > , >{
        let url = format!(
            "{}/api/v1/salesOrders/{}/tracking",
            self.baseurl,
            encode_path(&sales_order_key.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Voids all Trackings for a specific shipment in a sales order\n\nSample request:\r\n            \r\n\tDELETE /api/v1/salesOrders/Order123/shipment/Ship1/tracking\n\nSends a `DELETE` request to `/api/v1/salesOrders/{salesOrderKey}/shipment/{shipmentKey}/tracking`\n\nArguments:\n- `sales_order_key`: The sales order number\n- `shipment_key`: The shipment number\n"]    pub async fn delete_sales_orders_sales_order_key_shipment_shipment_key_tracking < 'a > (& 'a self , sales_order_key : & 'a str , shipment_key : & 'a str) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteTrackingResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrNone > , >{
        let url = format!(
            "{}/api/v1/salesOrders/{}/shipment/{}/tracking",
            self.baseurl,
            encode_path(&sales_order_key.to_string()),
            encode_path(&shipment_key.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Voids a specific tracking number for all shipments in a sales order\n\nSample request:\r\n            \r\n\tDELETE /api/v1/salesOrders/Order123/tracking/ABC123456789\n\nSends a `DELETE` request to `/api/v1/salesOrders/{salesOrderKey}/tracking/{trackingNumber}`\n\nArguments:\n- `sales_order_key`: The sales order number\n- `tracking_number`: The tracking number\n"]    pub async fn delete_sales_orders_sales_order_key_tracking_tracking_number < 'a > (& 'a self , sales_order_key : & 'a str , tracking_number : & 'a str) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteTrackingResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrNone > , >{
        let url = format!(
            "{}/api/v1/salesOrders/{}/tracking/{}",
            self.baseurl,
            encode_path(&sales_order_key.to_string()),
            encode_path(&tracking_number.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Voids a specific tracking number for a shipment in a sales order\n\nSample request:\r\n            \r\n\tDELETE /api/v1/salesOrders/Order123/shipment/Ship1/tracking/ABC123456789\n\nSends a `DELETE` request to `/api/v1/salesOrders/{salesOrderKey}/shipment/{shipmentKey}/tracking/{trackingNumber}`\n\nArguments:\n- `sales_order_key`: The sales order number\n- `shipment_key`: The shipment number\n- `tracking_number`: The tracking number\n"]    pub async fn delete_sales_orders_sales_order_key_shipment_shipment_key_tracking_tracking_number < 'a > (& 'a self , sales_order_key : & 'a str , shipment_key : & 'a str , tracking_number : & 'a str) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteTrackingResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTrackingNotFoundResponseOrNone > , >{
        let url = format!(
            "{}/api/v1/salesOrders/{}/shipment/{}/tracking/{}",
            self.baseurl,
            encode_path(&sales_order_key.to_string()),
            encode_path(&shipment_key.to_string()),
            encode_path(&tracking_number.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Updates shipment status\n\nSample request:\r\n            \r\n    PUT /api/v1/salesOrders/shipments/status\r\n    {\r\n\t    \"salesOrderKey\": \"109807833\",\r\n        \"shipmentKey\": \"2012351B\",\r\n        \"shipmentStatus\": \"Delivered\"\r\n\t}\n\nSends a `PUT` request to `/api/v1/salesOrders/shipments/status`\n\nArguments:\n- `body`: Request body containing SalesOrderKey, ShipmentKey and new ShipmentStatus\n"]    pub async fn put_sales_orders_shipments_status < 'a > (& 'a self , body : & 'a types :: ShipCompliantRestApiDomainsSalesOrderEntitiesUpdateShipmentStatusInput) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteSalesOrderResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrNone > , >{
        let url = format!("{}/api/v1/salesOrders/shipments/status", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Checks the compliance and tax due of a sales order without saving results for future use\n\nSample request:\r\n            \r\n\tPOST /api/v1/salesOrders/quote\r\n\t{\r\n\t  \"SalesOrder\": {\r\n\t    \"BillTo\": {\r\n\t      \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n\t      \"City\": \"Boulder\",\r\n\t      \"Company\": \"Sovos\",\r\n\t      \"Country\": \"US\",\r\n\t      \"County\": \"Boulder\",\r\n\t      \"Email\": \"example@sovos.com\",\r\n\t      \"Fax\": \"555-555-5555\",\r\n\t      \"FirstName\": \"Test First Name\",\r\n\t      \"LastName\": \"Test Last Name\",\r\n\t      \"Phone\": \"555-555-5555\",\r\n\t      \"State\": \"CO\",\r\n\t      \"Street1\": \"2465 Central Ave\",\r\n\t      \"Street2\": \"Ste 110\",\r\n\t      \"Zip1\": \"80301\",\r\n\t      \"Zip2\": \"5728\"\r\n\t    },\r\n\t    \"CustomerKey\": \"Custom123\",\r\n\t    \"Discounts\": [\r\n\t      {\r\n\t        \"Amount\": 0.00,\r\n\t        \"Code\": \"string\",\r\n\t        \"Type\": \"string\"\r\n\t      }\r\n\t    ],\r\n\t    \"FulfillmentType\": \"Daily\",\r\n\t    \"OrderType\": \"Internet\",\r\n\t    \"Payments\": [\r\n\t      {\r\n\t        \"Amount\": 0.00,\r\n\t        \"SubType\": \"VISA\",\r\n\t        \"TransactionID\": \"string\",\r\n\t        \"Type\": \"CreditCard\"\r\n\t      }\r\n\t    ],\r\n\t    \"PurchaseDate\": \"2020-11-01T00:00:00Z\",\r\n\t    \"SalesOrderKey\": \"Order123\",\r\n\t    \"SalesTaxCollected\": 0.00,\r\n\t    \"Shipments\": [\r\n\t      {\r\n\t        \"Discounts\": [\r\n\t          {\r\n\t            \"Amount\": 0.00,\r\n\t            \"Code\": \"string\",\r\n\t            \"Type\": \"string\"\r\n\t          }\r\n\t        ],\r\n\t        \"FulfillmentAccount\": \"Account123\",\r\n\t        \"FulfillmentHouse\": \"WineShipping\",\r\n\t        \"FulfillmentExceptionReason\": \"Test Exception Reason\",\r\n\t        \"FulfillmentExceptionType\": \"Test Exception Type\",\r\n\t        \"GiftNote\": \"Happy Birthday\",\r\n\t        \"Handling\": 0.00,\r\n\t        \"LicenseRelationship\": \"SupplierToConsumer\",\r\n\t        \"Packages\": [\r\n\t          {\r\n\t            \"TrackingNumber\": \"ABC123456789\"\r\n\t          }\r\n\t        ],\r\n\t        \"ShipDate\": \"2020-11-01T00:00:00Z\",\r\n\t        \"ShipmentItems\": [{\r\n\t          \"BrandKey\": \"Brand123\",\r\n\t          \"Discounts\": [\r\n\t            {\r\n\t              \"Amount\": 0.00,\r\n\t              \"Code\": \"string\",\r\n\t              \"Type\": \"string\"\r\n\t            }\r\n\t          ],\r\n\t          \"ProductKey\": \"Product123\",\r\n\t          \"ProductQuantity\": 2,\r\n\t          \"ProductUnitPrice\": 19.99,\r\n\t          \"CITB\": \"CITB\"\r\n\t        }],\r\n\t        \"ShipmentKey\": \"1\",\r\n\t        \"ShipmentStatus\": \"SentToFulfillment\",\r\n\t        \"Shipping\": 0.00,\r\n\t        \"ShippingService\": \"FEX\",\r\n\t        \"ShipTo\": {\r\n\t           \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n\t            \"City\": \"Boulder\",\r\n\t            \"Company\": \"Sovos\",\r\n\t            \"Country\": \"US\",\r\n\t            \"County\": \"Boulder\",\r\n\t            \"Email\": \"example@sovos.com\",\r\n\t            \"Fax\": \"555-555-5555\",\r\n\t            \"FirstName\": \"Test First Name\",\r\n\t            \"LastName\": \"Test Last Name\",\r\n\t            \"Phone\": \"555-555-5555\",\r\n\t            \"State\": \"CO\",\r\n\t            \"Street1\": \"2465 Central Ave\",\r\n\t            \"Street2\": \"Ste 110\",\r\n\t            \"Zip1\": \"80301\",\r\n\t            \"Zip2\": \"5728\"\r\n\t         },\r\n\t        \"SpecialInstructions\": \"Test Instructions\"\r\n\t      }\r\n\t    ],\r\n\t    \"Tags\": [\r\n\t      {\r\n\t        \"Name\": \"Test Tag\"\r\n\t      }\r\n\t    ]\r\n\t  },\r\n\t  \"AddressOption\": {\r\n\t    \"IgnoreStreetLevelErrors\": false,\r\n\t    \"RejectIfAddressSuggested\": true\r\n\t  }\r\n\t}\n\nSends a `POST` request to `/api/v1/salesOrders/quote`\n\nArguments:\n- `body`: The sales order to Quote\n"]    pub async fn post_sales_orders_quote < 'a > (& 'a self , body : & 'a types :: ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteSalesOrderInput) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesCheckComplianceResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone > , >{
        let url = format!("{}/api/v1/salesOrders/quote", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Checks the compliance of a sales order without saving results for future use\n\nSample request:\r\n            \r\n\tPOST /api/v1/salesOrders/quote/compliance\r\n\t{\r\n\t  \"SalesOrder\": {\r\n\t    \"BillTo\": {\r\n\t      \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n\t      \"City\": \"Boulder\",\r\n\t      \"Company\": \"Sovos\",\r\n\t      \"Country\": \"US\",\r\n\t      \"County\": \"Boulder\",\r\n\t      \"Email\": \"example@sovos.com\",\r\n\t      \"Fax\": \"555-555-5555\",\r\n\t      \"FirstName\": \"Test First Name\",\r\n\t      \"LastName\": \"Test Last Name\",\r\n\t      \"Phone\": \"555-555-5555\",\r\n\t      \"State\": \"CO\",\r\n\t      \"Street1\": \"2465 Central Ave\",\r\n\t      \"Street2\": \"Ste 110\",\r\n\t      \"Zip1\": \"80301\",\r\n\t      \"Zip2\": \"5728\"\r\n\t    },\r\n\t    \"CustomerKey\": \"Custom123\",\r\n\t    \"Discounts\": [\r\n\t      {\r\n\t        \"Amount\": 0.00,\r\n\t        \"Code\": \"string\",\r\n\t        \"Type\": \"string\"\r\n\t      }\r\n\t    ],\r\n\t    \"FulfillmentType\": \"Daily\",\r\n\t    \"OrderType\": \"Internet\",\r\n\t    \"Payments\": [\r\n\t      {\r\n\t        \"Amount\": 0.00,\r\n\t        \"SubType\": \"VISA\",\r\n\t        \"TransactionID\": \"string\",\r\n\t        \"Type\": \"CreditCard\"\r\n\t      }\r\n\t    ],\r\n\t    \"PurchaseDate\": \"2020-11-01T00:00:00Z\",\r\n\t    \"SalesOrderKey\": \"Order123\",\r\n\t    \"SalesTaxCollected\": 0.00,\r\n\t    \"Shipments\": [\r\n\t      {\r\n\t        \"Discounts\": [\r\n\t          {\r\n\t            \"Amount\": 0.00,\r\n\t            \"Code\": \"string\",\r\n\t            \"Type\": \"string\"\r\n\t          }\r\n\t        ],\r\n\t        \"FulfillmentAccount\": \"Account123\",\r\n\t        \"FulfillmentHouse\": \"WineShipping\",\r\n\t        \"FulfillmentExceptionReason\": \"Test Exception Reason\",\r\n\t        \"FulfillmentExceptionType\": \"Test Exception Type\",\r\n\t        \"GiftNote\": \"Happy Birthday\",\r\n\t        \"Handling\": 0.00,\r\n\t        \"LicenseRelationship\": \"SupplierToConsumer\",\r\n\t        \"Packages\": [\r\n\t          {\r\n\t            \"TrackingNumber\": \"ABC123456789\"\r\n\t          }\r\n\t        ],\r\n\t        \"ShipDate\": \"2020-11-01T00:00:00Z\",\r\n\t        \"ShipmentItems\": [{\r\n\t          \"BrandKey\": \"Brand123\",\r\n\t          \"Discounts\": [\r\n\t            {\r\n\t              \"Amount\": 0.00,\r\n\t              \"Code\": \"string\",\r\n\t              \"Type\": \"string\"\r\n\t            }\r\n\t          ],\r\n\t          \"ProductKey\": \"Product123\",\r\n\t          \"ProductQuantity\": 2,\r\n\t          \"ProductUnitPrice\": 19.99,\r\n\t          \"CITB\": \"CITB\"\r\n\t        }],\r\n\t        \"ShipmentKey\": \"1\",\r\n\t        \"ShipmentStatus\": \"SentToFulfillment\",\r\n\t        \"Shipping\": 0.00,\r\n\t        \"ShippingService\": \"FEX\",\r\n\t        \"ShipTo\": {\r\n\t           \"DateOfBirth\": \"1970-01-01T00:00:00Z\",\r\n\t            \"City\": \"Boulder\",\r\n\t            \"Company\": \"Sovos\",\r\n\t            \"Country\": \"US\",\r\n\t            \"County\": \"Boulder\",\r\n\t            \"Email\": \"example@sovos.com\",\r\n\t            \"Fax\": \"555-555-5555\",\r\n\t            \"FirstName\": \"Test First Name\",\r\n\t            \"LastName\": \"Test Last Name\",\r\n\t            \"Phone\": \"555-555-5555\",\r\n\t            \"State\": \"CO\",\r\n\t            \"Street1\": \"2465 Central Ave\",\r\n\t            \"Street2\": \"Ste 110\",\r\n\t            \"Zip1\": \"80301\",\r\n\t            \"Zip2\": \"5728\"\r\n\t         },\r\n\t        \"SpecialInstructions\": \"Test Instructions\"\r\n\t      }\r\n\t    ],\r\n\t    \"Tags\": [\r\n\t      {\r\n\t        \"Name\": \"Test Tag\"\r\n\t      }\r\n\t    ]\r\n\t  },\r\n\t  \"AddressOption\": {\r\n\t    \"IgnoreStreetLevelErrors\": false,\r\n\t    \"RejectIfAddressSuggested\": true\r\n\t  }\r\n\t}\n\nSends a `POST` request to `/api/v1/salesOrders/quote/compliance`\n\nArguments:\n- `body`: The sales order to Quote\n"]    pub async fn post_sales_orders_quote_compliance < 'a > (& 'a self , body : & 'a types :: ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteSalesOrderInput) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesQuoteComplianceResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone > , >{
        let url = format!("{}/api/v1/salesOrders/quote/compliance", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns the tax due for a sales order without saving results for future use\n\nSample request:\r\n            \r\n\tPOST /api/v1/salesOrders/quote/sales-tax\r\n\t{\r\n\t  \"ShipToAddress\": {\r\n\t\t\"City\":  \"Boulder\",\r\n\t    \"State\": \"CO\",\r\n\t    \"Street1\": \"2465 Central Ave\",\r\n\t    \"Street2\": \"Ste 110\",\r\n\t    \"Zip1\": \"80301\",\r\n\t    \"Zip2\": \"5728\"\r\n\t  },\r\n\t  \"EffectiveDate\": \"2020-11-01T00:00:00Z\",\r\n\t  \"TaxSaleType\": \"onsite\",\r\n\t  \"ShippingAndHandlingCollected\": 0.00,\r\n\t  \"OrderItems\": [\r\n\t    {\r\n\t      \"BrandKey\": \"Brand123\",\r\n\t      \"ProductKey\": \"Product123\",\r\n\t      \"ProductQuantity\": 2,\r\n\t      \"ProductUnitPrice\": 19.99\r\n\t    }\r\n\t  ]\r\n\t}\n\nSends a `POST` request to `/api/v1/salesOrders/quote/sales-tax`\n\nArguments:\n- `body`: The sales order to Quote the tax\n"]    pub async fn post_sales_orders_quote_sales_tax < 'a > (& 'a self , body : & 'a types :: ShipCompliantRestApiDomainsSalesOrderEntitiesQuoteTaxForSalesOrder) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesQuoteTaxSalesOrderResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone > , >{
        let url = format!("{}/api/v1/salesOrders/quote/sales-tax", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Returns the tax rate for a sales order based on the Address\n\nSample request:\r\n\r\n\tPOST /api/v1/salesOrders/quote/sales-tax-rate\r\n\t{\r\n\t  \"ShipToAddress\": {\r\n\t\t\"City\":  \"Boulder\",\r\n\t    \"State\": \"CO\",\r\n\t    \"Street1\": \"2465 Central Ave\",\r\n\t    \"Street2\": \"Ste 110\",\r\n\t    \"Zip1\": \"80301\",\r\n\t    \"Zip2\": \"5728\"\r\n\t  },\r\n\t  \"EffectiveDate\": \"2020-11-01T00:00:00Z\",\r\n\t  \"TaxSaleType\": \"onsite\",\r\n\t  \"LicenseRelationship\" :\"\"\r\n\t}\n\nSends a `POST` request to `/api/v1/salesOrders/quote/sales-tax-rate`\n\nArguments:\n- `body`: \n"]    pub async fn post_sales_orders_quote_sales_tax_rate < 'a > (& 'a self , body : & 'a types :: ShipCompliantRestApiDomainsSalesOrderEntitiesSalesTaxRatesByAddress) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesTaxRateSalesOrderByAddressResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderAddressValidationResponseOrNone > , >{
        let url = format!("{}/api/v1/salesOrders/quote/sales-tax-rate", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Adds a new Hold Location (previously PickUp location) to a shipment in a sales order\n\nSample request:\r\n\r\n    POST /api/v1/salesOrders/ORDER123/hold-location\r\n    {\r\n        \"shipmentKey\": \"Ship1\",\r\n        \"address\": {\r\n            \"city\": \"Boulder\",\r\n            \"company\": \"Sovos\",\r\n            \"country\": \"US\",\r\n            \"county\": \"Boulder\",\r\n            \"email\": \"example@sovos.com\",\r\n            \"fax\": \"555-555-5555\",\r\n            \"firstName\": \"Example First Name\",\r\n            \"lastName\": \"Example Last Name\",\r\n            \"phone\": \"555-555-5555\",\r\n            \"state\": \"CO\",\r\n            \"street1\": \"2465 Central Ave\",\r\n            \"street2\": \"Ste 110\",\r\n            \"zip1\": \"80301\",\r\n            \"zip2\": \"5728\"\r\n        }\r\n    }\n\nSends a `POST` request to `/api/v1/salesOrders/{salesOrderKey}/hold-location`\n\nArguments:\n- `sales_order_key`: Mandatory: Sales order identifier.\n- `body`: New hold location details.\n"]    pub async fn post_sales_orders_sales_order_key_hold_location < 'a > (& 'a self , sales_order_key : & 'a str , body : & 'a types :: ShipCompliantRestApiDomainsHoldLocationEntitiesAddUpdateHoldLocation) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostHoldLocationResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponseOrNone > , >{
        let url = format!(
            "{}/api/v1/salesOrders/{}/hold-location",
            self.baseurl,
            encode_path(&sales_order_key.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Deletes all hold locations from a Sales Order\n\nSample request:\r\n            \r\n\tDELETE /api/v1/salesOrders/ORDER123/hold-location\n\nSends a `DELETE` request to `/api/v1/salesOrders/{salesOrderKey}/hold-location`\n\nArguments:\n- `sales_order_key`: Mandatory: Sales order identifier.\n"]    pub async fn delete_sales_orders_sales_order_key_hold_location < 'a > (& 'a self , sales_order_key : & 'a str) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteHoldLocationResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponseOrNone > , >{
        let url = format!(
            "{}/api/v1/salesOrders/{}/hold-location",
            self.baseurl,
            encode_path(&sales_order_key.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Deletes one occurrence of a hold location from a Sales Order by shipment key\n\nSample request:\r\n            \r\n\tDELETE /api/v1/salesOrders/ORDER123/shipment/SHIP1/hold-location\n\nSends a `DELETE` request to `/api/v1/salesOrders/{salesOrderKey}/shipment/{shipmentKey}/hold-location`\n\nArguments:\n- `sales_order_key`: Mandatory: Sales order identifier.\n- `shipment_key`: Mandatory: shipment identifier.\n"]    pub async fn delete_sales_orders_sales_order_key_shipment_shipment_key_hold_location < 'a > (& 'a self , sales_order_key : & 'a str , shipment_key : & 'a str) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesDeleteHoldLocationResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationBadRequestResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesHoldLocationNotFoundResponseOrNone > , >{
        let url = format!(
            "{}/api/v1/salesOrders/{}/shipment/{}/hold-location",
            self.baseurl,
            encode_path(&sales_order_key.to_string()),
            encode_path(&shipment_key.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    #[doc = "Commits a new Wholesale Invoice to the system\n\nSample request:\r\n            \r\n\tPOST /api/v1/wholesaleInvoices\r\n\t{  \r\n\t    \"PersistOption\": \"OverrideExisting\",\r\n\t    \"Order\": {\r\n\t        \"BillingCustomerKey\": \"Billing123\",\r\n\t        \"ExternalSalesOrderKey\": \"External123\",\r\n\t        \"OrderNumber\": \"Order123\",\r\n\t        \"OrderReserved\": \"Order123\",\r\n\t        \"ReferenceNumber\": \"Reference123\",\r\n\t        \"RefundedOrderReference\": \"RefundedOrder123\",\r\n\t        \"Shipments\": [\r\n\t        {\r\n\t            \"DefaultWarehouse\": \"WineShipping\",\r\n\t            \"Freight\": 0.00,\r\n\t            \"ShipDate\": \"2020-11-01T00:00:00Z\",\r\n\t            \"ShipmentItems\": [\r\n\t            {\r\n\t                \"BrandKey\": \"Brand123\",\r\n\t                \"FobPointKey\": \"FOB\",\r\n\t                \"ItemReserved\": \"ItemReserved\",\r\n\t                \"ProductKey\": \"Product123\",\r\n\t                \"Quantity\": 1,\r\n\t                \"QuantityType\": \"CASE\",\r\n\t                \"UnitPrice\": 19.99\r\n\t            }\r\n\t            ],\r\n\t            \"ShipmentKey\": \"1\",\r\n\t            \"ShipmentReserved\": \"ShipmentReserved\",\r\n\t            \"ShippingCustomerKey\": \"Shipping123\",\r\n\t            \"ShippingService\": \"FEX\"\r\n\t        }\r\n\t        ],\r\n\t        \"Tags\": [\r\n\t        {\r\n\t            \"Name\": \"Test Tag\"\r\n\t        }\r\n\t        ],\r\n\t        \"TransactionDate\": \"2020-11-01T00:00:00Z\",\r\n\t        \"TaxCollected\": 0.00\r\n\t    }\r\n\t}\n\nSends a `POST` request to `/api/v1/wholesaleInvoices`\n\nArguments:\n- `body`: The new order\n"]    pub async fn post_wholesale_invoices < 'a > (& 'a self , body : & 'a types :: ShipCompliantRestApiDomainsWholesaleEntitiesPostInvoiceInput) -> Result < ResponseValue < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesPostWholesaleInvoiceResponse > , Error < types :: ShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderNotFoundResponseOrShipCompliantRestApiMiddlewaresSwaggerResponseTypesSalesOrderBadRequestResponseOrNone > , >{
        let url = format!("{}/api/v1/wholesaleInvoices", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

#[doc = r" Items consumers will typically use such as the Client."]
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
