//!
//! Support for Slack Files API methods
//!

use rsb_derive::Builder;
use rvstruct::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::blocks::{SlackBlock, SlackBlockText};
use crate::models::*;
use crate::ratectl::*;
use crate::scroller::*;
use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};
use futures::future::{BoxFuture, FutureExt};
use lazy_static::lazy_static;
use std::collections::HashMap;
use url::Url;

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/files.upload
    ///
    pub async fn files_upload(&self, req: &SlackApiFilesUploadRequest) -> ClientResult<()> {
        self.http_session_api
            .http_post("files.upload", req, Some(&SLACK_TIER2_METHOD_CONFIG))
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesUploadRequest {
    pub channels: Vec<SlackChannelId>,
    pub content: Option<String>,
    pub file: Option<Vec<u8>>,
    pub filename: Option<String>,
    pub filetype: Option<String>,
    pub initial_comment: Option<String>,
    pub thread_ts: Option<SlackTs>,
    pub title: Option<String>,
}
