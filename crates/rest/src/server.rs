use crate::routes::routes;
use actix_web::web::Data;
use actix_web::{dev, App, HttpServer};
use commons::{
    configuration::settings::Settings,
    id::{IdGenerator, StdIdGenerator},
    time::SystemClock,
};
use cqrs::command_bus::bus::CommandBus;
use sqlx::PgPool;
use std::{net::TcpListener, sync::Arc};
use storage::pool_from_settings;

#[derive(Clone)]
pub struct AppState {
    pub command_bus: Arc<CommandBus>,
    pub ids: Arc<dyn IdGenerator>,
    pub pool: PgPool,
}

#[macro_export]
macro_rules! build_app {
    ($state: expr) => {
        App::new()
            .app_data(Data::new($state))
            // Middleware is applied LIFO
            // These will wrap all outbound responses with matching status codes.
            //.wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, HandlerError::render_404))
            // These are our wrappers
            //.wrap(middleware::sentry::SentryWrapper::default())
            // Followed by the "official middleware" so they run first.
            // actix is getting increasingly tighter about CORS headers. Our server is
            // not a huge risk but does deliver XHR JSON content.
            // For now, let's be permissive and use NGINX (the wrapping server)
            // for finer grained specification.
            //.wrap(Cors::permissive())
            // Dockerflow
            // Remember to update .::web::middleware::DOCKER_FLOW_ENDPOINTS
            // when applying changes to endpoint names.
            // .service(web::resource("/__heartbeat__").route(web::get().to(handlers::heartbeat)))
            // .service(
            //     web::resource("/__lbheartbeat__").route(web::get().to(|_: HttpRequest| {
            //         // used by the load balancers, just return OK.
            //         HttpResponse::Ok()
            //             .content_type("application/json")
            //             .body("{}")
            //     })),
            // )
            // .service(
            //     web::resource("/__version__").route(web::get().to(|_: HttpRequest| {
            //         // return the contents of the version.json file created by circleci
            //         // and stored in the docker root
            //         HttpResponse::Ok()
            //             .content_type("application/json")
            //             .body(include_str!("../../version.json"))
            //     })),
            // )
            // .service(web::resource("/__error__").route(web::get().to(handlers::test_error)))
            .service(routes())
    };
}

pub struct Server(dev::Server);

impl Server {
    pub async fn from_settings(settings: &Settings) -> Result<Self, anyhow::Error> {
        let ids = Arc::new(StdIdGenerator);
        let pool = pool_from_settings(settings).await?;
        let state = AppState {
            command_bus: Arc::new(CommandBus::new(
                pool.clone(),
                Arc::new(SystemClock),
                ids.clone(),
            )),
            ids: ids,
            pool: pool.clone(),
        };

        Ok(Self(
            HttpServer::new(move || build_app!(state.clone()))
                .listen(Self::listener(settings)?)?
                .run(),
        ))
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.0.await
    }

    fn listener(settings: &Settings) -> Result<TcpListener, std::io::Error> {
        TcpListener::bind(format!("{}:{}", settings.server.host, settings.server.port))
    }
}
