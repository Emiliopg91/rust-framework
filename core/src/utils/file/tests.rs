use crate::utils::file::FileUtils;

#[test]
fn test_01_get_tmp_file() {
    assert!(FileUtils::new_tmp_file().is_ok());
}
