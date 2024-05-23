// const { invoke } = window.__TAURI__.tauri;
console.log("Script loaded");
// script.js

document.addEventListener('DOMContentLoaded', function() {
    const form = document.getElementById('myForm');
    if (form) {
        form.addEventListener('submit', async function(event) {
            event.preventDefault();

            const formData = new FormData(event.target);
            //const data = Object.fromEntries(formData.entries());
            const data = Object.fromEntries(formData.entries());
            console.log('Form data:', data);  // Ajoutez cette ligne pour déboguer
            try {
                const response = await window.__TAURI__.invoke('process_form', { data });
                console.log('Form submitted successfully:', response);
            } catch (error) {
                console.error('Error submitting form:', error);
            }
        });
    } else {
        console.error('Form with ID "myForm" not found.');
    }
});



