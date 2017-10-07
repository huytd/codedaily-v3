class Utils {
  static reloadAfter(timeMs) {
    setTimeout(() => {
      window.location.reload();
    }, timeMs);
  }


  static decodeEntities(encodedString) {
    let textArea = document.createElement('textarea');
    textArea.innerHTML = encodedString;
    return textArea.value;
  }


  static enterKeyPressed(ev) {
    console.log(ev);
    if (! ev) return false;

    return (
      ev.keyCode == 0x0D || // 13, Enter     [DEPRECATED] All browsers
      ev.charCode == 0x0D || // 13, Enter    [DEPRECATED] IE>8, Edge, Firefox, Chrome, Safari, Opera, iOS Safari
      ev.which == 0x0D || // 13, Enter       [DEPRECATED] IE>8, Edge, Firefox, Chrome, Safari, Opera, iOS Safari

      ev.key == 'Enter' || //                Firefox>28, Chrome>50, Safari>10, Opera>37, iOS Safari>10.2
      ev.code == 'Enter' || //               Firefox>37, Chrome>47, Safari>10, Opera>34, iOS Safari>10.2
      ev.code == 'NumpadEnter' ||
      ev.char == '\n' || //                  IE>8, Edge
      ev.char == '\r'
    );
  }
}


module.exports = Utils;
