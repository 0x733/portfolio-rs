use maud::{html, Markup};

pub fn view() -> Markup {
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
                                strong.main-name { "YOUR NAME" }
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
                                "I am a software engineer specializing in low-level systems programming, fullstack development, and application engineering. My technical focus lies in building scalable, secure, and modern digital experiences."
                                br;
                                br;
                                "I am highly proficient in systems-level and web development technologies, including "
                                i {
                                    b.purple { "Rust, TypeScript, Python, and Go." }
                                }
                                br;
                                br;
                                "My primary areas of expertise involve designing secure system utilities, cloud-native backend systems, and lightweight user interfaces that prioritize speed and privacy."
                                br;
                                br;
                                "Whenever possible, I build tools that automate system management, optimize configurations, and enhance overall developer workflows."
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
                                a.icon-colour.home-social-icons href="https://github.com/yourusername" target="_blank" rel="noreferrer" {
                                    i.fab.fa-github {}
                                }
                            }
                            li.social-icons {
                                a.icon-colour.home-social-icons href="https://x.com/yourusername" target="_blank" rel="noreferrer" {
                                    i.fab.fa-twitter {}
                                }
                            }
                            li.social-icons {
                                a.icon-colour.home-social-icons href="https://linkedin.com/in/yourusername" target="_blank" rel="noreferrer" {
                                    i.fab.fa-linkedin-in {}
                                }
                            }
                            li.social-icons {
                                a.icon-colour.home-social-icons href="https://www.instagram.com/yourusername" target="_blank" rel="noreferrer" {
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
