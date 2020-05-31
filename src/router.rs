use warp::{Filter, Reply};

use crate::handlers;

pub fn routes() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    handlers::ping::routes().or(handlers::echo::routes())
}
