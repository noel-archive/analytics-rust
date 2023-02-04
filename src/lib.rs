// 🐻‍❄️🐾 analytics-rs: Rust client and server implementation of the Noelware Analytics Protocol
// Copyright (c) 2022-2023 Noelware, LLC. <team@noelware.org>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod pb {
    include!(concat!(
        env!("OUT_DIR"),
        concat!("/noelware.analytics.protobufs.rs")
    ));
}

pub use pb::analytics_client::AnalyticsClient;
pub use pb::analytics_server::Analytics as AnalyticsService;
pub use pb::analytics_server::AnalyticsServer;
pub use pb::BuildFlavour;
pub use pb::ConnectionAckRequest;
pub use pb::ConnectionAckResponse;
pub use pb::ReceiveStatsRequest;
pub use pb::ReceiveStatsResponse;
