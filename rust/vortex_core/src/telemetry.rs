#[cfg(feature = "mesocarp_integration")]
use crate::integrations::mesocarp_wrapper;

pub fn publish_telemetry(data: &str) {
    #[cfg(feature = "mesocarp_integration")]
    {
        if let Err(e) = mesocarp_wrapper::send_message(data) {
            eprintln!("Failed to send telemetry via Mesocarp: {}", e);
        }
    }

    #[cfg(not(feature = "mesocarp_integration"))]
    {
        println!("Telemetry (no-op): {}", data);
    }
}
