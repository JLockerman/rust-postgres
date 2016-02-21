/// A Postgres type.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Type {
    /// BOOL - boolean, 'true'/'false'
    Bool,
    /// BYTEA - variable-length string, binary values escaped
    Bytea,
    /// CHAR - single character
    Char,
    /// NAME - 63-byte type for storing system identifiers
    Name,
    /// INT8 - ~18 digit integer, 8-byte storage
    Int8,
    /// INT2 - -32 thousand to 32 thousand, 2-byte storage
    Int2,
    /// INT2VECTOR - array of int2, used in system tables
    Int2Vector,
    /// INT4 - -2 billion to 2 billion integer, 4-byte storage
    Int4,
    /// REGPROC - registered procedure
    Regproc,
    /// TEXT - variable-length string, no limit specified
    Text,
    /// OID - object identifier(oid), maximum 4 billion
    Oid,
    /// TID - (block, offset), physical location of tuple
    Tid,
    /// XID - transaction id
    Xid,
    /// CID - command identifier type, sequence in transaction id
    Cid,
    /// OIDVECTOR - array of oids, used in system tables
    OidVector,
    /// PG_DDL_COMMAND - internal type for passing CollectedCommand
    PgDdlCommand,
    /// JSON
    Json,
    /// XML - XML content
    Xml,
    /// XML[]
    XmlArray,
    /// PG_NODE_TREE - string representing an internal node tree
    PgNodeTree,
    /// JSON[]
    JsonArray,
    /// SMGR - storage manager
    Smgr,
    /// POINT - geometric point '(x, y)'
    Point,
    /// LSEG - geometric line segment '(pt1,pt2)'
    Lseg,
    /// PATH - geometric path '(pt1,...)'
    Path,
    /// BOX - geometric box '(lower left,upper right)'
    Box,
    /// POLYGON - geometric polygon '(pt1,...)'
    Polygon,
    /// LINE - geometric line
    Line,
    /// LINE[]
    LineArray,
    /// CIDR - network IP address/netmask, network address
    Cidr,
    /// CIDR[]
    CidrArray,
    /// FLOAT4 - single-precision floating point number, 4-byte storage
    Float4,
    /// FLOAT8 - double-precision floating point number, 8-byte storage
    Float8,
    /// ABSTIME - absolute, limited-range date and time (Unix system time)
    Abstime,
    /// RELTIME - relative, limited-range time interval (Unix delta time)
    Reltime,
    /// TINTERVAL - (abstime,abstime), time interval
    Tinterval,
    /// UNKNOWN
    Unknown,
    /// CIRCLE - geometric circle '(center,radius)'
    Circle,
    /// CIRCLE[]
    CircleArray,
    /// MONEY - monetary amounts, $d,ddd.cc
    Money,
    /// MONEY[]
    MoneyArray,
    /// MACADDR - XX:XX:XX:XX:XX:XX, MAC address
    Macaddr,
    /// INET - IP address/netmask, host address, netmask optional
    Inet,
    /// BOOL[]
    BoolArray,
    /// BYTEA[]
    ByteaArray,
    /// CHAR[]
    CharArray,
    /// NAME[]
    NameArray,
    /// INT2[]
    Int2Array,
    /// INT2VECTOR[]
    Int2VectorArray,
    /// INT4[]
    Int4Array,
    /// REGPROC[]
    RegprocArray,
    /// TEXT[]
    TextArray,
    /// TID[]
    TidArray,
    /// XID[]
    XidArray,
    /// CID[]
    CidArray,
    /// OIDVECTOR[]
    OidVectorArray,
    /// BPCHAR[]
    BpcharArray,
    /// VARCHAR[]
    VarcharArray,
    /// INT8[]
    Int8Array,
    /// POINT[]
    PointArray,
    /// LSEG[]
    LsegArray,
    /// PATH[]
    PathArray,
    /// BOX[]
    BoxArray,
    /// FLOAT4[]
    Float4Array,
    /// FLOAT8[]
    Float8Array,
    /// ABSTIME[]
    AbstimeArray,
    /// RELTIME[]
    ReltimeArray,
    /// TINTERVAL[]
    TintervalArray,
    /// POLYGON[]
    PolygonArray,
    /// OID[]
    OidArray,
    /// ACLITEM - access control list
    Aclitem,
    /// ACLITEM[]
    AclitemArray,
    /// MACADDR[]
    MacaddrArray,
    /// INET[]
    InetArray,
    /// BPCHAR - char(length), blank-padded string, fixed storage length
    Bpchar,
    /// VARCHAR - varchar(length), non-blank-padded string, variable storage length
    Varchar,
    /// DATE - date
    Date,
    /// TIME - time of day
    Time,
    /// TIMESTAMP - date and time
    Timestamp,
    /// TIMESTAMP[]
    TimestampArray,
    /// DATE[]
    DateArray,
    /// TIME[]
    TimeArray,
    /// TIMESTAMPTZ - date and time with time zone
    Timestamptz,
    /// TIMESTAMPTZ[]
    TimestamptzArray,
    /// INTERVAL - @ <number> <units>, time interval
    Interval,
    /// INTERVAL[]
    IntervalArray,
    /// NUMERIC[]
    NumericArray,
    /// CSTRING[]
    CstringArray,
    /// TIMETZ - time of day with time zone
    Timetz,
    /// TIMETZ[]
    TimetzArray,
    /// BIT - fixed-length bit string
    Bit,
    /// BIT[]
    BitArray,
    /// VARBIT - variable-length bit string
    Varbit,
    /// VARBIT[]
    VarbitArray,
    /// NUMERIC - numeric(precision, decimal), arbitrary precision number
    Numeric,
    /// REFCURSOR - reference to cursor (portal name)
    Refcursor,
    /// REFCURSOR[]
    RefcursorArray,
    /// REGPROCEDURE - registered procedure (with args)
    Regprocedure,
    /// REGOPER - registered operator
    Regoper,
    /// REGOPERATOR - registered operator (with args)
    Regoperator,
    /// REGCLASS - registered class
    Regclass,
    /// REGTYPE - registered type
    Regtype,
    /// REGPROCEDURE[]
    RegprocedureArray,
    /// REGOPER[]
    RegoperArray,
    /// REGOPERATOR[]
    RegoperatorArray,
    /// REGCLASS[]
    RegclassArray,
    /// REGTYPE[]
    RegtypeArray,
    /// RECORD
    Record,
    /// CSTRING
    Cstring,
    /// ANY
    Any,
    /// ANYARRAY
    Anyarray,
    /// VOID
    Void,
    /// TRIGGER
    Trigger,
    /// LANGUAGE_HANDLER
    LanguageHandler,
    /// INTERNAL
    Internal,
    /// OPAQUE
    Opaque,
    /// ANYELEMENT
    Anyelement,
    /// RECORD[]
    RecordArray,
    /// ANYNONARRAY
    Anynonarray,
    /// TXID_SNAPSHOT[]
    TxidSnapshotArray,
    /// UUID - UUID datatype
    Uuid,
    /// UUID[]
    UuidArray,
    /// TXID_SNAPSHOT - txid snapshot
    TxidSnapshot,
    /// FDW_HANDLER
    FdwHandler,
    /// PG_LSN - PostgreSQL LSN datatype
    PgLsn,
    /// PG_LSN[]
    PgLsnArray,
    /// TSM_HANDLER
    TsmHandler,
    /// ANYENUM
    Anyenum,
    /// TSVECTOR - text representation for text search
    TsVector,
    /// TSQUERY - query representation for text search
    Tsquery,
    /// GTSVECTOR - GiST index internal text representation for text search
    GtsVector,
    /// TSVECTOR[]
    TsVectorArray,
    /// GTSVECTOR[]
    GtsVectorArray,
    /// TSQUERY[]
    TsqueryArray,
    /// REGCONFIG - registered text search configuration
    Regconfig,
    /// REGCONFIG[]
    RegconfigArray,
    /// REGDICTIONARY - registered text search dictionary
    Regdictionary,
    /// REGDICTIONARY[]
    RegdictionaryArray,
    /// JSONB - Binary JSON
    Jsonb,
    /// JSONB[]
    JsonbArray,
    /// ANYRANGE
    Anyrange,
    /// EVENT_TRIGGER
    EventTrigger,
    /// INT4RANGE - range of integers
    Int4Range,
    /// INT4RANGE[]
    Int4RangeArray,
    /// NUMRANGE - range of numerics
    NumRange,
    /// NUMRANGE[]
    NumRangeArray,
    /// TSRANGE - range of timestamps without time zone
    TsRange,
    /// TSRANGE[]
    TsRangeArray,
    /// TSTZRANGE - range of timestamps with time zone
    TstzRange,
    /// TSTZRANGE[]
    TstzRangeArray,
    /// DATERANGE - range of dates
    DateRange,
    /// DATERANGE[]
    DateRangeArray,
    /// INT8RANGE - range of bigints
    Int8Range,
    /// INT8RANGE[]
    Int8RangeArray,
    /// REGNAMESPACE - registered namespace
    Regnamespace,
    /// REGNAMESPACE[]
    RegnamespaceArray,
    /// REGROLE - registered role
    Regrole,
    /// REGROLE[]
    RegroleArray,
    /// An unknown type.
    Other(Other),
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.schema() {
            "public" | "pg_catalog" => {}
            schema => try!(write!(fmt, "{}.", schema)),
        }
        fmt.write_str(self.name())
    }
}

