use maud::{html, Markup};

pub fn view() -> Markup {
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
