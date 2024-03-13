use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AtprotoClient;
/**You should use this struct via [`AtprotoClient::com_atproto_temp_upgrade_repo_version`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComAtprotoTempUpgradeRepoVersionRequest {
    pub did: String,
    pub force: Option<bool>,
}
impl ComAtprotoTempUpgradeRepoVersionRequest {}
impl FluentRequest<'_, ComAtprotoTempUpgradeRepoVersionRequest> {
    ///Set the value of the force field.
    pub fn force(mut self, force: bool) -> Self {
        self.params.force = Some(force);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ComAtprotoTempUpgradeRepoVersionRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/com.atproto.temp.upgradeRepoVersion";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "did" : self.params.did }));
            if let Some(ref unwrapped) = self.params.force {
                r = r.json(json!({ "force" : unwrapped }));
            }
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}