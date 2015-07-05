/// SQLSTATE error codes
#[derive(PartialEq, Eq, Clone)]
pub enum SqlState {
    /// `00000`
    SuccessfulCompletion,
    /// `01000`
    Warning,
    /// `0100C`
    DynamicResultSetsReturned,
    /// `01008`
    ImplicitZeroBitPadding,
    /// `01003`
    NullValueEliminatedInSetFunction,
    /// `01007`
    PrivilegeNotGranted,
    /// `01006`
    PrivilegeNotRevoked,
    /// `01004`
    StringDataRightTruncationWarning,
    /// `01P01`
    DeprecatedFeature,
    /// `02000`
    NoData,
    /// `02001`
    NoAdditionalDynamicResultSetsReturned,
    /// `03000`
    SqlStatementNotYetComplete,
    /// `08000`
    ConnectionException,
    /// `08003`
    ConnectionDoesNotExist,
    /// `08006`
    ConnectionFailure,
    /// `08001`
    SqlclientUnableToEstablishSqlconnection,
    /// `08004`
    SqlserverRejectedEstablishmentOfSqlconnection,
    /// `08007`
    TransactionResolutionUnknown,
    /// `08P01`
    ProtocolViolation,
    /// `09000`
    TriggeredActionException,
    /// `0A000`
    FeatureNotSupported,
    /// `0B000`
    InvalidTransactionInitiation,
    /// `0F000`
    LocatorException,
    /// `0F001`
    InvalidLocatorException,
    /// `0L000`
    InvalidGrantor,
    /// `0LP01`
    InvalidGrantOperation,
    /// `0P000`
    InvalidRoleSpecification,
    /// `0Z000`
    DiagnosticsException,
    /// `0Z002`
    StackedDiagnosticsAccessedWithoutActiveHandler,
    /// `20000`
    CaseNotFound,
    /// `21000`
    CardinalityViolation,
    /// `22000`
    DataException,
    /// `2202E`
    ArraySubscriptError,
    /// `22021`
    CharacterNotInRepertoire,
    /// `22008`
    DatetimeFieldOverflow,
    /// `22012`
    DivisionByZero,
    /// `22005`
    ErrorInAssignment,
    /// `2200B`
    EscapeCharacterConflict,
    /// `22022`
    IndicatorOverflow,
    /// `22015`
    IntervalFieldOverflow,
    /// `2201E`
    InvalidArgumentForLogarithm,
    /// `22014`
    InvalidArgumentForNtileFunction,
    /// `22016`
    InvalidArgumentForNthValueFunction,
    /// `2201F`
    InvalidArgumentForPowerFunction,
    /// `2201G`
    InvalidArgumentForWidthBucketFunction,
    /// `22018`
    InvalidCharacterValueForCast,
    /// `22007`
    InvalidDatetimeFormat,
    /// `22019`
    InvalidEscapeCharacter,
    /// `2200D`
    InvalidEscapeOctet,
    /// `22025`
    InvalidEscapeSequence,
    /// `22P06`
    NonstandardUseOfEscapeCharacter,
    /// `22010`
    InvalidIndicatorParameterValue,
    /// `22023`
    InvalidParameterValue,
    /// `2201B`
    InvalidRegularExpression,
    /// `2201W`
    InvalidRowCountInLimitClause,
    /// `2201X`
    InvalidRowCountInResultOffsetClause,
    /// `22009`
    InvalidTimeZoneDisplacementValue,
    /// `2200C`
    InvalidUseOfEscapeCharacter,
    /// `2200G`
    MostSpecificTypeMismatch,
    /// `22004`
    NullValueNotAllowedData,
    /// `22002`
    NullValueNoIndicatorParameter,
    /// `22003`
    NumericValueOutOfRange,
    /// `22026`
    StringDataLengthMismatch,
    /// `22001`
    StringDataRightTruncationException,
    /// `22011`
    SubstringError,
    /// `22027`
    TrimError,
    /// `22024`
    UnterminatedCString,
    /// `2200F`
    ZeroLengthCharacterString,
    /// `22P01`
    FloatingPointException,
    /// `22P02`
    InvalidTextRepresentation,
    /// `22P03`
    InvalidBinaryRepresentation,
    /// `22P04`
    BadCopyFileFormat,
    /// `22P05`
    UntranslatableCharacter,
    /// `2200L`
    NotAnXmlDocument,
    /// `2200M`
    InvalidXmlDocument,
    /// `2200N`
    InvalidXmlContent,
    /// `2200S`
    InvalidXmlComment,
    /// `2200T`
    InvalidXmlProcessingInstruction,
    /// `23000`
    IntegrityConstraintViolation,
    /// `23001`
    RestrictViolation,
    /// `23502`
    NotNullViolation,
    /// `23503`
    ForeignKeyViolation,
    /// `23505`
    UniqueViolation,
    /// `23514`
    CheckViolation,
    /// `32P01`
    ExclusionViolation,
    /// `24000`
    InvalidCursorState,
    /// `25000`
    InvalidTransactionState,
    /// `25001`
    ActiveSqlTransaction,
    /// `25002`
    BranchTransactionAlreadyActive,
    /// `25008`
    HeldCursorRequiresSameIsolationLevel,
    /// `25003`
    InappropriateAccessModeForBranchTransaction,
    /// `25004`
    InappropriateIsolationLevelForBranchTransaction,
    /// `25005`
    NoActiveSqlTransactionForBranchTransaction,
    /// `25006`
    ReadOnlySqlTransaction,
    /// `25007`
    SchemaAndDataStatementMixingNotSupported,
    /// `25P01`
    NoActiveSqlTransaction,
    /// `25P02`
    InFailedSqlTransaction,
    /// `26000`
    InvalidSqlStatementName,
    /// `27000`
    TriggeredDataChangeViolation,
    /// `28000`
    InvalidAuthorizationSpecification,
    /// `28P01`
    InvalidPassword,
    /// `2B000`
    DependentPrivilegeDescriptorsStillExist,
    /// `2BP01`
    DependentObjectsStillExist,
    /// `2D000`
    InvalidTransactionTermination,
    /// `2F000`
    SqlRoutineException,
    /// `2F005`
    FunctionExecutedNoReturnStatement,
    /// `2F002`
    ModifyingSqlDataNotPermittedSqlRoutine,
    /// `2F003`
    ProhibitedSqlStatementAttemptedSqlRoutine,
    /// `2F004`
    ReadingSqlDataNotPermittedSqlRoutine,
    /// `34000`
    InvalidCursorName,
    /// `38000`
    ExternalRoutineException,
    /// `38001`
    ContainingSqlNotPermitted,
    /// `38002`
    ModifyingSqlDataNotPermittedExternalRoutine,
    /// `38003`
    ProhibitedSqlStatementAttemptedExternalRoutine,
    /// `38004`
    ReadingSqlDataNotPermittedExternalRoutine,
    /// `39000`
    ExternalRoutineInvocationException,
    /// `39001`
    InvalidSqlstateReturned,
    /// `39004`
    NullValueNotAllowedExternalRoutine,
    /// `39P01`
    TriggerProtocolViolated,
    /// `39P02`
    SrfProtocolViolated,
    /// `3B000`
    SavepointException,
    /// `3B001`
    InvalidSavepointException,
    /// `3D000`
    InvalidCatalogName,
    /// `3F000`
    InvalidSchemaName,
    /// `40000`
    TransactionRollback,
    /// `40002`
    TransactionIntegrityConstraintViolation,
    /// `40001`
    SerializationFailure,
    /// `40003`
    StatementCompletionUnknown,
    /// `40P01`
    DeadlockDetected,
    /// `42000`
    SyntaxErrorOrAccessRuleViolation,
    /// `42601`
    SyntaxError,
    /// `42501`
    InsufficientPrivilege,
    /// `42846`
    CannotCoerce,
    /// `42803`
    GroupingError,
    /// `42P20`
    WindowingError,
    /// `42P19`
    InvalidRecursion,
    /// `42830`
    InvalidForeignKey,
    /// `42602`
    InvalidName,
    /// `42622`
    NameTooLong,
    /// `42939`
    ReservedName,
    /// `42804`
    DatatypeMismatch,
    /// `42P18`
    IndeterminateDatatype,
    /// `42P21`
    CollationMismatch,
    /// `42P22`
    IndeterminateCollation,
    /// `42809`
    WrongObjectType,
    /// `42703`
    UndefinedColumn,
    /// `42883`
    UndefinedFunction,
    /// `42P01`
    UndefinedTable,
    /// `42P02`
    UndefinedParameter,
    /// `42704`
    UndefinedObject,
    /// `42701`
    DuplicateColumn,
    /// `42P03`
    DuplicateCursor,
    /// `42P04`
    DuplicateDatabase,
    /// `42723`
    DuplicateFunction,
    /// `42P05`
    DuplicatePreparedStatement,
    /// `42P06`
    DuplicateSchema,
    /// `42P07`
    DuplicateTable,
    /// `42712`
    DuplicateAliaas,
    /// `42710`
    DuplicateObject,
    /// `42702`
    AmbiguousColumn,
    /// `42725`
    AmbiguousFunction,
    /// `42P08`
    AmbiguousParameter,
    /// `42P09`
    AmbiguousAlias,
    /// `42P10`
    InvalidColumnReference,
    /// `42611`
    InvalidColumnDefinition,
    /// `42P11`
    InvalidCursorDefinition,
    /// `42P12`
    InvalidDatabaseDefinition,
    /// `42P13`
    InvalidFunctionDefinition,
    /// `42P14`
    InvalidPreparedStatementDefinition,
    /// `42P15`
    InvalidSchemaDefinition,
    /// `42P16`
    InvalidTableDefinition,
    /// `42P17`
    InvalidObjectDefinition,
    /// `44000`
    WithCheckOptionViolation,
    /// `53000`
    InsufficientResources,
    /// `53100`
    DiskFull,
    /// `53200`
    OutOfMemory,
    /// `53300`
    TooManyConnections,
    /// `53400`
    ConfigurationLimitExceeded,
    /// `54000`
    ProgramLimitExceeded,
    /// `54001`
    StatementTooComplex,
    /// `54011`
    TooManyColumns,
    /// `54023`
    TooManyArguments,
    /// `55000`
    ObjectNotInPrerequisiteState,
    /// `55006`
    ObjectInUse,
    /// `55P02`
    CantChangeRuntimeParam,
    /// `55P03`
    LockNotAvailable,
    /// `57000`
    OperatorIntervention,
    /// `57014`
    QueryCanceled,
    /// `57P01`
    AdminShutdown,
    /// `57P02`
    CrashShutdown,
    /// `57P03`
    CannotConnectNow,
    /// `57P04`
    DatabaseDropped,
    /// `58000`
    SystemError,
    /// `58030`
    IoError,
    /// `58P01`
    UndefinedFile,
    /// `58P02`
    DuplicateFile,
    /// `F0000`
    ConfigFileError,
    /// `F0001`
    LockFileExists,
    /// `HV000`
    FdwError,
    /// `HV005`
    FdwColumnNameNotFound,
    /// `HV002`
    FdwDynamicParameterValueNeeded,
    /// `HV010`
    FdwFunctionSequenceError,
    /// `HV021`
    FdwInconsistentDescriptorInformation,
    /// `HV024`
    FdwInvalidAttributeValue,
    /// `HV007`
    FdwInvalidColumnName,
    /// `HV008`
    FdwInvalidColumnNumber,
    /// `HV004`
    FdwInvalidDataType,
    /// `HV006`
    FdwInvalidDataTypeDescriptors,
    /// `HV091`
    FdwInvalidDescriptorFieldIdentifier,
    /// `HV00B`
    FdwInvalidHandle,
    /// `HV00C`
    FdwInvalidOptionIndex,
    /// `HV00D`
    FdwInvalidOptionName,
    /// `HV090`
    FdwInvalidStringLengthOrBufferLength,
    /// `HV00A`
    FdwInvalidStringFormat,
    /// `HV009`
    FdwInvalidUseOfNullPointer,
    /// `HV014`
    FdwTooManyHandles,
    /// `HV001`
    FdwOutOfMemory,
    /// `HV00P`
    FdwNoSchemas,
    /// `HV00J`
    FdwOptionNameNotFound,
    /// `HV00K`
    FdwReplyHandle,
    /// `HV00Q`
    FdwSchemaNotFound,
    /// `HV00R`
    FdwTableNotFound,
    /// `HV00L`
    FdwUnableToCreateExcecution,
    /// `HV00M`
    FdwUnableToCreateReply,
    /// `HV00N`
    FdwUnableToEstablishConnection,
    /// `P0000`
    PlpgsqlError,
    /// `P0001`
    RaiseException,
    /// `P0002`
    NoDataFound,
    /// `P0003`
    TooManyRows,
    /// `XX000`
    InternalError,
    /// `XX001`
    DataCorrupted,
    /// `XX002`
    IndexCorrupted,
    /// An unknown code
    Unknown(String)
}
static SQLSTATE_MAP: phf::Map<&'static str, SqlState> = ::phf::Map {
    key: 1897749892740154578,
    disps: &[
        (22, 137),
        (1, 167),
        (0, 0),
        (1, 1),
        (1, 0),
        (0, 37),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 4),
        (0, 0),
        (0, 6),
        (0, 8),
        (0, 23),
        (0, 149),
        (0, 174),
        (1, 7),
        (0, 32),
        (0, 1),
        (6, 144),
        (1, 15),
        (1, 23),
        (7, 60),
        (0, 180),
        (17, 143),
        (0, 26),
        (9, 85),
        (0, 78),
        (11, 10),
        (0, 105),
        (0, 4),
        (0, 34),
        (3, 139),
        (0, 5),
        (3, 29),
        (0, 170),
        (0, 35),
        (0, 12),
        (41, 103),
        (2, 31),
        (31, 43),
        (0, 62),
        (0, 139),
        (0, 7),
        (0, 220),
        (17, 51),
        (17, 81),
    ],
    entries: &[
        ("53100", SqlState::DiskFull),
        ("25006", SqlState::ReadOnlySqlTransaction),
        ("22024", SqlState::UnterminatedCString),
        ("02000", SqlState::NoData),
        ("2201E", SqlState::InvalidArgumentForLogarithm),
        ("2BP01", SqlState::DependentObjectsStillExist),
        ("25008", SqlState::HeldCursorRequiresSameIsolationLevel),
        ("42P03", SqlState::DuplicateCursor),
        ("0B000", SqlState::InvalidTransactionInitiation),
        ("42P21", SqlState::CollationMismatch),
        ("53300", SqlState::TooManyConnections),
        ("42P01", SqlState::UndefinedTable),
        ("HV007", SqlState::FdwInvalidColumnName),
        ("22015", SqlState::IntervalFieldOverflow),
        ("22016", SqlState::InvalidArgumentForNthValueFunction),
        ("22023", SqlState::InvalidParameterValue),
        ("42P02", SqlState::UndefinedParameter),
        ("25004", SqlState::InappropriateIsolationLevelForBranchTransaction),
        ("42809", SqlState::WrongObjectType),
        ("57P01", SqlState::AdminShutdown),
        ("2201G", SqlState::InvalidArgumentForWidthBucketFunction),
        ("39P02", SqlState::SrfProtocolViolated),
        ("44000", SqlState::WithCheckOptionViolation),
        ("XX001", SqlState::DataCorrupted),
        ("22018", SqlState::InvalidCharacterValueForCast),
        ("27000", SqlState::TriggeredDataChangeViolation),
        ("22003", SqlState::NumericValueOutOfRange),
        ("57P02", SqlState::CrashShutdown),
        ("P0002", SqlState::NoDataFound),
        ("HV004", SqlState::FdwInvalidDataType),
        ("22019", SqlState::InvalidEscapeCharacter),
        ("22P05", SqlState::UntranslatableCharacter),
        ("20000", SqlState::CaseNotFound),
        ("42622", SqlState::NameTooLong),
        ("HV001", SqlState::FdwOutOfMemory),
        ("F0000", SqlState::ConfigFileError),
        ("57P03", SqlState::CannotConnectNow),
        ("0100C", SqlState::DynamicResultSetsReturned),
        ("23502", SqlState::NotNullViolation),
        ("2201F", SqlState::InvalidArgumentForPowerFunction),
        ("25003", SqlState::InappropriateAccessModeForBranchTransaction),
        ("F0001", SqlState::LockFileExists),
        ("53000", SqlState::InsufficientResources),
        ("32P01", SqlState::ExclusionViolation),
        ("P0003", SqlState::TooManyRows),
        ("0F000", SqlState::LocatorException),
        ("HV00B", SqlState::FdwInvalidHandle),
        ("0P000", SqlState::InvalidRoleSpecification),
        ("2200B", SqlState::EscapeCharacterConflict),
        ("39000", SqlState::ExternalRoutineInvocationException),
        ("0Z000", SqlState::DiagnosticsException),
        ("25007", SqlState::SchemaAndDataStatementMixingNotSupported),
        ("HV00A", SqlState::FdwInvalidStringFormat),
        ("01P01", SqlState::DeprecatedFeature),
        ("22002", SqlState::NullValueNoIndicatorParameter),
        ("23514", SqlState::CheckViolation),
        ("HV005", SqlState::FdwColumnNameNotFound),
        ("58030", SqlState::IoError),
        ("42702", SqlState::AmbiguousColumn),
        ("22007", SqlState::InvalidDatetimeFormat),
        ("2200N", SqlState::InvalidXmlContent),
        ("42000", SqlState::SyntaxErrorOrAccessRuleViolation),
        ("2200C", SqlState::InvalidUseOfEscapeCharacter),
        ("39P01", SqlState::TriggerProtocolViolated),
        ("HV00N", SqlState::FdwUnableToEstablishConnection),
        ("38004", SqlState::ReadingSqlDataNotPermittedExternalRoutine),
        ("22004", SqlState::NullValueNotAllowedData),
        ("22012", SqlState::DivisionByZero),
        ("2F000", SqlState::SqlRoutineException),
        ("42P11", SqlState::InvalidCursorDefinition),
        ("3F000", SqlState::InvalidSchemaName),
        ("54000", SqlState::ProgramLimitExceeded),
        ("38002", SqlState::ModifyingSqlDataNotPermittedExternalRoutine),
        ("42725", SqlState::AmbiguousFunction),
        ("0A000", SqlState::FeatureNotSupported),
        ("42501", SqlState::InsufficientPrivilege),
        ("08003", SqlState::ConnectionDoesNotExist),
        ("3D000", SqlState::InvalidCatalogName),
        ("22P02", SqlState::InvalidTextRepresentation),
        ("42P09", SqlState::AmbiguousAlias),
        ("HV00C", SqlState::FdwInvalidOptionIndex),
        ("42703", SqlState::UndefinedColumn),
        ("21000", SqlState::CardinalityViolation),
        ("2201X", SqlState::InvalidRowCountInResultOffsetClause),
        ("22008", SqlState::DatetimeFieldOverflow),
        ("HV024", SqlState::FdwInvalidAttributeValue),
        ("01000", SqlState::Warning),
        ("42846", SqlState::CannotCoerce),
        ("25P01", SqlState::NoActiveSqlTransaction),
        ("08004", SqlState::SqlserverRejectedEstablishmentOfSqlconnection),
        ("23503", SqlState::ForeignKeyViolation),
        ("08000", SqlState::ConnectionException),
        ("2200M", SqlState::InvalidXmlDocument),
        ("38000", SqlState::ExternalRoutineException),
        ("HV00K", SqlState::FdwReplyHandle),
        ("40002", SqlState::TransactionIntegrityConstraintViolation),
        ("08006", SqlState::ConnectionFailure),
        ("39001", SqlState::InvalidSqlstateReturned),
        ("57P04", SqlState::DatabaseDropped),
        ("42804", SqlState::DatatypeMismatch),
        ("HV091", SqlState::FdwInvalidDescriptorFieldIdentifier),
        ("02001", SqlState::NoAdditionalDynamicResultSetsReturned),
        ("22P03", SqlState::InvalidBinaryRepresentation),
        ("0LP01", SqlState::InvalidGrantOperation),
        ("22P06", SqlState::NonstandardUseOfEscapeCharacter),
        ("3B000", SqlState::SavepointException),
        ("P0000", SqlState::PlpgsqlError),
        ("25001", SqlState::ActiveSqlTransaction),
        ("42939", SqlState::ReservedName),
        ("2200G", SqlState::MostSpecificTypeMismatch),
        ("42710", SqlState::DuplicateObject),
        ("42602", SqlState::InvalidName),
        ("HV00M", SqlState::FdwUnableToCreateReply),
        ("22010", SqlState::InvalidIndicatorParameterValue),
        ("2200T", SqlState::InvalidXmlProcessingInstruction),
        ("08P01", SqlState::ProtocolViolation),
        ("HV00J", SqlState::FdwOptionNameNotFound),
        ("42P06", SqlState::DuplicateSchema),
        ("HV00D", SqlState::FdwInvalidOptionName),
        ("22014", SqlState::InvalidArgumentForNtileFunction),
        ("2B000", SqlState::DependentPrivilegeDescriptorsStillExist),
        ("42P07", SqlState::DuplicateTable),
        ("03000", SqlState::SqlStatementNotYetComplete),
        ("2200F", SqlState::ZeroLengthCharacterString),
        ("P0001", SqlState::RaiseException),
        ("58000", SqlState::SystemError),
        ("08007", SqlState::TransactionResolutionUnknown),
        ("42P17", SqlState::InvalidObjectDefinition),
        ("58P01", SqlState::UndefinedFile),
        ("2201B", SqlState::InvalidRegularExpression),
        ("23505", SqlState::UniqueViolation),
        ("HV021", SqlState::FdwInconsistentDescriptorInformation),
        ("0F001", SqlState::InvalidLocatorException),
        ("54001", SqlState::StatementTooComplex),
        ("55P02", SqlState::CantChangeRuntimeParam),
        ("22027", SqlState::TrimError),
        ("42P16", SqlState::InvalidTableDefinition),
        ("42P14", SqlState::InvalidPreparedStatementDefinition),
        ("23001", SqlState::RestrictViolation),
        ("2F004", SqlState::ReadingSqlDataNotPermittedSqlRoutine),
        ("42704", SqlState::UndefinedObject),
        ("2200S", SqlState::InvalidXmlComment),
        ("42611", SqlState::InvalidColumnDefinition),
        ("54023", SqlState::TooManyArguments),
        ("39004", SqlState::NullValueNotAllowedExternalRoutine),
        ("57014", SqlState::QueryCanceled),
        ("01003", SqlState::NullValueEliminatedInSetFunction),
        ("53400", SqlState::ConfigurationLimitExceeded),
        ("HV00L", SqlState::FdwUnableToCreateExcecution),
        ("00000", SqlState::SuccessfulCompletion),
        ("42P19", SqlState::InvalidRecursion),
        ("0L000", SqlState::InvalidGrantor),
        ("2F002", SqlState::ModifyingSqlDataNotPermittedSqlRoutine),
        ("HV010", SqlState::FdwFunctionSequenceError),
        ("3B001", SqlState::InvalidSavepointException),
        ("22025", SqlState::InvalidEscapeSequence),
        ("42P13", SqlState::InvalidFunctionDefinition),
        ("22000", SqlState::DataException),
        ("40P01", SqlState::DeadlockDetected),
        ("55006", SqlState::ObjectInUse),
        ("42P05", SqlState::DuplicatePreparedStatement),
        ("2F003", SqlState::ProhibitedSqlStatementAttemptedSqlRoutine),
        ("HV00Q", SqlState::FdwSchemaNotFound),
        ("22011", SqlState::SubstringError),
        ("HV014", SqlState::FdwTooManyHandles),
        ("38001", SqlState::ContainingSqlNotPermitted),
        ("34000", SqlState::InvalidCursorName),
        ("22009", SqlState::InvalidTimeZoneDisplacementValue),
        ("42P15", SqlState::InvalidSchemaDefinition),
        ("XX002", SqlState::IndexCorrupted),
        ("01004", SqlState::StringDataRightTruncationWarning),
        ("42P04", SqlState::DuplicateDatabase),
        ("HV00R", SqlState::FdwTableNotFound),
        ("22026", SqlState::StringDataLengthMismatch),
        ("55000", SqlState::ObjectNotInPrerequisiteState),
        ("22005", SqlState::ErrorInAssignment),
        ("42723", SqlState::DuplicateFunction),
        ("09000", SqlState::TriggeredActionException),
        ("40003", SqlState::StatementCompletionUnknown),
        ("22022", SqlState::IndicatorOverflow),
        ("XX000", SqlState::InternalError),
        ("54011", SqlState::TooManyColumns),
        ("42701", SqlState::DuplicateColumn),
        ("2201W", SqlState::InvalidRowCountInLimitClause),
        ("42P10", SqlState::InvalidColumnReference),
        ("42P22", SqlState::IndeterminateCollation),
        ("25005", SqlState::NoActiveSqlTransactionForBranchTransaction),
        ("42P18", SqlState::IndeterminateDatatype),
        ("26000", SqlState::InvalidSqlStatementName),
        ("25002", SqlState::BranchTransactionAlreadyActive),
        ("40000", SqlState::TransactionRollback),
        ("42P12", SqlState::InvalidDatabaseDefinition),
        ("40001", SqlState::SerializationFailure),
        ("01007", SqlState::PrivilegeNotGranted),
        ("42P20", SqlState::WindowingError),
        ("22021", SqlState::CharacterNotInRepertoire),
        ("2F005", SqlState::FunctionExecutedNoReturnStatement),
        ("HV002", SqlState::FdwDynamicParameterValueNeeded),
        ("2D000", SqlState::InvalidTransactionTermination),
        ("25P02", SqlState::InFailedSqlTransaction),
        ("22001", SqlState::StringDataRightTruncationException),
        ("53200", SqlState::OutOfMemory),
        ("22P04", SqlState::BadCopyFileFormat),
        ("42830", SqlState::InvalidForeignKey),
        ("2202E", SqlState::ArraySubscriptError),
        ("57000", SqlState::OperatorIntervention),
        ("28000", SqlState::InvalidAuthorizationSpecification),
        ("28P01", SqlState::InvalidPassword),
        ("01006", SqlState::PrivilegeNotRevoked),
        ("24000", SqlState::InvalidCursorState),
        ("HV008", SqlState::FdwInvalidColumnNumber),
        ("01008", SqlState::ImplicitZeroBitPadding),
        ("42803", SqlState::GroupingError),
        ("42712", SqlState::DuplicateAliaas),
        ("HV006", SqlState::FdwInvalidDataTypeDescriptors),
        ("0Z002", SqlState::StackedDiagnosticsAccessedWithoutActiveHandler),
        ("23000", SqlState::IntegrityConstraintViolation),
        ("08001", SqlState::SqlclientUnableToEstablishSqlconnection),
        ("38003", SqlState::ProhibitedSqlStatementAttemptedExternalRoutine),
        ("25000", SqlState::InvalidTransactionState),
        ("42P08", SqlState::AmbiguousParameter),
        ("22P01", SqlState::FloatingPointException),
        ("58P02", SqlState::DuplicateFile),
        ("HV090", SqlState::FdwInvalidStringLengthOrBufferLength),
        ("42601", SqlState::SyntaxError),
        ("2200L", SqlState::NotAnXmlDocument),
        ("55P03", SqlState::LockNotAvailable),
        ("HV00P", SqlState::FdwNoSchemas),
        ("42883", SqlState::UndefinedFunction),
        ("2200D", SqlState::InvalidEscapeOctet),
        ("HV000", SqlState::FdwError),
        ("HV009", SqlState::FdwInvalidUseOfNullPointer),
    ]
};

