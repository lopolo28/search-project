# Search Project

# Run Local
Install Rust + Cargo
Run cargo run

# Run in docker
## Build Image
sudo docker build -t [project_name]

## Run Image
docker run -e GOOGLE_API_KEY=[...] -e GOOGLE_SEARCH_ENGINE_ID=[...] -p 8080:8080 --dns 8.8.8.8 --dns 8.8.4.4 search_project
