/// SQLSTATE error codes
#[derive(PartialEq, Eq, Clone, Debug)]
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
    WarningStringDataRightTruncation,
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
    InvalidLocatorSpecification,
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
    /// `2202H`
    InvalidTablesampleArgument,
    /// `2202G`
    InvalidTablesampleRepeat,
    /// `22009`
    InvalidTimeZoneDisplacementValue,
    /// `2200C`
    InvalidUseOfEscapeCharacter,
    /// `2200G`
    MostSpecificTypeMismatch,
    /// `22004`
    DataNullValueNotAllowed,
    /// `22002`
    NullValueNoIndicatorParameter,
    /// `22003`
    NumericValueOutOfRange,
    /// `22026`
    StringDataLengthMismatch,
    /// `22001`
    DataStringDataRightTruncation,
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
    /// `23P01`
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
    SqlRoutineModifyingSqlDataNotPermitted,
    /// `2F003`
    SqlRoutineProhibitedSqlStatementAttempted,
    /// `2F004`
    SqlRoutineReadingSqlDataNotPermitted,
    /// `34000`
    InvalidCursorName,
    /// `38000`
    ExternalRoutineException,
    /// `38001`
    ContainingSqlNotPermitted,
    /// `38002`
    ForeignRoutineModifyingSqlDataNotPermitted,
    /// `38003`
    ForeignRoutineProhibitedSqlStatementAttempted,
    /// `38004`
    ForeignRoutineReadingSqlDataNotPermitted,
    /// `39000`
    ExternalRoutineInvocationException,
    /// `39001`
    InvalidSqlstateReturned,
    /// `39004`
    ExternalRoutineInvocationNullValueNotAllowed,
    /// `39P01`
    TriggerProtocolViolated,
    /// `39P02`
    SrfProtocolViolated,
    /// `39P03`
    EventTriggerProtocolViolated,
    /// `3B000`
    SavepointException,
    /// `3B001`
    InvalidSavepointSpecification,
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
    DuplicateAlias,
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
    FdwUnableToCreateExecution,
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
    /// `P0004`
    AssertFailure,
    /// `XX000`
    InternalError,
    /// `XX001`
    DataCorrupted,
    /// `XX002`
    IndexCorrupted,
    /// An unknown code
    Other(String)
}
static SQLSTATE_MAP: phf::Map<&'static str, SqlState> = ::phf::Map {
    key: 1897749892740154578,
    disps: &[
        (0, 10),
        (1, 206),
        (0, 38),
        (0, 10),
        (0, 0),
        (1, 2),
        (0, 43),
        (0, 0),
        (2, 4),
        (0, 153),
        (0, 34),
        (0, 8),
        (0, 29),
        (0, 42),
        (0, 52),
        (0, 0),
        (1, 51),
        (2, 198),
        (9, 158),
        (1, 18),
        (0, 19),
        (0, 65),
        (1, 118),
        (1, 125),
        (3, 235),
        (0, 87),
        (0, 21),
        (3, 164),
        (0, 3),
        (4, 117),
        (11, 120),
        (0, 115),
        (40, 222),
        (0, 8),
        (8, 124),
        (1, 142),
        (0, 0),
        (0, 0),
        (10, 78),
        (1, 173),
        (10, 37),
        (2, 209),
        (18, 30),
        (1, 19),
        (0, 10),
        (0, 233),
        (2, 149),
        (0, 105),
    ],
    entries: &[
        ("42P03", SqlState::DuplicateCursor),
        ("22019", SqlState::InvalidEscapeCharacter),
        ("22022", SqlState::IndicatorOverflow),
        ("25002", SqlState::BranchTransactionAlreadyActive),
        ("2F004", SqlState::SqlRoutineReadingSqlDataNotPermitted),
        ("23505", SqlState::UniqueViolation),
        ("HV00J", SqlState::FdwOptionNameNotFound),
        ("42P17", SqlState::InvalidObjectDefinition),
        ("25000", SqlState::InvalidTransactionState),
        ("08001", SqlState::SqlclientUnableToEstablishSqlconnection),
        ("25008", SqlState::HeldCursorRequiresSameIsolationLevel),
        ("2201E", SqlState::InvalidArgumentForLogarithm),
        ("2200T", SqlState::InvalidXmlProcessingInstruction),
        ("2200F", SqlState::ZeroLengthCharacterString),
        ("HV002", SqlState::FdwDynamicParameterValueNeeded),
        ("2201B", SqlState::InvalidRegularExpression),
        ("22007", SqlState::InvalidDatetimeFormat),
        ("3B001", SqlState::InvalidSavepointSpecification),
        ("54023", SqlState::TooManyArguments),
        ("25P02", SqlState::InFailedSqlTransaction),
        ("2201X", SqlState::InvalidRowCountInResultOffsetClause),
        ("23P01", SqlState::ExclusionViolation),
        ("42P13", SqlState::InvalidFunctionDefinition),
        ("42712", SqlState::DuplicateAlias),
        ("39P02", SqlState::SrfProtocolViolated),
        ("23503", SqlState::ForeignKeyViolation),
        ("42P16", SqlState::InvalidTableDefinition),
        ("42000", SqlState::SyntaxErrorOrAccessRuleViolation),
        ("23000", SqlState::IntegrityConstraintViolation),
        ("53400", SqlState::ConfigurationLimitExceeded),
        ("38001", SqlState::ContainingSqlNotPermitted),
        ("2200D", SqlState::InvalidEscapeOctet),
        ("0B000", SqlState::InvalidTransactionInitiation),
        ("HV007", SqlState::FdwInvalidColumnName),
        ("42P15", SqlState::InvalidSchemaDefinition),
        ("HV00A", SqlState::FdwInvalidStringFormat),
        ("22026", SqlState::StringDataLengthMismatch),
        ("22011", SqlState::SubstringError),
        ("39P03", SqlState::EventTriggerProtocolViolated),
        ("42P02", SqlState::UndefinedParameter),
        ("XX000", SqlState::InternalError),
        ("2200S", SqlState::InvalidXmlComment),
        ("25007", SqlState::SchemaAndDataStatementMixingNotSupported),
        ("42710", SqlState::DuplicateObject),
        ("25P01", SqlState::NoActiveSqlTransaction),
        ("2200N", SqlState::InvalidXmlContent),
        ("01004", SqlState::WarningStringDataRightTruncation),
        ("42703", SqlState::UndefinedColumn),
        ("42P07", SqlState::DuplicateTable),
        ("26000", SqlState::InvalidSqlStatementName),
        ("01003", SqlState::NullValueEliminatedInSetFunction),
        ("22001", SqlState::DataStringDataRightTruncation),
        ("22012", SqlState::DivisionByZero),
        ("2F005", SqlState::FunctionExecutedNoReturnStatement),
        ("55P02", SqlState::CantChangeRuntimeParam),
        ("39P01", SqlState::TriggerProtocolViolated),
        ("3F000", SqlState::InvalidSchemaName),
        ("42501", SqlState::InsufficientPrivilege),
        ("22P01", SqlState::FloatingPointException),
        ("22004", SqlState::DataNullValueNotAllowed),
        ("HV00C", SqlState::FdwInvalidOptionIndex),
        ("HV00Q", SqlState::FdwSchemaNotFound),
        ("22P03", SqlState::InvalidBinaryRepresentation),
        ("22002", SqlState::NullValueNoIndicatorParameter),
        ("01007", SqlState::PrivilegeNotGranted),
        ("HV021", SqlState::FdwInconsistentDescriptorInformation),
        ("42P05", SqlState::DuplicatePreparedStatement),
        ("53000", SqlState::InsufficientResources),
        ("HV008", SqlState::FdwInvalidColumnNumber),
        ("42602", SqlState::InvalidName),
        ("40001", SqlState::SerializationFailure),
        ("25006", SqlState::ReadOnlySqlTransaction),
        ("00000", SqlState::SuccessfulCompletion),
        ("HV005", SqlState::FdwColumnNameNotFound),
        ("42939", SqlState::ReservedName),
        ("22P04", SqlState::BadCopyFileFormat),
        ("22015", SqlState::IntervalFieldOverflow),
        ("42P14", SqlState::InvalidPreparedStatementDefinition),
        ("01P01", SqlState::DeprecatedFeature),
        ("2200G", SqlState::MostSpecificTypeMismatch),
        ("HV001", SqlState::FdwOutOfMemory),
        ("08004", SqlState::SqlserverRejectedEstablishmentOfSqlconnection),
        ("42809", SqlState::WrongObjectType),
        ("HV009", SqlState::FdwInvalidUseOfNullPointer),
        ("58P02", SqlState::DuplicateFile),
        ("0Z000", SqlState::DiagnosticsException),
        ("08P01", SqlState::ProtocolViolation),
        ("42723", SqlState::DuplicateFunction),
        ("P0002", SqlState::NoDataFound),
        ("22021", SqlState::CharacterNotInRepertoire),
        ("01006", SqlState::PrivilegeNotRevoked),
        ("0LP01", SqlState::InvalidGrantOperation),
        ("P0004", SqlState::AssertFailure),
        ("0F000", SqlState::LocatorException),
        ("42611", SqlState::InvalidColumnDefinition),
        ("2200L", SqlState::NotAnXmlDocument),
        ("2200C", SqlState::InvalidUseOfEscapeCharacter),
        ("40003", SqlState::StatementCompletionUnknown),
        ("HV091", SqlState::FdwInvalidDescriptorFieldIdentifier),
        ("XX002", SqlState::IndexCorrupted),
        ("44000", SqlState::WithCheckOptionViolation),
        ("22P02", SqlState::InvalidTextRepresentation),
        ("54000", SqlState::ProgramLimitExceeded),
        ("24000", SqlState::InvalidCursorState),
        ("HV000", SqlState::FdwError),
        ("2201W", SqlState::InvalidRowCountInLimitClause),
        ("42P09", SqlState::AmbiguousAlias),
        ("F0001", SqlState::LockFileExists),
        ("57P01", SqlState::AdminShutdown),
        ("23001", SqlState::RestrictViolation),
        ("42P11", SqlState::InvalidCursorDefinition),
        ("22027", SqlState::TrimError),
        ("42725", SqlState::AmbiguousFunction),
        ("0L000", SqlState::InvalidGrantor),
        ("22003", SqlState::NumericValueOutOfRange),
        ("42702", SqlState::AmbiguousColumn),
        ("42830", SqlState::InvalidForeignKey),
        ("21000", SqlState::CardinalityViolation),
        ("58000", SqlState::SystemError),
        ("2BP01", SqlState::DependentObjectsStillExist),
        ("2201F", SqlState::InvalidArgumentForPowerFunction),
        ("0A000", SqlState::FeatureNotSupported),
        ("42P20", SqlState::WindowingError),
        ("28P01", SqlState::InvalidPassword),
        ("25003", SqlState::InappropriateAccessModeForBranchTransaction),
        ("01000", SqlState::Warning),
        ("08006", SqlState::ConnectionFailure),
        ("2200M", SqlState::InvalidXmlDocument),
        ("0100C", SqlState::DynamicResultSetsReturned),
        ("03000", SqlState::SqlStatementNotYetComplete),
        ("3D000", SqlState::InvalidCatalogName),
        ("2202E", SqlState::ArraySubscriptError),
        ("HV006", SqlState::FdwInvalidDataTypeDescriptors),
        ("42704", SqlState::UndefinedObject),
        ("HV004", SqlState::FdwInvalidDataType),
        ("HV00B", SqlState::FdwInvalidHandle),
        ("54001", SqlState::StatementTooComplex),
        ("2202G", SqlState::InvalidTablesampleRepeat),
        ("HV024", SqlState::FdwInvalidAttributeValue),
        ("2F002", SqlState::SqlRoutineModifyingSqlDataNotPermitted),
        ("F0000", SqlState::ConfigFileError),
        ("2D000", SqlState::InvalidTransactionTermination),
        ("25004", SqlState::InappropriateIsolationLevelForBranchTransaction),
        ("22P06", SqlState::NonstandardUseOfEscapeCharacter),
        ("53100", SqlState::DiskFull),
        ("42P10", SqlState::InvalidColumnReference),
        ("58030", SqlState::IoError),
        ("0P000", SqlState::InvalidRoleSpecification),
        ("08007", SqlState::TransactionResolutionUnknown),
        ("HV090", SqlState::FdwInvalidStringLengthOrBufferLength),
        ("27000", SqlState::TriggeredDataChangeViolation),
        ("0F001", SqlState::InvalidLocatorSpecification),
        ("22023", SqlState::InvalidParameterValue),
        ("22005", SqlState::ErrorInAssignment),
        ("57000", SqlState::OperatorIntervention),
        ("55P03", SqlState::LockNotAvailable),
        ("3B000", SqlState::SavepointException),
        ("09000", SqlState::TriggeredActionException),
        ("23502", SqlState::NotNullViolation),
        ("HV00D", SqlState::FdwInvalidOptionName),
        ("58P01", SqlState::UndefinedFile),
        ("40002", SqlState::TransactionIntegrityConstraintViolation),
        ("25005", SqlState::NoActiveSqlTransactionForBranchTransaction),
        ("42601", SqlState::SyntaxError),
        ("22024", SqlState::UnterminatedCString),
        ("22025", SqlState::InvalidEscapeSequence),
        ("22018", SqlState::InvalidCharacterValueForCast),
        ("42P22", SqlState::IndeterminateCollation),
        ("P0001", SqlState::RaiseException),
        ("42P04", SqlState::DuplicateDatabase),
        ("2202H", SqlState::InvalidTablesampleArgument),
        ("2F003", SqlState::SqlRoutineProhibitedSqlStatementAttempted),
        ("22014", SqlState::InvalidArgumentForNtileFunction),
        ("HV00M", SqlState::FdwUnableToCreateReply),
        ("HV014", SqlState::FdwTooManyHandles),
        ("08003", SqlState::ConnectionDoesNotExist),
        ("42P01", SqlState::UndefinedTable),
        ("57P04", SqlState::DatabaseDropped),
        ("42P21", SqlState::CollationMismatch),
        ("22009", SqlState::InvalidTimeZoneDisplacementValue),
        ("42804", SqlState::DatatypeMismatch),
        ("22016", SqlState::InvalidArgumentForNthValueFunction),
        ("57014", SqlState::QueryCanceled),
        ("42701", SqlState::DuplicateColumn),
        ("P0003", SqlState::TooManyRows),
        ("57P03", SqlState::CannotConnectNow),
        ("20000", SqlState::CaseNotFound),
        ("XX001", SqlState::DataCorrupted),
        ("42883", SqlState::UndefinedFunction),
        ("38000", SqlState::ExternalRoutineException),
        ("39004", SqlState::ExternalRoutineInvocationNullValueNotAllowed),
        ("P0000", SqlState::PlpgsqlError),
        ("2B000", SqlState::DependentPrivilegeDescriptorsStillExist),
        ("55006", SqlState::ObjectInUse),
        ("40P01", SqlState::DeadlockDetected),
        ("HV00R", SqlState::FdwTableNotFound),
        ("39000", SqlState::ExternalRoutineInvocationException),
        ("23514", SqlState::CheckViolation),
        ("22000", SqlState::DataException),
        ("22010", SqlState::InvalidIndicatorParameterValue),
        ("53300", SqlState::TooManyConnections),
        ("42P08", SqlState::AmbiguousParameter),
        ("HV00K", SqlState::FdwReplyHandle),
        ("22P05", SqlState::UntranslatableCharacter),
        ("2F000", SqlState::SqlRoutineException),
        ("HV010", SqlState::FdwFunctionSequenceError),
        ("42846", SqlState::CannotCoerce),
        ("25001", SqlState::ActiveSqlTransaction),
        ("38004", SqlState::ForeignRoutineReadingSqlDataNotPermitted),
        ("42P12", SqlState::InvalidDatabaseDefinition),
        ("53200", SqlState::OutOfMemory),
        ("42P19", SqlState::InvalidRecursion),
        ("42803", SqlState::GroupingError),
        ("54011", SqlState::TooManyColumns),
        ("39001", SqlState::InvalidSqlstateReturned),
        ("38003", SqlState::ForeignRoutineProhibitedSqlStatementAttempted),
        ("34000", SqlState::InvalidCursorName),
        ("42P18", SqlState::IndeterminateDatatype),
        ("2200B", SqlState::EscapeCharacterConflict),
        ("HV00N", SqlState::FdwUnableToEstablishConnection),
        ("HV00P", SqlState::FdwNoSchemas),
        ("42P06", SqlState::DuplicateSchema),
        ("02001", SqlState::NoAdditionalDynamicResultSetsReturned),
        ("HV00L", SqlState::FdwUnableToCreateExecution),
        ("57P02", SqlState::CrashShutdown),
        ("08000", SqlState::ConnectionException),
        ("2201G", SqlState::InvalidArgumentForWidthBucketFunction),
        ("42622", SqlState::NameTooLong),
        ("55000", SqlState::ObjectNotInPrerequisiteState),
        ("40000", SqlState::TransactionRollback),
        ("38002", SqlState::ForeignRoutineModifyingSqlDataNotPermitted),
        ("22008", SqlState::DatetimeFieldOverflow),
        ("01008", SqlState::ImplicitZeroBitPadding),
        ("28000", SqlState::InvalidAuthorizationSpecification),
        ("0Z002", SqlState::StackedDiagnosticsAccessedWithoutActiveHandler),
        ("02000", SqlState::NoData),
    ]
};

