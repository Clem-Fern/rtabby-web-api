// Function to copy token to clipboard
function copyToken() {
    var tokenDisplay = document.getElementById('tokenDisplay');
    var tempInput = document.createElement('input');
    tempInput.value = tokenDisplay.innerText;
    document.body.appendChild(tempInput);
    tempInput.select();
    document.execCommand('copy');
    document.body.removeChild(tempInput);
  
    // Change button text to "Copied"
    var copyBtn = document.querySelector('.copy-btn');
    copyBtn.innerText = 'Copied';
  }
  