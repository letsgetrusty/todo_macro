trait Serialize {
    fn to_json_string(&self) -> String;
    fn to_xml_string(&self) -> String;
    fn to_yaml_string(&self) -> String;
}

pub struct Post {
    id: Option<u32>,
    title: String,
    body: String,
}

impl Post {
    fn new(title: String, body: String) -> Self {
        Self { id: None, title, body }
    }
}

impl Serialize for Post {
    fn to_json_string(&self) -> String {
        todo!()
    }

    fn to_xml_string(&self) -> String {
        todo!()
    }

    fn to_yaml_string(&self) -> String {
        todo!()
    }
}

pub fn create_post(post: Post) -> Result<String, String> {
    todo!()
}

pub fn get_post(id: u32) -> Option<Post> {
    todo!()
}

pub fn update_post(id: u32, post: Post) -> Result<String, String> {
    todo!()
}

pub fn delete_post(id: u32) -> Result<String, String> {
    todo!()
}
