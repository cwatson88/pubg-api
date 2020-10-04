### Docker Steps

- docker build -t cwatson1988/pubg-rust-server .
- docker run -p 8080:8080 cwatson1988/pubg-rust-server

### Google Run Steps:

- gcloud builds submit --tag gcr.io/pubg-291421/pubg-rust-server --timeout=900
- gcloud run deploy --image gcr.io/pubg-291421/pubg-rust-server --platform managed
