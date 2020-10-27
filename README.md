# books
Demo rust application monitoring crd in k8s
Create the Book CRD and a custom book by applying the manifests in kubernetes/manifest. 
```bash
kubectl apply -f kubernetes/manifests
```
Then build and run the controller to monitor the books
```bash
cargo build --release
target/release/k8s-controller
```
Change something to the book and the controller should notify about the change  

