use app_core::Plugin;

#[derive(Debug)]
struct Counter;

impl Plugin for Counter {
    fn hook(&mut self, _: &mut app_core::Context) -> Result<(), app_core::Error> {
        let mut amount = 1_000_000_000;
        let start = std::time::Instant::now();
     
        // Stop timer
        let duration = start.elapsed();

        println!("Time elapsed in expensive_function() is: {:?}", duration);

        Ok(())
    }
}

#[no_mangle]
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Counter)
}
