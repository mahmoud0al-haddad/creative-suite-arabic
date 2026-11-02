(() => {
  'use strict';
  const fallback = { owner: 'mahmoud0al-haddad', repo: 'creative-suite-arabic', releaseTag: 'v2026.10-ar.1', releasePublished: false, setupFile: 'Creative-Suite-Arabic-Windows-x64-Setup.exe', upstreamOwner: 'storytold' };
  let config = { ...fallback };
  let language = 'ar';
  let configFailed = false;
  let activeScreenshot = 0;
  let availableScreenshots = [];
  const lightbox = document.getElementById('lightbox');
  const texts = {
    ar: { title: 'الحزمة الإبداعية العربية — 13 تطبيقًا، مشغّل واحد', draft: 'روابط النشر قيد التجهيز؛ سيظهر التنزيل عند نشر الإصدار.', published: 'Windows x64 · ملف Setup واحد · العربية وEnglish', error: 'تعذّر قراءة إعدادات الروابط. افتح هذه الصفحة عبر خادم محلي أو GitHub Pages.', download: 'تنزيل الحزمة لويندوز', release: 'عرض صفحة الإصدار' },
    en: { title: 'Arabic Creative Suite — 13 applications, one launcher', draft: 'Publishing links are being prepared. The download appears with the published release.', published: 'Windows x64 · One Setup · Arabic & English', error: 'Could not read link settings. Open this page through a local server or GitHub Pages.', download: 'Download for Windows', release: 'View the release' }
  };
  const githubName = /^[A-Za-z0-9_.-]+$/;
  function validGithubName(value) { return typeof value === 'string' && value.length > 0 && value.length <= 100 && value !== '.' && value !== '..' && githubName.test(value); }
  function safeHttps(value, allowedHosts) {
    if (typeof value !== 'string' || !value.trim()) return null;
    try {
      const url = new URL(value);
      return url.protocol === 'https:' && !url.username && !url.password && allowedHosts.includes(url.hostname) ? url.href : null;
    } catch { return null; }
  }
  function links() {
    const owner = validGithubName(config.owner) ? config.owner : fallback.owner;
    const repo = validGithubName(config.repo) ? config.repo : fallback.repo;
    const base = `https://github.com/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}`;
    const tag = typeof config.releaseTag === 'string' && config.releaseTag ? config.releaseTag : fallback.releaseTag;
    const release = `${base}/releases/tag/${encodeURIComponent(tag)}`;
    const customDownload = safeHttps(config.downloadUrl, ['github.com', 'objects.githubusercontent.com']);
    const asset = typeof config.setupFile === 'string' && config.setupFile ? `${base}/releases/download/${encodeURIComponent(tag)}/${encodeURIComponent(config.setupFile)}` : release;
    document.querySelectorAll('.repo-link').forEach(a => { a.href = safeHttps(config.sourceUrl || config.githubUrl, ['github.com']) || base; });
    document.querySelectorAll('.release-link').forEach(a => { a.href = release; });
    document.getElementById('download-link').href = config.releasePublished ? (customDownload || asset) : release;
    document.getElementById('release-version').textContent = tag;
    const upstreamOwner = validGithubName(config.upstreamOwner) ? config.upstreamOwner : fallback.upstreamOwner;
    document.querySelectorAll('.upstream-link').forEach(a => { a.href = `https://github.com/${encodeURIComponent(upstreamOwner)}/${encodeURIComponent(a.dataset.app)}`; });
  }
  function render() {
    document.documentElement.lang = language;
    document.documentElement.dir = language === 'ar' ? 'rtl' : 'ltr';
    document.title = texts[language].title;
    document.querySelectorAll('[data-ar][data-en]').forEach(node => { node.textContent = node.dataset[language]; });
    document.querySelectorAll('[data-label-ar][data-label-en]').forEach(node => { node.setAttribute('aria-label', language === 'ar' ? node.dataset.labelAr : node.dataset.labelEn); });
    const toggle = document.getElementById('language-toggle');
    toggle.replaceChildren(document.createTextNode(language === 'ar' ? 'English ↗' : 'العربية ↗'));
    toggle.lang = language === 'ar' ? 'en' : 'ar';
    toggle.setAttribute('aria-label', language === 'ar' ? 'Switch to English' : 'التبديل إلى العربية');
    document.getElementById('download-label').textContent = config.releasePublished ? texts[language].download : texts[language].release;
    const note = document.getElementById('release-note');
    note.textContent = configFailed ? texts[language].error : config.releasePublished ? texts[language].published : texts[language].draft;
    note.classList.toggle('config-error', configFailed);
    links();
    renderGallery();
  }
  function screenshotPath(path) {
    return typeof path === 'string' && /^assets\/screenshots\/[A-Za-z0-9_.-]+\.(png|jpg|jpeg|webp)$/i.test(path) && !path.includes('..') ? path : null;
  }
  function displayScreenshot(index) {
    if (!availableScreenshots.length) return;
    activeScreenshot = (index + availableScreenshots.length) % availableScreenshots.length;
    const shot = availableScreenshots[activeScreenshot];
    const image = document.getElementById('lightbox-image');
    image.src = shot.image;
    image.alt = language === 'ar' ? shot.altAr : shot.altEn;
    document.getElementById('lightbox-title').textContent = shot.title;
    document.getElementById('lightbox-caption').textContent = `${activeScreenshot + 1} / ${availableScreenshots.length} · ${language === 'ar' ? shot.descriptionAr : shot.descriptionEn}`;
    document.getElementById('lightbox-previous').disabled = availableScreenshots.length < 2;
    document.getElementById('lightbox-next').disabled = availableScreenshots.length < 2;
  }
  function renderGallery() {
    const grid = document.getElementById('gallery-grid');
    grid.replaceChildren();
    const configured = Array.isArray(config.screenshots) ? config.screenshots : [];
    availableScreenshots = configured.filter(shot => shot && typeof shot.title === 'string' && screenshotPath(shot.image));
    configured.forEach(shot => {
      if (!shot || typeof shot.title !== 'string') return;
      const card = document.createElement('figure'); card.className = 'gallery-card';
      const path = screenshotPath(shot.image);
      if (path) {
        const button = document.createElement('button'); button.type = 'button'; button.className = 'gallery-open';
        button.setAttribute('aria-label', language === 'ar' ? `تكبير صورة ${shot.title}` : `Enlarge ${shot.title} screenshot`);
        const image = document.createElement('img'); image.src = path; image.alt = language === 'ar' ? shot.altAr : shot.altEn; image.loading = 'lazy'; image.width = 1280; image.height = 800;
        button.append(image);
        button.addEventListener('click', () => { displayScreenshot(availableScreenshots.indexOf(shot)); lightbox.showModal(); document.getElementById('lightbox-close').focus(); });
        card.append(button);
      } else {
        const pending = document.createElement('div'); pending.className = 'gallery-pending';
        const mark = document.createElement('span'); mark.textContent = shot.title.substring(0, 2); mark.dir = 'ltr'; pending.append(mark);
        const label = document.createElement('small'); label.textContent = language === 'ar' ? 'لقطة قيد التجهيز' : 'Screenshot being prepared'; pending.append(label);card.append(pending);
      }
      const caption = document.createElement('figcaption'); const title = document.createElement('strong'); title.textContent = shot.title; title.dir = 'ltr'; caption.append(title);
      const description = document.createElement('span'); description.textContent = language === 'ar' ? shot.descriptionAr : shot.descriptionEn; caption.append(description); card.append(caption);grid.append(card);
    });
    document.getElementById('gallery-status').textContent = availableScreenshots.length === configured.length && configured.length ? '' : language === 'ar' ? 'تُضاف الصور بعد مراجعة الخصوصية والخطوط والأصول المعروضة.' : 'Screenshots are added after reviewing privacy, fonts and visible assets.';
    if (lightbox.open) displayScreenshot(activeScreenshot);
  }
  document.getElementById('lightbox-close').addEventListener('click', () => { lightbox.close(); });
  document.getElementById('lightbox-previous').addEventListener('click', () => { displayScreenshot(activeScreenshot - 1); });
  document.getElementById('lightbox-next').addEventListener('click', () => { displayScreenshot(activeScreenshot + 1); });
  lightbox.addEventListener('click', event => { if (event.target === lightbox) lightbox.close(); });
  lightbox.addEventListener('keydown', event => {
    if (event.key === 'Tab') {
      const buttons = [...lightbox.querySelectorAll('button:not(:disabled)')];
      const first = buttons[0];
      const last = buttons[buttons.length - 1];
      if (event.shiftKey && document.activeElement === first) { event.preventDefault(); last.focus(); }
      if (!event.shiftKey && document.activeElement === last) { event.preventDefault(); first.focus(); }
    }
    const direction = language === 'ar' ? -1 : 1;
    if (event.key === 'ArrowRight') { event.preventDefault(); displayScreenshot(activeScreenshot + direction); }
    if (event.key === 'ArrowLeft') { event.preventDefault(); displayScreenshot(activeScreenshot - direction); }
  });
  document.getElementById('language-toggle').addEventListener('click', () => {
    language = language === 'ar' ? 'en' : 'ar';
    try { localStorage.setItem('creative-suite-site-language', language); } catch { /* Optional persistence, no network tracking. */ }
    render();
  });
  try { const saved = localStorage.getItem('creative-suite-site-language'); if (saved === 'en' || saved === 'ar') language = saved; } catch { /* Storage may be unavailable. */ }
  fetch('data/config.json', { cache: 'no-cache' }).then(response => {
    if (!response.ok) throw new Error('Configuration unavailable');
    return response.json();
  }).then(value => {
    if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('Invalid configuration');
    config = { ...fallback, ...value, releasePublished: value.releasePublished === true };
    render();
  }).catch(() => { configFailed = true; render(); });
  render();
})();