impl SqlState {
    /// Creates a `SqlState` from its error code.
    pub fn from_code(s: String) -> SqlState {
        match SQLSTATE_MAP.get(&*s) {
            Some(state) => state.clone(),
            None => SqlState::Unknown(s)
        }
    }

    /// Returns the error code corresponding to the `SqlState`.
    pub fn code(&self) -> &str {
        match *self {
            SqlState::SuccessfulCompletion => "00000",
            SqlState::Warning => "01000",
            SqlState::DynamicResultSetsReturned => "0100C",
            SqlState::ImplicitZeroBitPadding => "01008",
            SqlState::NullValueEliminatedInSetFunction => "01003",
            SqlState::PrivilegeNotGranted => "01007",
            SqlState::PrivilegeNotRevoked => "01006",
            SqlState::StringDataRightTruncationWarning => "01004",
            SqlState::DeprecatedFeature => "01P01",
            SqlState::NoData => "02000",
            SqlState::NoAdditionalDynamicResultSetsReturned => "02001",
            SqlState::SqlStatementNotYetComplete => "03000",
            SqlState::ConnectionException => "08000",
            SqlState::ConnectionDoesNotExist => "08003",
            SqlState::ConnectionFailure => "08006",
            SqlState::SqlclientUnableToEstablishSqlconnection => "08001",
            SqlState::SqlserverRejectedEstablishmentOfSqlconnection => "08004",
            SqlState::TransactionResolutionUnknown => "08007",
            SqlState::ProtocolViolation => "08P01",
            SqlState::TriggeredActionException => "09000",
            SqlState::FeatureNotSupported => "0A000",
            SqlState::InvalidTransactionInitiation => "0B000",
            SqlState::LocatorException => "0F000",
            SqlState::InvalidLocatorException => "0F001",
            SqlState::InvalidGrantor => "0L000",
            SqlState::InvalidGrantOperation => "0LP01",
            SqlState::InvalidRoleSpecification => "0P000",
            SqlState::DiagnosticsException => "0Z000",
            SqlState::StackedDiagnosticsAccessedWithoutActiveHandler => "0Z002",
            SqlState::CaseNotFound => "20000",
            SqlState::CardinalityViolation => "21000",
            SqlState::DataException => "22000",
            SqlState::ArraySubscriptError => "2202E",
            SqlState::CharacterNotInRepertoire => "22021",
            SqlState::DatetimeFieldOverflow => "22008",
            SqlState::DivisionByZero => "22012",
            SqlState::ErrorInAssignment => "22005",
            SqlState::EscapeCharacterConflict => "2200B",
            SqlState::IndicatorOverflow => "22022",
            SqlState::IntervalFieldOverflow => "22015",
            SqlState::InvalidArgumentForLogarithm => "2201E",
            SqlState::InvalidArgumentForNtileFunction => "22014",
            SqlState::InvalidArgumentForNthValueFunction => "22016",
            SqlState::InvalidArgumentForPowerFunction => "2201F",
            SqlState::InvalidArgumentForWidthBucketFunction => "2201G",
            SqlState::InvalidCharacterValueForCast => "22018",
            SqlState::InvalidDatetimeFormat => "22007",
            SqlState::InvalidEscapeCharacter => "22019",
            SqlState::InvalidEscapeOctet => "2200D",
            SqlState::InvalidEscapeSequence => "22025",
            SqlState::NonstandardUseOfEscapeCharacter => "22P06",
            SqlState::InvalidIndicatorParameterValue => "22010",
            SqlState::InvalidParameterValue => "22023",
            SqlState::InvalidRegularExpression => "2201B",
            SqlState::InvalidRowCountInLimitClause => "2201W",
            SqlState::InvalidRowCountInResultOffsetClause => "2201X",
            SqlState::InvalidTimeZoneDisplacementValue => "22009",
            SqlState::InvalidUseOfEscapeCharacter => "2200C",
            SqlState::MostSpecificTypeMismatch => "2200G",
            SqlState::NullValueNotAllowedData => "22004",
            SqlState::NullValueNoIndicatorParameter => "22002",
            SqlState::NumericValueOutOfRange => "22003",
            SqlState::StringDataLengthMismatch => "22026",
            SqlState::StringDataRightTruncationException => "22001",
            SqlState::SubstringError => "22011",
            SqlState::TrimError => "22027",
            SqlState::UnterminatedCString => "22024",
            SqlState::ZeroLengthCharacterString => "2200F",
            SqlState::FloatingPointException => "22P01",
            SqlState::InvalidTextRepresentation => "22P02",
            SqlState::InvalidBinaryRepresentation => "22P03",
            SqlState::BadCopyFileFormat => "22P04",
            SqlState::UntranslatableCharacter => "22P05",
            SqlState::NotAnXmlDocument => "2200L",
            SqlState::InvalidXmlDocument => "2200M",
            SqlState::InvalidXmlContent => "2200N",
            SqlState::InvalidXmlComment => "2200S",
            SqlState::InvalidXmlProcessingInstruction => "2200T",
            SqlState::IntegrityConstraintViolation => "23000",
            SqlState::RestrictViolation => "23001",
            SqlState::NotNullViolation => "23502",
            SqlState::ForeignKeyViolation => "23503",
            SqlState::UniqueViolation => "23505",
            SqlState::CheckViolation => "23514",
            SqlState::ExclusionViolation => "32P01",
            SqlState::InvalidCursorState => "24000",
            SqlState::InvalidTransactionState => "25000",
            SqlState::ActiveSqlTransaction => "25001",
            SqlState::BranchTransactionAlreadyActive => "25002",
            SqlState::HeldCursorRequiresSameIsolationLevel => "25008",
            SqlState::InappropriateAccessModeForBranchTransaction => "25003",
            SqlState::InappropriateIsolationLevelForBranchTransaction => "25004",
            SqlState::NoActiveSqlTransactionForBranchTransaction => "25005",
            SqlState::ReadOnlySqlTransaction => "25006",
            SqlState::SchemaAndDataStatementMixingNotSupported => "25007",
            SqlState::NoActiveSqlTransaction => "25P01",
            SqlState::InFailedSqlTransaction => "25P02",
            SqlState::InvalidSqlStatementName => "26000",
            SqlState::TriggeredDataChangeViolation => "27000",
            SqlState::InvalidAuthorizationSpecification => "28000",
            SqlState::InvalidPassword => "28P01",
            SqlState::DependentPrivilegeDescriptorsStillExist => "2B000",
            SqlState::DependentObjectsStillExist => "2BP01",
            SqlState::InvalidTransactionTermination => "2D000",
            SqlState::SqlRoutineException => "2F000",
            SqlState::FunctionExecutedNoReturnStatement => "2F005",
            SqlState::ModifyingSqlDataNotPermittedSqlRoutine => "2F002",
            SqlState::ProhibitedSqlStatementAttemptedSqlRoutine => "2F003",
            SqlState::ReadingSqlDataNotPermittedSqlRoutine => "2F004",
            SqlState::InvalidCursorName => "34000",
            SqlState::ExternalRoutineException => "38000",
            SqlState::ContainingSqlNotPermitted => "38001",
            SqlState::ModifyingSqlDataNotPermittedExternalRoutine => "38002",
            SqlState::ProhibitedSqlStatementAttemptedExternalRoutine => "38003",
            SqlState::ReadingSqlDataNotPermittedExternalRoutine => "38004",
            SqlState::ExternalRoutineInvocationException => "39000",
            SqlState::InvalidSqlstateReturned => "39001",
            SqlState::NullValueNotAllowedExternalRoutine => "39004",
            SqlState::TriggerProtocolViolated => "39P01",
            SqlState::SrfProtocolViolated => "39P02",
            SqlState::SavepointException => "3B000",
            SqlState::InvalidSavepointException => "3B001",
            SqlState::InvalidCatalogName => "3D000",
            SqlState::InvalidSchemaName => "3F000",
            SqlState::TransactionRollback => "40000",
            SqlState::TransactionIntegrityConstraintViolation => "40002",
            SqlState::SerializationFailure => "40001",
            SqlState::StatementCompletionUnknown => "40003",
            SqlState::DeadlockDetected => "40P01",
            SqlState::SyntaxErrorOrAccessRuleViolation => "42000",
            SqlState::SyntaxError => "42601",
            SqlState::InsufficientPrivilege => "42501",
            SqlState::CannotCoerce => "42846",
            SqlState::GroupingError => "42803",
            SqlState::WindowingError => "42P20",
            SqlState::InvalidRecursion => "42P19",
            SqlState::InvalidForeignKey => "42830",
            SqlState::InvalidName => "42602",
            SqlState::NameTooLong => "42622",
            SqlState::ReservedName => "42939",
            SqlState::DatatypeMismatch => "42804",
            SqlState::IndeterminateDatatype => "42P18",
            SqlState::CollationMismatch => "42P21",
            SqlState::IndeterminateCollation => "42P22",
            SqlState::WrongObjectType => "42809",
            SqlState::UndefinedColumn => "42703",
            SqlState::UndefinedFunction => "42883",
            SqlState::UndefinedTable => "42P01",
            SqlState::UndefinedParameter => "42P02",
            SqlState::UndefinedObject => "42704",
            SqlState::DuplicateColumn => "42701",
            SqlState::DuplicateCursor => "42P03",
            SqlState::DuplicateDatabase => "42P04",
            SqlState::DuplicateFunction => "42723",
            SqlState::DuplicatePreparedStatement => "42P05",
            SqlState::DuplicateSchema => "42P06",
            SqlState::DuplicateTable => "42P07",
            SqlState::DuplicateAliaas => "42712",
            SqlState::DuplicateObject => "42710",
            SqlState::AmbiguousColumn => "42702",
            SqlState::AmbiguousFunction => "42725",
            SqlState::AmbiguousParameter => "42P08",
            SqlState::AmbiguousAlias => "42P09",
            SqlState::InvalidColumnReference => "42P10",
            SqlState::InvalidColumnDefinition => "42611",
            SqlState::InvalidCursorDefinition => "42P11",
            SqlState::InvalidDatabaseDefinition => "42P12",
            SqlState::InvalidFunctionDefinition => "42P13",
            SqlState::InvalidPreparedStatementDefinition => "42P14",
            SqlState::InvalidSchemaDefinition => "42P15",
            SqlState::InvalidTableDefinition => "42P16",
            SqlState::InvalidObjectDefinition => "42P17",
            SqlState::WithCheckOptionViolation => "44000",
            SqlState::InsufficientResources => "53000",
            SqlState::DiskFull => "53100",
            SqlState::OutOfMemory => "53200",
            SqlState::TooManyConnections => "53300",
            SqlState::ConfigurationLimitExceeded => "53400",
            SqlState::ProgramLimitExceeded => "54000",
            SqlState::StatementTooComplex => "54001",
            SqlState::TooManyColumns => "54011",
            SqlState::TooManyArguments => "54023",
            SqlState::ObjectNotInPrerequisiteState => "55000",
            SqlState::ObjectInUse => "55006",
            SqlState::CantChangeRuntimeParam => "55P02",
            SqlState::LockNotAvailable => "55P03",
            SqlState::OperatorIntervention => "57000",
            SqlState::QueryCanceled => "57014",
            SqlState::AdminShutdown => "57P01",
            SqlState::CrashShutdown => "57P02",
            SqlState::CannotConnectNow => "57P03",
            SqlState::DatabaseDropped => "57P04",
            SqlState::SystemError => "58000",
            SqlState::IoError => "58030",
            SqlState::UndefinedFile => "58P01",
            SqlState::DuplicateFile => "58P02",
            SqlState::ConfigFileError => "F0000",
            SqlState::LockFileExists => "F0001",
            SqlState::FdwError => "HV000",
            SqlState::FdwColumnNameNotFound => "HV005",
            SqlState::FdwDynamicParameterValueNeeded => "HV002",
            SqlState::FdwFunctionSequenceError => "HV010",
            SqlState::FdwInconsistentDescriptorInformation => "HV021",
            SqlState::FdwInvalidAttributeValue => "HV024",
            SqlState::FdwInvalidColumnName => "HV007",
            SqlState::FdwInvalidColumnNumber => "HV008",
            SqlState::FdwInvalidDataType => "HV004",
            SqlState::FdwInvalidDataTypeDescriptors => "HV006",
            SqlState::FdwInvalidDescriptorFieldIdentifier => "HV091",
            SqlState::FdwInvalidHandle => "HV00B",
            SqlState::FdwInvalidOptionIndex => "HV00C",
            SqlState::FdwInvalidOptionName => "HV00D",
            SqlState::FdwInvalidStringLengthOrBufferLength => "HV090",
            SqlState::FdwInvalidStringFormat => "HV00A",
            SqlState::FdwInvalidUseOfNullPointer => "HV009",
            SqlState::FdwTooManyHandles => "HV014",
            SqlState::FdwOutOfMemory => "HV001",
            SqlState::FdwNoSchemas => "HV00P",
            SqlState::FdwOptionNameNotFound => "HV00J",
            SqlState::FdwReplyHandle => "HV00K",
            SqlState::FdwSchemaNotFound => "HV00Q",
            SqlState::FdwTableNotFound => "HV00R",
            SqlState::FdwUnableToCreateExcecution => "HV00L",
            SqlState::FdwUnableToCreateReply => "HV00M",
            SqlState::FdwUnableToEstablishConnection => "HV00N",
            SqlState::PlpgsqlError => "P0000",
            SqlState::RaiseException => "P0001",
            SqlState::NoDataFound => "P0002",
            SqlState::TooManyRows => "P0003",
            SqlState::InternalError => "XX000",
            SqlState::DataCorrupted => "XX001",
            SqlState::IndexCorrupted => "XX002",
            SqlState::Unknown(ref s) => &**s,
        }
    }
}

