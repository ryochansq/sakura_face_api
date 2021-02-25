#[cfg(test)]
mod tests {
  use dotenv::dotenv;
  use sakura_face_api::http_client::post_face;

  #[actix_rt::test]
  async fn test_post_face() {
    dotenv().ok();
    let url = "https://www.sakuragakuin.jp/_img/graduates/graduates_idx_fujihira.jpg";
    post_face(url).await;
  }
}
