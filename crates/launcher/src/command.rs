use std::{fmt::Debug, sync::Arc};

use gpui::*;
#[derive(Debug, Clone)]
pub struct Command {
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
    pub subtitle: Option<String>,
    pub keywords: Vec<String>,
    pub handlers: Arc<dyn CommandHandler>,
}

#[derive(Debug, Clone)]
pub struct CommandContext {
    pub window: WindowHandle<()>,
}

#[derive(Debug, Clone)]
pub enum CommandResult {
    Success,
    SuccessKeepOpen,
    Error(String),
    Navigate(Vec<Command>),
}

pub trait CommandHandler: Send + Sync + Debug {
    fn execute(&self, ctx: &CommandContext) -> CommandResult;
}

impl Command {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        icon: Option<String>,
        subtitle: Option<String>,
        handler: impl CommandHandler + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon,
            subtitle,
            keywords: Vec::new(),
            handlers: Arc::new(handler),
        }
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = keywords;
        self
    }

    pub fn matches(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        self.title.to_lowercase().contains(&query)
            || self
                .keywords
                .iter()
                .any(|k| k.to_lowercase().contains(&query))
            || self
                .subtitle
                .as_ref()
                .map_or(false, |s| s.to_lowercase().contains(&query))
    }
}
