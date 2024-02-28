#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestStaffById {
    #[prost(string, tag = "1")]
    pub tenant_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseStaffById {
    #[prost(message, optional, tag = "1")]
    pub staff: ::core::option::Option<Staff>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestStaffFirstName {
    #[prost(string, tag = "1")]
    pub tenant_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub first_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseStaffByFirstName {
    #[prost(message, repeated, tag = "1")]
    pub staff: ::prost::alloc::vec::Vec<Staff>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseAddressByStaffId {
    #[prost(message, repeated, tag = "1")]
    pub address: ::prost::alloc::vec::Vec<Address>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseContactsByStaffId {
    #[prost(message, repeated, tag = "1")]
    pub contacts: ::prost::alloc::vec::Vec<Contact>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestStaffTypes {
    #[prost(string, tag = "1")]
    pub tenant_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseStaffTypes {
    #[prost(message, repeated, tag = "1")]
    pub staff_types: ::prost::alloc::vec::Vec<StaffType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestContactTypes {
    #[prost(string, tag = "1")]
    pub tenant_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseContactTypes {
    #[prost(message, repeated, tag = "1")]
    pub contact_types: ::prost::alloc::vec::Vec<ContactType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestStaffUpsert {
    #[prost(string, tag = "1")]
    pub tenant_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub staff: ::core::option::Option<Staff>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseStaffUpsert {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestAddressUpsert {
    #[prost(string, tag = "1")]
    pub tenant_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub staff_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub address: ::core::option::Option<Address>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseAddressUpsert {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Staff {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub first_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub last_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub email_address: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "5")]
    pub vehicle_registration: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "6")]
    pub staff_type_id: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub staff_type: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub contractor_id: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub sex: ::prost::alloc::string::String,
    #[prost(float, tag = "10")]
    pub hourly_rate: f32,
    #[prost(bool, tag = "11")]
    pub active: bool,
    #[prost(message, repeated, tag = "12")]
    pub address: ::prost::alloc::vec::Vec<Address>,
    #[prost(message, repeated, tag = "13")]
    pub contacts: ::prost::alloc::vec::Vec<Contact>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contact_type_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub contact_type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub contact: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub primary: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub street_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub suburb: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub post_code: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub state: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub country: ::prost::alloc::string::String,
    #[prost(bool, tag = "7")]
    pub primary: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContactType {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contact_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaffType {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub staff_type: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod staff_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct StaffServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StaffServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> StaffServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> StaffServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            StaffServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn get_staff_by_staff_id(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestStaffById>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseStaffById>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serverstaff.StaffService/GetStaffByStaffId",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("serverstaff.StaffService", "GetStaffByStaffId"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_staff_by_first_name(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestStaffFirstName>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseStaffByFirstName>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serverstaff.StaffService/GetStaffByFirstName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("serverstaff.StaffService", "GetStaffByFirstName"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_address_by_staff_id(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestStaffById>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseAddressByStaffId>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serverstaff.StaffService/GetAddressByStaffId",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("serverstaff.StaffService", "GetAddressByStaffId"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_contacts_by_staff_id(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestStaffById>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseContactsByStaffId>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serverstaff.StaffService/GetContactsByStaffId",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("serverstaff.StaffService", "GetContactsByStaffId"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_all_staff_type(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestStaffTypes>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseStaffTypes>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serverstaff.StaffService/GetAllStaffType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("serverstaff.StaffService", "GetAllStaffType"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_all_contact_type(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestContactTypes>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseContactTypes>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serverstaff.StaffService/GetAllContactType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("serverstaff.StaffService", "GetAllContactType"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn upsert_staff(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestStaffUpsert>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseStaffUpsert>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serverstaff.StaffService/UpsertStaff",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("serverstaff.StaffService", "UpsertStaff"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn upsert_address(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestAddressUpsert>,
        ) -> std::result::Result<
            tonic::Response<super::RequestAddressUpsert>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serverstaff.StaffService/UpsertAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("serverstaff.StaffService", "UpsertAddress"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod staff_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StaffServiceServer.
    #[async_trait]
    pub trait StaffService: Send + Sync + 'static {
        async fn get_staff_by_staff_id(
            &self,
            request: tonic::Request<super::RequestStaffById>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseStaffById>,
            tonic::Status,
        >;
        async fn get_staff_by_first_name(
            &self,
            request: tonic::Request<super::RequestStaffFirstName>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseStaffByFirstName>,
            tonic::Status,
        >;
        async fn get_address_by_staff_id(
            &self,
            request: tonic::Request<super::RequestStaffById>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseAddressByStaffId>,
            tonic::Status,
        >;
        async fn get_contacts_by_staff_id(
            &self,
            request: tonic::Request<super::RequestStaffById>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseContactsByStaffId>,
            tonic::Status,
        >;
        async fn get_all_staff_type(
            &self,
            request: tonic::Request<super::RequestStaffTypes>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseStaffTypes>,
            tonic::Status,
        >;
        async fn get_all_contact_type(
            &self,
            request: tonic::Request<super::RequestContactTypes>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseContactTypes>,
            tonic::Status,
        >;
        async fn upsert_staff(
            &self,
            request: tonic::Request<super::RequestStaffUpsert>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseStaffUpsert>,
            tonic::Status,
        >;
        async fn upsert_address(
            &self,
            request: tonic::Request<super::RequestAddressUpsert>,
        ) -> std::result::Result<
            tonic::Response<super::RequestAddressUpsert>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct StaffServiceServer<T: StaffService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: StaffService> StaffServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StaffServiceServer<T>
    where
        T: StaffService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/serverstaff.StaffService/GetStaffByStaffId" => {
                    #[allow(non_camel_case_types)]
                    struct GetStaffByStaffIdSvc<T: StaffService>(pub Arc<T>);
                    impl<
                        T: StaffService,
                    > tonic::server::UnaryService<super::RequestStaffById>
                    for GetStaffByStaffIdSvc<T> {
                        type Response = super::ResponseStaffById;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestStaffById>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StaffService>::get_staff_by_staff_id(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStaffByStaffIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serverstaff.StaffService/GetStaffByFirstName" => {
                    #[allow(non_camel_case_types)]
                    struct GetStaffByFirstNameSvc<T: StaffService>(pub Arc<T>);
                    impl<
                        T: StaffService,
                    > tonic::server::UnaryService<super::RequestStaffFirstName>
                    for GetStaffByFirstNameSvc<T> {
                        type Response = super::ResponseStaffByFirstName;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestStaffFirstName>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StaffService>::get_staff_by_first_name(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStaffByFirstNameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serverstaff.StaffService/GetAddressByStaffId" => {
                    #[allow(non_camel_case_types)]
                    struct GetAddressByStaffIdSvc<T: StaffService>(pub Arc<T>);
                    impl<
                        T: StaffService,
                    > tonic::server::UnaryService<super::RequestStaffById>
                    for GetAddressByStaffIdSvc<T> {
                        type Response = super::ResponseAddressByStaffId;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestStaffById>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StaffService>::get_address_by_staff_id(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAddressByStaffIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serverstaff.StaffService/GetContactsByStaffId" => {
                    #[allow(non_camel_case_types)]
                    struct GetContactsByStaffIdSvc<T: StaffService>(pub Arc<T>);
                    impl<
                        T: StaffService,
                    > tonic::server::UnaryService<super::RequestStaffById>
                    for GetContactsByStaffIdSvc<T> {
                        type Response = super::ResponseContactsByStaffId;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestStaffById>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StaffService>::get_contacts_by_staff_id(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetContactsByStaffIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serverstaff.StaffService/GetAllStaffType" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllStaffTypeSvc<T: StaffService>(pub Arc<T>);
                    impl<
                        T: StaffService,
                    > tonic::server::UnaryService<super::RequestStaffTypes>
                    for GetAllStaffTypeSvc<T> {
                        type Response = super::ResponseStaffTypes;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestStaffTypes>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StaffService>::get_all_staff_type(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllStaffTypeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serverstaff.StaffService/GetAllContactType" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllContactTypeSvc<T: StaffService>(pub Arc<T>);
                    impl<
                        T: StaffService,
                    > tonic::server::UnaryService<super::RequestContactTypes>
                    for GetAllContactTypeSvc<T> {
                        type Response = super::ResponseContactTypes;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestContactTypes>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StaffService>::get_all_contact_type(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllContactTypeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serverstaff.StaffService/UpsertStaff" => {
                    #[allow(non_camel_case_types)]
                    struct UpsertStaffSvc<T: StaffService>(pub Arc<T>);
                    impl<
                        T: StaffService,
                    > tonic::server::UnaryService<super::RequestStaffUpsert>
                    for UpsertStaffSvc<T> {
                        type Response = super::ResponseStaffUpsert;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestStaffUpsert>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StaffService>::upsert_staff(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpsertStaffSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serverstaff.StaffService/UpsertAddress" => {
                    #[allow(non_camel_case_types)]
                    struct UpsertAddressSvc<T: StaffService>(pub Arc<T>);
                    impl<
                        T: StaffService,
                    > tonic::server::UnaryService<super::RequestAddressUpsert>
                    for UpsertAddressSvc<T> {
                        type Response = super::RequestAddressUpsert;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestAddressUpsert>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StaffService>::upsert_address(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpsertAddressSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: StaffService> Clone for StaffServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: StaffService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: StaffService> tonic::server::NamedService for StaffServiceServer<T> {
        const NAME: &'static str = "serverstaff.StaffService";
    }
}
