const { invoke } = window.__TAURI__.tauri;
const { sendNotification, isPermissionGranted } = window.__TAURI__.notification;
console.log("Script loaded");

document.addEventListener("DOMContentLoaded", async () => {
  let permissionGranted = await isPermissionGranted();
  if (!permissionGranted) {
    const permission = await requestPermission();
    permissionGranted = permission === "granted";
  }
  if (permissionGranted) {
    sendNotification("Tauri is awesome!");
    sendNotification({ title: "TAURI", body: "Tauri is awesome!" });
  }
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
        showNotification("Email Envoyé", "Votre mail a été envoyé avec succès");
      } catch (error) {
        console.error("Error submitting form:", error);
        showNotification(
          "Erreur",
          "Une erreur s'est produite lors de l'envoi de l'email.",
        );
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
    console.log("message", message);
    updateTextField(message, objet);
    originalText = dynamicTextarea.value;

    const entrepriseName = entrepriseInput.value;

    // Replace the placeholder with the entered company name
    let updatedText = originalText;
    if (entrepriseName) {
      updatedText = originalText.replaceAll(placeholder, entrepriseName);
    }

    dynamicTextarea.value = updatedText;
  });

  async function fetchEmails() {
    try {
      const data = await invoke("get_email_addresses");
      console.log(data);
      return data.email_list;
    } catch (error) {
      console.error("Failed to fetch emails:", error);
      return [];
    }
  }

  const emails = await fetchEmails();
  console.log(emails);
  const emailInput = document.getElementById("email");
  const autocompleteList = document.getElementById("autocomplete-list");

  emailInput.addEventListener("input", function () {
    const inputValue = this.value.toLowerCase();
    autocompleteList.innerHTML = "";
    if (!inputValue) {
      return;
    }
    const filteredEmails = emails.filter((email) =>
      email.toLowerCase().includes(inputValue)
    );
    filteredEmails.forEach((email) => {
      const suggestionItem = document.createElement("div");
      suggestionItem.className = "autocomplete-suggestion";
      suggestionItem.innerText = email;
      console.log("Event");
      suggestionItem.addEventListener("click", function () {
        emailInput.value = email;
        autocompleteList.innerHTML = "";
      });
      autocompleteList.appendChild(suggestionItem);
    });
  });

  document.addEventListener("click", function (e) {
    if (e.target !== emailInput) {
      autocompleteList.innerHTML = "";
    }
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
  textField.value = message;
  textSubject.value = objet;
}

function showNotification(title, body) {
  if (Notification.permission === "granted") {
    new Notification(title, { body });
  } else if (Notification.permission !== "denied") {
    Notification.requestPermission().then((permission) => {
      if (permission === "granted") {
        new Notification(title, { body });
      }
    });
  }
}
