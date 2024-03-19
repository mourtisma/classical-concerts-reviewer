pub struct RepositoryResult<GetModelDto> {
    pub items: Vec<GetModelDto>,
    pub total_count: u64,
    pub num_pages: Option<u64>,
}