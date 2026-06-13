use axum::{
    routing::get,
    Router,
};
use maud::{html, Markup, DOCTYPE, PreEscaped};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

fn navbar(current_route: &str) -> Markup {
    html! {
        nav.navbar.navbar-expand-md.fixed-top.navbar-dark {
            div.container {
                a.navbar-brand.d-flex href="/" {
                    span.purple style="font-weight: 800; font-size: 1.5em; letter-spacing: 1px;" { "HN" }
                }
                button.navbar-toggler type="button" data-bs-toggle="collapse" data-bs-target="#responsive-navbar-nav" aria-controls="responsive-navbar-nav" aria-expanded="false" aria-label="Toggle navigation" {
                    span {}
                    span {}
                    span {}
                }
                div.collapse.navbar-collapse id="responsive-navbar-nav" {
                    ul.navbar-nav.ms-auto {
                        li.nav-item {
                            a.nav-link.active[current_route == "/"] href="/" {
                                i.fa.fa-home style="margin-right: 4px;" {} " Home"
                            }
                        }
                        li.nav-item {
                            a.nav-link.active[current_route == "/about"] href="/about" {
                                i.fa.fa-user style="margin-right: 4px;" {} " About"
                            }
                        }
                        li.nav-item {
                            a.nav-link.active[current_route == "/project"] href="/project" {
                                i.fa.fa-code-branch style="margin-right: 4px;" {} " Projects"
                            }
                        }
                        li.nav-item.fork-btn {
                            a.fork-btn-inner.btn.btn-primary href="https://github.com/0x733/portfolio" target="_blank" {
                                i.fa-solid.fa-code-fork style="font-size: 1.2em; margin-right: 4px;" {}
                                i.fa-solid.fa-star style="font-size: 1.1em;" {}
                            }
                        }
                    }
                }
            }
        }
    }
}

