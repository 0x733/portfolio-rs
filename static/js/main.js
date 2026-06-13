// Plain vanilla JS for frontend transitions and animations
// Zero dependencies, ultra-lightweight

window.addEventListener('DOMContentLoaded', () => {
  // 1. Preloader fade out
  const preloader = document.getElementById('preloader');
  if (preloader) {
    // Check if page is already loaded
    const fadeOut = () => {
      preloader.style.transition = 'opacity 0.4s ease';
      preloader.style.opacity = '0';
      setTimeout(() => {
        preloader.style.display = 'none';
      }, 400);
    };
    if (document.readyState === 'complete') {
      setTimeout(fadeOut, 500);
    } else {
      window.addEventListener('load', () => {
        setTimeout(fadeOut, 500);
      });
    }
  }

  // 2. Sticky Navbar scroll listener
  const nav = document.querySelector('.navbar');
  if (nav) {
    const checkScroll = () => {
      if (window.scrollY >= 20) {
        nav.classList.add('sticky');
      } else {
        nav.classList.remove('sticky');
      }
    };
    window.addEventListener('scroll', checkScroll);
    checkScroll(); // Initial check
  }

  // 3. Mobile Navbar collapse toggle
  const toggler = document.querySelector('.navbar-toggler');
  const navCollapse = document.getElementById('responsive-navbar-nav');
  if (toggler && navCollapse) {
    toggler.addEventListener('click', () => {
      const isExpanded = navCollapse.classList.contains('show');
      if (isExpanded) {
        navCollapse.classList.remove('show');
        toggler.setAttribute('aria-expanded', 'false');
        toggler.classList.add('collapsed');
      } else {
        navCollapse.classList.add('show');
        toggler.setAttribute('aria-expanded', 'true');
        toggler.classList.remove('collapsed');
      }
    });
  }

  // 4. Typewriter Effect
  const typewriter = document.getElementById('typewriter');
  if (typewriter) {
    const strings = [
      "Systems & Security Engineer",
      "Android App Developer",
      "Open Source Contributor",
      "Linux Automation Specialist"
    ];
    let strIdx = 0;
    let charIdx = 0;
    let isDeleting = false;
    
    function tick() {
      const currentText = strings[strIdx];
      let nextTimeout = 60; // typing speed
      
      if (isDeleting) {
        charIdx--;
        const slice = currentText.substring(0, charIdx);
        typewriter.innerHTML = `${slice}<span class="Typewriter__cursor">|</span>`;
        nextTimeout = 30; // deleting is faster
        if (charIdx === 0) {
          isDeleting = false;
          strIdx = (strIdx + 1) % strings.length;
          nextTimeout = 500;
        }
      } else {
        charIdx++;
        const slice = currentText.substring(0, charIdx);
        typewriter.innerHTML = `${slice}<span class="Typewriter__cursor">|</span>`;
        if (charIdx === currentText.length) {
          isDeleting = true;
          nextTimeout = 1500; // pause at full text
        }
      }
      setTimeout(tick, nextTimeout);
    }
    setTimeout(tick, 100);
  }

  // 5. Canvas Particle System
  const canvas = document.getElementById('particle-canvas');
  if (canvas) {
    const ctx = canvas.getContext('2d');
    if (ctx) {
      function resizeCanvas() {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
      }
      resizeCanvas();
      window.addEventListener('resize', resizeCanvas);
      
      const particles = [];
      const numParticles = 100;
      
      class Particle {
        constructor() {
          this.reset(true);
        }
        
        reset(initial = false) {
          this.x = initial ? Math.random() * canvas.width : 0;
          this.y = Math.random() * canvas.height;
          this.vx = 0.1 + Math.random() * 0.4;
          this.vy = (Math.random() - 0.5) * 0.1;
          this.radius = 0.5 + Math.random() * 1.5;
          this.opacity = 0.1 + Math.random() * 0.4;
          this.opacitySpeed = (0.2 + Math.random() * 0.8) * 0.005;
        }
        
        update() {
          this.x += this.vx;
          this.y += this.vy;
          
          this.opacity += this.opacitySpeed;
          if (this.opacity <= 0.05 || this.opacity >= 0.5) {
            this.opacitySpeed = -this.opacitySpeed;
          }
          
          if (this.x > canvas.width || this.y < 0 || this.y > canvas.height) {
            this.reset();
          }
        }
        
        draw() {
          ctx.beginPath();
          ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2);
          ctx.fillStyle = `rgba(200, 137, 230, ${this.opacity})`;
          ctx.fill();
        }
      }
      
      for (let i = 0; i < numParticles; i++) {
        particles.push(new Particle());
      }
      
      function animate() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        for (let i = 0; i < particles.length; i++) {
          particles[i].update();
          particles[i].draw();
        }
        requestAnimationFrame(animate);
      }
      animate();
    }
  }
});
