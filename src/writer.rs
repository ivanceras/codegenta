/// generic string writer
/// use this for writing source code
#[derive(Debug)]
pub struct Writer {
    pub src: String,
}

impl Writer {
    #[inline]
    pub fn new() -> Self {
        Writer { src: String::new() }
    }

    #[inline]
    pub fn append(&mut self, str: &str) -> &mut Self {
        self.src.push_str(str);
        self
    }

    #[inline]
    pub fn appendln(&mut self, str: &str) -> &mut Self {
        self.append(str);
        self.ln()
    }

    #[inline]
    pub fn tab(&mut self) -> &mut Self {
        self.append("    ")
    }
    #[inline]
    pub fn tabs(&mut self, n: u32) -> &mut Self {
        for _ in 0..n {
            self.tab();
        }
        self
    }
    #[inline]
    pub fn ln(&mut self) -> &mut Self {
        self.append("\n")
    }
    #[inline]
    pub fn ln_tab(&mut self) -> &mut Self {
        self.ln();
        self.tab()
    }
    #[inline]
    pub fn ln_tabs(&mut self, n: u32) -> &mut Self {
        self.ln();
        self.tabs(n)
    }
    #[inline]
    pub fn comma(&mut self) -> &mut Self {
        self.append(",")
    }
    #[inline]
    pub fn sp(&mut self) -> &mut Self {
        self.append(" ")
    }
    #[inline]
    pub fn commasp(&mut self) -> &mut Self {
        self.comma().sp()
    }

    #[inline]
    pub fn comment(&mut self, comment: &str) -> &mut Self {
        self.append("//");
        self.append(comment);
        self
    }

    #[inline]
    /// TODO: make escaping more formal
    pub fn doc_comment(&mut self, comment: &str) -> &mut Self {
        let splinters: Vec<&str> = comment.split('\n').collect();
        for sp in splinters {
            self.append("/// ");
            self.appendln(sp);
        }
        self
    }

    #[inline]
    pub fn inner_doc_comment(&mut self, comment: &str) -> &mut Self {
        self.append("//!");
        self.append(comment)
    }
}
