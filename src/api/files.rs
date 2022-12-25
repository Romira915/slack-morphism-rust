//!
//! Support for Slack Files API methods
//!

use futures::future::FutureExt;

use mpart_async::client::MultipartRequest;
use mpart_async::filestream::FileStream;
use rsb_derive::Builder;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::*;
use crate::ratectl::*;

use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/files.upload
    ///
    pub async fn files_upload(&self, req: &SlackApiFilesUploadRequest) -> ClientResult<()> {
        self.http_session_api
            .http_post_form_data("files.upload", req, Some(&SLACK_TIER2_METHOD_CONFIG))
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesUploadRequest {
    pub channels: Vec<SlackChannelId>,
    pub content: Option<String>,
    pub file: Option<String>,
    pub filename: Option<String>,
    pub filetype: Option<String>,
    pub initial_comment: Option<String>,
    pub thread_ts: Option<SlackTs>,
    pub title: Option<String>,
}

impl From<SlackApiFilesUploadRequest> for MultipartRequest<FileStream> {
    fn from(value: SlackApiFilesUploadRequest) -> Self {
        let mut multi_part = MultipartRequest::default();

        let channels = value
            .channels
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",");
        multi_part.add_field("channels", &channels);
        if let Some(content) = &value.content {
            multi_part.add_field("content", content);
        }
        if let Some(file) = &value.file {
            multi_part.add_file("file", file);
        }
        if let Some(filename) = &value.filename {
            multi_part.add_field("filename", filename);
        }
        if let Some(filetype) = &value.filetype {
            multi_part.add_field("filetype", filetype);
        }
        if let Some(initial_comment) = &value.filetype {
            multi_part.add_field("initial_comment", initial_comment);
        }
        if let Some(thread_ts) = &value.thread_ts {
            multi_part.add_field("thread_ts", thread_ts.as_ref());
        }
        if let Some(title) = &value.title {
            multi_part.add_field("title", title);
        }

        multi_part
    }
}
