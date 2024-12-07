use crate::Category;

pub(crate) fn status_code(category: Option<&Category>) -> i32 {
    let Some(category) = category else {
        return -32000;
    };

    match category {
        Category::BadRequest => -32600,

        Category::Custom {
            json_rpc_status, ..
        } => *json_rpc_status,
    }
}
