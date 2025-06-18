pub use bindings::CeVIO::Talk::RemoteService2::{
    IPhonemeData2, IPhonemeDataArray2, IServiceControl2, IServiceControl2V40,
    IServiceControl2V40Part, ISpeakingState2, IStringArray2, ITalker2, ITalker2V40,
    ITalker2V40Part, ITalkerComponent2, ITalkerComponentArray2, PhonemeData2, PhonemeDataArray2,
    ServiceControl2, ServiceControl2V40, SpeakingState2, StringArray2, Talker2, Talker2V40,
    TalkerComponent2, TalkerComponentCollection2,
};

// ITalker2V40及びIServiceControl2V40がMutexでラップされることを前提としてSendを実装する
#[cfg(feature = "unsafe_send")]
use std::marker::Send;

#[cfg(feature = "unsafe_send")]
unsafe impl Send for IServiceControl2V40 {}

#[cfg(feature = "unsafe_send")]
unsafe impl Send for ITalker2V40 {}

mod bindings;
