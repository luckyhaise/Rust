// const { invoke } = window.__TAURI__.core;

const input = document.querySelector('input[name="input"]');
const lang = document.querySelector('#lang');
const list = document.querySelector('ol[name="wordlist"]');
const but = document.querySelector('button')

async function js(){
but.addEventListener('click', () => {
  const li= document.createElement('li'); 
  const wordinput = document.createElement('input');
  const bte = document.createElement('button');
  const btdel = document.createElement('button');
  if (input.value.trim() == '') {
    console.log("Please input something");
    return;
  }
  wordinput.readOnly=true;
  wordinput.name="word"
  wordinput.type="text"
  // edit button 
  bte.name="e"
  bte.textContent="Edit"
  bte.addEventListener('click',()=> 
{
  const isreadonly= wordinput.readOnly;
   wordinput.readOnly= !isreadonly
  bte.textContent = isreadonly? "Save":"Edit";
    

})

  // delete button
  btdel.name="del"
  btdel.textContent="Delete"
  btdel.addEventListener('click',()=>{
    list.removeChild(li)
  })




  //  assigning input value to the list
  wordinput.value = input.value 
  // appending list
  li.appendChild(wordinput)
  li.appendChild(bte)
  li.appendChild(btdel)

  list.appendChild(li)
  
  input.value = ''


    
});
}