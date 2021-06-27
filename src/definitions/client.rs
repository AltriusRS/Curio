use std::collections::HashMap;
use crate::definitions::connection::Connection;
use crate::definitions::domain::Domain;

pub struct Client {
    connections: HashMap<Domain, Connection>
}