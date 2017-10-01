class Utils {
  static reloadAfter(time) {
    setTimeout(() => {
      window.location.reload();
    }, time);
  }

  static decodeEntities(encodedString) {
    var textArea = document.createElement('textarea');
    textArea.innerHTML = encodedString;
    return textArea.value;
  }
}

module.exports = Utils;