impl Type {
    /// Returns the `Type` corresponding to the provided `Oid` if it
    /// corresponds to a built-in type.
    pub fn from_oid(oid: Oid) -> Option<Type> {
        match oid {
            16 => Some(Type::Bool),
            17 => Some(Type::Bytea),
            18 => Some(Type::Char),
            19 => Some(Type::Name),
            20 => Some(Type::Int8),
            21 => Some(Type::Int2),
            22 => Some(Type::Int2Vector),
            23 => Some(Type::Int4),
            24 => Some(Type::Regproc),
            25 => Some(Type::Text),
            26 => Some(Type::Oid),
            27 => Some(Type::Tid),
            28 => Some(Type::Xid),
            29 => Some(Type::Cid),
            30 => Some(Type::OidVector),
            32 => Some(Type::PgDdlCommand),
            114 => Some(Type::Json),
            142 => Some(Type::Xml),
            143 => Some(Type::XmlArray),
            194 => Some(Type::PgNodeTree),
            199 => Some(Type::JsonArray),
            210 => Some(Type::Smgr),
            600 => Some(Type::Point),
            601 => Some(Type::Lseg),
            602 => Some(Type::Path),
            603 => Some(Type::Box),
            604 => Some(Type::Polygon),
            628 => Some(Type::Line),
            629 => Some(Type::LineArray),
            650 => Some(Type::Cidr),
            651 => Some(Type::CidrArray),
            700 => Some(Type::Float4),
            701 => Some(Type::Float8),
            702 => Some(Type::Abstime),
            703 => Some(Type::Reltime),
            704 => Some(Type::Tinterval),
            705 => Some(Type::Unknown),
            718 => Some(Type::Circle),
            719 => Some(Type::CircleArray),
            790 => Some(Type::Money),
            791 => Some(Type::MoneyArray),
            829 => Some(Type::Macaddr),
            869 => Some(Type::Inet),
            1000 => Some(Type::BoolArray),
            1001 => Some(Type::ByteaArray),
            1002 => Some(Type::CharArray),
            1003 => Some(Type::NameArray),
            1005 => Some(Type::Int2Array),
            1006 => Some(Type::Int2VectorArray),
            1007 => Some(Type::Int4Array),
            1008 => Some(Type::RegprocArray),
            1009 => Some(Type::TextArray),
            1010 => Some(Type::TidArray),
            1011 => Some(Type::XidArray),
            1012 => Some(Type::CidArray),
            1013 => Some(Type::OidVectorArray),
            1014 => Some(Type::BpcharArray),
            1015 => Some(Type::VarcharArray),
            1016 => Some(Type::Int8Array),
            1017 => Some(Type::PointArray),
            1018 => Some(Type::LsegArray),
            1019 => Some(Type::PathArray),
            1020 => Some(Type::BoxArray),
            1021 => Some(Type::Float4Array),
            1022 => Some(Type::Float8Array),
            1023 => Some(Type::AbstimeArray),
            1024 => Some(Type::ReltimeArray),
            1025 => Some(Type::TintervalArray),
            1027 => Some(Type::PolygonArray),
            1028 => Some(Type::OidArray),
            1033 => Some(Type::Aclitem),
            1034 => Some(Type::AclitemArray),
            1040 => Some(Type::MacaddrArray),
            1041 => Some(Type::InetArray),
            1042 => Some(Type::Bpchar),
            1043 => Some(Type::Varchar),
            1082 => Some(Type::Date),
            1083 => Some(Type::Time),
            1114 => Some(Type::Timestamp),
            1115 => Some(Type::TimestampArray),
            1182 => Some(Type::DateArray),
            1183 => Some(Type::TimeArray),
            1184 => Some(Type::Timestamptz),
            1185 => Some(Type::TimestamptzArray),
            1186 => Some(Type::Interval),
            1187 => Some(Type::IntervalArray),
            1231 => Some(Type::NumericArray),
            1263 => Some(Type::CstringArray),
            1266 => Some(Type::Timetz),
            1270 => Some(Type::TimetzArray),
            1560 => Some(Type::Bit),
            1561 => Some(Type::BitArray),
            1562 => Some(Type::Varbit),
            1563 => Some(Type::VarbitArray),
            1700 => Some(Type::Numeric),
            1790 => Some(Type::Refcursor),
            2201 => Some(Type::RefcursorArray),
            2202 => Some(Type::Regprocedure),
            2203 => Some(Type::Regoper),
            2204 => Some(Type::Regoperator),
            2205 => Some(Type::Regclass),
            2206 => Some(Type::Regtype),
            2207 => Some(Type::RegprocedureArray),
            2208 => Some(Type::RegoperArray),
            2209 => Some(Type::RegoperatorArray),
            2210 => Some(Type::RegclassArray),
            2211 => Some(Type::RegtypeArray),
            2249 => Some(Type::Record),
            2275 => Some(Type::Cstring),
            2276 => Some(Type::Any),
            2277 => Some(Type::Anyarray),
            2278 => Some(Type::Void),
            2279 => Some(Type::Trigger),
            2280 => Some(Type::LanguageHandler),
            2281 => Some(Type::Internal),
            2282 => Some(Type::Opaque),
            2283 => Some(Type::Anyelement),
            2287 => Some(Type::RecordArray),
            2776 => Some(Type::Anynonarray),
            2949 => Some(Type::TxidSnapshotArray),
            2950 => Some(Type::Uuid),
            2951 => Some(Type::UuidArray),
            2970 => Some(Type::TxidSnapshot),
            3115 => Some(Type::FdwHandler),
            3220 => Some(Type::PgLsn),
            3221 => Some(Type::PgLsnArray),
            3310 => Some(Type::TsmHandler),
            3500 => Some(Type::Anyenum),
            3614 => Some(Type::TsVector),
            3615 => Some(Type::Tsquery),
            3642 => Some(Type::GtsVector),
            3643 => Some(Type::TsVectorArray),
            3644 => Some(Type::GtsVectorArray),
            3645 => Some(Type::TsqueryArray),
            3734 => Some(Type::Regconfig),
            3735 => Some(Type::RegconfigArray),
            3769 => Some(Type::Regdictionary),
            3770 => Some(Type::RegdictionaryArray),
            3802 => Some(Type::Jsonb),
            3807 => Some(Type::JsonbArray),
            3831 => Some(Type::Anyrange),
            3838 => Some(Type::EventTrigger),
            3904 => Some(Type::Int4Range),
            3905 => Some(Type::Int4RangeArray),
            3906 => Some(Type::NumRange),
            3907 => Some(Type::NumRangeArray),
            3908 => Some(Type::TsRange),
            3909 => Some(Type::TsRangeArray),
            3910 => Some(Type::TstzRange),
            3911 => Some(Type::TstzRangeArray),
            3912 => Some(Type::DateRange),
            3913 => Some(Type::DateRangeArray),
            3926 => Some(Type::Int8Range),
            3927 => Some(Type::Int8RangeArray),
            4089 => Some(Type::Regnamespace),
            4090 => Some(Type::RegnamespaceArray),
            4096 => Some(Type::Regrole),
            4097 => Some(Type::RegroleArray),
            _ => None,
        }
    }

