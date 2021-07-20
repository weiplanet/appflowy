use crate::{
    entities::{app::parser::AppId, view::parser::*},
    errors::{ErrorBuilder, WorkspaceError, WorkspaceErrorCode},
    sql_tables::view::ViewType,
};
use flowy_derive::{ProtoBuf, ProtoBuf_Enum};
use std::convert::TryInto;

#[derive(Debug, ProtoBuf_Enum)]
pub enum ViewTypeIdentifier {
    Docs = 0,
}

impl std::default::Default for ViewTypeIdentifier {
    fn default() -> Self { ViewTypeIdentifier::Docs }
}

#[derive(Default, ProtoBuf)]
pub struct CreateViewRequest {
    #[pb(index = 1)]
    pub app_id: String,

    #[pb(index = 2)]
    pub name: String,

    #[pb(index = 3)]
    pub desc: String,

    #[pb(index = 4, one_of)]
    pub thumbnail: Option<String>,

    #[pb(index = 5)]
    pub view_type: ViewTypeIdentifier,
}

pub struct CreateViewParams {
    pub app_id: String,
    pub name: String,
    pub desc: String,
    pub thumbnail: String,
    pub view_type: ViewType,
}

impl TryInto<CreateViewParams> for CreateViewRequest {
    type Error = WorkspaceError;

    fn try_into(self) -> Result<CreateViewParams, Self::Error> {
        let name = ViewName::parse(self.name)
            .map_err(|e| {
                ErrorBuilder::new(WorkspaceErrorCode::ViewNameInvalid)
                    .msg(e)
                    .build()
            })?
            .0;

        let app_id = AppId::parse(self.app_id)
            .map_err(|e| {
                ErrorBuilder::new(WorkspaceErrorCode::AppIdInvalid)
                    .msg(e)
                    .build()
            })?
            .0;

        let thumbnail = match self.thumbnail {
            None => "".to_string(),
            Some(thumbnail) => {
                ViewThumbnail::parse(thumbnail)
                    .map_err(|e| {
                        ErrorBuilder::new(WorkspaceErrorCode::ViewThumbnailName)
                            .msg(e)
                            .build()
                    })?
                    .0
            },
        };

        let view_type = ViewTypeCheck::parse(self.view_type).unwrap().0;
        Ok(CreateViewParams {
            app_id,
            name,
            desc: self.desc,
            thumbnail,
            view_type,
        })
    }
}

#[derive(ProtoBuf, Default, Debug)]
pub struct View {
    #[pb(index = 1)]
    pub id: String,

    #[pb(index = 2)]
    pub app_id: String,

    #[pb(index = 3)]
    pub name: String,

    #[pb(index = 4)]
    pub desc: String,

    #[pb(index = 5)]
    pub view_type: ViewTypeIdentifier,
}