/// IdentifiedClientState defines a client state with an additional client
/// identifier field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifiedClientState {
    /// client identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// client state
    #[prost(message, optional, tag="2")]
    pub client_state: ::core::option::Option<::prost_types::Any>,
}
/// ConsensusStateWithHeight defines a consensus state with an additional height field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusStateWithHeight {
    /// consensus state height
    #[prost(message, optional, tag="1")]
    pub height: ::core::option::Option<Height>,
    /// consensus state
    #[prost(message, optional, tag="2")]
    pub consensus_state: ::core::option::Option<::prost_types::Any>,
}
/// ClientConsensusStates defines all the stored consensus states for a given
/// client.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientConsensusStates {
    /// client identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// consensus states and their heights associated with the client
    #[prost(message, repeated, tag="2")]
    pub consensus_states: ::prost::alloc::vec::Vec<ConsensusStateWithHeight>,
}
/// ClientUpdateProposal is a governance proposal. If it passes, the client is
/// updated with the provided header. The update may fail if the header is not
/// valid given certain conditions specified by the client implementation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientUpdateProposal {
    /// the title of the update proposal
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    /// the description of the proposal
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// the client identifier for the client to be updated if the proposal passes
    #[prost(string, tag="3")]
    pub client_id: ::prost::alloc::string::String,
    /// the header used to update the client if the proposal passes
    #[prost(message, optional, tag="4")]
    pub header: ::core::option::Option<::prost_types::Any>,
}
/// Height is a monotonically increasing data type
/// that can be compared against another Height for the purposes of updating and
/// freezing clients
///
/// Normally the RevisionHeight is incremented at each height while keeping RevisionNumber
/// the same. However some consensus algorithms may choose to reset the
/// height in certain conditions e.g. hard forks, state-machine breaking changes
/// In these cases, the RevisionNumber is incremented so that height continues to
/// be monitonically increasing even as the RevisionHeight gets reset
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Height {
    /// the revision that the client is currently on
    #[prost(uint64, tag="1")]
    pub revision_number: u64,
    /// the height within the given revision
    #[prost(uint64, tag="2")]
    pub revision_height: u64,
}
/// Params defines the set of IBC light client parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// allowed_clients defines the list of allowed client state types.
    #[prost(string, repeated, tag="1")]
    pub allowed_clients: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GenesisState defines the ibc client submodule's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// client states with their corresponding identifiers
    #[prost(message, repeated, tag="1")]
    pub clients: ::prost::alloc::vec::Vec<IdentifiedClientState>,
    /// consensus states from each client
    #[prost(message, repeated, tag="2")]
    pub clients_consensus: ::prost::alloc::vec::Vec<ClientConsensusStates>,
    /// metadata from each client
    #[prost(message, repeated, tag="3")]
    pub clients_metadata: ::prost::alloc::vec::Vec<IdentifiedGenesisMetadata>,
    #[prost(message, optional, tag="4")]
    pub params: ::core::option::Option<Params>,
    /// create localhost on initialization
    #[prost(bool, tag="5")]
    pub create_localhost: bool,
    /// the sequence for the next generated client identifier
    #[prost(uint64, tag="6")]
    pub next_client_sequence: u64,
}
/// GenesisMetadata defines the genesis type for metadata that clients may return
/// with ExportMetadata
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisMetadata {
    /// store key of metadata without clientID-prefix
    #[prost(bytes="vec", tag="1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// metadata value
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// IdentifiedGenesisMetadata has the client metadata with the corresponding client id.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifiedGenesisMetadata {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub client_metadata: ::prost::alloc::vec::Vec<GenesisMetadata>,
}
/// MsgCreateClient defines a message to create an IBC client
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateClient {
    /// light client state
    #[prost(message, optional, tag="1")]
    pub client_state: ::core::option::Option<::prost_types::Any>,
    /// consensus state associated with the client that corresponds to a given
    /// height.
    #[prost(message, optional, tag="2")]
    pub consensus_state: ::core::option::Option<::prost_types::Any>,
    /// signer address
    #[prost(string, tag="3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgCreateClientResponse defines the Msg/CreateClient response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateClientResponse {
}
/// MsgUpdateClient defines an sdk.Msg to update a IBC client state using
/// the given header.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClient {
    /// client unique identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// header to update the light client
    #[prost(message, optional, tag="2")]
    pub header: ::core::option::Option<::prost_types::Any>,
    /// signer address
    #[prost(string, tag="3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgUpdateClientResponse defines the Msg/UpdateClient response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClientResponse {
}
/// MsgUpgradeClient defines an sdk.Msg to upgrade an IBC client to a new client state
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpgradeClient {
    /// client unique identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// upgraded client state
    #[prost(message, optional, tag="2")]
    pub client_state: ::core::option::Option<::prost_types::Any>,
    /// upgraded consensus state, only contains enough information to serve as a basis of trust in update logic
    #[prost(message, optional, tag="3")]
    pub consensus_state: ::core::option::Option<::prost_types::Any>,
    /// proof that old chain committed to new client
    #[prost(bytes="vec", tag="4")]
    pub proof_upgrade_client: ::prost::alloc::vec::Vec<u8>,
    /// proof that old chain committed to new consensus state
    #[prost(bytes="vec", tag="5")]
    pub proof_upgrade_consensus_state: ::prost::alloc::vec::Vec<u8>,
    /// signer address
    #[prost(string, tag="6")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgUpgradeClientResponse defines the Msg/UpgradeClient response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpgradeClientResponse {
}
/// MsgSubmitMisbehaviour defines an sdk.Msg type that submits Evidence for
/// light client misbehaviour.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitMisbehaviour {
    /// client unique identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// misbehaviour used for freezing the light client
    #[prost(message, optional, tag="2")]
    pub misbehaviour: ::core::option::Option<::prost_types::Any>,
    /// signer address
    #[prost(string, tag="3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgSubmitMisbehaviourResponse defines the Msg/SubmitMisbehaviour response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitMisbehaviourResponse {
}
# [doc = r" Generated client implementations."] pub mod msg_client { # ! [allow (unused_variables , dead_code , missing_docs)] use tonic :: codegen :: * ; # [doc = " Msg defines the ibc/client Msg service."] pub struct MsgClient < T > { inner : tonic :: client :: Grpc < T > , } impl MsgClient < tonic :: transport :: Channel > { # [doc = r" Attempt to create a new client by connecting to a given endpoint."] pub async fn connect < D > (dst : D) -> Result < Self , tonic :: transport :: Error > where D : std :: convert :: TryInto < tonic :: transport :: Endpoint > , D :: Error : Into < StdError > , { let conn = tonic :: transport :: Endpoint :: new (dst) ? . connect () . await ? ; Ok (Self :: new (conn)) } } impl < T > MsgClient < T > where T : tonic :: client :: GrpcService < tonic :: body :: BoxBody > , T :: ResponseBody : Body + HttpBody + Send + 'static , T :: Error : Into < StdError > , < T :: ResponseBody as HttpBody > :: Error : Into < StdError > + Send , { pub fn new (inner : T) -> Self { let inner = tonic :: client :: Grpc :: new (inner) ; Self { inner } } pub fn with_interceptor (inner : T , interceptor : impl Into < tonic :: Interceptor >) -> Self { let inner = tonic :: client :: Grpc :: with_interceptor (inner , interceptor) ; Self { inner } } # [doc = " CreateClient defines a rpc handler method for MsgCreateClient."] pub async fn create_client (& mut self , request : impl tonic :: IntoRequest < super :: MsgCreateClient > ,) -> Result < tonic :: Response < super :: MsgCreateClientResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/ibc.core.client.v1.Msg/CreateClient") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " UpdateClient defines a rpc handler method for MsgUpdateClient."] pub async fn update_client (& mut self , request : impl tonic :: IntoRequest < super :: MsgUpdateClient > ,) -> Result < tonic :: Response < super :: MsgUpdateClientResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/ibc.core.client.v1.Msg/UpdateClient") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " UpgradeClient defines a rpc handler method for MsgUpgradeClient."] pub async fn upgrade_client (& mut self , request : impl tonic :: IntoRequest < super :: MsgUpgradeClient > ,) -> Result < tonic :: Response < super :: MsgUpgradeClientResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/ibc.core.client.v1.Msg/UpgradeClient") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " SubmitMisbehaviour defines a rpc handler method for MsgSubmitMisbehaviour."] pub async fn submit_misbehaviour (& mut self , request : impl tonic :: IntoRequest < super :: MsgSubmitMisbehaviour > ,) -> Result < tonic :: Response < super :: MsgSubmitMisbehaviourResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/ibc.core.client.v1.Msg/SubmitMisbehaviour") ; self . inner . unary (request . into_request () , path , codec) . await } } impl < T : Clone > Clone for MsgClient < T > { fn clone (& self) -> Self { Self { inner : self . inner . clone () , } } } impl < T > std :: fmt :: Debug for MsgClient < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "MsgClient {{ ... }}") } } }# [doc = r" Generated server implementations."] pub mod msg_server { # ! [allow (unused_variables , dead_code , missing_docs)] use tonic :: codegen :: * ; # [doc = "Generated trait containing gRPC methods that should be implemented for use with MsgServer."] # [async_trait] pub trait Msg : Send + Sync + 'static { # [doc = " CreateClient defines a rpc handler method for MsgCreateClient."] async fn create_client (& self , request : tonic :: Request < super :: MsgCreateClient >) -> Result < tonic :: Response < super :: MsgCreateClientResponse > , tonic :: Status > ; # [doc = " UpdateClient defines a rpc handler method for MsgUpdateClient."] async fn update_client (& self , request : tonic :: Request < super :: MsgUpdateClient >) -> Result < tonic :: Response < super :: MsgUpdateClientResponse > , tonic :: Status > ; # [doc = " UpgradeClient defines a rpc handler method for MsgUpgradeClient."] async fn upgrade_client (& self , request : tonic :: Request < super :: MsgUpgradeClient >) -> Result < tonic :: Response < super :: MsgUpgradeClientResponse > , tonic :: Status > ; # [doc = " SubmitMisbehaviour defines a rpc handler method for MsgSubmitMisbehaviour."] async fn submit_misbehaviour (& self , request : tonic :: Request < super :: MsgSubmitMisbehaviour >) -> Result < tonic :: Response < super :: MsgSubmitMisbehaviourResponse > , tonic :: Status > ; } # [doc = " Msg defines the ibc/client Msg service."] # [derive (Debug)] pub struct MsgServer < T : Msg > { inner : _Inner < T > , } struct _Inner < T > (Arc < T > , Option < tonic :: Interceptor >) ; impl < T : Msg > MsgServer < T > { pub fn new (inner : T) -> Self { let inner = Arc :: new (inner) ; let inner = _Inner (inner , None) ; Self { inner } } pub fn with_interceptor (inner : T , interceptor : impl Into < tonic :: Interceptor >) -> Self { let inner = Arc :: new (inner) ; let inner = _Inner (inner , Some (interceptor . into ())) ; Self { inner } } } impl < T , B > Service < http :: Request < B >> for MsgServer < T > where T : Msg , B : HttpBody + Send + Sync + 'static , B :: Error : Into < StdError > + Send + 'static , { type Response = http :: Response < tonic :: body :: BoxBody > ; type Error = Never ; type Future = BoxFuture < Self :: Response , Self :: Error > ; fn poll_ready (& mut self , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error >> { Poll :: Ready (Ok (())) } fn call (& mut self , req : http :: Request < B >) -> Self :: Future { let inner = self . inner . clone () ; match req . uri () . path () { "/ibc.core.client.v1.Msg/CreateClient" => { # [allow (non_camel_case_types)] struct CreateClientSvc < T : Msg > (pub Arc < T >) ; impl < T : Msg > tonic :: server :: UnaryService < super :: MsgCreateClient > for CreateClientSvc < T > { type Response = super :: MsgCreateClientResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: MsgCreateClient >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . create_client (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = CreateClientSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/ibc.core.client.v1.Msg/UpdateClient" => { # [allow (non_camel_case_types)] struct UpdateClientSvc < T : Msg > (pub Arc < T >) ; impl < T : Msg > tonic :: server :: UnaryService < super :: MsgUpdateClient > for UpdateClientSvc < T > { type Response = super :: MsgUpdateClientResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: MsgUpdateClient >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . update_client (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = UpdateClientSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/ibc.core.client.v1.Msg/UpgradeClient" => { # [allow (non_camel_case_types)] struct UpgradeClientSvc < T : Msg > (pub Arc < T >) ; impl < T : Msg > tonic :: server :: UnaryService < super :: MsgUpgradeClient > for UpgradeClientSvc < T > { type Response = super :: MsgUpgradeClientResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: MsgUpgradeClient >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . upgrade_client (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = UpgradeClientSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/ibc.core.client.v1.Msg/SubmitMisbehaviour" => { # [allow (non_camel_case_types)] struct SubmitMisbehaviourSvc < T : Msg > (pub Arc < T >) ; impl < T : Msg > tonic :: server :: UnaryService < super :: MsgSubmitMisbehaviour > for SubmitMisbehaviourSvc < T > { type Response = super :: MsgSubmitMisbehaviourResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: MsgSubmitMisbehaviour >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . submit_misbehaviour (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = SubmitMisbehaviourSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } _ => Box :: pin (async move { Ok (http :: Response :: builder () . status (200) . header ("grpc-status" , "12") . header ("content-type" , "application/grpc") . body (tonic :: body :: BoxBody :: empty ()) . unwrap ()) }) , } } } impl < T : Msg > Clone for MsgServer < T > { fn clone (& self) -> Self { let inner = self . inner . clone () ; Self { inner } } } impl < T : Msg > Clone for _Inner < T > { fn clone (& self) -> Self { Self (self . 0 . clone () , self . 1 . clone ()) } } impl < T : std :: fmt :: Debug > std :: fmt :: Debug for _Inner < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{:?}" , self . 0) } } impl < T : Msg > tonic :: transport :: NamedService for MsgServer < T > { const NAME : & 'static str = "ibc.core.client.v1.Msg" ; } }/// QueryClientStateRequest is the request type for the Query/ClientState RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientStateRequest {
    /// client state unique identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
}
/// QueryClientStateResponse is the response type for the Query/ClientState RPC
/// method. Besides the client state, it includes a proof and the height from
/// which the proof was retrieved.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientStateResponse {
    /// client state associated with the request identifier
    #[prost(message, optional, tag="1")]
    pub client_state: ::core::option::Option<::prost_types::Any>,
    /// merkle proof of existence
    #[prost(bytes="vec", tag="2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag="3")]
    pub proof_height: ::core::option::Option<Height>,
}
/// QueryClientStatesRequest is the request type for the Query/ClientStates RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientStatesRequest {
    /// pagination request
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryClientStatesResponse is the response type for the Query/ClientStates RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientStatesResponse {
    /// list of stored ClientStates of the chain.
    #[prost(message, repeated, tag="1")]
    pub client_states: ::prost::alloc::vec::Vec<IdentifiedClientState>,
    /// pagination response
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryConsensusStateRequest is the request type for the Query/ConsensusState
/// RPC method. Besides the consensus state, it includes a proof and the height
/// from which the proof was retrieved.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConsensusStateRequest {
    /// client identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// consensus state revision number
    #[prost(uint64, tag="2")]
    pub revision_number: u64,
    /// consensus state revision height
    #[prost(uint64, tag="3")]
    pub revision_height: u64,
    /// latest_height overrrides the height field and queries the latest stored
    /// ConsensusState
    #[prost(bool, tag="4")]
    pub latest_height: bool,
}
/// QueryConsensusStateResponse is the response type for the Query/ConsensusState
/// RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConsensusStateResponse {
    /// consensus state associated with the client identifier at the given height
    #[prost(message, optional, tag="1")]
    pub consensus_state: ::core::option::Option<::prost_types::Any>,
    /// merkle proof of existence
    #[prost(bytes="vec", tag="2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag="3")]
    pub proof_height: ::core::option::Option<Height>,
}
/// QueryConsensusStatesRequest is the request type for the Query/ConsensusStates
/// RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConsensusStatesRequest {
    /// client identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryConsensusStatesResponse is the response type for the