impl SqlState {
    /// Creates a `SqlState` from its error code.
    pub fn from_code(s: String) -> SqlState {
        match SQLSTATE_MAP.get(&*s) {
            Some(state) => state.clone(),
            None => SqlState::Other(s)
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
            SqlState::WarningStringDataRightTruncation => "01004",
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
            SqlState::InvalidLocatorSpecification => "0F001",
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
            SqlState::InvalidTablesampleArgument => "2202H",
            SqlState::InvalidTablesampleRepeat => "2202G",
            SqlState::InvalidTimeZoneDisplacementValue => "22009",
            SqlState::InvalidUseOfEscapeCharacter => "2200C",
            SqlState::MostSpecificTypeMismatch => "2200G",
            SqlState::DataNullValueNotAllowed => "22004",
            SqlState::NullValueNoIndicatorParameter => "22002",
            SqlState::NumericValueOutOfRange => "22003",
            SqlState::StringDataLengthMismatch => "22026",
            SqlState::DataStringDataRightTruncation => "22001",
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
            SqlState::ExclusionViolation => "23P01",
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
            SqlState::SqlRoutineModifyingSqlDataNotPermitted => "2F002",
            SqlState::SqlRoutineProhibitedSqlStatementAttempted => "2F003",
            SqlState::SqlRoutineReadingSqlDataNotPermitted => "2F004",
            SqlState::InvalidCursorName => "34000",
            SqlState::ExternalRoutineException => "38000",
            SqlState::ContainingSqlNotPermitted => "38001",
            SqlState::ForeignRoutineModifyingSqlDataNotPermitted => "38002",
            SqlState::ForeignRoutineProhibitedSqlStatementAttempted => "38003",
            SqlState::ForeignRoutineReadingSqlDataNotPermitted => "38004",
            SqlState::ExternalRoutineInvocationException => "39000",
            SqlState::InvalidSqlstateReturned => "39001",
            SqlState::ExternalRoutineInvocationNullValueNotAllowed => "39004",
            SqlState::TriggerProtocolViolated => "39P01",
            SqlState::SrfProtocolViolated => "39P02",
            SqlState::EventTriggerProtocolViolated => "39P03",
            SqlState::SavepointException => "3B000",
            SqlState::InvalidSavepointSpecification => "3B001",
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
            SqlState::DuplicateAlias => "42712",
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
            SqlState::FdwUnableToCreateExecution => "HV00L",
            SqlState::FdwUnableToCreateReply => "HV00M",
            SqlState::FdwUnableToEstablishConnection => "HV00N",
            SqlState::PlpgsqlError => "P0000",
            SqlState::RaiseException => "P0001",
            SqlState::NoDataFound => "P0002",
            SqlState::TooManyRows => "P0003",
            SqlState::AssertFailure => "P0004",
            SqlState::InternalError => "XX000",
            SqlState::DataCorrupted => "XX001",
            SqlState::IndexCorrupted => "XX002",
            SqlState::Other(ref s) => s,
        }
    }
}