fn footer() -> Markup {
    html! {
        footer.footer {
            div.container {
                div.row {
                    div.col-md-4.footer-copywright {
                        h3 { "Designed and Developed by Hüseyin" }
                    }
                    div.col-md-4.footer-copywright {
                        h3 { "Copyright © 2026 HN" }
                    }
                    div.col-md-4.footer-body {
                        ul.footer-icons {
                            li.social-icons {
                                a href="https://github.com/0x733" target="_blank" rel="noopener noreferrer" style="color: white;" {
                                    i.fab.fa-github {}
                                }
                            }
                            li.social-icons {
                                a href="https://x.com/root_huseyin" target="_blank" rel="noopener noreferrer" style="color: white;" {
                                    i.fab.fa-twitter {}
                                }
                            }
                            li.social-icons {
                                a href="https://open.spotify.com/user/31rvan75oheli5saqslq7jeyo5ty" target="_blank" rel="noopener noreferrer" style="color: white;" {
                                    i.fab.fa-spotify {}
                                }
                            }
                            li.social-icons {
                                a href="https://www.instagram.com/hsyknx/?hl=tr" target="_blank" rel="noopener noreferrer" style="color: white;" {
                                    i.fab.fa-instagram {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn layout(current_route: &str, title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="theme-color" content="#000000";
                meta name="description" content="Hüseyin — Developer, Designer, Creator";
                link rel="icon" href="/favicon.png";
                title { (title) }
                
                // CDNs
                link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet";
                link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.2/css/all.min.css" rel="stylesheet";
                link href="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/devicon.min.css" rel="stylesheet";
                
                // GitHub Calendar style
                link rel="stylesheet" href="https://unpkg.com/github-calendar@latest/dist/github-calendar-responsive.css";
                
                // Custom CSS
                link rel="stylesheet" href="/css/style.css";
                link rel="stylesheet" href="/css/App.css";
            }
            body {
                div id="preloader" {}
                
                div.App id="scroll" {
                    (navbar(current_route))
                    
                    div id="tsparticles" {}
                    
                    main {
                        (content)
                    }
                    
                    (footer())
                }
                
                // JS Scripts
                script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js" {}
                script src="https://cdn.jsdelivr.net/npm/tsparticles@2.12.0/tsparticles.bundle.min.js" {}
                script src="https://unpkg.com/typed.js@2.1.0/dist/typed.umd.js" {}
                script src="https://unpkg.com/github-calendar@latest/dist/github-calendar.min.js" {}
                
                // Preloader & Particles & Navbar scroll script
                script {
                    (PreEscaped(r#"
                        // Preloader
                        window.addEventListener('load', () => {
                            setTimeout(() => {
                                const pre = document.getElementById('preloader');
                                if (pre) {
                                    pre.style.transition = 'opacity 0.4s ease';
                                    pre.style.opacity = '0';
                                    setTimeout(() => {
                                        pre.style.display = 'none';
                                    }, 400);
                                }
                            }, 500);
                        });

                        // Navbar scroll color handler
                        window.addEventListener('scroll', () => {
                            const nav = document.querySelector('.navbar');
                            if (window.scrollY >= 20) {
                                nav.classList.add('sticky');
                            } else {
                                nav.classList.remove('sticky');
                            }
                        });

                        // tsParticles initialization
                        tsParticles.load("tsparticles", {
                            particles: {
                                number: {
                                    value: 160,
                                    density: {
                                        enable: true,
                                        value_area: 1500
                                    }
                                },
                                links: {
                                    enable: false,
                                    opacity: 0.03
                                },
                                move: {
                                    enable: true,
                                    direction: "right",
                                    speed: 0.5,
                                    random: false,
                                    straight: false
                                },
                                size: {
                                    value: 1
                                },
                                opacity: {
                                    value: { min: 0.05, max: 0.5 },
                                    animation: {
                                        enable: true,
                                        speed: 1,
                                        minimumValue: 0.05
                                    }
                                }
                            },
                            interactivity: {
                                events: {
                                    onClick: {
                                        enable: true,
                                        mode: "push"
                                    }
                                },
                                modes: {
                                    push: {
                                        quantity: 1
                                    }
                                }
                            },
                            detectRetina: true
                        });
                    "#))
                }
            }
        }
    }
}

fn home_view() -> Markup {
    html! {
        section {
            div.container-fluid.home-section id="home" {
                div.container.home-content {
                    div.row {
                        div.col-md-7.home-header {
                            h1.heading style="padding-bottom: 15px;" {
                                "Hi There! "
                                span.wave role="img" aria-labelledby="wave" { "👋🏻" }
                            }
                            h1.heading-name {
                                "I'M "
                                strong.main-name { "HÜSEYİN" }
                            }
                            div style="padding: 50px; text-align: left;" {
                                span id="typewriter" {}
                            }
                        }
                        div.col-md-5 style="padding-bottom: 20px;" {
                            img.img-fluid src="/Assets/home-main.svg" alt="home pic" style="max-height: 450px;";
                        }
                    }
                }
            }
            
            // Home2 Intro section
            div.container-fluid.home-about-section id="about" {
                div.container {
                    div.row {
                        div.col-md-8.home-about-description {
                            h1 style="font-size: 2.6em;" {
                                "LET ME "
                                span.purple { "INTRODUCE" }
                                " MYSELF"
                            }
                            p.home-about-body {
                                "I am a software engineer specializing in low-level systems programming, security daemon development, and native mobile applications. My technical focus lies at the intersection of OS-level security, systems optimization, and building local-first architectures."
                                br;
                                br;
                                "I am highly proficient in systems-level languages and application development frameworks, including "
                                i {
                                    b.purple { "Rust, Kotlin, Python, C++, and Dart." }
                                }
                                br;
                                br;
                                "My primary areas of expertise involve designing secure system utilities, automated auditing engines, and lightweight mobile applications that prioritize privacy and offline performance."
                                br;
                                br;
                                "Whenever possible, I build tools that automate system management, optimize Linux environments, and enhance overall device control."
                            }
                        }
                        div.col-md-4.myAvtar {
                            div.tilt-container {
                                img.img-fluid src="/Assets/avatar.svg" alt="avatar";
                            }
                        }
                    }
                }
            }
            
            // Social Links section
            div.container {
                div.row style="padding-top: 50px; padding-bottom: 80px;" {
                    div.col-md-12.home-about-social {
                        h1 { "FIND ME ON" }
                        p {
                            "Feel free to "
                            span.purple { "connect " }
                            "with me"
                        }
                        ul.home-about-social-links {
                            li.social-icons {
                                a.icon-colour.home-social-icons href="https://github.com/0x733" target="_blank" rel="noreferrer" {
                                    i.fab.fa-github {}
                                }
                            }
                            li.social-icons {
                                a.icon-colour.home-social-icons href="https://x.com/root_huseyin" target="_blank" rel="noreferrer" {
                                    i.fab.fa-twitter {}
                                }
                            }
                            li.social-icons {
                                a.icon-colour.home-social-icons href="https://open.spotify.com/user/31rvan75oheli5saqslq7jeyo5ty" target="_blank" rel="noreferrer" {
                                    i.fab.fa-spotify {}
                                }
                            }
                            li.social-icons {
                                a.icon-colour.home-social-icons href="https://www.instagram.com/hsyknx/?hl=tr" target="_blank" rel="noreferrer" {
                                    i.fab.fa-instagram {}
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Typed.js initialization script for home
        script {
            (PreEscaped(r#"
                new Typed('#typewriter', {
                    strings: [
                        "Systems & Security Engineer",
                        "Android App Developer",
                        "Open Source Contributor",
                        "Linux Automation Specialist"
                    ],
                    typeSpeed: 60,
                    backSpeed: 30,
                    loop: true,
                    backDelay: 1500
                });
            "#))
        }
    }
}

fn about_view() -> Markup {
    html! {
        div.container-fluid.about-section {
            div.container {
                div.row style="justify-content: center; padding: 10px;" {
                    div.col-md-7 style="justify-content: center; padding-top: 30px; padding-bottom: 50px;" {
                        h1 style="font-size: 2.1em; padding-bottom: 20px;" {
                            "Know Who "
                            strong.purple { "I'M" }
                        }
                        // About Card content
                        div.card.quote-card-view {
                            div.card-body {
                                blockquote.blockquote.mb-0 {
                                    p style="text-align: justify;" {
                                        "Hello, I am "
                                        span.purple { "Hüseyin" }
                                        ", based in "
                                        span.purple { "Ankara, Turkey" }
                                        "."
                                        br;
                                        "I work as a "
                                        span.purple { "Systems and Mobile Software Engineer" }
                                        "."
                                        br;
                                        br;
                                        "In addition to software development, I dedicate time to areas that enhance my engineering expertise:"
                                    }
                                    ul {
                                        li.about-activity {
                                            i.fa-solid.fa-angles-right.purple style="margin-right: 8px;" {}
                                            " Customizing desktop environments and Linux kernel configurations"
                                        }
                                        li.about-activity {
                                            i.fa-solid.fa-angles-right.purple style="margin-right: 8px;" {}
                                            " Researching systems security, network protocols, and reverse engineering"
                                        }
                                        li.about-activity {
                                            i.fa-solid.fa-angles-right.purple style="margin-right: 8px;" {}
                                            " Designing local-first database solutions and offline-first mobile synchronization"
                                        }
                                    }
                                    p style="color: rgb(155 126 172);" {
                                        "\"Designing efficient systems is the art of making complexity invisible.\""
                                    }
                                    footer.blockquote-footer { "Hüseyin" }
                                }
                            }
                        }
                    }
                    div.col-md-5.about-img style="padding-top: 120px; padding-bottom: 50px;" {
                        img.img-fluid src="/Assets/about.png" alt="about";
                    }
                }
                
                // Professional Skillset
                h1.project-heading {
                    "Professional "
                    strong.purple { "Skillset" }
                }
                
                div.row style="justify-content: center; padding-bottom: 50px;" {
                    div.col-6.col-md-2.tech-icons {
                        i.devicon-cplusplus-line {}
                        div.tech-icons-text { "C++" }
                    }
                    div.col-6.col-md-2.tech-icons {
                        i.devicon-rust-plain {}
                        div.tech-icons-text { "Rust" }
                    }
                    div.col-6.col-md-2.tech-icons {
                        i.devicon-kotlin-plain {}
                        div.tech-icons-text { "Kotlin" }
                    }
                    div.col-6.col-md-2.tech-icons {
                        i.devicon-flutter-plain {}
                        div.tech-icons-text { "Dart (Flutter)" }
                    }
                    div.col-6.col-md-2.tech-icons {
                        i.devicon-python-plain {}
                        div.tech-icons-text { "Python" }
                    }
                    div.col-6.col-md-2.tech-icons {
                        i.devicon-java-plain {}
                        div.tech-icons-text { "Java" }
                    }
                    div.col-6.col-md-2.tech-icons {
                        i.devicon-typescript-plain {}
                        div.tech-icons-text { "TypeScript" }
                    }
                    div.col-6.col-md-2.tech-icons {
                        i.devicon-javascript-plain {}
                        div.tech-icons-text { "JavaScript" }
                    }
                    div.col-6.col-md-2.tech-icons {
                        i.devicon-bash-plain {}
                        div.tech-icons-text { "Bash / Shell" }
                    }
                }
                
                // Tools I use
                h1.project-heading {
                    strong.purple { "Tools" }
                    " I use"
                }
                
                div.row style="justify-content: center; padding-bottom: 50px;" {
                    div.col-6.col-md-2.tech-icons {
                        i.devicon-linux-plain {}
                        div.tech-icons-text { "Linux" }
                    }
                    div.col-6.col-md-2.tech-icons {
                        i.devicon-archlinux-plain {}
                        div.tech-icons-text { "Arch Linux" }
                    }
                    div.col-6.col-md-2.tech-icons {
                        i.devicon-vscode-plain {}
                        div.tech-icons-text { "VS Code" }
                    }
                    div.col-6.col-md-2.tech-icons {
                        i.devicon-androidstudio-plain {}
                        div.tech-icons-text { "Android Studio" }
                    }
                    div.col-6.col-md-2.tech-icons {
                        i.devicon-neovim-plain {}
                        div.tech-icons-text { "Neovim" }
                    }
                    div.col-6.col-md-2.tech-icons {
                        i.devicon-git-plain {}
                        div.tech-icons-text { "Git" }
                    }
                    div.col-6.col-md-2.tech-icons {
                        i.devicon-docker-plain {}
                        div.tech-icons-text { "Docker" }
                    }
                }
                
                // GitHub Calendar
                div.row style="justify-content: center; padding-bottom: 10px; color: white;" {
                    h1.project-heading style="padding-bottom: 20px;" {
                        "Days I "
                        strong.purple { "Code" }
                    }
                    div.calendar {}
                }
            }
        }
        
        // GitHub Calendar script
        script {
            (PreEscaped(r#"
                GitHubCalendar(".calendar", "0x733", { responsive: true, tooltips: true });
            "#))
        }
    }
}

fn projects_view() -> Markup {
    html! {
        div.container-fluid.project-section {
            div.container {
                h1.project-heading {
                    "My Recent "
                    strong.purple { "Works" }
                }
                p style="color: white;" {
                    "Here are a few projects I've worked on recently."
                }
                div.row style="justify-content: center; padding-bottom: 10px;" {
                    // Project 1: Vardi
                    div.col-md-4.project-card {
                        div.card.project-card-view {
                            img.card-img-top src="/Assets/Projects/vardi.png" alt="card-img";
                            div.card-body {
                                h5.card-title { "Vardi" }
                                p.card-text style="text-align: justify;" {
                                    "A free and open-source (FOSS), local-first Android application designed to track cargo carriers and shipments across Turkey efficiently with absolute privacy."
                                }
                                a.btn.btn-primary href="https://github.com/0x733/vardi" target="_blank" {
                                    i.fab.fa-github {} " GitHub"
                                }
                            }
                        }
                    }
                    
                    // Project 2: Sysguard
                    div.col-md-4.project-card {
                        div.card.project-card-view {
                            img.card-img-top src="/Assets/Projects/sysguard.png" alt="card-img";
                            div.card-body {
                                h5.card-title { "Sysguard" }
                                p.card-text style="text-align: justify;" {
                                    "A lightweight system monitoring and resource protection daemon written in Rust, providing automated process auditing and telemetry tracking on Linux."
                                }
                                a.btn.btn-primary href="https://github.com/0x733/sysguard" target="_blank" {
                                    i.fab.fa-github {} " GitHub"
                                }
                            }
                        }
                    }
                    
                    // Project 3: NextDNS-Control
                    div.col-md-4.project-card {
                        div.card.project-card-view {
                            img.card-img-top src="/Assets/Projects/nextdns_control.png" alt="card-img";
                            div.card-body {
                                h5.card-title { "NextDNS-Control" }
                                p.card-text style="text-align: justify;" {
                                    "A native Android application written in Kotlin to easily control and configure NextDNS filtering profiles, domain rules, and queries on the fly."
                                }
                                a.btn.btn-primary href="https://github.com/0x733/NextDNS-Control" target="_blank" {
                                    i.fab.fa-github {} " GitHub"
                                }
                            }
                        }
                    }
                    
                    // Project 4: Jarvis Linux
                    div.col-md-4.project-card {
                        div.card.project-card-view {
                            img.card-img-top src="/Assets/Projects/jarvis_linux.png" alt="card-img";
                            div.card-body {
                                h5.card-title { "Jarvis Linux" }
                                p.card-text style="text-align: justify;" {
                                    "An intelligent Python-based command-line companion and automation utility designed for streamline developer setups and Linux desktop control."
                                }
                                a.btn.btn-primary href="https://github.com/0x733/jarvis-linux" target="_blank" {
                                    i.fab.fa-github {} " GitHub"
                                }
                            }
                        }
                    }
                    
                    // Project 5: Finova Expense
                    div.col-md-4.project-card {
                        div.card.project-card-view {
                            img.card-img-top src="/Assets/Projects/finova_expense.png" alt="card-img";
                            div.card-body {
                                h5.card-title { "Finova Expense" }
                                p.card-text style="text-align: justify;" {
                                    "A modern, cross-platform personal finance and expense management application built with Flutter/Dart utilizing local-first storage design."
                                }
                                a.btn.btn-primary href="https://github.com/0x733/finova_expense" target="_blank" {
                                    i.fab.fa-github {} " GitHub"
                                }
                            }
                        }
                    }
                    
                    // Project 6: Auditguard
                    div.col-md-4.project-card {
                        div.card.project-card-view {
                            img.card-img-top src="/Assets/Projects/auditguard.png" alt="card-img";
                            div.card-body {
                                h5.card-title { "Auditguard" }
                                p.card-text style="text-align: justify;" {
                                    "A security monitoring daemon developed in Rust that performs automated kernel syscall analysis and reporting on Linux systems."
                                }
                                a.btn.btn-primary href="https://github.com/0x733/auditguard" target="_blank" {
                                    i.fab.fa-github {} " GitHub"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn home_handler() -> Markup {
    layout("/", "Hüseyin | Portfolio", home_view())
}

async fn about_handler() -> Markup {
    layout("/about", "About | Hüseyin", about_view())
}

async fn project_handler() -> Markup {
    layout("/project", "Projects | Hüseyin", projects_view())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home_handler))
        .route("/about", get(about_handler))
        .route("/project", get(project_handler))
        .fallback_service(ServeDir::new("static"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
