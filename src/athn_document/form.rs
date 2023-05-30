#[derive(PartialEq, Debug)]
pub enum FormField {
    Submit(SubmitField),
}

#[derive(PartialEq, Debug)]
pub struct SubmitField {
    pub id: String,
    pub dest: String,
    pub label: Option<String>,
    pub redirect: bool,
}
