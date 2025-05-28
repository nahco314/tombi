// テーマの初期化
function initTheme() {
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme) {
    document.documentElement.classList.add(savedTheme);
  } else {
    // システムの設定を確認
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    document.documentElement.classList.add(prefersDark ? 'dark-mode' : 'light-mode');
  }
}

// テーマの切り替え
function toggleTheme() {
  const isDark = document.documentElement.classList.contains('dark-mode');
  const newTheme = isDark ? 'light-mode' : 'dark-mode';

  document.documentElement.classList.remove('dark-mode', 'light-mode');
  document.documentElement.classList.add(newTheme);
  localStorage.setItem('theme', newTheme);
}

// 初期化
document.addEventListener('DOMContentLoaded', initTheme);

// テーマ切り替えボタンのイベントリスナーを設定
document.addEventListener('DOMContentLoaded', () => {
  const themeToggle = document.getElementById('theme-toggle');
  if (themeToggle) {
    themeToggle.addEventListener('click', toggleTheme);
  }
});
