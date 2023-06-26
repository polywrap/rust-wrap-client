use polywrap_client::client::PolywrapClient;
use polywrap_client_default_config::SystemClientConfig;
use polywrap_core::uri::Uri;

const URI: &str =
    "http/https://raw.githubusercontent.com/polywrap/client-readiness/main/wraps/public";

// #[test]
// fn sanity() {
//     let client = PolywrapClient::new(SystemClientConfig::default().into());

//     let result = client
//         .invoke::<u32>(
//             &Uri::try_from(URI).unwrap(),
//             "i8Method",
//             Some(&msgpack!({
//                 "first": 2,
//                 "second": 40
//             })),
//             None,
//             None,
//         )
//         .unwrap();

//     assert_eq!(result, 42);
// }
