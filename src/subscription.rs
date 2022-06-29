use crate::state::*;
use crate::util::*;
use actix::prelude::*;
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use futures::StreamExt;
use serde_json::json;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use sugarfunge_api_types::sugarfunge;
use crossbeam::channel;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// websocket connection is long running connection
pub struct SubcriptionServiceWS {
    data: web::Data<AppState>,
    /// Client must send ping at least once per CLIENT_TIMEOUT seconds,
    /// otherwise we drop connection.
    last_client_heartbeat: Instant,
    subs: HashMap<String, SpawnHandle>,
}

pub fn map_ws_subxt_err(e: subxt::GenericError<std::convert::Infallible>) -> actix_web::Error {
    // TODO: json_err should be a json Value to improve UX
    let json_err = json!(e.to_string());
    let req_error = RequestError {
        message: json_err,
        description: "Subxt error".into(),
    };
    let req_error = serde_json::to_string_pretty(&req_error).unwrap();
    error::ErrorBadRequest(req_error)
}

impl SubcriptionServiceWS {
    pub fn new(data: web::Data<AppState>) -> Self {
        Self {
            data,
            last_client_heartbeat: Instant::now(),
            subs: HashMap::new(),
        }
    }

    fn subscribe(&mut self, ctx: &mut <Self as Actor>::Context) {
        let api = self.data.api.clone();

        let (tx, rx) = channel::unbounded();

        // api.events().subscribe().await.into

        let task = async move {

            let mut events = api.events().subscribe().await.unwrap()
            .filter_events::<(sugarfunge::balances::events::Transfer,)>();

            while let Some(event) = events.next().await {
                println!("Balance transfer event: {event:?}");
                tx.send(event).unwrap();
            }
        }
        .into_actor(self);

        let sub: SpawnHandle = ctx.spawn(task);

        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {

            if let Ok(Ok(event)) = rx.try_recv() {
                ctx.text(format!("{:#?}", event.event));
            }
        });
    }

    fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.last_client_heartbeat) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for SubcriptionServiceWS {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
        self.subscribe(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SubcriptionServiceWS {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {msg:?}");
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.last_client_heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.last_client_heartbeat = Instant::now();
            }
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

/// WebSocket handshake and start `SubcriptionServiceWS` actor.
pub async fn ws(
    data: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(SubcriptionServiceWS::new(data), &req, stream)
}
