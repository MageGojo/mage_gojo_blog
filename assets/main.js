(() => {
  // main.ts
  function scrollToContent() {
    let el = document.getElementById("content");
    el?.scrollIntoView({ behavior: "smooth" });
  }
  window.scrollToContent = scrollToContent;
  document.addEventListener("DOMContentLoaded", () => {
    const navbar = document.getElementById("navbar");
    const content = document.getElementById("content");
    if (!navbar || !content) return;
    let watching = false;
    const updateNav = () => {
      if (!watching) return;
      const rect = content.getBoundingClientRect();
      const middle = window.innerHeight / 2;
      if (rect.top <= middle) {
        navbar.classList.add("nav-hidden");
      } else {
        navbar.classList.remove("nav-hidden");
      }
    };
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          watching = entry.isIntersecting;
          updateNav();
        });
      },
      {
        threshold: 0
      }
    );
    observer.observe(content);
    window.addEventListener("scroll", updateNav, { passive: true });
    const textElement = document.getElementById("typewriter-text");
    if (!textElement) {
      console.error('Element with id "typewriter-text" not found.');
      return;
    }
    const texts = [
      "\u4E03\u767E\u5F04\u91CC\u8BDD\u7476\u4E61\uFF0C\u5E03\u52AA\u513F\u5973\u60C5\u610F\u957F\u3002",
      "\u94DC\u9F13\u58F0\u58F0\u9707\u5C71\u5C97\uFF0C\u4E94\u5F69\u7CEF\u996D\u98D8\u6E05\u9999\u3002"
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
    const btn = document.getElementById("mobile-menu-btn");
    const icon = btn?.querySelector("i");
    const overlay = document.getElementById("mobile-overlay");
    const panel = document.getElementById("mobile-menu-panel");
    let isOpen = false;
    function toggleMenu() {
      isOpen = !isOpen;
      if (isOpen) {
        overlay?.classList.remove("hidden");
        panel?.classList.remove("hidden");
        void panel?.offsetWidth;
        overlay?.classList.remove("opacity-0");
        panel?.classList.remove("opacity-0", "scale-95");
        panel?.classList.add("opacity-100", "scale-100");
        icon?.classList.remove("fa-bars");
        icon?.classList.add("fa-times", "rotate-90");
      } else {
        overlay?.classList.add("opacity-0");
        panel?.classList.remove("opacity-100", "scale-100");
        panel?.classList.add("opacity-0", "scale-95");
        icon?.classList.remove("fa-times", "rotate-90");
        icon?.classList.add("fa-bars");
        setTimeout(() => {
          if (!isOpen) {
            overlay?.classList.add("hidden");
            panel?.classList.add("hidden");
            closeAllSubmenus();
          }
        }, 300);
      }
    }
    btn?.addEventListener("click", (e) => {
      e.stopPropagation();
      toggleMenu();
    });
    overlay?.addEventListener("click", () => {
      if (isOpen) toggleMenu();
    });
    const toggles = document.querySelectorAll(".submenu-toggle");
    toggles.forEach((toggle) => {
      toggle.addEventListener("click", function() {
        const targetId = this.getAttribute("data-target");
        if (!targetId) return;
        const content2 = document.getElementById(targetId);
        const arrow = this.querySelector(".arrow-icon");
        if (content2) {
          const isOpened = content2.style.maxHeight && content2.style.maxHeight !== "0px";
          closeAllSubmenus();
          if (!isOpened) {
            const height = content2.scrollHeight;
            content2.style.maxHeight = height + "px";
            arrow?.classList.add("rotate-90");
            this.classList.add("text-yao-red");
          }
        }
      });
    });
    function closeAllSubmenus() {
      document.querySelectorAll(".submenu-content").forEach((el) => {
        el.style.maxHeight = "0px";
      });
      document.querySelectorAll(".arrow-icon").forEach((el) => {
        el.classList.remove("rotate-90");
      });
      document.querySelectorAll(".submenu-toggle").forEach((el) => {
        el.classList.remove("text-yao-red");
      });
    }
  });
})();
