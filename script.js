let clickCount = 0;

const ctaBtn = document.getElementById('cta-btn');
const counterDisplay = document.getElementById('counter-display');

ctaBtn.addEventListener('click', () => {
  clickCount++;
  counterDisplay.textContent = `按钮已被点击 ${clickCount} 次`;

  ctaBtn.style.transform = 'scale(0.95)';
  setTimeout(() => {
    ctaBtn.style.transform = '';
  }, 150);
});

const contactForm = document.getElementById('contact-form');
contactForm.addEventListener('submit', (e) => {
  e.preventDefault();
  alert('消息已发送！（演示项目，实际不会发送）');
  contactForm.reset();
});

document.querySelectorAll('nav a').forEach((link) => {
  link.addEventListener('click', (e) => {
    e.preventDefault();
    const targetId = link.getAttribute('href').substring(1);
    const targetElement = document.getElementById(targetId);
    if (targetElement) {
      targetElement.scrollIntoView({
        behavior: 'smooth',
        block: 'start',
      });
    }
  });
});
