use crate::Category;

pub(crate) fn status_code(category: Option<&Category>) -> u16 {
    let Some(category) = category else {
        return 500;
    };

    match category {
        Category::BadRequest => 400,

        Category::Custom { http_status, .. } => *http_status,
    }
}
