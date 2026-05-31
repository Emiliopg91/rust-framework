use crate::shell::Shell;

#[tokio::test]
async fn test_01_run_program_ok() {
    println!();
    assert!(
        Shell::command("which")
            .unwrap()
            .arg("ls")
            .run(true)
            .await
            .is_ok()
    )
}

#[tokio::test]
async fn test_02_run_program_err() {
    println!();
    assert!(
        Shell::command("which")
            .unwrap()
            .arg("lss")
            .run(true)
            .await
            .is_err()
    );
}
