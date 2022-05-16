#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionAckEvent {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionAckResponse {
    #[prost(bool, tag="1")]
    pub connected: bool,
    #[prost(float, tag="2")]
    pub ping: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveStatsEvent {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveStatsResponse {
    #[prost(string, tag="1")]
    pub product: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub commit_sha: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub build_date: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub build_flavour: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceUuidEvent {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceUuidResponse {
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod analytics_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct AnalyticsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AnalyticsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AnalyticsClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AnalyticsClient<InterceptedService<T, F>>
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
            AnalyticsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn connection_ack(
            &mut self,
            request: impl tonic::IntoRequest<super::ConnectionAckEvent>,
        ) -> Result<tonic::Response<super::ConnectionAckResponse>, tonic::Status> {
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
                "/noelware.analytics.grpc.server.v1.connection.Analytics/ConnectionAck",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn retrieve_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveStatsEvent>,
        ) -> Result<tonic::Response<super::RetrieveStatsResponse>, tonic::Status> {
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
                "/noelware.analytics.grpc.server.v1.connection.Analytics/RetrieveStats",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_instance_uuid(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceUuidEvent>,
        ) -> Result<tonic::Response<super::GetInstanceUuidResponse>, tonic::Status> {
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
                "/noelware.analytics.grpc.server.v1.connection.Analytics/GetInstanceUUID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
