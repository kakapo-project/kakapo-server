

//FIXME: this doesn't work!!!!
export const hide = () => {

  let elems = document.getElementsByClassName('react-contextmenu')

  for (let i = 0; i < elems.length; i++) {
    setTimeout(() => {
      if (!elems[i]) { return }

      elems[i].style.opacity = '0';
      elems[i].classList.remove('react-contextmenu--visible');
    }, 20 /*TODO: this is really troubling */)
  }
}