#[cfg(test)]
mod company_test {
    use crate::company_type::*;

    #[test]
    fn valid_xml() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
			<Data>
				<id>1</id>
				<name>MWNZ</name>
				<description>..is awesome</description>
			</Data>"#
            .to_string();

        let company = Company::try_from_xml(xml).unwrap();

        assert_eq!(
            company,
            Company {
                id: 1,
                name: String::from("MWNZ"),
                description: String::from("..is awesome")
            }
        );
    }

    #[test]
    fn valid_xml_additional_tag() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
			<Data>
				<id>1</id>
				<name>MWNZ</name>
				<description>..is awesome</description>
				<location>New Zealand</location>
			</Data>"#
            .to_string();

        let company = Company::try_from_xml(xml).unwrap();

        assert_eq!(
            company,
            Company {
                id: 1,
                name: String::from("MWNZ"),
                description: String::from("..is awesome")
            }
        );
    }

    /// During testing I found that the parser doesn't care about the container name.
    /// This may be a problem should the case below be intended to fail.
    #[test]
    fn valid_xml_container_name() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
			<OtherContainer>
				<id>1</id>
				<name>MWNZ</name>
				<description>..is awesome</description>
			</OtherContainer>"#
            .to_string();

        let company = Company::try_from_xml(xml).unwrap();

        assert_eq!(
            company,
            Company {
                id: 1,
                name: String::from("MWNZ"),
                description: String::from("..is awesome")
            }
        );
    }

    #[test]
    fn valid_xml_empty_string() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
			<Data>
				<id>1</id>
				<name>MWNZ</name>
				<description></description>
			</Data>"#
            .to_string();

        let company = Company::try_from_xml(xml).unwrap();

        assert_eq!(
            company,
            Company {
                id: 1,
                name: String::from("MWNZ"),
                description: String::from("")
            }
        );
    }

    #[test]
    fn valid_xml_negative_id() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
			<Data>
				<id>-100</id>
				<name>MWNZ</name>
				<description>..is awesome</description>
			</Data>"#
            .to_string();

        let company = Company::try_from_xml(xml).unwrap();

        assert_eq!(
            company,
            Company {
                id: -100,
                name: String::from("MWNZ"),
                description: String::from("..is awesome")
            }
        );
    }

    #[test]
    fn invalid_xml_tag_name() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
			<Data>
				<company_id>1</company_id>
				<name>MWNZ</name>
				<description>..is awesome</description>
			</Data>"#
            .to_string();

        match Company::try_from_xml(xml) {
            Ok(_) => panic!("Parsed invalid xml."),
            Err(_) => { /* Expected */ }
        };
    }
}
