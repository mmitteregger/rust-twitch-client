use std::collections::HashMap;
use hyper::Url;

use model::TwitchLinks;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct Paging {
    offset: Option<u32>,
    limit: Option<u8>,
}

impl Paging {
    pub fn default() -> Paging {
        Paging {
            offset: None,
            limit: None,
        }
    }

    pub fn new(offset: u32, limit: u8) -> Paging {
        if limit < 1 || limit > 100 {
            panic!("Limit needs to be between 1 (inclusive) and 100 (inclusive)");
        }

        Paging {
            offset: Some(offset),
            limit: Some(limit),
        }
    }

    pub fn from_url(url: Url) -> Option<Paging> {
        let query_pairs_option: Option<Vec<(String, String)>> = url.query_pairs();
        query_pairs_option.map_or(None, |query_pairs| {
            let mut map = HashMap::new();

            for (key, value) in query_pairs {
                map.insert(key, value);
            }

            let limit_option = map.get("limit").map_or(None, |s| s.parse::<u8>().ok());
            let offset_option = map.get("offset").map_or(None, |s| s.parse::<u32>().ok());

            match limit_option {
                Some(_) => {
                    Some(Paging {
                        offset: offset_option.or(Some(0u32)),
                        limit: limit_option,
                    })
                },
                None => None,
            }
        })
    }

    pub fn limit(&self) -> Option<u8> {
        self.limit
    }

    pub fn offset(&self) -> Option<u32> {
        self.offset
    }

    pub fn is_default(&self) -> bool {
        self.limit.is_none()
    }
}

pub trait Paged: TwitchLinks {
    fn current_page_link(&self) -> &String {
        self.get_expected_link("self")
    }

    fn next_page_link(&self) -> &String {
        self.get_expected_link("next")
    }

    fn paging(&self) -> ::model::paging::Paging {
        let link = self.current_page_link();
        let url = match ::hyper::Url::parse(link) {
            Ok(url) => url,
            Err(err) => {
                panic!("Expected current page link to be a valid url but got: {}, Error: {}",
                    link, err);
            }
        };
        match ::model::paging::Paging::from_url(url) {
            Some(paging) => paging,
            None => {
                panic!("Expected current page link to contain paging but got: {}", link);
            }
        }
    }
}

//macro_rules! impl_paged {
//    ($t:ty) => {
//        impl Paged for $t {
//            fn current_page_link(&self) -> &String {
//                let link_key = "self";
//                match self.links.get(link_key) {
//                    Some(link) => link,
//                    None => {
//                        panic!("Expected links to contain {} but got: {:?}", link_key, self.links);
//                    }
//                }
//            }
//
//            fn next_page_link(&self) -> &String {
//                let link_key = "next";
//                match self.links.get(link_key) {
//                    Some(link) => link,
//                    None => {
//                        panic!("Expected links to contain {} but got: {:?}", link_key, self.links);
//                    }
//                }
//            }
//
//            fn paging(&self) -> ::model::paging::Paging {
//                let link = self.current_page_link();
//                let url = match ::hyper::Url::parse(link) {
//                    Ok(url) => url,
//                    Err(err) => {
//                        panic!("Expected current page link to be a valid url but got: {}, Error: {}",
//                            link, err);
//                    }
//                };
//                match ::model::paging::Paging::from_url(url) {
//                    Some(paging) => paging,
//                    None => {
//                        panic!("Expected current page link to contain paging but got: {}", link);
//                    }
//                }
//            }
//        }
//    };
//}
