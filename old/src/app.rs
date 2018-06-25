use std::env::current_dir;
use std::io::{Read, Write};
use std::ops::Deref;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use std::{fs, thread};

use base64;
use clap;
use handlebars::Handlebars;
use log;
use rocket;
use rocket::config::{Config, Environment, LoggingLevel};
use rocket_contrib::Template;
#[cfg(feature = "sodium")]
use sodiumoxide;
use sys_info;
use toml;

use super::{
    cache::{self, Cache}, context::Context, dao::{self, Dao, Migration}, env, graphql,
    i18n::{self, Dao as LocaleDao}, plugins, queue::{self, Queue}, result::Result, rfc::RFC822,
    router, security,
};

pub struct App {
    ctx: Context,
}

impl App {
    pub fn main() -> Result<()> {
        if cfg!(feature = "sodium") {
            sodiumoxide::init();
        }
        let generate_nginx =
            clap::SubCommand::with_name("generate:nginx").about("Generate nginx.conf");
        let generate_config =
            clap::SubCommand::with_name("generate:config").about("Generate config file");

        let db_migrate = clap::SubCommand::with_name("db:migrate")
            .about("Migrate the schema back to the latest version");
        let db_rollback = clap::SubCommand::with_name("db:rollback")
            .about("Rolls the schema back to the previous version");
        let db_seed = clap::SubCommand::with_name("db:seed").about("Loads the seed data");
        let db_dump = clap::SubCommand::with_name("db:dump").about("Dumps database as json files");
        let db_version = clap::SubCommand::with_name("db:version")
            .about("Retrieves the current schema version number");
        let db_restore = clap::SubCommand::with_name("db:restore")
            .about("Restores database from file")
            .arg(
                clap::Arg::with_name("name")
                    .short("n")
                    .help("Sets a custom config file")
                    .takes_value(true),
            );

        let routes = clap::SubCommand::with_name("routes")
            .about("Print out all defined routes in match order");

        let cache_clear = clap::SubCommand::with_name("cache:clear").about("Clear all cache items");
        let cache_list = clap::SubCommand::with_name("cache:list").about("List all cache items");

        let matches = clap::App::new(env::NAME)
            .version(env::VERSION)
            .author(env::AUTHORS)
            .about(env::DESCRIPTION)
            .before_help(env::BANNER)
            .after_help(env::HOMEPAGE)
            .subcommand(generate_nginx)
            .subcommand(generate_config)
            .subcommand(db_rollback)
            .subcommand(db_migrate)
            .subcommand(db_seed)
            .subcommand(db_dump)
            .subcommand(db_version)
            .subcommand(db_restore)
            .subcommand(cache_list)
            .subcommand(cache_clear)
            .subcommand(routes)
            .get_matches();

        if let Some(_) = matches.subcommand_matches("generate:config") {
            return Self::generate_config();
        }
        if let Some(_) = matches.subcommand_matches("routes") {
            return Self::routes();
        }
        let app = Self::new()?;
        if let Some(_) = matches.subcommand_matches("generate:nginx") {
            return app.generate_nginx();
        }
        if let Some(_) = matches.subcommand_matches("db:migrate") {
            return app.db_migrate();
        }
        if let Some(_) = matches.subcommand_matches("db:rollback") {
            return app.db_rollback();
        }
        if let Some(_) = matches.subcommand_matches("db:seed") {
            return app.db_seed();
        }
        if let Some(_) = matches.subcommand_matches("db:dump") {
            return app.db_dump();
        }
        if let Some(_) = matches.subcommand_matches("db:version") {
            return app.db_version();
        }
        if let Some(_) = matches.subcommand_matches("cache:clear") {
            return app.cache_clear();
        }
        if let Some(_) = matches.subcommand_matches("cache:list") {
            return app.cache_list();
        }
        if let Some(matches) = matches.subcommand_matches("db:restore") {
            if let Some(name) = matches.value_of("name") {
                return app.db_restore(&s!(name));
            }
        }

        app.server()
    }
}

