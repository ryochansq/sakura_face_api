#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use sakura_face_api::http_client::post_face;

    #[actix_rt::test]
    #[allow(unused_must_use)]
    async fn success_post_face() {
        dotenv().ok();
        let url = "https://www.sakuragakuin.jp/_img/graduates/graduates_idx_fujihira.jpg";
        post_face(url).await;
    }

    #[actix_rt::test]
    #[should_panic]
    #[allow(unused_must_use)]
    async fn fail_detect() {
        dotenv().ok();
        let wrong_url = "https://www.sakuragakuin.jp";
        post_face(wrong_url).await;
    }
}
