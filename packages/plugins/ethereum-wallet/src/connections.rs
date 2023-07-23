use std::collections::HashMap;

use crate::{
    connection::Connection,
    networks::{from_alias, KnownNetwork},
};

use super::wrap::types::Connection as SchemaConnection;

#[derive(Debug, Clone)]
pub struct Connections {
    pub connections: HashMap<String, Connection>,
    pub default_network: String,
}

impl Connections {
    pub fn new(connections: HashMap<String, Connection>, default_network: Option<String>) -> Self {
        let mainnet_string = String::from("mainnet");
        let (default_network, connections) = if let Some(default_network) = default_network {
            (default_network, connections)
        } else if connections.get("mainnet").is_some() {
            (mainnet_string, connections)
        } else {
            let mainnet_connection = Connection::from_network(KnownNetwork::Mainnet, None).unwrap();
            let connections = HashMap::from([("mainnet".to_string(), mainnet_connection)]);
            (mainnet_string, connections)
        };

        Self {
            connections,
            default_network,
        }
    }

    pub fn get_connection(&self, connection: Option<SchemaConnection>) -> Connection {
        match connection {
            Some(connection) => {
                if let Some(network) = connection.network_name_or_chain_id {
                    return if let Some(con) = self.connections.get(&network) {
                        con.clone()
                    } else {
                        Connection::from_network(from_alias(&network).unwrap(), None).unwrap()
                    };
                };

                if let Some(node) = connection.node {
                    Connection::from_node(node, None).unwrap()
                } else {
                    panic!("Node given is not correct")
                }
            }
            None => {
                if let Some(c) = self.connections.get(&self.default_network) {
                    c.clone()
                } else {
                    panic!("{}", format!("Connection: {:#?} not found", connection))
                }
            }
        }
    }
}
