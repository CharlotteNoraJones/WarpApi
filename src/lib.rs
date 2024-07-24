mod filters {
    use super::handlers;
    use warp::Filter;

    pub fn list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("holodeck")
            .and(warp::get())
            .and_then(handlers::handle_list)
    }
}

mod handlers {
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn handle_list() -> Result<impl warp::Reply, Infallible> {
        Ok(StatusCode::ACCEPTED)
    }
}

#[cfg(test)]
mod tests {
    use super::filters;
    use warp::http::StatusCode;
    use warp::test::request;

    #[tokio::test]
    async fn it_works() {
        let api = filters::list();

        let response = request().method("GET").path("/holodeck").reply(&api).await;

        assert_eq!(response.status(), StatusCode::ACCEPTED);
    }
}
