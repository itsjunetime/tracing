use core::cell::Cell;

use tracing::Instrument as _;

thread_local! {
    pub static VARIABLE: Cell<usize> = const { Cell::new(0) };
}

#[test]
fn spans_macros_are_are_send() {
    assert_send(send_fn());
}

fn get_variable() -> usize {
    VARIABLE.with(|v| {
        let g = v.get();
        v.set(g + 1);
        g
    })
}

fn assert_send(_: impl Send + Sync) {}

async fn send_fn() {
    // todo: expand this to more than `debug_span!`
    async move {}
        .instrument(tracing::debug_span!("Value", "value" = get_variable()))
        .await;
}
