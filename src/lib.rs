pub mod content;
pub mod file;
pub mod include;
pub mod user_contents;
mod util;
mod zeruformatter;

pub use json_filter_sorted::*;

pub use content::Content;
pub use file::File;
pub use include::Include;
pub use user_contents::UserContents;
pub use util::Number;

#[cfg(test)]
#[cfg_attr(tarpaulin, ignore)]
mod tests {
    use serde_bytes::ByteBuf;

    use super::*;

    #[test]
    fn test_verification() {
        let content = Content::from_buf(ByteBuf::from(CONTENT.1.as_bytes())).unwrap();
        let key = CONTENT.0.into();
        let result = content.verify(key);
        assert_eq!(result, true)
    }

    #[test]
    fn test_unicode_verification() {
        let content = Content::from_buf(ByteBuf::from(CONTENT_UNICODE.1.as_bytes())).unwrap();
        let key = CONTENT_UNICODE.0.into();
        let result = content.verify(key);
        assert_eq!(result, true)
    }

    #[test]
    fn test_unicode_un_escaped_verification() {
        let content =
            Content::from_buf(ByteBuf::from(CONTENT_UNICODE_UNESCAPED.1.as_bytes())).unwrap();
        let key = CONTENT_UNICODE_UNESCAPED.0.into();
        let result = content.verify(key);
        assert_eq!(result, true)
    }

    const CONTENT: (&str, &str) = (
        "1JUDmCT4UCSdnPsJAHBoXNkDS61Y31Ue52",
        r#"
			{
			"address": "1JUDmCT4UCSdnPsJAHBoXNkDS61Y31Ue52",
			"address_index": 36579623,
			"background-color": "white",
			"cloneable": true,
			"cloned_from": "1RedkCkVaXuVXrqCMpoXQS29bwaqsuFdL",
			"description": "Home of the bots",
			"files": {
			"data-default/users/content.json-default": {
			"sha512": "4e37699bd5336b9c33ce86a3eb73b82e87460535793401874a653afeddefee59",
			"size": 735
			},
			"index.html": {
			"sha512": "087c6ae46aacc5661f7da99ce10dacc0428dbd48aa7bbdc1df9c2da6e81b1d93",
			"size": 466
			}
			},
			"ignore": "((js|css)/(?!all.(js|css))|data/.*db|data/users/.*/.*)",
			"includes": {
				"data/users/content.json": {
				"signers": [],
				"signers_required": 1
				}
			},
			"inner_path": "content.json",
			"merged_type": "ZeroMe",
			"modified": 1471656205.079839,
			"postmessage_nonce_security": true,
			"sign": [
				60601328857260736769667767617236149396007806053808183569130735997086722937268,
				43661716327244911082383801335054839207111588960552431293232589470692186442781
			],
			"signers_sign": "HEMH4/a7LXic4PYgMj/4toV5jI5z+SX6Bnmo3mP0HoyIGy6e7rUbilJYAH3MrgCT/IXzIn7cnIlhL8VARh7CeUg=",
			"signs": {
				"1JUDmCT4UCSdnPsJAHBoXNkDS61Y31Ue52": "G5qMkd9+n0FMLm2KA4FAN3cz/vaGY/oSYd2k/edx4C+TIv76NQI37NsjXVWtkckMoxvp6rhW8PHZy9Q1MNtmIAM="
			},
			"signs_required": 1,
			"title": "Bot Hub",
			"zeronet_version": "0.4.0"
			}"#,
    );

    const CONTENT_UNICODE: (&str, &str) = (
        "16MQxEQe1U32zDGTKWs1rnc1mU3iL42EB3",
        r#"{
		"address": "16MQxEQe1U32zDGTKWs1rnc1mU3iL42EB3",
		"description": "",
		"files": {},
		"inner_path": "content.json",
		"modified": 1659645497,
		"signers_sign": "Gy9gpZYdEChTxIAxCYfljl8xBBJP0iOYe8Arfs5eMyEdXbSSKmymPGJV1VegBkLt2Br9ltpwWmEarvGXR6Gi638=",
		"signs": {"16MQxEQe1U32zDGTKWs1rnc1mU3iL42EB3": "HNG1x0Bdx2wtC0HDhjIZ7VKEQ6OGhnLGBMXrlCzSAvgKGoPGSQRroPjP+SNtjdWptQ1xHi/efEFFWJx/2aRnZm0="},
		"signs_required": 1,
		"title": "My New Site \\ud83d\\ude01",
		"zeronet_version": "0.8.0"
		}"#,
    );

    const CONTENT_UNICODE_UNESCAPED: (&str, &str) = (
        "16MQxEQe1U32zDGTKWs1rnc1mU3iL42EB3",
        r#"{
		"address": "16MQxEQe1U32zDGTKWs1rnc1mU3iL42EB3",
		"description": "",
		"files": {},
		"inner_path": "content.json",
		"modified": 1659645953,
		"signers_sign": "Gy9gpZYdEChTxIAxCYfljl8xBBJP0iOYe8Arfs5eMyEdXbSSKmymPGJV1VegBkLt2Br9ltpwWmEarvGXR6Gi638=",
		"signs": {"16MQxEQe1U32zDGTKWs1rnc1mU3iL42EB3": "HNF0cx1cGJhyuv9SiIv/PD+drXfX5mvf/cray09ZtDiNenTFfU0SmuAslzhcvn78gY+7y8d+K5S83prn35jiilg="},
		"signs_required": 1,
		"title": "My New Site \ud83d\ude01",
		"zeronet_version": "0.8.0"
		}"#,
    );
}
