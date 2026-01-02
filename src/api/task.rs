use http::Method;
use serde_json::Value;

use crate::{Result, client::RequestOptions, types::path::PathParam};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct TaskService {
    client: Client,
}

#[cfg(feature = "async")]
impl TaskService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// GET /api/task/
    pub async fn get(&self) -> Result<Value> {
        let segments = ["api", "task"];
        self.client
            .request_json(
                Method::GET,
                &segments,
                Option::<&()>::None,
                Option::<&()>::None,
                RequestOptions::default(),
            )
            .await
    }

    /// GET /api/task/info
    pub async fn get_info(&self) -> Result<Value> {
        let segments = ["api", "task", "info"];
        self.client
            .request_json(
                Method::GET,
                &segments,
                Option::<&()>::None,
                Option::<&()>::None,
                RequestOptions::default(),
            )
            .await
    }

    /// GET /api/task/unique-tasks
    pub async fn get_unique_tasks(&self) -> Result<Value> {
        let segments = ["api", "task", "unique-tasks"];
        self.client
            .request_json(
                Method::GET,
                &segments,
                Option::<&()>::None,
                Option::<&()>::None,
                RequestOptions::default(),
            )
            .await
    }

    /// GET /api/task/{id}
    pub async fn get_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "task", id.as_str()];
        self.client
            .request_json(
                Method::GET,
                &segments,
                Option::<&()>::None,
                Option::<&()>::None,
                RequestOptions::default(),
            )
            .await
    }
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingTaskService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingTaskService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// GET /api/task/
    pub fn get(&self) -> Result<Value> {
        let segments = ["api", "task"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/task/info
    pub fn get_info(&self) -> Result<Value> {
        let segments = ["api", "task", "info"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/task/unique-tasks
    pub fn get_unique_tasks(&self) -> Result<Value> {
        let segments = ["api", "task", "unique-tasks"];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }

    /// GET /api/task/{id}
    pub fn get_by_id(&self, id: impl Into<PathParam>) -> Result<Value> {
        let id = id.into();
        let segments = ["api", "task", id.as_str()];
        self.client.request_json(
            Method::GET,
            &segments,
            Option::<&()>::None,
            Option::<&()>::None,
            RequestOptions::default(),
        )
    }
}
