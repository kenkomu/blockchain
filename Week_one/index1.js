/**
 * <div id ="sample"> </div>
 * <input type = "text" id ="input"/>
 * 
 */
 const output = document.getElementById("sample");
 const input = document.getElementById("input");
 
 input .addEventListener("input", (e)=>{
     const txt = e.value;
     console.log(txt);
     output.innerText = txt;
 })