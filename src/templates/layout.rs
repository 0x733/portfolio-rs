use maud::{html, Markup, DOCTYPE, PreEscaped};

pub fn navbar(current_route: &str) -> Markup {
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

pub fn footer() -> Markup {
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

pub fn render(current_route: &str, title: &str, content: Markup) -> Markup {
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
                
                // Custom CSS
                link rel="stylesheet" href="/css/index.css";
                link rel="stylesheet" href="/css/style.css?v=2.1.1";
                link rel="stylesheet" href="/css/App.css";
            }
            body {
                div id="preloader" {}
                
                div.App id="scroll" {
                    (navbar(current_route))
                    
                    canvas id="particle-canvas" style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; z-index: -1; pointer-events: none;" {}
                    
                    main {
                        (content)
                    }
                    
                    (footer())
                }
                
                // WASM Instantiation (Loads frontend compiled from Rust/WASM)
                script type="module" {
                    (PreEscaped(r#"
                        import init from "/js/wasm/portfolio_frontend.js";
                        init();
                    "#))
                }
            }
        }
    }
}
