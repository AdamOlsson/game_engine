pub mod post_process_filter;
pub mod post_process_pipeline;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum PostProcessFilterId {
    Gray,
}


impl std::fmt::Display for PostProcessFilterId{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostProcessFilterId::Gray => write!(f, "PostProcessFilterId::Gray "),
        }
    }
}
