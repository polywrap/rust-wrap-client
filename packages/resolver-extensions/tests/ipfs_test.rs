use polywrap_core::uri::Uri;
use polywrap_msgpack::msgpack;
use polywrap_resolver_extensions::uri_resolver_wrapper::MaybeUriOrManifest;
mod utils;
use utils::get_client_with_module;

use crate::utils::load_wrap;

#[test]
fn client_sanity() {
    let (_manifest, module) = load_wrap("./build");
    let client = get_client_with_module(&module);

    let result = client.invoke::<Option<MaybeUriOrManifest>>(
        &Uri::try_from("wrap://ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1").unwrap(),
        "tryResolveUri", 
        Some(&msgpack!({
            "authority": "ipfs",
            "path": "QmfVRmLRPt6A3tLW7dgsPztUULUjFWys7Kz7Ytjenc2rHV"
        })),
        None,
        None,
    ).unwrap();

    assert_eq!(result.unwrap().manifest, None);
}