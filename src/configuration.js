// config.js
document.addEventListener("DOMContentLoaded", () => {
  const { invoke } = window.__TAURI__.tauri;
  const { appWindow } = window.__TAURI__.window;
  const form = document.getElementById("config-form");
  const cancelBtn = document.getElementById("cancel-btn");

  cancelBtn.addEventListener("click", () => {
    appWindow.close();
  });

  form.addEventListener("submit", (e) => {
    e.preventDefault();
    const formData = {
      envoyeur: document.getElementById("email").value, // TODO: Change this to have only one time the email
      nom: document.getElementById("nom").value,
      prenom: document.getElementById("prenom").value,
      telephone: document.getElementById("phone").value,
    };
    const credentials = {
      username: document.getElementById("email").value,
      password: document.getElementById("password").value,
    };
    invoke("save_config", { config: formData, credentials: credentials })
      .then(() => {
        alert("Configuration enregistrée avec succès!");
        appWindow.close();
      })
      .catch((error) => {
        console.error(
          "Erreur lors de l'enregistrement de la configuration:",
          error,
        );
      });
  });
});
