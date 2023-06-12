use std::sync::Arc;

use polywrap_client::client::PolywrapClient;
use polywrap_tests_utils::helpers::get_tests_path_string;
use polywrap_core::{
    uri::Uri,
    client::ClientConfig,
    resolution::uri_resolver::UriResolver
};
use polywrap_client_builder::types::BuilderConfig;
use polywrap_resolvers::{
    resolver_vec,
    static_resolver::{StaticResolver, StaticResolverLike},
    recursive_resolver::RecursiveResolver,
    extendable_uri_resolver::ExtendableUriResolver
};

pub fn client_without_cache() -> PolywrapClient {
    let builder: BuilderConfig = polywrap_client_default_config::build();
    let config = build_resolver_without_cache(builder);
    PolywrapClient::new(config)
}

fn build_resolver_without_cache(builder: BuilderConfig) -> ClientConfig {
    let mut static_resolvers: Vec<StaticResolverLike> = vec![];

    if let Some(wrappers) = builder.wrappers {
        for (uri, w) in wrappers {
            static_resolvers.push(StaticResolverLike::Wrapper(uri, w));
        }
    }

    if let Some(packages) = builder.packages {
        for (uri, p) in packages {
            static_resolvers.push(StaticResolverLike::Package(uri, p));
        }
    }

    if let Some(redirects) = builder.redirects {
        for r in redirects {
            static_resolvers.push(StaticResolverLike::Redirect(r));
        }
    }

    ClientConfig {
        envs: builder.envs.clone(),
        interfaces: builder.interfaces.clone(),
        resolver: Arc::new(RecursiveResolver::from(
            resolver_vec![
                StaticResolver::from(static_resolvers),
                ExtendableUriResolver::new(None),
            ])
        ),
    }
}

pub struct UriCase {
    pub id: String,
    pub uri: Uri,
}

pub fn prepare_uris() -> Vec<UriCase> {
    let path = get_tests_path_string();
    let fs_uri = UriCase {
        id: "fs_uri".to_string(),
        uri: Uri::try_from(format!("fs/{path}/subinvoke/00-subinvoke/implementations/rs")).unwrap(),
    };
    let http_uri = UriCase {
        id: "http_uri".to_string(),
        uri: Uri::try_from(format!("http/https://raw.githubusercontent.com/polywrap/wrap-test-harness/master/cases/subinvoke/00-subinvoke/implementations/rs")).unwrap(),
    };
    let ipfs_uri = UriCase {
        id: "ipfs_uri".to_string(),
        uri: Uri::try_from("/ipfs/Qmf7jukQhTQekdSgKfdnFtB6ERTN6V7aT4oYpzesDyr2cS").unwrap(),
    };
    vec![fs_uri, http_uri, ipfs_uri]
}
