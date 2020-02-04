use vapi_sys::VSL_tag_e;

macro_rules! enum_map {
    (pub enum $name:ident : $foreign:ident {
        $($variant:ident = $foreign_variant:ident),*,
    }) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum $name {
            $($variant),*
        }

        impl From<$foreign> for $name {
            fn from(f: $foreign) -> $name {
                match f {
                    $($foreign::$foreign_variant => $name::$variant),*
                }
            }
        }

        impl $name {
            pub fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

enum_map! {
    pub enum VslTag : VSL_tag_e {
        _Bogus = SLT__Bogus,
        Debug = SLT_Debug,
        Error = SLT_Error,
        CLI = SLT_CLI,
        SessOpen = SLT_SessOpen,
        SessClose = SLT_SessClose,
        BackendOpen = SLT_BackendOpen,
        BackendReuse = SLT_BackendReuse,
        BackendClose = SLT_BackendClose,
        HttpGarbage = SLT_HttpGarbage,
        Proxy = SLT_Proxy,
        ProxyGarbage = SLT_ProxyGarbage,
        Backend = SLT_Backend,
        Length = SLT_Length,
        FetchError = SLT_FetchError,
        ReqMethod = SLT_ReqMethod,
        ReqURL = SLT_ReqURL,
        ReqProtocol = SLT_ReqProtocol,
        ReqStatus = SLT_ReqStatus,
        ReqReason = SLT_ReqReason,
        ReqHeader = SLT_ReqHeader,
        ReqUnset = SLT_ReqUnset,
        ReqLost = SLT_ReqLost,
        RespMethod = SLT_RespMethod,
        RespURL = SLT_RespURL,
        RespProtocol = SLT_RespProtocol,
        RespStatus = SLT_RespStatus,
        RespReason = SLT_RespReason,
        RespHeader = SLT_RespHeader,
        RespUnset = SLT_RespUnset,
        RespLost = SLT_RespLost,
        BereqMethod = SLT_BereqMethod,
        BereqURL = SLT_BereqURL,
        BereqProtocol = SLT_BereqProtocol,
        BereqStatus = SLT_BereqStatus,
        BereqReason = SLT_BereqReason,
        BereqHeader = SLT_BereqHeader,
        BereqUnset = SLT_BereqUnset,
        BereqLost = SLT_BereqLost,
        BerespMethod = SLT_BerespMethod,
        BerespURL = SLT_BerespURL,
        BerespProtocol = SLT_BerespProtocol,
        BerespStatus = SLT_BerespStatus,
        BerespReason = SLT_BerespReason,
        BerespHeader = SLT_BerespHeader,
        BerespUnset = SLT_BerespUnset,
        BerespLost = SLT_BerespLost,
        ObjMethod = SLT_ObjMethod,
        ObjURL = SLT_ObjURL,
        ObjProtocol = SLT_ObjProtocol,
        ObjStatus = SLT_ObjStatus,
        ObjReason = SLT_ObjReason,
        ObjHeader = SLT_ObjHeader,
        ObjUnset = SLT_ObjUnset,
        ObjLost = SLT_ObjLost,
        BogoHeader = SLT_BogoHeader,
        LostHeader = SLT_LostHeader,
        TTL = SLT_TTL,
        Fetch_Body = SLT_Fetch_Body,
        VCL_acl = SLT_VCL_acl,
        VCL_call = SLT_VCL_call,
        VCL_trace = SLT_VCL_trace,
        VCL_return = SLT_VCL_return,
        ReqStart = SLT_ReqStart,
        Hit = SLT_Hit,
        HitPass = SLT_HitPass,
        ExpBan = SLT_ExpBan,
        ExpKill = SLT_ExpKill,
        WorkThread = SLT_WorkThread,
        ESI_xmlerror = SLT_ESI_xmlerror,
        Hash = SLT_Hash,
        Backend_health = SLT_Backend_health,
        VCL_Log = SLT_VCL_Log,
        VCL_Error = SLT_VCL_Error,
        Gzip = SLT_Gzip,
        Link = SLT_Link,
        Begin = SLT_Begin,
        End = SLT_End,
        VSL = SLT_VSL,
        Storage = SLT_Storage,
        Timestamp = SLT_Timestamp,
        ReqAcct = SLT_ReqAcct,
        PipeAcct = SLT_PipeAcct,
        BereqAcct = SLT_BereqAcct,
        VfpAcct = SLT_VfpAcct,
        Witness = SLT_Witness,
        BackendStart = SLT_BackendStart,
        H2RxHdr = SLT_H2RxHdr,
        H2RxBody = SLT_H2RxBody,
        H2TxHdr = SLT_H2TxHdr,
        H2TxBody = SLT_H2TxBody,
        HitMiss = SLT_HitMiss,
        Filters = SLT_Filters,
        SessError = SLT_SessError,
        VCL_use = SLT_VCL_use,
        _Reserved = SLT__Reserved,
        _Batch = SLT__Batch,
    }
}