impl App {
    fn new() -> Result<Self> {
        let name = Self::config_file();
        log::info!("load config from file {}", name);
        let mut file = fs::File::open(name)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let cfg: env::Config = toml::from_slice(&buf)?;

        Ok(Self {
            ctx: Context::new(&cfg)?,
        })
    }
    fn generate_config() -> Result<()> {
        let localhost = "localhost";

        let cfg = env::Config {
            name: s!("www.change-me.com"),
            secret_key: base64::encode(&security::random_bytes(32)),
            env: rocket::config::Environment::Development.to_string(),
            languages: vec![s!("en-US"), s!("zh-Hans"), s!("zh-Hant")],
            workers: 32,
            http: env::Http {
                theme: s!("bootstrap"),
                limits: 1 << 25,
                port: 8080,
                origins: vec![s!("http://localhost:3000"), s!("http://localhost:8080")],
            },
            database: env::Database {
                #[cfg(feature = "postgresql")]
                postgresql: dao::postgresql::Config {
                    host: s!(localhost),
                    port: 5432,
                    name: s!(env::NAME),
                    user: s!("postgres"),
                    password: s!(""),
                },
                #[cfg(feature = "mysql")]
                mysql: dao::mysql::Config {
                    host: s!(localhost),
                    port: 3306,
                    name: s!(env::NAME),
                    user: s!("postgres"),
                    password: s!(""),
                },
            },
            cache: env::Cache {
                namespace: s!("www.change-me.com"),
                #[cfg(feature = "ch-redis")]
                redis: cache::redis::Config {
                    host: s!(localhost),
                    port: 6379,
                    db: 6,
                    password: None,
                },
            },
            queue: env::Queue {
                name: s!("tasks"),
                #[cfg(feature = "mq-rabbit")]
                rabbitmq: queue::rabbitmq::Config {
                    host: s!(localhost),
                    port: 5672,
                    _virtual: s!(env::NAME),
                    user: s!("guest"),
                    password: s!("guest"),
                },
            },
            #[cfg(any(feature = "mq-aws", feature = "st-aws"))]
            aws: env::Aws {
                access_key_id: s!("change-me"),
                secret_access_key: s!("change-me"),
            },
            elasticsearch: env::ElasticSearch {
                hosts: vec![s!("http://localhost:9200")],
            },
            storage: env::Storage {
                #[cfg(feature = "st-nfs")]
                nfs: env::Nfs {
                    end_point: s!("/upload"),
                    local_root: s!("tmp/upload"),
                },
                #[cfg(feature = "st-s3")]
                s3: env::S3 {
                    region: s!("us-west-2"),
                    bucket: s!("www.change-me.com"),
                },
            },
        };
        let buf = toml::to_vec(&cfg)?;

        log::info!("generate file {}", Self::config_file());
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(Self::config_file())?;
        file.write_all(&buf)?;

        Ok(())
    }
    fn routes() -> Result<()> {
        let mut app = rocket::custom(
            Config::build(Environment::Production)
                .log_level(LoggingLevel::Critical)
                .finalize()?,
            false,
        );
        for (pt, rt) in router::routes() {
            app = app.mount(pt, rt);
        }
        println!("{:8} {:<8} {}", "METHOD", "RANK", "PATH");
        for it in app.routes() {
            println!("{:8} {:<8} {}", it.method, it.rank, it.uri.path());
        }
        Ok(())
    }
    fn generate_nginx(&self) -> Result<()> {
        let cur = current_dir()?;
        let mut fd = fs::OpenOptions::new()
            .read(true)
            .open(Path::new("templates").join("nginx.conf.hbs"))?;
        let mut buf = String::new();
        fd.read_to_string(&mut buf)?;

        let body = Handlebars::new().render_template(
            &buf,
            &json!({
                    "name": self.ctx.config.name,
                    "port": self.ctx.config.http.port,
                    "root": cur,
                    "version":"v1",
                }),
        )?;

        let root = Path::new("tmp");
        fs::create_dir_all(&root)?;
        let file = root.join("nginx.conf");
        log::info!("generate file {}", file.display());
        let mut tpl = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o644)
            .open(file)?;
        tpl.write_all(body.as_bytes())?;

