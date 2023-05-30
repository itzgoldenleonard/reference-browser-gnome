mod parse_tests {
    use super::super::*;

    #[test]
    fn basic_example() {
        let expected = Document::builder().build();

        let content = "\n+++ Meta\nTI Test\nST Subtitle test\nAU Author 1\nAU Author 2\nLI CC0-1.0\nLA en\nCH 0\n+++ Header\n=> /index.athn Homepage\n=> /about.athn About\n+++\n\n()\nLittle text line makes the test fail\n=> https://example.com/ Link line with label, the next one will be without\n=> https://localhost/\n```Preformatted line\n'''Textual preformatted line\n---\n1* Unordered list\n2* Subitem\n6* Subsubsubsubsubitem\n1- 1. Ordered list\n1- 2. With multiple lines\n2- a) And subitems\n\\/ Dropdown | This is a dropdown line\n_! Note admonition\n*! Warning admonition\n!! Danger admonition\n1# Heading 1\n2# Heading 2\n4# Heading 4\n>> I never said that  - Albert Einstein\n+++ Footer\nThis is just a boring old footer\n=> /privacy.athn Privacy policy";

        let document = parse(content.lines(), Document::builder(), Section::Main).unwrap();

        assert_ne!(document.build(), expected);
    }

    #[test]
    fn form() {
        let expected = Document::builder().build();

        let content = "+++ Meta\nTI Form test\n+++\nThe next line is where the first form starts\n+++ Form\nThis form has all the different types of form fields in it\n[] Send:submit \\dest /one\n+++\nThen the second form\n+++ Form\nThis form has some funky fields with weird configurations to push the parser to its limits\n[] Send:submit \\dest /two\n";

        let document = parse(content.lines(), Document::builder(), Section::Main).unwrap();

        assert_eq!(document.build(), expected);
    }
}

mod metadatabuilder_tests {
    use super::super::*;

    #[test]
    fn build_test() {
        let expected_title = String::new();

        let metadata_obj = Metadata::builder().build();

        assert_eq!(expected_title, metadata_obj.title);
    }

    #[test]
    fn set_title() {
        let expected = String::from("Hello world!");

        let metadata_obj = Metadata::builder()
            .title("Hello world!".to_string())
            .build();

        assert_eq!(metadata_obj.title, expected);
    }

    #[test]
    fn set_subtitle() {
        let expected = Some(String::from("Hello world!"));

        let metadata_obj = Metadata::builder()
            .subtitle("Hello world!".to_string())
            .build();

        assert_eq!(metadata_obj.subtitle, expected);
    }

    #[test]
    fn set_single_author() {
        let expected = Some(vec!["Author 1".to_string()]);

        let metadata_obj = Metadata::builder()
            .add_author_unfailing("Author 1".to_string())
            .build();

        assert_eq!(metadata_obj.author, expected);
    }

    #[test]
    fn set_multiple_author() {
        let expected = Some(vec!["Author 1".to_string(), "Author 2".to_string()]);

        let metadata_obj = Metadata::builder()
            .add_author_unfailing("Author 1".to_string())
            .add_author_unfailing("Author 2".to_string())
            .build();

        assert_eq!(metadata_obj.author, expected);
    }

    #[test]
    fn set_too_many_authors() {
        let expected = Some(vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
            "10".to_string(),
            "11".to_string(),
            "12".to_string(),
            "13".to_string(),
            "14".to_string(),
            "15".to_string(),
            "16".to_string(),
        ]);

        let metadata_obj = Metadata::builder()
            .add_author_unfailing("1".to_string())
            .add_author_unfailing("2".to_string())
            .add_author_unfailing("3".to_string())
            .add_author_unfailing("4".to_string())
            .add_author_unfailing("5".to_string())
            .add_author_unfailing("6".to_string())
            .add_author_unfailing("7".to_string())
            .add_author_unfailing("8".to_string())
            .add_author_unfailing("9".to_string())
            .add_author_unfailing("10".to_string())
            .add_author_unfailing("11".to_string())
            .add_author_unfailing("12".to_string())
            .add_author_unfailing("13".to_string())
            .add_author_unfailing("14".to_string())
            .add_author_unfailing("15".to_string())
            .add_author_unfailing("16".to_string())
            .add_author_unfailing("17".to_string())
            .add_author_unfailing("18".to_string())
            .build();

