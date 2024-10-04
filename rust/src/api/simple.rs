#[cfg(feature = "mobile")]
use flutter_rust_bridge::frb;


#[cfg_attr(feature = "mobile",frb(sync))] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[cfg_attr(feature = "mobile",frb(init))]
pub fn init_app() {
    // Default utilities - feel free to customize
    
}