    /// Returns the OID of the `Type`.
    pub fn oid(&self) -> Oid {
        match *self {
            Type::Bool => 16,
            Type::Bytea => 17,
            Type::Char => 18,
            Type::Name => 19,
            Type::Int8 => 20,
            Type::Int2 => 21,
            Type::Int2Vector => 22,
            Type::Int4 => 23,
            Type::Regproc => 24,
            Type::Text => 25,
            Type::Oid => 26,
            Type::Tid => 27,
            Type::Xid => 28,
            Type::Cid => 29,
            Type::OidVector => 30,
            Type::PgDdlCommand => 32,
            Type::Json => 114,
            Type::Xml => 142,
            Type::XmlArray => 143,
            Type::PgNodeTree => 194,
            Type::JsonArray => 199,
            Type::Smgr => 210,
            Type::Point => 600,
            Type::Lseg => 601,
            Type::Path => 602,
            Type::Box => 603,
            Type::Polygon => 604,
            Type::Line => 628,
            Type::LineArray => 629,
            Type::Cidr => 650,
            Type::CidrArray => 651,
            Type::Float4 => 700,
            Type::Float8 => 701,
            Type::Abstime => 702,
            Type::Reltime => 703,
            Type::Tinterval => 704,
            Type::Unknown => 705,
            Type::Circle => 718,
            Type::CircleArray => 719,
            Type::Money => 790,
            Type::MoneyArray => 791,
            Type::Macaddr => 829,
            Type::Inet => 869,
            Type::BoolArray => 1000,
            Type::ByteaArray => 1001,
            Type::CharArray => 1002,
            Type::NameArray => 1003,
            Type::Int2Array => 1005,
            Type::Int2VectorArray => 1006,
            Type::Int4Array => 1007,
            Type::RegprocArray => 1008,
            Type::TextArray => 1009,
            Type::TidArray => 1010,
            Type::XidArray => 1011,
            Type::CidArray => 1012,
            Type::OidVectorArray => 1013,
            Type::BpcharArray => 1014,
            Type::VarcharArray => 1015,
            Type::Int8Array => 1016,
            Type::PointArray => 1017,
            Type::LsegArray => 1018,
            Type::PathArray => 1019,
            Type::BoxArray => 1020,
            Type::Float4Array => 1021,
            Type::Float8Array => 1022,
            Type::AbstimeArray => 1023,
            Type::ReltimeArray => 1024,
            Type::TintervalArray => 1025,
            Type::PolygonArray => 1027,
            Type::OidArray => 1028,
            Type::Aclitem => 1033,
            Type::AclitemArray => 1034,
            Type::MacaddrArray => 1040,
            Type::InetArray => 1041,
            Type::Bpchar => 1042,
            Type::Varchar => 1043,
            Type::Date => 1082,
            Type::Time => 1083,
            Type::Timestamp => 1114,
            Type::TimestampArray => 1115,
            Type::DateArray => 1182,
            Type::TimeArray => 1183,
            Type::Timestamptz => 1184,
            Type::TimestamptzArray => 1185,
            Type::Interval => 1186,
            Type::IntervalArray => 1187,
            Type::NumericArray => 1231,
            Type::CstringArray => 1263,
            Type::Timetz => 1266,
            Type::TimetzArray => 1270,
            Type::Bit => 1560,
            Type::BitArray => 1561,
            Type::Varbit => 1562,
            Type::VarbitArray => 1563,
            Type::Numeric => 1700,
            Type::Refcursor => 1790,
            Type::RefcursorArray => 2201,
            Type::Regprocedure => 2202,
            Type::Regoper => 2203,
            Type::Regoperator => 2204,
            Type::Regclass => 2205,
            Type::Regtype => 2206,
            Type::RegprocedureArray => 2207,
            Type::RegoperArray => 2208,
            Type::RegoperatorArray => 2209,
            Type::RegclassArray => 2210,
            Type::RegtypeArray => 2211,
            Type::Record => 2249,
            Type::Cstring => 2275,
            Type::Any => 2276,
            Type::Anyarray => 2277,
            Type::Void => 2278,
            Type::Trigger => 2279,
            Type::LanguageHandler => 2280,
            Type::Internal => 2281,
            Type::Opaque => 2282,
            Type::Anyelement => 2283,
            Type::RecordArray => 2287,
            Type::Anynonarray => 2776,
            Type::TxidSnapshotArray => 2949,
            Type::Uuid => 2950,
            Type::UuidArray => 2951,
            Type::TxidSnapshot => 2970,
            Type::FdwHandler => 3115,
            Type::PgLsn => 3220,
            Type::PgLsnArray => 3221,
            Type::TsmHandler => 3310,
            Type::Anyenum => 3500,
            Type::TsVector => 3614,
            Type::Tsquery => 3615,
            Type::GtsVector => 3642,
            Type::TsVectorArray => 3643,
            Type::GtsVectorArray => 3644,
            Type::TsqueryArray => 3645,
            Type::Regconfig => 3734,
            Type::RegconfigArray => 3735,
            Type::Regdictionary => 3769,
            Type::RegdictionaryArray => 3770,
            Type::Jsonb => 3802,
            Type::JsonbArray => 3807,
            Type::Anyrange => 3831,
            Type::EventTrigger => 3838,
            Type::Int4Range => 3904,
            Type::Int4RangeArray => 3905,
            Type::NumRange => 3906,
            Type::NumRangeArray => 3907,
            Type::TsRange => 3908,
            Type::TsRangeArray => 3909,
            Type::TstzRange => 3910,
            Type::TstzRangeArray => 3911,
            Type::DateRange => 3912,
            Type::DateRangeArray => 3913,
            Type::Int8Range => 3926,
            Type::Int8RangeArray => 3927,
            Type::Regnamespace => 4089,
            Type::RegnamespaceArray => 4090,
            Type::Regrole => 4096,
            Type::RegroleArray => 4097,
            Type::Other(ref u) => u.oid(),
        }
    }

