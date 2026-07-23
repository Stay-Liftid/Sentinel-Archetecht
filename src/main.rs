use the_remote_viewer::AttackDetection; // Replace with your crate name if different

fn main() {
    // Initialize the detection engine with a starting threshold
    let detector = AttackDetection::new(0.9);

    // Process a sample incoming event
    let result = detector.process_event("DDoS", "192.168.1.100");
    println!("Event Result: {:#?}", result);

    // Retrieve metrics for verification
    if let Some(metrics) = detector.get_metrics("DDoS", "192.168.1.100") {
        println!("Retrieved Metrics: {:#?}", metrics);
    }
}
