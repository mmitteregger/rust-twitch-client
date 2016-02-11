pub trait ToQueryString {
    fn to_query_string(&self) -> String;
}

#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub struct TopGamesParams {
    offset: Option<u32>,
    limit: Option<u8>,
}

impl TopGamesParams {
    pub fn offset(&self) -> Option<u32> {
        self.offset
    }
    pub fn limit(&self) -> Option<u8> {
        self.limit
    }
}

impl ToQueryString for TopGamesParams {
    fn to_query_string(&self) -> String {
        params_to_query_string(vec![
            ("offset", &self.offset.map(|offset| offset.to_string())),
            ("limit", &self.limit.map(|limit| limit.to_string())),
        ])
    }
}

#[derive(Default)]
pub struct TopGamesParamsBuilder {
    offset: Option<u32>,
    limit: Option<u8>,
}

impl TopGamesParamsBuilder {
    pub fn offset(mut self, offset: u32) -> TopGamesParamsBuilder {
        self.offset = Some(offset);
        self
    }
    pub fn limit(mut self, limit: u8) -> TopGamesParamsBuilder {
        self.limit = Some(limit);
        self
    }
    pub fn build(self) -> TopGamesParams {
        TopGamesParams {
            offset: self.offset,
            limit: self.limit,
        }
    }
}


fn params_to_query_string(params: Vec<(&str, &Option<String>)>) -> String {
    let mut query_string = String::new();

    for (param_name, param_value) in params {
        match *param_value {
            Some(ref value) => {
                if query_string.is_empty() {
                    query_string.push('?');
                } else {
                    query_string.push('&');
                }

                query_string.push_str(param_name);
                query_string.push('=');
                query_string.push_str(&value);
            },
            None => {},
        }
    }

    query_string
}
