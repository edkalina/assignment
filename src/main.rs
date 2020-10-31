mod types;

use serde::Deserialize;
use types::{Input, Substitution};
use warp::Filter;

#[derive(Debug, Deserialize)]
struct AssignmentRequest {
    input: String,
    substitution: String,
}

#[tokio::main]
async fn main() {
    let api = warp::post().and(warp::path("api"));

    let assignment =
        warp::path("assignment")
            .and(warp::body::json())
            .map(|req: AssignmentRequest| {
                // TODO: handle errors
                let subst = Substitution::from_str(&req.substitution).unwrap();
                let input = Input::from_str(&req.input).unwrap();
                subst.get_output(&input).unwrap().to_string()
            });

    let api_routes = api.and(assignment);

    let root = warp::get()
        .and(warp::path::end())
        .map(|| warp::reply::html(include_str!("index.html")));

    let routes = root.or(api_routes);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await
}
