declare global {
  interface Window {
    scrollToContent?: () => void;
  }
}

function scrollToContent(): void {
  let el: HTMLElement | null = document.getElementById("content");
  el?.scrollIntoView({ behavior: "smooth" });
}

window.scrollToContent = scrollToContent;

document.addEventListener("DOMContentLoaded", () => {
  const navbar = document.getElementById("navbar");
  const content = document.getElementById("content");
  if (!navbar || !content) return;

  let watching = false;

  // 连续判断函数
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

  // IntersectionObserver：只负责“开始 / 停止”监听
  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        watching = entry.isIntersecting;
        updateNav();
      });
    },
    {
      threshold: 0,
    },
  );
  observer.observe(content);
  // scroll 才负责实时更新
  window.addEventListener("scroll", updateNav, { passive: true });

  // 安全地在 DOMContentLoaded 内获取元素
  const textElement = document.getElementById("typewriter-text");
  if (!textElement) {
    console.error('Element with id "typewriter-text" not found.');
    return;
  }

  const texts: string[] = [
    "七百弄里话瑶乡，布努儿女情意长。",
    "铜鼓声声震山岗，五彩糯饭飘清香。",
  ];

  let textIndex = 0;
  let charIndex = 0;
  let isDeleting = false;

  function type(): void {
    const currentText = texts[textIndex];

    // 计算安全的 endIndex，避免负值或越界
    const endIndex = isDeleting
      ? Math.max(0, charIndex - 1)
      : Math.min(currentText.length, charIndex + 1);

    // 现在 textElement 一定不为 null，可以安全访问
    textElement.textContent = currentText.substring(0, endIndex);

    charIndex += isDeleting ? -1 : 1;

    let typeSpeed = isDeleting ? 50 : 150;
    if (!isDeleting && charIndex >= currentText.length) {
      typeSpeed = 2000;
      isDeleting = true;
    } else if (isDeleting && charIndex <= 0) {
      isDeleting = false;
      textIndex = (textIndex + 1) % texts.length;
      charIndex = 0;
      typeSpeed = 500;
    }

    window.setTimeout(type, typeSpeed);
  }

  // 启动打字机特效
  type();
});
