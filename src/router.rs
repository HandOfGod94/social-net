use warp::{Filter, Reply};

use crate::echo;
use crate::ping;

pub fn routes() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    ping::routes().or(echo::routes())
}
