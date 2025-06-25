const { invoke } = window.__TAURI__.core;
const input = document.querySelector('#Val')
const Output = document.querySelector('#Out')
const but = document.querySelector('button')

but.addEventListener('click', async () => {
const Value = parseInt(input.value,10)
const Unit = document.querySelector("input[name='unit']:checked")?.value
if(!Unit){
    alert("Please select a Unit to convert into")
}
try {
    const result = await invoke('tempconverter', {
        value : Value,
        unit : Unit,
      } );

      Output.value = result;


    } 
    catch(e){
        alert("Rust conversation failed" + e)
    }




})

