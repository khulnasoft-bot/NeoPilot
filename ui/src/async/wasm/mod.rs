// TODO: Add wasm async logic here
pub fn spawn_local<F>(future: F)
where
    F: std::future::Future<Output = ()> + 'static,
{
    // Spawn a local future
}
