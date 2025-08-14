(function initDigitalRain() {
  const container = document.getElementById('digital-rain');
  if (!container) return;

  const canvas = document.createElement('canvas');
  const ctx = canvas.getContext('2d');
  container.appendChild(canvas);

  function resize() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
  }

  resize();
  window.addEventListener('resize', resize);

  const characters = 'アァカサタナハマヤャラワガザダバパイィキシチニヒミリヰギジヂビピウゥクスツヌフムユュルグズブプエェケセテネヘメレヱゲゼデベペオォコソトノホモヨョロゴゾドボポヴ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ';
  const fontSize = 16;
  let columns = Math.floor(canvas.width / fontSize);
  let drops = [];

  // 3D-like tilt settings (subtle, slow oscillation)
  const tiltMaxDeg = 3; // maximum tilt angle in degrees
  const tiltPeriodSec = 24; // full oscillation period in seconds
  const tiltMaxShear = Math.tan((tiltMaxDeg * Math.PI) / 180);

  // Time-based motion calibration to preserve current visual speed
  let lastTime = null;
  let rowsPerSecond = null; // will be calibrated from initial frame timing
  let calibFrames = 0;
  let calibTimeSum = 0;
  const CALIBRATION_FRAMES = 30; // ~0.5s at 60fps
  const RESET_RATE_PER_SEC = 1.5; // approx 2.5% per frame at 60fps

  function resetDrops() {
    columns = Math.floor(canvas.width / fontSize);
    drops = Array.from({ length: columns }, () => Math.random() * canvas.height / fontSize);
  }

  resetDrops();
  window.addEventListener('resize', resetDrops);

  let animationId = null;
  let running = false;

  function draw(now) {
    // Compute time delta
    if (lastTime === null) lastTime = now || performance.now();
    let dt = Math.max(0, ((now || performance.now()) - lastTime) / 1000);
    lastTime = now || performance.now();
    // Avoid huge jumps on tab switch
    if (dt > 0.1) dt = 0.1;

    // Calibrate rowsPerSecond to current perceived speed (1 row per frame baseline)
    if (rowsPerSecond == null) {
      calibTimeSum += dt;
      calibFrames += 1;
      if (calibFrames >= CALIBRATION_FRAMES) {
        const avgFrameTime = calibTimeSum / calibFrames; // seconds per frame
        const measuredFps = avgFrameTime > 0 ? (1 / avgFrameTime) : 60;
        rowsPerSecond = measuredFps; // match old 1 row per frame visual speed
      }
    }
    const effectiveRowsPerSecond = rowsPerSecond || 60; // fallback during calibration

    // Theme-aware faintness using time-based decay to keep trail length constant
    const themeAttr = document.documentElement.getAttribute('data-theme');
    const prefersDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
    const isDark = (themeAttr ? themeAttr === 'dark' : prefersDark);
    // Choose decay rates so that at 60fps they approximate prior fadeAlpha of 0.10 (dark) / 0.16 (light)
    const decayPerSec = isDark ? (-60 * Math.log(0.9)) : (-60 * Math.log(0.84));
    const fadeAlpha = 1 - Math.exp(-decayPerSec * dt);
    const glyphAlpha = isDark ? 0.50 : 0.32; // dimmer in light mode

    // Fade frame
    ctx.fillStyle = 'rgba(0, 0, 0, ' + fadeAlpha.toFixed(4) + ')';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Compute current tilt shear factor based on time
    const t = (now || performance.now()) / 1000; // seconds
    const shear = tiltMaxShear * Math.sin((2 * Math.PI * t) / tiltPeriodSec);

    ctx.fillStyle = 'rgba(0, 255, 65, ' + glyphAlpha + ')';
    ctx.font = fontSize + 'px "JetBrains Mono", monospace';

    const centerX = canvas.width / 2;
    for (let i = 0; i < drops.length; i++) {
      const text = characters.charAt(Math.floor(Math.random() * characters.length));
      const xBase = i * fontSize;
      const y = drops[i] * fontSize;
      const dist = (xBase - centerX) / centerX; // -1 (left) .. 1 (right)
      const x = xBase + shear * y * dist; // tilt around center axis
      ctx.fillText(text, x, y);

      if (y > canvas.height) {
        // Time-based reset probability
        const p = 1 - Math.exp(-RESET_RATE_PER_SEC * dt);
        if (Math.random() < p) drops[i] = 0;
      }
      drops[i] += effectiveRowsPerSecond * dt;
    }

    if (running) {
      animationId = requestAnimationFrame(draw);
    } else {
      animationId = null;
    }
  }

  // Ensure canvas sits behind content
  Object.assign(container.style, {
    position: 'fixed',
    left: '0',
    top: '0',
    width: '100vw',
    height: '100vh',
    pointerEvents: 'none',
    zIndex: '0'
  });
  Object.assign(canvas.style, { display: 'block', width: '100%', height: '100%' });

  function start() {
    if (running) return;
    running = true;
    // reset timing to avoid jump on resume
    lastTime = null;
    animationId = requestAnimationFrame(draw);
  }

  function stop() {
    running = false;
    if (animationId) {
      cancelAnimationFrame(animationId);
      animationId = null;
    }
    // Clear background
    ctx.clearRect(0, 0, canvas.width, canvas.height);
  }

  // Expose controls
  window.matrixRain = {
    start,
    stop,
    isRunning: function() { return running; }
  };

  // Autostart
  start();
})();
