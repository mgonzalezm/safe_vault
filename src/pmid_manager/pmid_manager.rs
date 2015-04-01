// Copyright 2015 MaidSafe.net limited
// This MaidSafe Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
// By contributing code to the MaidSafe Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0, found in the root
// directory of this project at LICENSE, COPYING and CONTRIBUTOR respectively and also
// available at: http://www.maidsafe.net/licenses
// Unless required by applicable law or agreed to in writing, the MaidSafe Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS
// OF ANY KIND, either express or implied.
// See the Licences for the specific language governing permissions and limitations relating to
// use of the MaidSafe
// Software.

#![allow(dead_code)]

extern crate routing;
extern crate maidsafe_types;

mod database;

pub struct PmidManager {
  db_ : database::PmidManagerDatabase
}

impl PmidManager {
  pub fn new() -> PmidManager {
    PmidManager { db_: database::PmidManagerDatabase::new() }
  }

  pub fn handle_put(&mut self, dest_address: &routing::DestinationAddress, data : &Vec<u8>) ->Result<routing::Action, routing::RoutingError> {
    if self.db_.put_data(&dest_address.dest, data.len() as u64) {
      let mut destinations : Vec<routing::DhtIdentity> = Vec::new();
      destinations.push(routing::DhtIdentity { id: routing::types::vector_as_u8_64_array(dest_address.dest.clone()) });
      Ok(routing::Action::SendOn(destinations))
    } else {
      Err(routing::RoutingError::InvalidRequest)
    }
  }
}