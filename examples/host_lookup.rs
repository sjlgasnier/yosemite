// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use tracing_subscriber::prelude::*;

// Asynchronous host lookup:
//    cargo run --example host_lookup -- <host>
//
// Synchronous host lookup:
//    cargo run --example host_lookup --no-default-features --features sync -- <host>

#[cfg(all(feature = "tokio", not(feature = "sync")))]
#[tokio::main]
async fn main() {
    use yosemite::RouterApi;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .unwrap();

    let host = std::env::args().nth(1).expect("host");
    let result = RouterApi::default().lookup_name(&host).await.unwrap();

    tracing::info!("destination = {result:?}");
}

#[cfg(all(feature = "sync", not(feature = "tokio")))]
fn main() {
    use yosemite::RouterApi;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .unwrap();

    let host = std::env::args().nth(1).expect("host");
    let result = RouterApi::default().lookup_name(&host).unwrap();

    tracing::info!("destination = {result:?}");
}