    /// Returns the kind of this type.
    pub fn kind(&self) -> &Kind {
        match *self {
            Type::Bool => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Bytea => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Char => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Name => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Int8 => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Int2 => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Int2Vector => {
                const V: &'static Kind = &Kind::Array(Type::Int2);
                V
            }
            Type::Int4 => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Regproc => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Text => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Oid => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Tid => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Xid => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Cid => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::OidVector => {
                const V: &'static Kind = &Kind::Array(Type::Oid);
                V
            }
            Type::PgDdlCommand => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::Json => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Xml => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::XmlArray => {
                const V: &'static Kind = &Kind::Array(Type::Xml);
                V
            }
            Type::PgNodeTree => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::JsonArray => {
                const V: &'static Kind = &Kind::Array(Type::Json);
                V
            }
            Type::Smgr => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Point => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Lseg => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Path => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Box => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Polygon => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Line => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::LineArray => {
                const V: &'static Kind = &Kind::Array(Type::Line);
                V
            }
            Type::Cidr => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::CidrArray => {
                const V: &'static Kind = &Kind::Array(Type::Cidr);
                V
            }
            Type::Float4 => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Float8 => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Abstime => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Reltime => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Tinterval => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Unknown => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Circle => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::CircleArray => {
                const V: &'static Kind = &Kind::Array(Type::Circle);
                V
            }
            Type::Money => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::MoneyArray => {
                const V: &'static Kind = &Kind::Array(Type::Money);
                V
            }
            Type::Macaddr => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Inet => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::BoolArray => {
                const V: &'static Kind = &Kind::Array(Type::Bool);
                V
            }
            Type::ByteaArray => {
                const V: &'static Kind = &Kind::Array(Type::Bytea);
                V
            }
            Type::CharArray => {
                const V: &'static Kind = &Kind::Array(Type::Char);
                V
            }
            Type::NameArray => {
                const V: &'static Kind = &Kind::Array(Type::Name);
                V
            }
            Type::Int2Array => {
                const V: &'static Kind = &Kind::Array(Type::Int2);
                V
            }
            Type::Int2VectorArray => {
                const V: &'static Kind = &Kind::Array(Type::Int2Vector);
                V
            }
            Type::Int4Array => {
                const V: &'static Kind = &Kind::Array(Type::Int4);
                V
            }
            Type::RegprocArray => {
                const V: &'static Kind = &Kind::Array(Type::Regproc);
                V
            }
            Type::TextArray => {
                const V: &'static Kind = &Kind::Array(Type::Text);
                V
            }
            Type::TidArray => {
                const V: &'static Kind = &Kind::Array(Type::Tid);
                V
            }
            Type::XidArray => {
                const V: &'static Kind = &Kind::Array(Type::Xid);
                V
            }
            Type::CidArray => {
                const V: &'static Kind = &Kind::Array(Type::Cid);
                V
            }
            Type::OidVectorArray => {
                const V: &'static Kind = &Kind::Array(Type::OidVector);
                V
            }
            Type::BpcharArray => {
                const V: &'static Kind = &Kind::Array(Type::Bpchar);
                V
            }
            Type::VarcharArray => {
                const V: &'static Kind = &Kind::Array(Type::Varchar);
                V
            }
            Type::Int8Array => {
                const V: &'static Kind = &Kind::Array(Type::Int8);
                V
            }
            Type::PointArray => {
                const V: &'static Kind = &Kind::Array(Type::Point);
                V
            }
            Type::LsegArray => {
                const V: &'static Kind = &Kind::Array(Type::Lseg);
                V
            }
            Type::PathArray => {
                const V: &'static Kind = &Kind::Array(Type::Path);
                V
            }
            Type::BoxArray => {
                const V: &'static Kind = &Kind::Array(Type::Box);
                V
            }
            Type::Float4Array => {
                const V: &'static Kind = &Kind::Array(Type::Float4);
                V
            }
            Type::Float8Array => {
                const V: &'static Kind = &Kind::Array(Type::Float8);
                V
            }
            Type::AbstimeArray => {
                const V: &'static Kind = &Kind::Array(Type::Abstime);
                V
            }
            Type::ReltimeArray => {
                const V: &'static Kind = &Kind::Array(Type::Reltime);
                V
            }
            Type::TintervalArray => {
                const V: &'static Kind = &Kind::Array(Type::Tinterval);
                V
            }
            Type::PolygonArray => {
                const V: &'static Kind = &Kind::Array(Type::Polygon);
                V
            }
            Type::OidArray => {
                const V: &'static Kind = &Kind::Array(Type::Oid);
                V
            }
            Type::Aclitem => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::AclitemArray => {
                const V: &'static Kind = &Kind::Array(Type::Aclitem);
                V
            }
            Type::MacaddrArray => {
                const V: &'static Kind = &Kind::Array(Type::Macaddr);
                V
            }
            Type::InetArray => {
                const V: &'static Kind = &Kind::Array(Type::Inet);
                V
            }
            Type::Bpchar => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Varchar => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Date => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Time => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Timestamp => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::TimestampArray => {
                const V: &'static Kind = &Kind::Array(Type::Timestamp);
                V
            }
            Type::DateArray => {
                const V: &'static Kind = &Kind::Array(Type::Date);
                V
            }
            Type::TimeArray => {
                const V: &'static Kind = &Kind::Array(Type::Time);
                V
            }
            Type::Timestamptz => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::TimestamptzArray => {
                const V: &'static Kind = &Kind::Array(Type::Timestamptz);
                V
            }
            Type::Interval => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::IntervalArray => {
                const V: &'static Kind = &Kind::Array(Type::Interval);
                V
            }
            Type::NumericArray => {
                const V: &'static Kind = &Kind::Array(Type::Numeric);
                V
            }
            Type::CstringArray => {
                const V: &'static Kind = &Kind::Array(Type::Cstring);
                V
            }
            Type::Timetz => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::TimetzArray => {
                const V: &'static Kind = &Kind::Array(Type::Timetz);
                V
            }
            Type::Bit => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::BitArray => {
                const V: &'static Kind = &Kind::Array(Type::Bit);
                V
            }
            Type::Varbit => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::VarbitArray => {
                const V: &'static Kind = &Kind::Array(Type::Varbit);
                V
            }
            Type::Numeric => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Refcursor => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::RefcursorArray => {
                const V: &'static Kind = &Kind::Array(Type::Refcursor);
                V
            }
            Type::Regprocedure => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Regoper => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Regoperator => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Regclass => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Regtype => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::RegprocedureArray => {
                const V: &'static Kind = &Kind::Array(Type::Regprocedure);
                V
            }
            Type::RegoperArray => {
                const V: &'static Kind = &Kind::Array(Type::Regoper);
                V
            }
            Type::RegoperatorArray => {
                const V: &'static Kind = &Kind::Array(Type::Regoperator);
                V
            }
            Type::RegclassArray => {
                const V: &'static Kind = &Kind::Array(Type::Regclass);
                V
            }
            Type::RegtypeArray => {
                const V: &'static Kind = &Kind::Array(Type::Regtype);
                V
            }
            Type::Record => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::Cstring => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::Any => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::Anyarray => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::Void => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::Trigger => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::LanguageHandler => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::Internal => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::Opaque => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::Anyelement => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::RecordArray => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::Anynonarray => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::TxidSnapshotArray => {
                const V: &'static Kind = &Kind::Array(Type::TxidSnapshot);
                V
            }
            Type::Uuid => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::UuidArray => {
                const V: &'static Kind = &Kind::Array(Type::Uuid);
                V
            }
            Type::TxidSnapshot => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::FdwHandler => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::PgLsn => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::PgLsnArray => {
                const V: &'static Kind = &Kind::Array(Type::PgLsn);
                V
            }
            Type::TsmHandler => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::Anyenum => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::TsVector => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::Tsquery => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::GtsVector => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::TsVectorArray => {
                const V: &'static Kind = &Kind::Array(Type::TsVector);
                V
            }
            Type::GtsVectorArray => {
                const V: &'static Kind = &Kind::Array(Type::GtsVector);
                V
            }
            Type::TsqueryArray => {
                const V: &'static Kind = &Kind::Array(Type::Tsquery);
                V
            }
            Type::Regconfig => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::RegconfigArray => {
                const V: &'static Kind = &Kind::Array(Type::Regconfig);
                V
            }
            Type::Regdictionary => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::RegdictionaryArray => {
                const V: &'static Kind = &Kind::Array(Type::Regdictionary);
                V
            }
            Type::Jsonb => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::JsonbArray => {
                const V: &'static Kind = &Kind::Array(Type::Jsonb);
                V
            }
            Type::Anyrange => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::EventTrigger => {
                const V: &'static Kind = &Kind::Pseudo;
                V
            }
            Type::Int4Range => {
                const V: &'static Kind = &Kind::Range(Type::Int4);
                V
            }
            Type::Int4RangeArray => {
                const V: &'static Kind = &Kind::Array(Type::Int4Range);
                V
            }
            Type::NumRange => {
                const V: &'static Kind = &Kind::Range(Type::Numeric);
                V
            }
            Type::NumRangeArray => {
                const V: &'static Kind = &Kind::Array(Type::NumRange);
                V
            }
            Type::TsRange => {
                const V: &'static Kind = &Kind::Range(Type::Timestamp);
                V
            }
            Type::TsRangeArray => {
                const V: &'static Kind = &Kind::Array(Type::TsRange);
                V
            }
            Type::TstzRange => {
                const V: &'static Kind = &Kind::Range(Type::Timestamptz);
                V
            }
            Type::TstzRangeArray => {
                const V: &'static Kind = &Kind::Array(Type::TstzRange);
                V
            }
            Type::DateRange => {
                const V: &'static Kind = &Kind::Range(Type::Date);
                V
            }
            Type::DateRangeArray => {
                const V: &'static Kind = &Kind::Array(Type::DateRange);
                V
            }
            Type::Int8Range => {
                const V: &'static Kind = &Kind::Range(Type::Int8);
                V
            }
            Type::Int8RangeArray => {
                const V: &'static Kind = &Kind::Array(Type::Int8Range);
                V
            }
            Type::Regnamespace => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::RegnamespaceArray => {
                const V: &'static Kind = &Kind::Array(Type::Regnamespace);
                V
            }
            Type::Regrole => {
                const V: &'static Kind = &Kind::Simple;
                V
            }
            Type::RegroleArray => {
                const V: &'static Kind = &Kind::Array(Type::Regrole);
                V
            }
            Type::Other(ref u) => u.kind(),
        }
    }

