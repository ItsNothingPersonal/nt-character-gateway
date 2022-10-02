# NT Character Gateway

Gateway to retrieve the character data from the google spreadsheets

## Features

- maps an api key stored in redis to the actual google spreadsheet id
- handles the authentication with Google via a service account
- handles all the interaction with the character sheets in google drive
- displays the character information as a json string
- optional docker image for easier deployment
- written in rust with axum and tokio amongst others

## Installation

### How to run the gateway locally

- Get yourself a service account in the [Google Cloud Console](https://console.cloud.google.com/apis/credentials)
- clone the project
```bash
  git clone <repo url>
  cd <working directory>
```
- Save the service account json as credentials.json in the project root directory    
- run the project
```bash  
  cd <working directory>
  cargo run
```

### How to run the gateway with docker
- Get yourself a service account in the [Google Cloud Console](https://console.cloud.google.com/apis/credentials)
- clone the project
```bash
  git clone <repo url>
  cd <working directory>
```
- build the image
```bash  
  docker build .
```
- start the container
```bash
docker run -d -p 80:3000 -e HOST='0.0.0.0' -e PORT='3000' -e SERVICE_ACCOUNT_INFORMATION='<credentials string>' <docker-image-hash>
```
## Acknowledgements
 - [rust](https://www.rust-lang.org)
 - [Axum](https://github.com/tokio-rs/axum)
 - [tokio](https://tokio.rs)
 

## Authors

- [@ItsNothingPersonal](https://www.github.com/itsnothingpersonal)


## License

[![MIT License](https://img.shields.io/apm/l/atomic-design-ui.svg?)](https://choosealicense.com/licenses/mit/)