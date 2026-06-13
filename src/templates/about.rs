use maud::{html, Markup};

pub fn view() -> Markup {
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
                                        span.purple { "Your Name" }
                                        ", based in "
                                        span.purple { "Your City, Country" }
                                        "."
                                        br;
                                        "I work as a "
                                        span.purple { "Full Stack & Systems Developer" }
                                        "."
                                        br;
                                        br;
                                        "In addition to software development, I dedicate time to areas that enhance my engineering expertise:"
                                    }
                                    ul {
                                        li.about-activity {
                                            i.fa-solid.fa-angles-right.purple style="margin-right: 8px;" {}
                                            " Building high-performance backend APIs and microservices"
                                        }
                                        li.about-activity {
                                            i.fa-solid.fa-angles-right.purple style="margin-right: 8px;" {}
                                            " Researching web security, database design, and DevOps automation"
                                        }
                                        li.about-activity {
                                            i.fa-solid.fa-angles-right.purple style="margin-right: 8px;" {}
                                            " Designing responsive client interfaces and modular frontend architectures"
                                        }
                                    }
                                    p style="color: rgb(155 126 172);" {
                                        "\"Designing efficient systems is the art of making complexity invisible.\""
                                    }
                                    footer.blockquote-footer { "Your Name" }
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
                
            }
        }
    }
}
