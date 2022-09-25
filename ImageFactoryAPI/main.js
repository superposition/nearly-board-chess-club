// Libraries
const express= require('express');
const { createCanvas, loadImage } = require('canvas');
const fs = require('fs')
const functions = require('./functions.js');
const { NFTStorage,File , Blob} = require('nft.storage')
const fetch = require('node-fetch')

// server constants
let app=express();
let port = 3000
app.use(express.static('./'))

//nft.storage constants
const NFT_STORAGE_TOKEN = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJkaWQ6ZXRocjoweDRBZDIzRDJlMkQxNTJEZDhmMzhlY0Q0OGY1NDc4YzZGQkUwRkJhQzMiLCJpc3MiOiJuZnQtc3RvcmFnZSIsImlhdCI6MTY2NDA1MzIwNDIwMSwibmFtZSI6Ik5lYXJDaGVzcyJ9.Q2kXoE6niNozRThXw4fmABwf1_d8EEIUmLzsXPiPXqo'
const client = new NFTStorage({ token: NFT_STORAGE_TOKEN })

// PNG variables
const width=height=300






function createPNG(res){
    const cnv = createCanvas(width, height)
    const ctx = cnv.getContext('2d')
    
    let masks = fs.readdirSync('./Textures')
    console.log((masks.length+1)*functions.randn_bm())
    let mask = masks[Math.floor((masks.length+1)*functions.randn_bm())]
    // peice
    loadImage(`Textures/`+mask).then((mask)=>{     
        loadImage(`pieces/${Math.ceil(Math.random()*6)}.png`).then(async (peiceImg)=>{
            ctx.drawImage(mask,0,0,500,500)
            ctx.globalCompositeOperation = 'destination-in';
            ctx.drawImage(peiceImg,width/4,height/16,150,250)
            ctx.globalCompositeOperation = 'destination-atop';


            //Linear Gradient
            let grad = ctx.createLinearGradient(0,0,width,height);
            grad.addColorStop(0,functions.createColorND()+"aa"); // choosing random color
            grad.addColorStop(1,functions.createColorND())+"aa"; // choosing random color

            
            // // background
            // ctx.fillStyle = functions.createColorND();
            // ctx.fillRect(0, 0, width, height);

                
            ctx.fillStyle = grad;
            ctx.fillRect(0,0,width,height)
            


            let buffer =  cnv.toBuffer('image/png')
            
            fs.writeFileSync('./image.png',buffer)
            
            // let img  = new File(buffer, 'image.png', { type: 'image/png' })

            let blob = new Blob([buffer])
            const cid = await client.storeBlob(blob)
            console.log(cid)
            res.send(cid)
        })
    })


}


app.use('./', express.static('./')) // Temporary to see image
app.get('/new',async (req,res)=>{
    createPNG(res)
    


    let cid = ''
    // res.send(`<img src="http://localhost:${port}/image.png">`)
})


app.listen(port, () => {
    console.log(`listening on port ${port}`)
    
});
    
