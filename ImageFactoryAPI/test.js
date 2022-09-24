const { NFTStorage } = require('nft.storage') 
const fetch = require('fetch').fetchUrl


// read the API key from an environment variable. You'll need to set this before running the example!
const API_KEY = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJkaWQ6ZXRocjoweDRBZDIzRDJlMkQxNTJEZDhmMzhlY0Q0OGY1NDc4YzZGQkUwRkJhQzMiLCJpc3MiOiJuZnQtc3RvcmFnZSIsImlhdCI6MTY2NDA1MzIwNDIwMSwibmFtZSI6Ik5lYXJDaGVzcyJ9.Q2kXoE6niNozRThXw4fmABwf1_d8EEIUmLzsXPiPXqo'


  const client = new NFTStorage({ token: API_KEY })
  const metadata = client.store(nft)

  console.log('NFT data stored!')
  console.log('Metadata URI: ', metadata.url)


storeExampleNFT()