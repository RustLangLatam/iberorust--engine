use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams, Debug, Default)]
#[into_params(parameter_in = Query)]
pub struct PaginationAndFilters {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub search: Option<String>,
    pub role: Option<String>,
    pub tag: Option<String>,
}
