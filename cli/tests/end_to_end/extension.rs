use phylum_cli::commands::extensions::permissions::Permissions;

use crate::common::{create_lockfile, create_project, TestCli};

/// Test Phylum API functions.
#[tokio::test]
pub async fn get_user_info() {
    let test_cli = TestCli::builder().with_config(true).build();

    test_cli
        .extension("console.log(await PhylumApi.getUserInfo())")
        .build()
        .run()
        .success()
        .stdout(predicates::str::contains("email"));
}

#[tokio::test]
pub async fn get_access_token() {
    let test_cli = TestCli::builder().with_config(true).build();

    test_cli
        .extension("console.log(await PhylumApi.getAccessToken())")
        .build()
        .run()
        .success()
        .stdout(predicates::str::contains("ey"));
}

#[tokio::test]
pub async fn get_refresh_token() {
    let test_cli = TestCli::builder().with_config(true).build();

    test_cli
        .extension("console.log(await PhylumApi.getRefreshToken())")
        .build()
        .run()
        .success()
        .stdout(predicates::str::contains("ey"));
}

#[tokio::test]
pub async fn get_package_details() {
    let test_cli = TestCli::builder().with_config(true).build();

    test_cli
        .extension("console.log(await PhylumApi.getPackageDetails('express', '4.18.1', 'npm'))")
        .build()
        .run()
        .success()
        .stdout(predicates::str::contains("vulnerability: 1"));
}

#[tokio::test]
pub async fn get_project_details() {
    let test_cli = TestCli::builder().with_config(true).build();

    let project = create_project().await;
    let permissions =
        Permissions { net: Some(vec![String::from("123")]).into(), ..Permissions::default() };

    let project_details = format!("console.log(await PhylumApi.getProjectDetails({project:?}))");
    test_cli
        .extension(&project_details)
        .with_permissions(permissions)
        .build()
        .run()
        .success()
        .stdout(predicates::str::contains(r#"name: "integration-tests""#));
}

#[tokio::test]
pub async fn parse_lockfile() {
    let test_cli = TestCli::builder().with_config(true).build();

    let lockfile = create_lockfile(test_cli.temp_path());
    let lockfile_str = lockfile.to_string_lossy().into_owned();
    let permissions =
        Permissions { read: Some(vec![lockfile_str]).into(), ..Permissions::default() };

    let parse_lockfile = format!(
        "const packages = await PhylumApi.parseLockfile({lockfile:?}, 'yarn');
             console.log(packages);",
    );
    test_cli
        .extension(&parse_lockfile)
        .with_permissions(permissions)
        .build()
        .run()
        .success()
        .stdout(predicates::str::contains(
            r#"[ { name: "accepts", version: "1.3.8", type: "npm" } ]"#,
        ));
}

#[tokio::test]
pub async fn get_job_status() {
    let test_cli = TestCli::builder().with_config(true).build();

    let lockfile = create_lockfile(test_cli.temp_path());
    let project = create_project().await;
    let lockfile_str = lockfile.to_string_lossy().into_owned();
    let permissions =
        Permissions { read: Some(vec![lockfile_str]).into(), ..Permissions::default() };

    let analyze = format!(
        "const jobId = await PhylumApi.analyze({lockfile:?}, {project:?});
             console.log(await PhylumApi.getJobStatus(jobId));"
    );
    test_cli
        .extension(&analyze)
        .with_permissions(permissions)
        .build()
        .run()
        .success()
        .stdout(predicates::str::contains(r#"name: "accepts""#));
}

/// Ensure shared state is async and thread safe.
#[test]
pub fn async_state() {
    let test_cli = TestCli::builder().with_config(true).build();

    test_cli
        .extension(
            r#"
            const promises = [];
            promises.push(PhylumApi.getUserInfo());
            promises.push(PhylumApi.getUserInfo());
            promises.push(PhylumApi.getUserInfo());
            promises.push(PhylumApi.getUserInfo());
            promises.push(PhylumApi.getUserInfo());
            await Promise.all(promises);
        "#,
        )
        .build()
        .run()
        .success();
}