const { invoke } = window.__TAURI__.tauri;
console.log("Script loaded");

document.addEventListener("DOMContentLoaded", async () => {
  try {
    console.log("Initialisation avant load_mail_config");
    const mailNames = await invoke("load_mail_config");
    updateSelectMenu(mailNames);
  } catch (error) {
    console.error("Error loading mail config:", error);
  }

  const form = document.getElementById("myForm");
  if (form) {
    form.addEventListener("submit", async function (event) {
      event.preventDefault();

      const formData = new FormData(event.target);
      const data = Object.fromEntries(formData.entries());
      console.log("Form data:", data); // Ajoutez cette ligne pour déboguer
      try {
        const response = await invoke("process_form", { data });
        console.log("Form submitted successfully:", response);
      } catch (error) {
        console.error("Error submitting form:", error);
      }
    });
  } else {
    console.error('Form with ID "myForm" not found.');
  }

  // Ce code replace l'entreprise

  const entrepriseInput = document.getElementById("entreprise");
  const dynamicTextarea = document.getElementById("message");
  const placeholder = "[entreprise]";

  // Stock the original text with placeholder
  let originalText = dynamicTextarea.value;

  entrepriseInput.addEventListener("input", () => {
    const entrepriseName = entrepriseInput.value;

    // Replace the placeholder with the entered company name
    let updatedText = originalText;
    if (entrepriseName) {
      updatedText = originalText.replaceAll(placeholder, entrepriseName);
    }

    dynamicTextarea.value = updatedText;
  });

  const selectMenu = document.querySelector("#mailTemplateSelect");
  selectMenu.addEventListener("change", async (event) => {
    console.log("Change button");
    const selectedMail = event.target.value;
    const message = await invoke("change_message", {
      templateChosen: selectedMail,
    });
    updateTextField(message);
    originalText = dynamicTextarea.value;
  });
});

function updateSelectMenu(mailNames) {
  console.log("Mise à jour des options");
  const selectMenu = document.querySelector("#mailTemplateSelect");
  mailNames.forEach((name) => {
    const option = document.createElement("option");
    option.value = name;
    option.textContent = name;
    selectMenu.appendChild(option);
  });
}

function updateTextField(message) {
  console.log("Si je vois ça on est bon");
  const textField = document.querySelector("#message");
  textField.textContent = message;
}
