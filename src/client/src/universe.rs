// Copyright 2022 The Engula Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use engula_apis::*;

use crate::{Client, Database, Result};

#[derive(Clone)]
pub struct Universe {
    inner: Arc<UniverseInner>,
}

impl Universe {
    pub async fn connect(url: impl Into<String>) -> Result<Universe> {
        let client = Client::connect(url.into()).await?;
        let inner = UniverseInner { client };
        Ok(Universe {
            inner: Arc::new(inner),
        })
    }

    pub fn database(&self, name: &str) -> Database {
        self.inner.new_database(name.to_owned())
    }

    pub async fn create_database(&self, name: &str) -> Result<Database> {
        let desc = DatabaseDesc {
            name: name.to_owned(),
            ..Default::default()
        };
        let req = CreateDatabaseRequest { desc: Some(desc) };
        let req = database_request_union::Request::CreateDatabase(req);
        self.inner.database_union_call(req).await?;
        Ok(self.database(name))
    }

    pub async fn delete_database(&self, name: &str) -> Result<()> {
        let req = DeleteDatabaseRequest {
            name: name.to_owned(),
        };
        let req = database_request_union::Request::DeleteDatabase(req);
        self.inner.database_union_call(req).await?;
        Ok(())
    }
}

struct UniverseInner {
    client: Client,
}

impl UniverseInner {
    fn new_database(&self, name: String) -> Database {
        Database::new(name, self.client.clone())
    }

    async fn database_union_call(
        &self,
        req: database_request_union::Request,
    ) -> Result<database_response_union::Response> {
        self.client.database_union(req).await
    }
}