impl fmt::Debug for SqlState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            SqlState::SuccessfulCompletion => "SuccessfulCompletion",
            SqlState::Warning => "Warning",
            SqlState::DynamicResultSetsReturned => "DynamicResultSetsReturned",
            SqlState::ImplicitZeroBitPadding => "ImplicitZeroBitPadding",
            SqlState::NullValueEliminatedInSetFunction => "NullValueEliminatedInSetFunction",
            SqlState::PrivilegeNotGranted => "PrivilegeNotGranted",
            SqlState::PrivilegeNotRevoked => "PrivilegeNotRevoked",
            SqlState::StringDataRightTruncationWarning => "StringDataRightTruncationWarning",
            SqlState::DeprecatedFeature => "DeprecatedFeature",
            SqlState::NoData => "NoData",
            SqlState::NoAdditionalDynamicResultSetsReturned => "NoAdditionalDynamicResultSetsReturned",
            SqlState::SqlStatementNotYetComplete => "SqlStatementNotYetComplete",
            SqlState::ConnectionException => "ConnectionException",
            SqlState::ConnectionDoesNotExist => "ConnectionDoesNotExist",
            SqlState::ConnectionFailure => "ConnectionFailure",
            SqlState::SqlclientUnableToEstablishSqlconnection => "SqlclientUnableToEstablishSqlconnection",
            SqlState::SqlserverRejectedEstablishmentOfSqlconnection => "SqlserverRejectedEstablishmentOfSqlconnection",
            SqlState::TransactionResolutionUnknown => "TransactionResolutionUnknown",
            SqlState::ProtocolViolation => "ProtocolViolation",
            SqlState::TriggeredActionException => "TriggeredActionException",
            SqlState::FeatureNotSupported => "FeatureNotSupported",
            SqlState::InvalidTransactionInitiation => "InvalidTransactionInitiation",
            SqlState::LocatorException => "LocatorException",
            SqlState::InvalidLocatorException => "InvalidLocatorException",
            SqlState::InvalidGrantor => "InvalidGrantor",
            SqlState::InvalidGrantOperation => "InvalidGrantOperation",
            SqlState::InvalidRoleSpecification => "InvalidRoleSpecification",
            SqlState::DiagnosticsException => "DiagnosticsException",
            SqlState::StackedDiagnosticsAccessedWithoutActiveHandler => "StackedDiagnosticsAccessedWithoutActiveHandler",
            SqlState::CaseNotFound => "CaseNotFound",
            SqlState::CardinalityViolation => "CardinalityViolation",
            SqlState::DataException => "DataException",
            SqlState::ArraySubscriptError => "ArraySubscriptError",
            SqlState::CharacterNotInRepertoire => "CharacterNotInRepertoire",
            SqlState::DatetimeFieldOverflow => "DatetimeFieldOverflow",
            SqlState::DivisionByZero => "DivisionByZero",
            SqlState::ErrorInAssignment => "ErrorInAssignment",
            SqlState::EscapeCharacterConflict => "EscapeCharacterConflict",
            SqlState::IndicatorOverflow => "IndicatorOverflow",
            SqlState::IntervalFieldOverflow => "IntervalFieldOverflow",
            SqlState::InvalidArgumentForLogarithm => "InvalidArgumentForLogarithm",
            SqlState::InvalidArgumentForNtileFunction => "InvalidArgumentForNtileFunction",
            SqlState::InvalidArgumentForNthValueFunction => "InvalidArgumentForNthValueFunction",
            SqlState::InvalidArgumentForPowerFunction => "InvalidArgumentForPowerFunction",
            SqlState::InvalidArgumentForWidthBucketFunction => "InvalidArgumentForWidthBucketFunction",
            SqlState::InvalidCharacterValueForCast => "InvalidCharacterValueForCast",
            SqlState::InvalidDatetimeFormat => "InvalidDatetimeFormat",
            SqlState::InvalidEscapeCharacter => "InvalidEscapeCharacter",
            SqlState::InvalidEscapeOctet => "InvalidEscapeOctet",
            SqlState::InvalidEscapeSequence => "InvalidEscapeSequence",
            SqlState::NonstandardUseOfEscapeCharacter => "NonstandardUseOfEscapeCharacter",
            SqlState::InvalidIndicatorParameterValue => "InvalidIndicatorParameterValue",
            SqlState::InvalidParameterValue => "InvalidParameterValue",
            SqlState::InvalidRegularExpression => "InvalidRegularExpression",
            SqlState::InvalidRowCountInLimitClause => "InvalidRowCountInLimitClause",
            SqlState::InvalidRowCountInResultOffsetClause => "InvalidRowCountInResultOffsetClause",
            SqlState::InvalidTimeZoneDisplacementValue => "InvalidTimeZoneDisplacementValue",
            SqlState::InvalidUseOfEscapeCharacter => "InvalidUseOfEscapeCharacter",
            SqlState::MostSpecificTypeMismatch => "MostSpecificTypeMismatch",
            SqlState::NullValueNotAllowedData => "NullValueNotAllowedData",
            SqlState::NullValueNoIndicatorParameter => "NullValueNoIndicatorParameter",
            SqlState::NumericValueOutOfRange => "NumericValueOutOfRange",
            SqlState::StringDataLengthMismatch => "StringDataLengthMismatch",
            SqlState::StringDataRightTruncationException => "StringDataRightTruncationException",
            SqlState::SubstringError => "SubstringError",
            SqlState::TrimError => "TrimError",
            SqlState::UnterminatedCString => "UnterminatedCString",
            SqlState::ZeroLengthCharacterString => "ZeroLengthCharacterString",
            SqlState::FloatingPointException => "FloatingPointException",
            SqlState::InvalidTextRepresentation => "InvalidTextRepresentation",
            SqlState::InvalidBinaryRepresentation => "InvalidBinaryRepresentation",
            SqlState::BadCopyFileFormat => "BadCopyFileFormat",
            SqlState::UntranslatableCharacter => "UntranslatableCharacter",
            SqlState::NotAnXmlDocument => "NotAnXmlDocument",
            SqlState::InvalidXmlDocument => "InvalidXmlDocument",
            SqlState::InvalidXmlContent => "InvalidXmlContent",
            SqlState::InvalidXmlComment => "InvalidXmlComment",
            SqlState::InvalidXmlProcessingInstruction => "InvalidXmlProcessingInstruction",
            SqlState::IntegrityConstraintViolation => "IntegrityConstraintViolation",
            SqlState::RestrictViolation => "RestrictViolation",
            SqlState::NotNullViolation => "NotNullViolation",
            SqlState::ForeignKeyViolation => "ForeignKeyViolation",
            SqlState::UniqueViolation => "UniqueViolation",
            SqlState::CheckViolation => "CheckViolation",
            SqlState::ExclusionViolation => "ExclusionViolation",
            SqlState::InvalidCursorState => "InvalidCursorState",
            SqlState::InvalidTransactionState => "InvalidTransactionState",
            SqlState::ActiveSqlTransaction => "ActiveSqlTransaction",
            SqlState::BranchTransactionAlreadyActive => "BranchTransactionAlreadyActive",
            SqlState::HeldCursorRequiresSameIsolationLevel => "HeldCursorRequiresSameIsolationLevel",
            SqlState::InappropriateAccessModeForBranchTransaction => "InappropriateAccessModeForBranchTransaction",
            SqlState::InappropriateIsolationLevelForBranchTransaction => "InappropriateIsolationLevelForBranchTransaction",
            SqlState::NoActiveSqlTransactionForBranchTransaction => "NoActiveSqlTransactionForBranchTransaction",
            SqlState::ReadOnlySqlTransaction => "ReadOnlySqlTransaction",
            SqlState::SchemaAndDataStatementMixingNotSupported => "SchemaAndDataStatementMixingNotSupported",
            SqlState::NoActiveSqlTransaction => "NoActiveSqlTransaction",
            SqlState::InFailedSqlTransaction => "InFailedSqlTransaction",
            SqlState::InvalidSqlStatementName => "InvalidSqlStatementName",
            SqlState::TriggeredDataChangeViolation => "TriggeredDataChangeViolation",
            SqlState::InvalidAuthorizationSpecification => "InvalidAuthorizationSpecification",
            SqlState::InvalidPassword => "InvalidPassword",
            SqlState::DependentPrivilegeDescriptorsStillExist => "DependentPrivilegeDescriptorsStillExist",
            SqlState::DependentObjectsStillExist => "DependentObjectsStillExist",
            SqlState::InvalidTransactionTermination => "InvalidTransactionTermination",
            SqlState::SqlRoutineException => "SqlRoutineException",
            SqlState::FunctionExecutedNoReturnStatement => "FunctionExecutedNoReturnStatement",
            SqlState::ModifyingSqlDataNotPermittedSqlRoutine => "ModifyingSqlDataNotPermittedSqlRoutine",
            SqlState::ProhibitedSqlStatementAttemptedSqlRoutine => "ProhibitedSqlStatementAttemptedSqlRoutine",
            SqlState::ReadingSqlDataNotPermittedSqlRoutine => "ReadingSqlDataNotPermittedSqlRoutine",
            SqlState::InvalidCursorName => "InvalidCursorName",
            SqlState::ExternalRoutineException => "ExternalRoutineException",
            SqlState::ContainingSqlNotPermitted => "ContainingSqlNotPermitted",
            SqlState::ModifyingSqlDataNotPermittedExternalRoutine => "ModifyingSqlDataNotPermittedExternalRoutine",
            SqlState::ProhibitedSqlStatementAttemptedExternalRoutine => "ProhibitedSqlStatementAttemptedExternalRoutine",
            SqlState::ReadingSqlDataNotPermittedExternalRoutine => "ReadingSqlDataNotPermittedExternalRoutine",
            SqlState::ExternalRoutineInvocationException => "ExternalRoutineInvocationException",
            SqlState::InvalidSqlstateReturned => "InvalidSqlstateReturned",
            SqlState::NullValueNotAllowedExternalRoutine => "NullValueNotAllowedExternalRoutine",
            SqlState::TriggerProtocolViolated => "TriggerProtocolViolated",
            SqlState::SrfProtocolViolated => "SrfProtocolViolated",
            SqlState::SavepointException => "SavepointException",
            SqlState::InvalidSavepointException => "InvalidSavepointException",
            SqlState::InvalidCatalogName => "InvalidCatalogName",
            SqlState::InvalidSchemaName => "InvalidSchemaName",
            SqlState::TransactionRollback => "TransactionRollback",
            SqlState::TransactionIntegrityConstraintViolation => "TransactionIntegrityConstraintViolation",
            SqlState::SerializationFailure => "SerializationFailure",
            SqlState::StatementCompletionUnknown => "StatementCompletionUnknown",
            SqlState::DeadlockDetected => "DeadlockDetected",
            SqlState::SyntaxErrorOrAccessRuleViolation => "SyntaxErrorOrAccessRuleViolation",
            SqlState::SyntaxError => "SyntaxError",
            SqlState::InsufficientPrivilege => "InsufficientPrivilege",
            SqlState::CannotCoerce => "CannotCoerce",
            SqlState::GroupingError => "GroupingError",
            SqlState::WindowingError => "WindowingError",
            SqlState::InvalidRecursion => "InvalidRecursion",
            SqlState::InvalidForeignKey => "InvalidForeignKey",
            SqlState::InvalidName => "InvalidName",
            SqlState::NameTooLong => "NameTooLong",
            SqlState::ReservedName => "ReservedName",
            SqlState::DatatypeMismatch => "DatatypeMismatch",
            SqlState::IndeterminateDatatype => "IndeterminateDatatype",
            SqlState::CollationMismatch => "CollationMismatch",
            SqlState::IndeterminateCollation => "IndeterminateCollation",
            SqlState::WrongObjectType => "WrongObjectType",
            SqlState::UndefinedColumn => "UndefinedColumn",
            SqlState::UndefinedFunction => "UndefinedFunction",
            SqlState::UndefinedTable => "UndefinedTable",
            SqlState::UndefinedParameter => "UndefinedParameter",
            SqlState::UndefinedObject => "UndefinedObject",
            SqlState::DuplicateColumn => "DuplicateColumn",
            SqlState::DuplicateCursor => "DuplicateCursor",
            SqlState::DuplicateDatabase => "DuplicateDatabase",
            SqlState::DuplicateFunction => "DuplicateFunction",
            SqlState::DuplicatePreparedStatement => "DuplicatePreparedStatement",
            SqlState::DuplicateSchema => "DuplicateSchema",
            SqlState::DuplicateTable => "DuplicateTable",
            SqlState::DuplicateAliaas => "DuplicateAliaas",
            SqlState::DuplicateObject => "DuplicateObject",
            SqlState::AmbiguousColumn => "AmbiguousColumn",
            SqlState::AmbiguousFunction => "AmbiguousFunction",
            SqlState::AmbiguousParameter => "AmbiguousParameter",
            SqlState::AmbiguousAlias => "AmbiguousAlias",
            SqlState::InvalidColumnReference => "InvalidColumnReference",
            SqlState::InvalidColumnDefinition => "InvalidColumnDefinition",
            SqlState::InvalidCursorDefinition => "InvalidCursorDefinition",
            SqlState::InvalidDatabaseDefinition => "InvalidDatabaseDefinition",
            SqlState::InvalidFunctionDefinition => "InvalidFunctionDefinition",
            SqlState::InvalidPreparedStatementDefinition => "InvalidPreparedStatementDefinition",
            SqlState::InvalidSchemaDefinition => "InvalidSchemaDefinition",
            SqlState::InvalidTableDefinition => "InvalidTableDefinition",
            SqlState::InvalidObjectDefinition => "InvalidObjectDefinition",
            SqlState::WithCheckOptionViolation => "WithCheckOptionViolation",
            SqlState::InsufficientResources => "InsufficientResources",
            SqlState::DiskFull => "DiskFull",
            SqlState::OutOfMemory => "OutOfMemory",
            SqlState::TooManyConnections => "TooManyConnections",
            SqlState::ConfigurationLimitExceeded => "ConfigurationLimitExceeded",
            SqlState::ProgramLimitExceeded => "ProgramLimitExceeded",
            SqlState::StatementTooComplex => "StatementTooComplex",
            SqlState::TooManyColumns => "TooManyColumns",
            SqlState::TooManyArguments => "TooManyArguments",
            SqlState::ObjectNotInPrerequisiteState => "ObjectNotInPrerequisiteState",
            SqlState::ObjectInUse => "ObjectInUse",
            SqlState::CantChangeRuntimeParam => "CantChangeRuntimeParam",
            SqlState::LockNotAvailable => "LockNotAvailable",
            SqlState::OperatorIntervention => "OperatorIntervention",
            SqlState::QueryCanceled => "QueryCanceled",
            SqlState::AdminShutdown => "AdminShutdown",
            SqlState::CrashShutdown => "CrashShutdown",
            SqlState::CannotConnectNow => "CannotConnectNow",
            SqlState::DatabaseDropped => "DatabaseDropped",
            SqlState::SystemError => "SystemError",
            SqlState::IoError => "IoError",
            SqlState::UndefinedFile => "UndefinedFile",
            SqlState::DuplicateFile => "DuplicateFile",
            SqlState::ConfigFileError => "ConfigFileError",
            SqlState::LockFileExists => "LockFileExists",
            SqlState::FdwError => "FdwError",
            SqlState::FdwColumnNameNotFound => "FdwColumnNameNotFound",
            SqlState::FdwDynamicParameterValueNeeded => "FdwDynamicParameterValueNeeded",
            SqlState::FdwFunctionSequenceError => "FdwFunctionSequenceError",
            SqlState::FdwInconsistentDescriptorInformation => "FdwInconsistentDescriptorInformation",
            SqlState::FdwInvalidAttributeValue => "FdwInvalidAttributeValue",
            SqlState::FdwInvalidColumnName => "FdwInvalidColumnName",
            SqlState::FdwInvalidColumnNumber => "FdwInvalidColumnNumber",
            SqlState::FdwInvalidDataType => "FdwInvalidDataType",
            SqlState::FdwInvalidDataTypeDescriptors => "FdwInvalidDataTypeDescriptors",
            SqlState::FdwInvalidDescriptorFieldIdentifier => "FdwInvalidDescriptorFieldIdentifier",
            SqlState::FdwInvalidHandle => "FdwInvalidHandle",
            SqlState::FdwInvalidOptionIndex => "FdwInvalidOptionIndex",
            SqlState::FdwInvalidOptionName => "FdwInvalidOptionName",
            SqlState::FdwInvalidStringLengthOrBufferLength => "FdwInvalidStringLengthOrBufferLength",
            SqlState::FdwInvalidStringFormat => "FdwInvalidStringFormat",
            SqlState::FdwInvalidUseOfNullPointer => "FdwInvalidUseOfNullPointer",
            SqlState::FdwTooManyHandles => "FdwTooManyHandles",
            SqlState::FdwOutOfMemory => "FdwOutOfMemory",
            SqlState::FdwNoSchemas => "FdwNoSchemas",
            SqlState::FdwOptionNameNotFound => "FdwOptionNameNotFound",
            SqlState::FdwReplyHandle => "FdwReplyHandle",
            SqlState::FdwSchemaNotFound => "FdwSchemaNotFound",
            SqlState::FdwTableNotFound => "FdwTableNotFound",
            SqlState::FdwUnableToCreateExcecution => "FdwUnableToCreateExcecution",
            SqlState::FdwUnableToCreateReply => "FdwUnableToCreateReply",
            SqlState::FdwUnableToEstablishConnection => "FdwUnableToEstablishConnection",
            SqlState::PlpgsqlError => "PlpgsqlError",
            SqlState::RaiseException => "RaiseException",
            SqlState::NoDataFound => "NoDataFound",
            SqlState::TooManyRows => "TooManyRows",
            SqlState::InternalError => "InternalError",
            SqlState::DataCorrupted => "DataCorrupted",
            SqlState::IndexCorrupted => "IndexCorrupted",
            SqlState::Unknown(ref s) => return write!(fmt, "Unknown({:?})", s),
        };
        fmt.write_str(s)
    }
}
