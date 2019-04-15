extern crate boguin;
extern crate rustls;
extern crate webpki;
extern crate webpki_roots;
extern crate http_with_url as http;

#[macro_use]
extern crate lazy_static;
extern crate proc_macro;

use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;
use boguin::Client;

lazy_static! {
    static ref CLIENT: Arc<Mutex<Client>> = Arc::new(Mutex::new(Client::new()));
}


fn main() {
    spawn_thread("test1", test1);
    spawn_thread("test2", test1);
    spawn_thread("test3", test1);
    spawn_thread("test4", test1);
    thread::sleep(Duration::from_secs(160));
}

fn testget() {
    let _ = CLIENT.lock().and_then(|mut client| {

        let url = http::Url::parse("https://httpbin.org/status/418").unwrap();
        let request = http::Request::new(url, ());
        let _ = client.fetch(request).map(|response:http::Response<String>| {
            println!("{}", response.status());
        }).map_err(|err| {
            *client = Client::new();
            println!("error {:?}", err);
        });
        thread::sleep(Duration::from_secs(1));
        Ok(())
    }).map_err(|_e| eprintln!("problem during lock client"));

}

pub fn test1() {
    loop {
        let _ = testget();
        thread::sleep(Duration::from_millis(50));
    }
}

fn spawn_thread(name: &'static str, worker: fn()) {
    let _ = thread::Builder::new()
        .name(name.to_string())
        .spawn(move || loop {
            worker();
        });
}
