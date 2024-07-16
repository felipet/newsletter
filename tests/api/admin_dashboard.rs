//! test/api/admin_dashboard.rs

use crate::helpers::{assert_is_redirect_to, spawn_app};

#[actix_web::test]
async fn you_moust_be_logged_in_to_access_the_admin_dashboard() {
    // Prepare
    let test_app = spawn_app().await;

    // Test
    let response = test_app.get_admin_dashboard().await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}