        assert_eq!(metadata_obj.author, expected);
    }

    #[test]
    fn set_license() {
        let expected = Some(vec!["CC0-1.0".to_string()]);

        let metadata_obj = Metadata::builder()
            .add_license_unfailing("CC0-1.0".to_string())
            .build();

        assert_eq!(metadata_obj.license, expected);
    }

    #[test]
    fn set_multiple_licenses() {
        let expected = Some(vec!["CC0-1.0".to_string(), "Unlicense".to_string()]);

        let metadata_obj = Metadata::builder()
            .add_license_unfailing("CC0-1.0".to_string())
            .add_license_unfailing("Unlicense".to_string())
            .build();

        assert_eq!(metadata_obj.license, expected);
    }

    #[test]
    fn set_too_many_licenses() {
        let expected = Some(vec![
            "CC0-1.0".to_string(),
            "CC0-1.0".to_string(),
            "CC0-1.0".to_string(),
            "CC0-1.0".to_string(),
        ]);

        let metadata_obj = Metadata::builder()
            .add_license_unfailing("CC0-1.0".to_string())
            .add_license_unfailing("CC0-1.0".to_string())
            .add_license_unfailing("CC0-1.0".to_string())
            .add_license_unfailing("CC0-1.0".to_string())
            .add_license_unfailing("CC0-1.0".to_string())
            .add_license_unfailing("Unlicense".to_string())
            .build();

        assert_eq!(metadata_obj.license, expected);
    }

    #[test]
    fn set_lang() {
        let expected = Some(vec!["en".to_string()]);

        let metadata_obj = Metadata::builder()
            .add_language_unfailing("en".to_string())
            .build();

        assert_eq!(metadata_obj.language, expected);
    }

    #[test]
    fn set_multiple_langs() {
        let expected = Some(vec!["en_US".to_string(), "en_GB".to_string()]);

        let metadata_obj = Metadata::builder()
            .add_language_unfailing("en_US".to_string())
            .add_language_unfailing("en_GB".to_string())
            .build();

        assert_eq!(metadata_obj.language, expected);
    }

    #[test]
    fn set_cache() {
        let expected = Some(100);

        let metadata_obj = Metadata::builder().cache(100).build();

        assert_eq!(metadata_obj.cache, expected);
    }
}

mod documentbuilder_tests {
    use super::super::*;

    #[test]
    fn build_test() {
        let expected_obj = Document {
            metadata: Metadata::builder().build(),
            main: vec![],
            header: None,
            footer: None,
        };

        let document_obj = Document::builder().build();

        assert_eq!(document_obj, expected_obj);
    }

    #[test]
    fn single_main_line() {
        use MainLine::*;

        let expected = vec![SeparatorLine];

        let document_obj = Document::builder().add_main_line(SeparatorLine).build();

        assert_eq!(document_obj.main, expected);
    }

    #[test]
    fn multiple_main_lines() {
        use MainLine::*;

        let expected = vec![
            SeparatorLine,
            HeadingLine(Level::One, "Line 2".to_string()),
            TextLine("Line 3".to_string()),
        ];

        let document_obj = Document::builder()
            .add_main_line(SeparatorLine)
            .add_main_line(HeadingLine(Level::One, "Line 2".to_string()))
            .add_main_line(TextLine("Line 3".to_string()))
            .build();

        assert_eq!(document_obj.main, expected);
    }

    #[test]
    fn single_header_line() {
        use HeaderLine::*;

        let expected = Some(vec![LinkLine(Link {
            url: "https://localhost:3000/".to_string(),
            label: None,
        })]);

        let document_obj = Document::builder()
            .add_header_line(LinkLine(Link {
                url: "https://localhost:3000/".to_string(),
                label: None,
            }))
            .build();

        assert_eq!(document_obj.header, expected);
    }

    #[test]
    fn multiple_header_lines() {
        use HeaderLine::*;

        let expected = Some(vec![
            LinkLine(Link {
                url: "https://localhost:3000/".to_string(),
                label: None,
            }),
            LinkLine(Link {
                url: "https://localhost:3000/index.athn".to_string(),
                label: Some("index".to_string()),
            }),
        ]);

        let document_obj = Document::builder()
            .add_header_line(LinkLine(Link {
                url: "https://localhost:3000/".to_string(),
                label: None,
            }))
            .add_header_line(LinkLine(Link {
                url: "https://localhost:3000/index.athn".to_string(),
                label: Some("index".to_string()),
            }))
            .build();

        assert_eq!(document_obj.header, expected);
    }

    #[test]
    fn single_footer_line() {
        use FooterLine::*;

        let expected = Some(vec![LinkLine(Link {
            url: "https://localhost:3000/".to_string(),
            label: None,
        })]);

        let document_obj = Document::builder()
            .add_footer_line(LinkLine(Link {
                url: "https://localhost:3000/".to_string(),
                label: None,
            }))
            .build();

        assert_eq!(document_obj.footer, expected);
    }

    #[test]
    fn multiple_footer_lines() {
        use FooterLine::*;

        let expected = Some(vec![
            LinkLine(Link {
                url: "https://localhost:3000/".to_string(),
                label: None,
            }),
            LinkLine(Link {
                url: "https://localhost:3000/index.athn".to_string(),
                label: Some("index".to_string()),
            }),
        ]);

        let document_obj = Document::builder()
            .add_footer_line(LinkLine(Link {
                url: "https://localhost:3000/".to_string(),
                label: None,
            }))
            .add_footer_line(LinkLine(Link {
                url: "https://localhost:3000/index.athn".to_string(),
                label: Some("index".to_string()),
            }))
            .build();

        assert_eq!(document_obj.footer, expected);
    }
}