/// Query/ConsensusStates RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConsensusStatesResponse {
    /// consensus states associated with the identifier
    #[prost(message, repeated, tag="1")]
    pub consensus_states: ::prost::alloc::vec::Vec<ConsensusStateWithHeight>,
    /// pagination response
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryClientParamsRequest is the request type for the Query/ClientParams RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientParamsRequest {
}
/// QueryClientParamsResponse is the response type for the Query/ClientParams RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
# [doc = r" Generated client implementations."] pub mod query_client { # ! [allow (unused_variables , dead_code , missing_docs)] use tonic :: codegen :: * ; # [doc = " Query provides defines the gRPC querier service"] pub struct QueryClient < T > { inner : tonic :: client :: Grpc < T > , } impl QueryClient < tonic :: transport :: Channel > { # [doc = r" Attempt to create a new client by connecting to a given endpoint."] pub async fn connect < D > (dst : D) -> Result < Self , tonic :: transport :: Error > where D : std :: convert :: TryInto < tonic :: transport :: Endpoint > , D :: Error : Into < StdError > , { let conn = tonic :: transport :: Endpoint :: new (dst) ? . connect () . await ? ; Ok (Self :: new (conn)) } } impl < T > QueryClient < T > where T : tonic :: client :: GrpcService < tonic :: body :: BoxBody > , T :: ResponseBody : Body + HttpBody + Send + 'static , T :: Error : Into < StdError > , < T :: ResponseBody as HttpBody > :: Error : Into < StdError > + Send , { pub fn new (inner : T) -> Self { let inner = tonic :: client :: Grpc :: new (inner) ; Self { inner } } pub fn with_interceptor (inner : T , interceptor : impl Into < tonic :: Interceptor >) -> Self { let inner = tonic :: client :: Grpc :: with_interceptor (inner , interceptor) ; Self { inner } } # [doc = " ClientState queries an IBC light client."] pub async fn client_state (& mut self , request : impl tonic :: IntoRequest < super :: QueryClientStateRequest > ,) -> Result < tonic :: Response < super :: QueryClientStateResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/ibc.core.client.v1.Query/ClientState") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " ClientStates queries all the IBC light clients of a chain."] pub async fn client_states (& mut self , request : impl tonic :: IntoRequest < super :: QueryClientStatesRequest > ,) -> Result < tonic :: Response < super :: QueryClientStatesResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/ibc.core.client.v1.Query/ClientStates") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " ConsensusState queries a consensus state associated with a client state at"] # [doc = " a given height."] pub async fn consensus_state (& mut self , request : impl tonic :: IntoRequest < super :: QueryConsensusStateRequest > ,) -> Result < tonic :: Response < super :: QueryConsensusStateResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/ibc.core.client.v1.Query/ConsensusState") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " ConsensusStates queries all the consensus state associated with a given"] # [doc = " client."] pub async fn consensus_states (& mut self , request : impl tonic :: IntoRequest < super :: QueryConsensusStatesRequest > ,) -> Result < tonic :: Response < super :: QueryConsensusStatesResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/ibc.core.client.v1.Query/ConsensusStates") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " ClientParams queries all parameters of the ibc client."] pub async fn client_params (& mut self , request : impl tonic :: IntoRequest < super :: QueryClientParamsRequest > ,) -> Result < tonic :: Response < super :: QueryClientParamsResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/ibc.core.client.v1.Query/ClientParams") ; self . inner . unary (request . into_request () , path , codec) . await } } impl < T : Clone > Clone for QueryClient < T > { fn clone (& self) -> Self { Self { inner : self . inner . clone () , } } } impl < T > std :: fmt :: Debug for QueryClient < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "QueryClient {{ ... }}") } } }# [doc = r" Generated server implementations."] pub mod query_server { # ! [allow (unused_variables , dead_code , missing_docs)] use tonic :: codegen :: * ; # [doc = "Generated trait containing gRPC methods that should be implemented for use with QueryServer."] # [async_trait] pub trait Query : Send + Sync + 'static { # [doc = " ClientState queries an IBC light client."] async fn client_state (& self , request : tonic :: Request < super :: QueryClientStateRequest >) -> Result < tonic :: Response < super :: QueryClientStateResponse > , tonic :: Status > ; # [doc = " ClientStates queries all the IBC light clients of a chain."] async fn client_states (& self , request : tonic :: Request < super :: QueryClientStatesRequest >) -> Result < tonic :: Response < super :: QueryClientStatesResponse > , tonic :: Status > ; # [doc = " ConsensusState queries a consensus state associated with a client state at"] # [doc = " a given height."] async fn consensus_state (& self , request : tonic :: Request < super :: QueryConsensusStateRequest >) -> Result < tonic :: Response < super :: QueryConsensusStateResponse > , tonic :: Status > ; # [doc = " ConsensusStates queries all the consensus state associated with a given"] # [doc = " client."] async fn consensus_states (& self , request : tonic :: Request < super :: QueryConsensusStatesRequest >) -> Result < tonic :: Response < super :: QueryConsensusStatesResponse > , tonic :: Status > ; # [doc = " ClientParams queries all parameters of the ibc client."] async fn client_params (& self , request : tonic :: Request < super :: QueryClientParamsRequest >) -> Result < tonic :: Response < super :: QueryClientParamsResponse > , tonic :: Status > ; } # [doc = " Query provides defines the gRPC querier service"] # [derive (Debug)] pub struct QueryServer < T : Query > { inner : _Inner < T > , } struct _Inner < T > (Arc < T > , Option < tonic :: Interceptor >) ; impl < T : Query > QueryServer < T > { pub fn new (inner : T) -> Self { let inner = Arc :: new (inner) ; let inner = _Inner (inner , None) ; Self { inner } } pub fn with_interceptor (inner : T , interceptor : impl Into < tonic :: Interceptor >) -> Self { let inner = Arc :: new (inner) ; let inner = _Inner (inner , Some (interceptor . into ())) ; Self { inner } } } impl < T , B > Service < http :: Request < B >> for QueryServer < T > where T : Query , B : HttpBody + Send + Sync + 'static , B :: Error : Into < StdError > + Send + 'static , { type Response = http :: Response < tonic :: body :: BoxBody > ; type Error = Never ; type Future = BoxFuture < Self :: Response , Self :: Error > ; fn poll_ready (& mut self , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error >> { Poll :: Ready (Ok (())) } fn call (& mut self , req : http :: Request < B >) -> Self :: Future { let inner = self . inner . clone () ; match req . uri () . path () { "/ibc.core.client.v1.Query/ClientState" => { # [allow (non_camel_case_types)] struct ClientStateSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryClientStateRequest > for ClientStateSvc < T > { type Response = super :: QueryClientStateResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryClientStateRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . client_state (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = ClientStateSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/ibc.core.client.v1.Query/ClientStates" => { # [allow (non_camel_case_types)] struct ClientStatesSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryClientStatesRequest > for ClientStatesSvc < T > { type Response = super :: QueryClientStatesResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryClientStatesRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . client_states (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = ClientStatesSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/ibc.core.client.v1.Query/ConsensusState" => { # [allow (non_camel_case_types)] struct ConsensusStateSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryConsensusStateRequest > for ConsensusStateSvc < T > { type Response = super :: QueryConsensusStateResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryConsensusStateRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . consensus_state (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = ConsensusStateSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/ibc.core.client.v1.Query/ConsensusStates" => { # [allow (non_camel_case_types)] struct ConsensusStatesSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryConsensusStatesRequest > for ConsensusStatesSvc < T > { type Response = super :: QueryConsensusStatesResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryConsensusStatesRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . consensus_states (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = ConsensusStatesSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/ibc.core.client.v1.Query/ClientParams" => { # [allow (non_camel_case_types)] struct ClientParamsSvc < T : Query > (pub Arc < T >) ; impl < T : Query > tonic :: server :: UnaryService < super :: QueryClientParamsRequest > for ClientParamsSvc < T > { type Response = super :: QueryClientParamsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: QueryClientParamsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . client_params (request) . await } ; Box :: pin (fut) } } let inner = self . inner . clone () ; let fut = async move { let interceptor = inner . 1 . clone () ; let inner = inner . 0 ; let method = ClientParamsSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = if let Some (interceptor) = interceptor { tonic :: server :: Grpc :: with_interceptor (codec , interceptor) } else { tonic :: server :: Grpc :: new (codec) } ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } _ => Box :: pin (async move { Ok (http :: Response :: builder () . status (200) . header ("grpc-status" , "12") . header ("content-type" , "application/grpc") . body (tonic :: body :: BoxBody :: empty ()) . unwrap ()) }) , } } } impl < T : Query > Clone for QueryServer < T > { fn clone (& self) -> Self { let inner = self . inner . clone () ; Self { inner } } } impl < T : Query > Clone for _Inner < T > { fn clone (& self) -> Self { Self (self . 0 . clone () , self . 1 . clone ()) } } impl < T : std :: fmt :: Debug > std :: fmt :: Debug for _Inner < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{:?}" , self . 0) } } impl < T : Query > tonic :: transport :: NamedService for QueryServer < T > { const NAME : & 'static str = "ibc.core.client.v1.Query" ; } }