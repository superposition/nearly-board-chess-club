# Board Chess NFT image creator API

This an API that creates an NFT image for the board chess club. The properties are chosen with a binomial distripution making some properies rarer than others

## How to run

1. Install node

`sudo apt install node`

2. Install dependencies

`npm install express fs canvas nft.storage node-fetch`

3. Run the app

`node main.js`

and it will run on localhost:3000 (you can change the port within the code)

To make a new NFT image make a `GET /new`.