    /// Returns the schema of this type.
    pub fn schema(&self) -> &str {
        match *self {
            Type::Other(ref u) => u.schema(),
            _ => "pg_catalog",
        }
    }

    /// Returns the name of this type.
    pub fn name(&self) -> &str {
        match *self {
            Type::Bool => "bool",
            Type::Bytea => "bytea",
            Type::Char => "char",
            Type::Name => "name",
            Type::Int8 => "int8",
            Type::Int2 => "int2",
            Type::Int2Vector => "int2vector",
            Type::Int4 => "int4",
            Type::Regproc => "regproc",
            Type::Text => "text",
            Type::Oid => "oid",
            Type::Tid => "tid",
            Type::Xid => "xid",
            Type::Cid => "cid",
            Type::OidVector => "oidvector",
            Type::PgDdlCommand => "pg_ddl_command",
            Type::Json => "json",
            Type::Xml => "xml",
            Type::XmlArray => "_xml",
            Type::PgNodeTree => "pg_node_tree",
            Type::JsonArray => "_json",
            Type::Smgr => "smgr",
            Type::Point => "point",
            Type::Lseg => "lseg",
            Type::Path => "path",
            Type::Box => "box",
            Type::Polygon => "polygon",
            Type::Line => "line",
            Type::LineArray => "_line",
            Type::Cidr => "cidr",
            Type::CidrArray => "_cidr",
            Type::Float4 => "float4",
            Type::Float8 => "float8",
            Type::Abstime => "abstime",
            Type::Reltime => "reltime",
            Type::Tinterval => "tinterval",
            Type::Unknown => "unknown",
            Type::Circle => "circle",
            Type::CircleArray => "_circle",
            Type::Money => "money",
            Type::MoneyArray => "_money",
            Type::Macaddr => "macaddr",
            Type::Inet => "inet",
            Type::BoolArray => "_bool",
            Type::ByteaArray => "_bytea",
            Type::CharArray => "_char",
            Type::NameArray => "_name",
            Type::Int2Array => "_int2",
            Type::Int2VectorArray => "_int2vector",
            Type::Int4Array => "_int4",
            Type::RegprocArray => "_regproc",
            Type::TextArray => "_text",
            Type::TidArray => "_tid",
            Type::XidArray => "_xid",
            Type::CidArray => "_cid",
            Type::OidVectorArray => "_oidvector",
            Type::BpcharArray => "_bpchar",
            Type::VarcharArray => "_varchar",
            Type::Int8Array => "_int8",
            Type::PointArray => "_point",
            Type::LsegArray => "_lseg",
            Type::PathArray => "_path",
            Type::BoxArray => "_box",
            Type::Float4Array => "_float4",
            Type::Float8Array => "_float8",
            Type::AbstimeArray => "_abstime",
            Type::ReltimeArray => "_reltime",
            Type::TintervalArray => "_tinterval",
            Type::PolygonArray => "_polygon",
            Type::OidArray => "_oid",
            Type::Aclitem => "aclitem",
            Type::AclitemArray => "_aclitem",
            Type::MacaddrArray => "_macaddr",
            Type::InetArray => "_inet",
            Type::Bpchar => "bpchar",
            Type::Varchar => "varchar",
            Type::Date => "date",
            Type::Time => "time",
            Type::Timestamp => "timestamp",
            Type::TimestampArray => "_timestamp",
            Type::DateArray => "_date",
            Type::TimeArray => "_time",
            Type::Timestamptz => "timestamptz",
            Type::TimestamptzArray => "_timestamptz",
            Type::Interval => "interval",
            Type::IntervalArray => "_interval",
            Type::NumericArray => "_numeric",
            Type::CstringArray => "_cstring",
            Type::Timetz => "timetz",
            Type::TimetzArray => "_timetz",
            Type::Bit => "bit",
            Type::BitArray => "_bit",
            Type::Varbit => "varbit",
            Type::VarbitArray => "_varbit",
            Type::Numeric => "numeric",
            Type::Refcursor => "refcursor",
            Type::RefcursorArray => "_refcursor",
            Type::Regprocedure => "regprocedure",
            Type::Regoper => "regoper",
            Type::Regoperator => "regoperator",
            Type::Regclass => "regclass",
            Type::Regtype => "regtype",
            Type::RegprocedureArray => "_regprocedure",
            Type::RegoperArray => "_regoper",
            Type::RegoperatorArray => "_regoperator",
            Type::RegclassArray => "_regclass",
            Type::RegtypeArray => "_regtype",
            Type::Record => "record",
            Type::Cstring => "cstring",
            Type::Any => "any",
            Type::Anyarray => "anyarray",
            Type::Void => "void",
            Type::Trigger => "trigger",
            Type::LanguageHandler => "language_handler",
            Type::Internal => "internal",
            Type::Opaque => "opaque",
            Type::Anyelement => "anyelement",
            Type::RecordArray => "_record",
            Type::Anynonarray => "anynonarray",
            Type::TxidSnapshotArray => "_txid_snapshot",
            Type::Uuid => "uuid",
            Type::UuidArray => "_uuid",
            Type::TxidSnapshot => "txid_snapshot",
            Type::FdwHandler => "fdw_handler",
            Type::PgLsn => "pg_lsn",
            Type::PgLsnArray => "_pg_lsn",
            Type::TsmHandler => "tsm_handler",
            Type::Anyenum => "anyenum",
            Type::TsVector => "tsvector",
            Type::Tsquery => "tsquery",
            Type::GtsVector => "gtsvector",
            Type::TsVectorArray => "_tsvector",
            Type::GtsVectorArray => "_gtsvector",
            Type::TsqueryArray => "_tsquery",
            Type::Regconfig => "regconfig",
            Type::RegconfigArray => "_regconfig",
            Type::Regdictionary => "regdictionary",
            Type::RegdictionaryArray => "_regdictionary",
            Type::Jsonb => "jsonb",
            Type::JsonbArray => "_jsonb",
            Type::Anyrange => "anyrange",
            Type::EventTrigger => "event_trigger",
            Type::Int4Range => "int4range",
            Type::Int4RangeArray => "_int4range",
            Type::NumRange => "numrange",
            Type::NumRangeArray => "_numrange",
            Type::TsRange => "tsrange",
            Type::TsRangeArray => "_tsrange",
            Type::TstzRange => "tstzrange",
            Type::TstzRangeArray => "_tstzrange",
            Type::DateRange => "daterange",
            Type::DateRangeArray => "_daterange",
            Type::Int8Range => "int8range",
            Type::Int8RangeArray => "_int8range",
            Type::Regnamespace => "regnamespace",
            Type::RegnamespaceArray => "_regnamespace",
            Type::Regrole => "regrole",
            Type::RegroleArray => "_regrole",
            Type::Other(ref u) => u.name(),
        }
    }
}
