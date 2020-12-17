
use std::collections::HashMap;
use std::sync::{Arc, Mutex};



#[derive(Debug, Clone)]
pub struct App {
    shared: Arc<Mutex<HashMap<String, String>>>,
}
impl App {
    pub fn new() -> App {

        let  map: HashMap<String, String> = HashMap::new();
        let shared = Arc::new( Mutex::new( map));
        App { shared: shared }
    }

    pub fn set(&self, key: String, value: String) {
                let mut state = self.shared.lock().unwrap();
                let _prev = state.insert(
                    key,
                    value
                );
    }

    pub fn get(&self, key: &String) -> Option<String> {
        let state = self.shared.lock().unwrap();
        state.get(key).map(|sender| sender.clone())
    }

    pub fn list(&self)
    {
        let state = self.shared.lock().unwrap();
        if state.len()==0 {
            println!("no known apps");
        }else{
            for (key, value) in state.iter() {
            println!("{:?} / {:?}", key, value);
            }
        }
}
  
}

