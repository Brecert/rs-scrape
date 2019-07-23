extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
struct OpenGraphProtocol {

	#[serde(skip_serializing_if="Option::is_none")]
  title: Option<String>,
  
	#[serde(skip_serializing_if="Option::is_none")]
  url: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  image: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  audio: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  description: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  site_name: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  video: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MaskIcon {
	src: Option<String>,
	color: Option<String>,
}

#[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct Metadata {
	#[serde(skip_serializing_if="Option::is_none")]
  title: Option<String>,

	#[serde(skip_serializing_if="Option::is_none")]
  site: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  mask_icon: Option<MaskIcon>,

	#[serde(skip_serializing_if="Option::is_none")]
  creator: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  url: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  image: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  image_alt: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  video: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  video_type: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  audio: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  audio_type: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  description: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  name: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  color: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  format: Option<String>
}

pub fn get_metadata(url: &str) -> Metadata {

	let mut res = reqwest::get(url).unwrap(); 
	assert!(res.status().is_success());

	let body = res.text().unwrap();

	let fragment = Html::parse_document(&body);

	// create a selector to get the meta properties
	let meta = Selector::parse(r#"meta,link"#).unwrap();
	let mut metadata = Metadata::default();

	// iterate over the selected meta properties using the selector
	for property in fragment.select(&meta) {
		match property.value().attr("property") {
			// twitter
			Some("twitter:card") => metadata.format = property.value().attr("content").map(|c| c.to_string()),
			Some("twitter:description") => metadata.description = property.value().attr("content").map(|c| c.to_string()),
			Some("twitter:site") => metadata.site = property.value().attr("content").map(|c| c.to_string()),
			Some("twitter:image") => metadata.image = property.value().attr("content").map(|c| c.to_string()),
			Some("twitter:image:alt") => metadata.image_alt = property.value().attr("content").map(|c| c.to_string()),
			Some("twitter:creator") => metadata.creator = property.value().attr("content").map(|c| c.to_string()),


			// ogp
			Some("og:title") => metadata.title = property.value().attr("content").map(|c| c.to_string()),
			Some("og:site_name") => metadata.name = property.value().attr("content").map(|c| c.to_string()),
			
			Some("og:image") => metadata.image = property.value().attr("content").map(|c| c.to_string()),
			Some("og:image:url") => metadata.image = property.value().attr("content").map(|c| c.to_string()),
			Some("og:image:secure_url") => metadata.image = property.value().attr("content").map(|c| c.to_string()),
			Some("og:image:alt") => metadata.image_alt = property.value().attr("content").map(|c| c.to_string()),
			

			Some("og:video") => metadata.video = property.value().attr("content").map(|c| c.to_string()),
			Some("og:video:url") => metadata.video = property.value().attr("content").map(|c| c.to_string()),
			Some("og:video:secure_url") => metadata.video = property.value().attr("content").map(|c| c.to_string()),
			Some("og:video:type") => metadata.video_type = property.value().attr("content").map(|c| c.to_string()),
			

			Some("og:audio") => metadata.audio = property.value().attr("content").map(|c| c.to_string()),
			Some("og:audio:url") => metadata.audio = property.value().attr("content").map(|c| c.to_string()),
			Some("og:audio:secure_url") => metadata.audio = property.value().attr("content").map(|c| c.to_string()),
			Some("og:audio:type") => metadata.audio_type = property.value().attr("content").map(|c| c.to_string()),


			Some("og:description") => metadata.description = property.value().attr("content").map(|c| c.to_string()),
			Some("og:url") => metadata.url = property.value().attr("content").map(|c| c.to_string()),

			// other
			Some("theme-color") => metadata.color = property.value().attr("content").map(|c| c.to_string()),
			_ => (),
		};

		match property.value().attr("name") {
			Some("theme-color") => metadata.color = property.value().attr("content").map(|c| c.to_string()),
			_ => (),
		}

		match property.value().attr("rel") {
			Some("mask-icon") => metadata.mask_icon = Some(MaskIcon { 
				src: property.value().attr("href").map(|c| c.to_string()),
				color: property.value().attr("color").map(|c| c.to_string())
			}),
			_ => (),
		}
	}
	
	return metadata
}