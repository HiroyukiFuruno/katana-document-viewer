pub(super) struct HtmlSourcePlainText;

impl HtmlSourcePlainText {
    pub(super) fn scan(html: &str) -> String {
        HtmlTextScanner::new(html).scan()
    }
}

struct HtmlTextScanner<'a> {
    html: &'a str,
    cursor: usize,
    output: String,
}

impl<'a> HtmlTextScanner<'a> {
    fn new(html: &'a str) -> Self {
        Self {
            html,
            cursor: 0,
            output: String::new(),
        }
    }

    fn scan(mut self) -> String {
        while self.cursor < self.html.len() {
            if self.current_slice().starts_with('<') {
                self.skip_tag_or_ignored_block();
            } else {
                self.push_current_char();
            }
        }
        decode_html_entities(&self.output)
    }

    fn skip_tag_or_ignored_block(&mut self) {
        let tag = self.tag_name();
        self.skip_tag();
        let Some(tag_name) = tag.as_deref() else {
            return;
        };
        if matches!(tag_name, "style" | "script") {
            self.skip_until_closing_tag(tag_name);
        }
    }

    fn tag_name(&self) -> Option<String> {
        let rest = self.current_slice().strip_prefix('<')?;
        let rest = rest.trim_start_matches('/');
        let name = rest
            .chars()
            .take_while(|character| character.is_ascii_alphanumeric())
            .collect::<String>();
        (!name.is_empty()).then(|| name.to_ascii_lowercase())
    }

    fn skip_tag(&mut self) {
        let Some(end) = self.current_slice().find('>') else {
            self.cursor = self.html.len();
            return;
        };
        self.cursor += end + 1;
        self.output.push(' ');
    }

    fn skip_until_closing_tag(&mut self, tag: &str) {
        let closing = format!("</{tag}>");
        let lower = self.current_slice().to_ascii_lowercase();
        let Some(offset) = lower.find(&closing) else {
            self.cursor = self.html.len();
            return;
        };
        self.cursor += offset + closing.len();
        self.output.push(' ');
    }

    fn push_current_char(&mut self) {
        let Some(character) = self.current_slice().chars().next() else {
            self.cursor = self.html.len();
            return;
        };
        self.output.push(character);
        self.cursor += character.len_utf8();
    }

    fn current_slice(&self) -> &'a str {
        &self.html[self.cursor..]
    }
}

fn decode_html_entities(value: &str) -> String {
    value
        .replace("&quot;", "\"")
        .replace("&#34;", "\"")
        .replace("&lt;", "<")
        .replace("&#60;", "<")
        .replace("&gt;", ">")
        .replace("&#62;", ">")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
}
