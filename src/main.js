const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

const addresses = ["john@example.com", "jane@example.com", "alice@example.com"];

const searchInput = document.getElementById("searchInput");
const addressList = document.getElementById("addressList");

function handleResultSelection(event) {
  const selectedAddress = event.target.value;
  searchInput.value = selectedAddress;
}

function populateAddressList(results) {
  addressList.innerHTML = ""; // Effacer les anciennes options

  results.forEach(result => {
    const option = document.createElement("option");
    option.textContent = result;
    addressList.appendChild(option);
  });
}

searchInput.addEventListener("input", function(event) {
  const searchTerm = event.target.value.toLowerCase();
  const filteredAddresses = addresses.filter(address => address.toLowerCase().includes(searchTerm));
  populateAddressList(filteredAddresses);
});

addressList.addEventListener("change", handleResultSelection);

