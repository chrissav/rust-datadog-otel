use opentelemetry::global;
use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry::{
    trace::{Span, TraceContextExt, Tracer},
    Key,
};
use opentelemetry_datadog::{new_pipeline, ApiVersion};
use std::env;
use std::println;
use std::thread;
use std::time::Duration;

fn banana() {
    let tracer = global::tracer("component-banana");
    let mut span = tracer.start("banana");
    span.set_attribute(Key::new("span.type").string("sql"));
    span.set_attribute(Key::new("sql.query").string("SELECT * FROM table"));

    println!("msg from banana!");
    thread::sleep(Duration::from_secs(3));

    span.end()
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let agent_host = env::var("DD_AGENT_HOST").unwrap_or("none".to_string());

    let tracer = new_pipeline()
        .with_service_name("rust-service")
        .with_version(ApiVersion::Version05)
        .with_agent_endpoint(format!("http://{agent_host}:8126"))
        .install_simple()?;

    loop {
        println!("msg from kiwi!");
        tracer.in_span("kiwi", |cx| {
            let span = cx.span();
            span.set_attribute(Key::new("span.type").string("web"));
            span.set_attribute(Key::new("http.url").string("http://localhost:8080/kiwi"));
            span.set_attribute(Key::new("http.method").string("GET"));
            span.set_attribute(Key::new("http.status_code").i64(200));

            thread::sleep(Duration::from_millis(6));
            banana();
            thread::sleep(Duration::from_secs(10));
        });
        thread::sleep(Duration::from_secs(10));
    }

    shutdown_tracer_provider();

    Ok(())
}
