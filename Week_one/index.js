function readSync(){
    const fs = require('fs')
    const path = require("path")
    const file = path.join(__dirname, "sample.txt");

    try{
        const data = fs.readFileSync(file, 'utf8');
        console.log(data);
    }catch (err){
        console.error(err);
    }
}
readSync();
