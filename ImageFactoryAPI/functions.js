// Source: https://stackoverflow.com/questions/25582882/javascript-math-random-normal-distribution-gaussian-bell-curve
// Outputs a normal distribution number from 0 to 1 where the mean is 0.5

function randn_bm() {
    let u = 0, v = 0;
    while(u === 0) u = Math.random(); //Converting [0,1) to (0,1)
    while(v === 0) v = Math.random();
    let num = Math.sqrt( -2.0 * Math.log( u ) ) * Math.cos( 2.0 * Math.PI * v );
    num = num / 10.0 + 0.5; // Translate to 0 -> 1
    if (num > 1 || num < 0) return randn_bm() // resample between 0 and 1
    return num
  }

//outputs a random color with a normal distribution  
function createColorND(){
    let r = Math.floor(randn_bm()*255).toString(16)
    let g = Math.floor(randn_bm()*255).toString(16)
    let b = Math.floor(randn_bm()*255).toString(16)

    return `#${r+g+b}`

}


module.exports = {createColorND, randn_bm}