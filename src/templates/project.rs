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
                    // Project 1
                    div.col-md-4.project-card {
                        div.card.project-card-view {
                            img.card-img-top src="/Assets/Projects/blog.png" alt="card-img";
                            div.card-body {
                                h5.card-title { "Project One" }
                                p.card-text style="text-align: justify;" {
                                    "A detailed description of your first project. Highlight the problem it solves, the technologies utilized, and key achievements."
                                }
                                a.btn.btn-primary href="https://github.com/yourusername/project-one" target="_blank" {
                                    i.fab.fa-github {} " GitHub"
                                }
                            }
                        }
                    }
                    
                    // Project 2
                    div.col-md-4.project-card {
                        div.card.project-card-view {
                            img.card-img-top src="/Assets/Projects/codeEditor.png" alt="card-img";
                            div.card-body {
                                h5.card-title { "Project Two" }
                                p.card-text style="text-align: justify;" {
                                    "A detailed description of your second project. Highlight the problem it solves, the technologies utilized, and key achievements."
                                }
                                a.btn.btn-primary href="https://github.com/yourusername/project-two" target="_blank" {
                                    i.fab.fa-github {} " GitHub"
                                }
                            }
                        }
                    }
                    
                    // Project 3
                    div.col-md-4.project-card {
                        div.card.project-card-view {
                            img.card-img-top src="/Assets/Projects/chatify.png" alt="card-img";
                            div.card-body {
                                h5.card-title { "Project Three" }
                                p.card-text style="text-align: justify;" {
                                    "A detailed description of your third project. Highlight the problem it solves, the technologies utilized, and key achievements."
                                }
                                a.btn.btn-primary href="https://github.com/yourusername/project-three" target="_blank" {
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
