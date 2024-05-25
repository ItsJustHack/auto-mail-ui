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
      const templateChosen =
        document.getElementById("mailTemplateSelect").value;
      console.log("Form data:", data); // Ajoutez cette ligne pour déboguer
      try {
        const response = await invoke("process_form", { data, templateChosen });
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
    const [message, objet] = await invoke("change_message", {
      templateChosen: selectedMail,
    });
    console.log("objet", objet);
    updateTextField(message, objet);
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

function updateTextField(message, objet) {
  console.log("Si je vois ça on est bon");
  const textField = document.querySelector("#message");
  const textSubject = document.querySelector("#subject");
  textField.textContent = message;
  textSubject.value = objet;
}
