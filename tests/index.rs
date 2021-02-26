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
        let url = "https://www.sakuragakuin.jp/_img/club/club_prowrestling_ph.jpg";
        let res = detect_and_findsimilars(url).await.unwrap();
        assert_eq!(res.face_list.len(), 2);
        assert_eq!(res.similar_list.len(), 0);
        let res = findsimilars(&res.face_list[0].faceId).await;
        println!("=== response ===");
        println!("{:?}", res);
        println!("=== response ===");
    }
}
