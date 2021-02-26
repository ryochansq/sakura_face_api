#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use sakura_face_api::http_client::{detect_and_findsimilars, findsimilars};

    #[actix_rt::test]
    async fn success_detect_and_findsimilars() {
        dotenv().ok();
        let url = "https://www.sakuragakuin.jp/_img/graduates/graduates_idx_fujihira.jpg";
        let res = detect_and_findsimilars(url).await;
        println!("=== response ===");
        println!("{:?}", res);
        println!("=== response ===");
    }

    #[actix_rt::test]
    async fn fail_detect() {
        dotenv().ok();
        let wrong_url = "https://www.sakuragakuin.jp";
        let res = detect_and_findsimilars(wrong_url).await;
        println!("=== response ===");
        println!("{:?}", res);
        println!("=== response ===");
    }

    #[actix_rt::test]
    async fn many_faces() {
        dotenv().ok();
        let url = "https://www.sakuragakuin.jp/image.php?tieid=46";
        let face_list = detect_and_findsimilars(url).await.unwrap().face_list;
        assert_eq!(face_list.len(), 8);
        let res = findsimilars(&face_list[0].faceId).await;
        println!("=== response ===");
        println!("{:?}", res);
        println!("=== response ===");
    }
}
