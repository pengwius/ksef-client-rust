pub fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::xml_escape;

    #[test]
    fn escapes_empty() {
        assert_eq!(xml_escape(""), "");
    }

    #[test]
    fn escapes_none() {
        let input = "plain text 123";
        assert_eq!(xml_escape(input), "plain text 123");
    }

    #[test]
    fn escapes_all_entities() {
        let input = "&<>'\"";
        let expected = "&amp;&lt;&gt;&apos;&quot;";
        assert_eq!(xml_escape(input), expected);
    }

    #[test]
    fn escapes_mixed() {
        let input = "Tom & Jerry <Best> 'Show' \"Cartoon\"";
        let expected = "Tom &amp; Jerry &lt;Best&gt; &apos;Show&apos; &quot;Cartoon&quot;";
        assert_eq!(xml_escape(input), expected);
    }

    #[test]
    fn escapes_existing_entity() {
        assert_eq!(xml_escape("&amp;"), "&amp;amp;");
    }
}