        Ok(())
    }
    fn db_migrate(&self) -> Result<()> {
        let db = self.ctx.db.get()?;
        let db = Dao::new(db.deref());
        db.migrate()
    }
    fn db_rollback(&self) -> Result<()> {
        let db = self.ctx.db.get()?;
        let db = Dao::new(db.deref());
        db.rollback()
    }
    fn db_seed(&self) -> Result<()> {
        let root = Path::new("db").join("seed");
        let db = self.ctx.db.get()?;
        let db = Dao::new(db.deref());
        self.load_locales(&db, &root)?;
        // TODO
        // plugins::mall::seed::load(db, &root)?;
        // plugins::cbeta::seed::load(db)?;
        // plugins::nut::seed::administrator(db)?;
        log::info!("Done!!!");
        Ok(())
    }
    fn db_dump(&self) -> Result<()> {
        let db = self.ctx.db.get()?;
        let db = Dao::new(db.deref());
        db.dump()
    }
    fn db_version(&self) -> Result<()> {
        let db = self.ctx.db.get()?;
        let db = Dao::new(db.deref());
        println!("{:16} {}", "VERSION", "RUN ON");
        for (v, r) in db.versions()? {
            println!("{:16} {}", v, r.to_rfc822());
        }
        Ok(())
    }
    fn db_restore(&self, _name: &String) -> Result<()> {
        let db = self.ctx.db.get()?;
        let db = Dao::new(db.deref());
        db.restore()
    }
    fn cache_list(&self) -> Result<()> {
        let items = self.ctx.cache.keys()?;
        println!("{:64} {}", "KEY", "TTL");
        for (key, ttl) in items {
            println!("{:64} {}", key, ttl);
        }
        Ok(())
    }
    fn cache_clear(&self) -> Result<()> {
        let cnt = self.ctx.cache.clear()?;
        log::info!("remove {} items from cache", cnt);
        Ok(())
    }
    fn server(&self) -> Result<()> {
        // http
        let mut app = rocket::custom(
            rocket::config::Config::build(self.ctx.config.env()?)
                .address("localhost")
                .workers(self.ctx.config.workers)
                .secret_key(self.ctx.config.secret_key.clone())
                .port(self.ctx.config.http.port)
                .limits(self.ctx.config.http.limits())
                .extra(
                    "template_dir",
                    match Path::new("themes")
                        .join(&self.ctx.config.http.theme)
                        .join("views")
                        .to_str()
                    {
                        Some(v) => v,
                        None => "views",
                    },
                )
                .finalize()?,
            false,
        ).manage(self.ctx.db.clone())
            .manage(self.ctx.cache.clone())
            .manage(self.ctx.secret_box.clone())
            .manage(self.ctx.queue.clone())
            .manage(graphql::schema::Schema::new(
                graphql::query::Query {},
                graphql::mutation::Mutation {},
            ))
            .manage(self.ctx.config.clone());

        if !self.ctx.config.is_prod() {
            app = app.mount("/", routes!(router::get_assets))
        }

        for (pt, rt) in router::routes() {
            app = app.mount(pt, rt);
        }

        let cors = self.ctx.config.http.cors();
        thread::spawn(|| {
            // Template::custom(|engines| {
            //     engines
            //         .handlebars
            //         .register_global_function("eq", format::eq);
            // })
            app.attach(Template::fairing())
                .attach(cors)
                .catch(router::catchers())
                .launch();
        });

        // worker
        loop {
            let name = sys_info::hostname()?;
            log::info!("starting worker thread {}", name);
            match self.ctx.queue.consume(name, &Arc::new(self.ctx.clone())) {
                Ok(_) => log::info!("exiting worker"),
                Err(e) => log::error!("{:?}", e),
            };
            thread::sleep(Duration::from_secs(10));
        }
    }
}

impl App {
    fn config_file() -> &'static str {
        return "config.toml";
    }

    fn load_locales<D: LocaleDao>(&self, db: &D, root: &PathBuf) -> Result<()> {
        let dir = root.join("locales");
        log::info!("load locales");
        let (total, inserted) = i18n::sync(db, dir)?;
        log::info!("total {}, inserted {}", total, inserted);
        Ok(())
    }
}