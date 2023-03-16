use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::local::blocking::Client;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
struct Plan {
    id: Option<i32>,
    title: String,
    from_hr: Option<i32>,
    from_min: Option<i32>,
    to_hr: Option<i32>,
    to_min: Option<i32>,
    started: Option<bool>,
}

fn test(base: &str, stage: AdHoc) {
    // Number of posts we're going to create/read/delete.
    const N: usize = 20;

    // NOTE: If we had more than one test running concurently that dispatches
    // DB-accessing requests, we'd need transactions or to serialize all tests.
    let client = Client::tracked(rocket::build().attach(stage)).unwrap();

    // Clear everything from the database.
    assert_eq!(client.delete(base).dispatch().status(), Status::Ok);
    assert_eq!(
        client.get(base).dispatch().into_json::<Vec<i64>>(),
        Some(vec![])
    );

    // Add some random posts, ensure they're listable and readable.
    for i in 1..=N {
        let title = format!("My Post - {}", i);
        let post = Plan {
            id: None,
            title: title.clone(),
            from_hr: Some(1),
            from_min: Some(2),
            to_hr: Some(3),
            to_min: Some(4),
            started: Some(false),
        };

        // Create a new post.
        let response = client.post(base).json(&post).dispatch().into_json::<Plan>();
        assert_eq!(response.unwrap(), post);

        // Ensure the index shows one more post.
        let list = client
            .get(base)
            .dispatch()
            .into_json::<Vec<Plan>>()
            .unwrap();
        assert_eq!(list.len(), i);

        // The last in the index is the new one; ensure contents match.
        let last = list.last().unwrap();
        assert!(last.id.is_some());
        assert!(post.id.is_none());
        let mut comp_post = post.clone();
        comp_post.id = last.id;
        assert_eq!(*last, comp_post);
    }

    // Now delete all of the posts.
    for _ in 1..=N {
        // Get a valid ID from the index.
        let list = client
            .get(base)
            .dispatch()
            .into_json::<Vec<Plan>>()
            .unwrap();
        let plan = list.get(0).expect("have post");

        // Delete that post.
        let response = client
            .delete(format!("{}/{}", base, plan.id.unwrap()))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    // Ensure they're all gone.
    let list = client
        .get(base)
        .dispatch()
        .into_json::<Vec<Plan>>()
        .unwrap();
    assert!(list.is_empty());

    // Trying to delete should now 404.
    let response = client.delete(format!("{}/{}", base, 1)).dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_diesel() {
    test("/", crate::plan::stage())
}
