#[derive(Clone, Debug, Default)]
pub struct MultipartForm {
    fields: Vec<(String, String)>,
    files: Vec<FilePart>,
}

#[derive(Clone, Debug)]
pub struct FilePart {
    name: String,
    filename: String,
    content_type: Option<String>,
    bytes: Vec<u8>,
}

impl MultipartForm {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.push((name.into(), value.into()));
        self
    }

    pub fn file_bytes(
        mut self,
        name: impl Into<String>,
        filename: impl Into<String>,
        bytes: Vec<u8>,
    ) -> Self {
        self.files.push(FilePart {
            name: name.into(),
            filename: filename.into(),
            content_type: None,
            bytes,
        });
        self
    }

    pub fn file_bytes_with_content_type(
        mut self,
        name: impl Into<String>,
        filename: impl Into<String>,
        content_type: impl Into<String>,
        bytes: Vec<u8>,
    ) -> Self {
        self.files.push(FilePart {
            name: name.into(),
            filename: filename.into(),
            content_type: Some(content_type.into()),
            bytes,
        });
        self
    }

    pub(crate) fn fields(&self) -> &[(String, String)] {
        &self.fields
    }

    pub(crate) fn files(&self) -> &[FilePart] {
        &self.files
    }
}

impl FilePart {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn filename(&self) -> &str {
        &self.filename
    }

    pub(crate) fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}
