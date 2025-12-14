(() => {
  // main.ts
  function scrollToContent() {
    let el = document.getElementById("content");
    el?.scrollIntoView({ behavior: "smooth" });
  }
  window.scrollToContent = scrollToContent;
  document.addEventListener("DOMContentLoaded", () => {
    const textElement = document.getElementById("typewriter-text");
    if (!textElement) {
      console.error('Element with id "typewriter-text" not found.');
      return;
    }
    const texts = [
      "\u4E03\u767E\u5F04\u91CC\u8BDD\u7476\u4E61\uFF0C\u5E03\u52AA\u513F\u5973\u60C5\u610F\u957F\u3002",
      "\u94DC\u9F13\u58F0\u58F0\u9707\u5C71\u5C97\uFF0C\u4E94\u5F69\u7CEF\u996D\u98D8\u6E05\u9999\u3002",
      "\u884C\u5230\u6C34\u7A77\u5904\uFF0C\u5750\u770B\u4E91\u8D77\u65F6\u3002"
    ];
    let textIndex = 0;
    let charIndex = 0;
    let isDeleting = false;
    function type() {
      const currentText = texts[textIndex];
      const endIndex = isDeleting ? Math.max(0, charIndex - 1) : Math.min(currentText.length, charIndex + 1);
      textElement.textContent = currentText.substring(0, endIndex);
      charIndex += isDeleting ? -1 : 1;
      let typeSpeed = isDeleting ? 50 : 150;
      if (!isDeleting && charIndex >= currentText.length) {
        typeSpeed = 2e3;
        isDeleting = true;
      } else if (isDeleting && charIndex <= 0) {
        isDeleting = false;
        textIndex = (textIndex + 1) % texts.length;
        charIndex = 0;
        typeSpeed = 500;
      }
      window.setTimeout(type, typeSpeed);
    }
    type();
  });
})();
