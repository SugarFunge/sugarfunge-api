use crate::state::*;
use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use crossbeam::channel;
use futures::StreamExt;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use sugarfunge_api_types::sugarfunge;

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

impl SubcriptionServiceWS {
    pub fn new(data: web::Data<AppState>) -> Self {
        Self {
            data,
            last_client_heartbeat: Instant::now(),
            subs: HashMap::new(),
        }
    }

    fn subscribe(&mut self, ctx: &mut <Self as Actor>::Context) {
        let (tx_origin, rx) = channel::unbounded();
        let api = self.data.api.clone();
        let tx = tx_origin.clone();

        let balances_task = async move {

            let mut balances_events = api.events().subscribe().await.unwrap().filter_events::<(
                    sugarfunge::balances::events::Deposit,
                    sugarfunge::balances::events::Transfer,
            )>();

            while let Some(event) = balances_events.next().await {
                if let Ok(event) = event {
                    let event = match event.event {
                        (None, Some(event)) => serde_json::to_string_pretty(&event),
                        (Some(event), None) => serde_json::to_string_pretty(&event),
                        _ => panic!("Invalid event"),
                    };
                    if let Ok(event) = event {
                        tx.send(event).unwrap();
                    }
                }
            }
        }

        .into_actor(self);

        let api = self.data.api.clone();

        let tx = tx_origin.clone();
        
        let asset_task = async move {
            let mut asset_events = api.events().subscribe().await.unwrap().filter_events::<(
                sugarfunge::asset::events::Transferred,
                sugarfunge::asset::events::Mint,
            )>();

            while let Some(event) = asset_events.next().await {
                if let Ok(event) = event {
                    let event = match event.event {
                        (None, Some(event)) => serde_json::to_string_pretty(&event),
                        (Some(event), None) => serde_json::to_string_pretty(&event),
                        _ => panic!("Invalid event"),
                    };
                    if let Ok(event) = event {
                        tx.send(event).unwrap();
                    }
                }
            }
        }

        .into_actor(self);

        let api = self.data.api.clone();

        let tx = tx_origin.clone();

        let bag_task = async move {
            let mut bag_events = api.events().subscribe().await.unwrap().filter_events::<(
                sugarfunge::bag::events::Created,
                sugarfunge::bag::events::Deposit,
            )>();

            while let Some(event) = bag_events.next().await {
                if let Ok(event) = event {
                    let event = match event.event {
                        (None, Some(event)) => serde_json::to_string_pretty(&event),
                        (Some(event), None) => serde_json::to_string_pretty(&event),
                        _ => panic!("Invalid event"),
                    };
                    if let Ok(event) = event {
                        tx.send(event).unwrap();
                    }
                }
            }
        }

        .into_actor(self);

        let sub: SpawnHandle = ctx.spawn(balances_task);

        self.subs.insert("balances_events".into(), sub);

        let sub: SpawnHandle = ctx.spawn(asset_task);

        self.subs.insert("asset_events".into(), sub);

        let sub: SpawnHandle = ctx.spawn(bag_task);

        self.subs.insert("bag_events".into(), sub);

        ctx.run_interval(HEARTBEAT_INTERVAL, move |_act, ctx| {
            if let Ok(event) = rx.try_recv() {
                ctx.text(format!("{:#?}", event));
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
            Ok(ws::Message::Text(text)) => ctx.text(format!("echo: {}", text)),
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
