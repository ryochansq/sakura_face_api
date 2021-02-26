use crate::types::*;

pub fn get_similar_member_list(similar_list: Vec<SimilarFace>) -> Vec<SimilarMember> {
    similar_list
        .into_iter()
        .map(|x| SimilarMember {
            name: id_to_member(&x.persistedFaceId),
            confidence: x.confidence,
        })
        .collect::<Vec<SimilarMember>>()
}

fn id_to_member(face_id: &str) -> String {
    let name = match face_id {
        "abb8a01e-d3ef-4bdb-9fca-7ca4fdbd5f3d" => "武藤 彩未",
        "bac6bc5c-deb9-4a15-96f0-709f44a67a41" => "松井 愛莉",
        "77b22104-caab-451f-927d-114e381e17ef" => "三吉 彩花",
        "4de25f38-5cfb-42eb-b987-a77a29dd45bb" => "中元 すず香",
        "ec087a48-60da-4862-9412-6a1fb36c1dc8" => "杉本 愛莉鈴",
        "59730cf7-2e80-4a6a-a5fc-06bd741a8bf5" => "堀内 まり菜",
        "746aa19d-2703-4f2c-93b8-838eb53d44bc" => "飯田 來麗",
        "f8d13bff-80a3-46df-9a95-786bf3500045" => "杉﨑 寧々",
        "14710924-9f64-4605-a2ab-06f55e9e2220" => "佐藤 日向",
        "fdba5f4a-8b02-4159-a03a-22e928437f9c" => "菊地 最愛",
        "c53206f0-13b5-44fd-9abd-e4e9499576db" => "水野 由結",
        "a09ecebd-d049-4d41-89c0-d7147c384ab5" => "田口 華",
        "174eaa01-a778-4d4d-961c-c6edc54f2888" => "野津 友那乃",
        "cadcfcb5-aeb2-4835-a9fd-822e6512f2e9" => "磯野 莉音",
        "0a8ff520-7a75-487d-aa09-730b40d36422" => "大賀 咲希",
        "3a2cb9e8-13f9-4c80-90f7-515bb5fab0df" => "白井 沙樹",
        "2549d52e-272f-4886-b6f0-202cf2c272fc" => "倉島 颯良",
        "52e70554-1d25-4b8e-8158-c9004c7ce3c8" => "黒澤 美澪奈",
        "d72ce5cc-37be-401f-9b77-d2548f5a144b" => "山出 愛子",
        "745d5e95-b303-49ca-91ac-15f029bed5e4" => "岡田 愛",
        "2d1b0b1e-9f2c-4850-9842-fd6775b54114" => "岡崎 百々子",
        "c07e37f3-ddab-4df4-a084-62e974a798ef" => "新谷 ゆづみ",
        "c490a2df-094e-4b45-9e05-2ef31cdbd597" => "麻生 真彩",
        "0780f7ff-8e03-46b3-ac94-153ca5d5963d" => "日髙 麻鈴",
        "5509f044-2102-4151-80f1-4dae4477fd17" => "藤平 華乃",
        "8eeab0cb-20a6-49ba-9353-cafb8095b4c3" => "吉田 爽葉香",
        "aea625b3-1921-44f5-bfba-37bf57aa8971" => "有友 緒心",
        "e15d9782-d11a-46b3-8849-7d5a64d3500a" => "森 萌々穂",
        "8ed47f3b-d743-442c-8bb1-03cea5816a8d" => "白鳥 沙南",
        "db4e3999-ba2e-484e-9558-e68a84cf0525" => "野中 ここな",
        "06912d98-7677-4871-9032-57ae0bb3d438" => "田中 美空",
        "c345f145-5bbd-4ba7-ba6c-dfafcb4f74fe" => "八木 美樹",
        "5635a943-0edf-42d2-82ce-8aabb26b6127" => "佐藤 愛桜",
        "e8e406ca-3349-422f-8faf-dcaa5c4a871e" => "戸高 美湖",
        "fa5f520f-7a4e-4c3e-8ff2-c016f06cbe3b" => "野崎 結愛",
        "f031f45f-1e87-4370-836e-fd2d9aab5286" => "木村 咲愛",
        _ => panic!("face_id not found"),
    };
    String::from(name)
}
