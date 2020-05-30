use warp::{get, path, post};
use warp::{Filter, Reply};

use crate::echo;
use crate::ping;

pub fn routes() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let gets = get().and(path!("ping").and_then(ping::handler));

    let posts = post().and(path!("echo").and(echo::json_body()).and_then(echo::handler));

    gets.or(posts)
}
