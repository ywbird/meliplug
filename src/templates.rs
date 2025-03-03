use maud::{ html, PreEscaped, DOCTYPE };

pub struct PageGen {
    title: String,
    content: PreEscaped<String>,
    head: PreEscaped<String>,
    header: PreEscaped<String>,
    footer: PreEscaped<String>,
}

impl PageGen {
    pub fn new(title: String, content: PreEscaped<String>) -> Self {
        Self {
            title: title.clone(),
            content,
            head: html! { head {
		title { (title.clone()) }
            } },
            header: html! { header {
		nav {
                ul {
		    li {
			a href={"/"} { "Home" }
		    }
		    li {
			a href={"/about"} { "About" }
		    }
                }
	    } } },
            footer: html! { footer { } },
        }
    }

    pub fn render(&self) -> String {
        let markup = html! {
            (DOCTYPE)
            html {
		(self.head)
                body {
                    (self.header)
                    (self.content)
                    (self.footer)
                }
            }
        };

        markup.into_string()
    }
}
