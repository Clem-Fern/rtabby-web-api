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
  
function logout() {
  // Delete token from cookies (assuming you are using cookies for storing the token)
  document.cookie = 'token=; Max-Age=0'
  // Redirect to the home page or any desired URL
  window.location.href = "/";
}