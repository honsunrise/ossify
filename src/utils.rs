use crate::ser::percent_encode;

#[inline]
pub fn escape_path(url_path: &str) -> String {
    url_path
        .split('/')
        .map(|segment| percent_encode(segment))
        .collect::<Vec<_>>()
        .join("/")
}
