export {};
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

  const btn = document.getElementById("mobile-menu-btn");
  const icon = btn?.querySelector("i");
  const overlay = document.getElementById("mobile-overlay");
  const panel = document.getElementById("mobile-menu-panel");

  // 状态标记
  let isOpen = false;

  // --- 1. 切换主菜单函数 ---
  function toggleMenu() {
    isOpen = !isOpen;

    if (isOpen) {
      // 打开逻辑
      // 1. 显示元素 (移除 hidden)
      overlay?.classList.remove("hidden");
      panel?.classList.remove("hidden");

      // 2. 强制浏览器重排 (Reflow)，为了让接下来的 transition 生效
      // 如果没有这一步，hidden 移除的同时加 opacity-100，浏览器会直接显示最终状态没有动画
      void panel?.offsetWidth;

      // 3. 添加动画状态 (Fade In / Scale Up)
      overlay?.classList.remove("opacity-0");
      panel?.classList.remove("opacity-0", "scale-95");
      panel?.classList.add("opacity-100", "scale-100");

      // 4. 图标变化
      icon?.classList.remove("fa-bars");
      icon?.classList.add("fa-times", "rotate-90"); // 加点旋转让图标变化更生动
    } else {
      // 关闭逻辑
      // 1. 撤销动画状态
      overlay?.classList.add("opacity-0");
      panel?.classList.remove("opacity-100", "scale-100");
      panel?.classList.add("opacity-0", "scale-95");

      icon?.classList.remove("fa-times", "rotate-90");
      icon?.classList.add("fa-bars");

      // 2. 等待动画结束后再隐藏元素 (300ms 与 CSS duration 一致)
      setTimeout(() => {
        if (!isOpen) {
          //再一次检查防止快速点击导致的闪烁
          overlay?.classList.add("hidden");
          panel?.classList.add("hidden");
          // 关闭菜单时，顺便收起所有二级菜单
          closeAllSubmenus();
        }
      }, 300);
    }
  }

  // --- 2. 绑定点击事件 ---
  btn?.addEventListener("click", (e) => {
    e.stopPropagation();
    toggleMenu();
  });

  overlay?.addEventListener("click", () => {
    if (isOpen) toggleMenu();
  });

  // --- 3. 二级菜单丝滑展开 (核心优化) ---
  const toggles = document.querySelectorAll(".submenu-toggle");

  toggles.forEach((toggle) => {
    toggle.addEventListener("click", function (this: HTMLElement) {
      const targetId = this.getAttribute("data-target");
      if (!targetId) return;

      const content = document.getElementById(targetId);
      const arrow = this.querySelector(".arrow-icon");

      if (content) {
        // 检查当前高度
        // 如果 max-height 有值且不为 0px，说明是打开状态
        const isOpened =
          content.style.maxHeight && content.style.maxHeight !== "0px";

        // 手风琴模式：先关闭其他所有
        closeAllSubmenus();

        if (!isOpened) {
          // === 打开 ===
          // 关键：获取 scrollHeight (内容的真实高度)
          const height = content.scrollHeight;
          // 设置 max-height 为真实高度，触发 CSS transition
          content.style.maxHeight = height + "px";
          arrow?.classList.add("rotate-90");
          this.classList.add("text-yao-red"); // 高亮标题
        }
        // 如果是 else (即原本是打开的)，closeAllSubmenus 已经把它关掉了，所以不需要额外代码
      }
    });
  });

  // 辅助：关闭所有二级菜单
  function closeAllSubmenus() {
    document.querySelectorAll(".submenu-content").forEach((el) => {
      // 强制转换为 HTMLElement 才能访问 style
      (el as HTMLElement).style.maxHeight = "0px";
    });
    document.querySelectorAll(".arrow-icon").forEach((el) => {
      el.classList.remove("rotate-90");
    });
    document.querySelectorAll(".submenu-toggle").forEach((el) => {
      el.classList.remove("text-yao-red");
    });
  }
});
