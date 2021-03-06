error_chain!{
    foreign_links {
        StdIo(::std::io::Error);
        StdSystemTime(::std::time::SystemTimeError);
        StdStrUtf8(::std::str::Utf8Error);
        StdNumParseInt(::std::num::ParseIntError);

        SerdeJson(::serde_json::Error);
        Redis(::redis::RedisError);
        R2d2(::r2d2::Error);
        ChronoParse(::chrono::ParseError);
        Diesel(::diesel::result::Error);
        Base64Decode(::base64::DecodeError);
        Amqp(::amqp::AMQPError);
        Ini(::ini::ini::Error);
        LanguageTags(::language_tags::Error);
        LettreEmail(::lettre_email::error::Error);
        LettreSmtp(::lettre::smtp::error::Error);
        UrlParse(::url::ParseError);
        TomlSer(::toml::ser::Error);
        TomlDe(::toml::de::Error);
        Log4rs(::log4rs::Error);
        Validator(::validator::ValidationErrors);
        Mustache(::mustache::Error);
        Regex(::regex::Error);
        Hyper(::hyper::Error);
        RocketConfig(::rocket::config::ConfigError);
        RocketLaunch(::rocket::error::LaunchError);
        Rss(::rss::Error);
        SerdeYaml(::serde_yaml::Error);
    }
}
