use kube::api::{Informer, Object, WatchEvent, RawApi};
use crate::book::book::Book;
use kube::config;
use kube::client::APIClient;

pub const NAMESPACE: &str = "default";

pub fn watch(){
    let kubeconfig = config::load_kube_config().expect("[ERROR] Failed to load kubeconfig");
    let client = APIClient::new(kubeconfig);
    let resource = RawApi::customResource("books")
        .group("example.netapp.com")
        .within(&NAMESPACE);
    // A raw informer does not kube the k8s openapi spec. we dont need it as we are using a custom crd
    let informer = Informer::raw(client, resource).init().expect("[ERROR] Informer failed to initialize");
    loop {
        informer.poll().expect("[ERROR] Informer polling failed");
        while let Some(event) = informer.pop() {
            handle(event);
        }
    }
}
// This holds a description of the kube::api::Object
type KubeBook = Object<Book,String>;
fn handle(event: WatchEvent<KubeBook>){
    match event {
        WatchEvent::Added(book)=> {
            println!("[INFO] Added a book {} with title '{}'", book.metadata.name, book.spec.title)
        },
        WatchEvent::Deleted(book) => {
            println!("[INFO] Deleted a book {}", book.metadata.name)
        },
        _ => {
            println!("[INFO] A change was made to the book")
        }

    }

}