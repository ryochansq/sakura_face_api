#[cfg(test)]
mod tests {
    use actix_web::web::Bytes;
    use dotenv::dotenv;
    use std::fs::File;
    use std::io::Read;

    use sakura_face_api::http_client::{detect_and_findsimilars, findsimilars};

    #[actix_rt::test]
    async fn success() {
        dotenv().ok();
        let mut file = File::open("tests/images/fujihira.jpg").unwrap();
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf).unwrap();
        let image_binary = Bytes::from(buf);
        let res = detect_and_findsimilars(image_binary).await.unwrap();
        let name = &res.similar_list.get(0).unwrap().name;
        let confidence = res.similar_list.get(0).unwrap().confidence;
        assert_eq!(name, "藤平 華乃");
        assert!(confidence > 0.9);
    }

    #[actix_rt::test]
    async fn unsupported_file() {
        dotenv().ok();
        let mut file = File::open("tests/test.rs").unwrap();
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf).unwrap();
        let image_binary = Bytes::from(buf);
        let res = detect_and_findsimilars(image_binary).await.unwrap_err();
        assert_eq!(res.status_code, 400);
    }

    #[actix_rt::test]
    async fn two_faces() {
        dotenv().ok();
        let mut file = File::open("tests/images/prowrestling.jpg").unwrap();
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf).unwrap();
        let image_binary = Bytes::from(buf);
        let res = detect_and_findsimilars(image_binary).await.unwrap();
        assert_eq!(res.face_list.len(), 2);
        assert_eq!(res.similar_list.len(), 0);
        let res = findsimilars(&res.face_list[0].faceId).await.unwrap();
        assert!(res.similar_list.len() > 0);
    }

    #[actix_rt::test]
    async fn no_face() {
        dotenv().ok();
        let mut file = File::open("tests/images/logo.png").unwrap();
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf).unwrap();
        let image_binary = Bytes::from(buf);
        let res = detect_and_findsimilars(image_binary).await.unwrap();
        assert_eq!(res.face_list.len(), 0);
        assert_eq!(res.similar_list.len(), 0);
    }
}
